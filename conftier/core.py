import importlib.util
import inspect
import os
from dataclasses import MISSING, _create_fn, asdict, fields, is_dataclass
from pathlib import Path
from typing import (
    Any,
    Callable,
    Dict,
    Generic,
    List,
    Literal,
    Optional,
    Tuple,
    Type,
    TypeVar,
    Union,
    cast,
)

import yaml

from conftier.utils.logger import logger

try:
    from pydantic import BaseModel

    PYDANTIC_AVAILABLE = True
except ImportError:
    PYDANTIC_AVAILABLE = False
    BaseModel = object  # type: ignore


T = TypeVar("T")
PydanticT = TypeVar("PydanticT", bound="BaseModel")
DataclassT = TypeVar("DataclassT")
DictConfigT = Dict[str, Any]

# Constants for schema types
SCHEMA_TYPE_PYDANTIC = "pydantic"
SCHEMA_TYPE_DATACLASS = "dataclass"
SCHEMA_TYPE_DICT = "dict"
SchemaType = Literal[SCHEMA_TYPE_PYDANTIC, SCHEMA_TYPE_DATACLASS, SCHEMA_TYPE_DICT]


class ConfigModel:
    """
    A unified configuration model that wraps Pydantic models, dataclasses, and
    dictionaries.

    This class provides a consistent interface for different types of configuration
    models, handling validation, serialization, and nested structure management.
    """

    def __init__(self, schema_type: SchemaType, model_instance: Any):
        """
        Initialize a ConfigModel

        Args:
            schema_type: Type of schema ('pydantic', 'dataclass', or 'dict')
            model_instance: The actual model instance
        """
        self.schema_type: SchemaType = schema_type
        self._model = model_instance

    @classmethod
    def from_schema(
        cls, schema: Type[Any], data: Optional[Dict[str, Any]] = None
    ) -> "ConfigModel":
        """
        Create a ConfigModel from a schema type and data

        Args:
            schema: The schema type (Pydantic model, dataclass, or dict)
            data: Optional data to initialize the model with

        Returns:
            Initialized ConfigModel
        """
        # Determine schema type
        schema_type: SchemaType
        instance: Any

        if (
            PYDANTIC_AVAILABLE
            and isinstance(schema, type)
            and issubclass(schema, BaseModel)
        ):
            schema_type = SCHEMA_TYPE_PYDANTIC
            if data:
                try:
                    instance = schema(**data)
                except Exception as e:
                    logger.error(f"Warning: Failed to create Pydantic model: {e}")
                    instance = schema()
            else:
                instance = schema()
        elif is_dataclass(schema) or (
            isinstance(schema, type) and is_dataclass(schema)
        ):
            schema_type = SCHEMA_TYPE_DATACLASS
            if data:
                # Handle nested dataclasses
                kwargs = cls._prepare_dataclass_kwargs(schema, data)
                instance = schema(**kwargs)
            else:
                instance = schema()
        elif isinstance(schema, dict) or schema is dict:
            schema_type = SCHEMA_TYPE_DICT
            instance = data.copy() if data else {}
        else:
            raise TypeError("schema must be a pydantic model, dataclass, or dict")

        return cls(schema_type, instance)

    @staticmethod
    def _prepare_dataclass_kwargs(
        dataclass_type: Type[Any], data: Dict[str, Any]
    ) -> Dict[str, Any]:
        """Prepare kwargs for dataclass initialization with proper handling of nested
        dataclasses.

        Args:
            dataclass_type: Target dataclass type
            data: Data dictionary

        Returns:
            Dictionary of kwargs suitable for initializing the dataclass
        """
        if not data:
            return {}

        kwargs: Dict[str, Any] = {}

        # Process each field in the dataclass
        for field in fields(dataclass_type):
            field_name = field.name

            # Skip if field not in data
            if field_name not in data:
                continue

            field_value = data[field_name]
            field_type = field.type

            # Handle nested dataclasses
            if isinstance(field_value, dict) and is_dataclass(field_type):
                # Recursively handle nested dataclass
                nested_kwargs = ConfigModel._prepare_dataclass_kwargs(
                    field_type, field_value
                )
                kwargs[field_name] = field_type(**nested_kwargs)
            else:
                kwargs[field_name] = field_value

        return kwargs

    @property
    def model(self) -> Any:
        """Get the underlying model instance"""
        return self._model

    def to_dict(self) -> Dict[str, Any]:
        """
        Convert the model to a dictionary

        Returns:
            Dictionary representation of the model
        """
        if self.schema_type == SCHEMA_TYPE_PYDANTIC:
            return self._model.model_dump()
        elif self.schema_type == SCHEMA_TYPE_DATACLASS:
            return asdict(self._model)
        else:  # dict
            return self._model.copy()

    def get_value(self, key: str) -> Any:
        """
        Get a value from the model by key

        Args:
            key: Key to get (supports dot notation for nested access)

        Returns:
            Value at the specified key
        """
        if "." in key:
            parts = key.split(".")
            current = self._model

            for part in parts:
                if hasattr(current, part):
                    current = getattr(current, part)
                elif isinstance(current, dict) and part in current:
                    current = current[part]
                else:
                    return None

            return current
        else:
            if hasattr(self._model, key):
                return getattr(self._model, key)
            elif isinstance(self._model, dict) and key in self._model:
                return self._model[key]
            return None

    def update(self, data: Dict[str, Any]) -> None:
        """
        Update the model with new data

        Args:
            data: Dictionary of values to update
        """
        if self.schema_type == SCHEMA_TYPE_PYDANTIC:
            for key, value in data.items():
                if hasattr(self._model, key):
                    # Handle nested updates
                    if (
                        isinstance(value, dict)
                        and hasattr(self._model, key)
                        and isinstance(getattr(self._model, key), BaseModel)
                    ):
                        nested_model = getattr(self._model, key)
                        for nested_key, nested_value in value.items():
                            setattr(nested_model, nested_key, nested_value)
                    else:
                        setattr(self._model, key, value)
        elif self.schema_type == SCHEMA_TYPE_DATACLASS:
            for key, value in data.items():
                if hasattr(self._model, key):
                    # Handle nested updates
                    if (
                        isinstance(value, dict)
                        and hasattr(self._model, key)
                        and is_dataclass(getattr(self._model, key))
                    ):
                        nested_model = getattr(self._model, key)
                        for nested_key, nested_value in value.items():
                            setattr(nested_model, nested_key, nested_value)
                    else:
                        setattr(self._model, key, value)
        else:  # dict
            self._model.update(data)

    def merge(self, other: "ConfigModel") -> "ConfigModel":
        """
        Merge with another ConfigModel

        Args:
            other: Another ConfigModel to merge with

        Returns:
            New ConfigModel with merged data
        """
        base_dict = self.to_dict()
        other_dict = other.to_dict()

        merged_dict = deep_update(base_dict, other_dict)

        schema = type(self._model)
        return ConfigModel.from_schema(schema, merged_dict)


class ConfigManager(Generic[T]):
    """
    Core configuration manager that handles loading, merging, and accessing
    configurations.
    """

    def __init__(
        self,
        config_name: str,
        config_schema: Type[T],  # Supports pydantic.BaseModel, dataclass, or dict
        version: str = "1.0.0",
        auto_create: bool = True,
    ):
        """
        Initialize the configuration manager

        Args:
            config_name: Framework name, used to determine config file paths
            config_schema: Configuration schema definition (pydantic model, dataclass,
                or dict)
            version: Configuration schema version
            auto_create: Whether to automatically create default config files
        """
        self.config_name: str = config_name
        self.config_schema: Type[T] = config_schema
        self.version: str = version
        self.auto_create: bool = auto_create
        self.schema_type: SchemaType

        if (
            PYDANTIC_AVAILABLE
            and isinstance(config_schema, type)
            and issubclass(config_schema, BaseModel)
        ):
            self.schema_type = SCHEMA_TYPE_PYDANTIC
        elif is_dataclass(config_schema) or (
            isinstance(config_schema, type) and is_dataclass(config_schema)
        ):
            self.schema_type = SCHEMA_TYPE_DATACLASS
        elif isinstance(config_schema, dict):
            self.schema_type = SCHEMA_TYPE_DICT
        else:
            raise TypeError(
                "config_schema must be a pydantic model, dataclass, or dict"
            )

        # Configuration instances
        self._config: Optional[T] = None
        self._default_config: Optional[T] = None
        self._user_config: Optional[T] = None
        self._project_config: Optional[T] = None

        # ConfigModel instances for internal use
        self._config_model: Optional[ConfigModel] = None
        self._default_config_model: Optional[ConfigModel] = None
        self._user_config_model: Optional[ConfigModel] = None
        self._project_config_model: Optional[ConfigModel] = None

        # Configuration file paths
        self.user_config_path: Path = get_user_config_path(config_name)
        self.project_root: Optional[Path] = find_project_root()
        self.project_config_path: Optional[Path] = get_project_config_path(
            config_name, str(self.project_root) if self.project_root else None
        )

        if auto_create:
            self._ensure_user_config_exists()

    def _ensure_user_config_exists(self) -> None:
        """Ensure user config file and directory exist"""
        if not self.user_config_path.parent.exists():
            self.user_config_path.parent.mkdir(parents=True, exist_ok=True)

        if not self.user_config_path.exists():
            default_config = self._get_default_dict()
            with open(self.user_config_path, "w") as f:
                yaml.dump(default_config, f, default_flow_style=False, sort_keys=False)

    def _get_default_dict(self) -> Dict[str, Any]:
        """Get default configuration as a dictionary"""
        default_model = ConfigModel.from_schema(self.config_schema)
        return default_model.to_dict()

    def _to_schema_type(self, config_dict: Dict[str, Any]) -> T:
        """Convert dictionary to the schema type"""
        config_model = ConfigModel.from_schema(self.config_schema, config_dict)
        return cast(T, config_model.model)

    def load(self) -> T:
        """
        Load and merge all configuration levels

        Returns:
            Merged final configuration object (same type as schema)
        """
        default_config_model: ConfigModel = ConfigModel.from_schema(self.config_schema)

        # Load user and project configs using helper method
        user_config_model, _ = self._load_config_from_path(self.user_config_path)
        project_config_model, _ = self._load_config_from_path(self.project_config_path)

        # Merge configurations using ConfigModel
        merged_config_model = default_config_model
        if user_config_model:
            merged_config_model = merged_config_model.merge(user_config_model)
        if project_config_model:
            merged_config_model = merged_config_model.merge(project_config_model)

        # Store ConfigModel instances for internal use
        self._default_config_model = default_config_model
        self._user_config_model = user_config_model
        self._project_config_model = project_config_model
        self._config_model = merged_config_model

        # Store schema-typed instances
        self._default_config = cast(T, default_config_model.model)
        self._user_config = cast(
            Optional[T], user_config_model.model if user_config_model else None
        )
        self._project_config = cast(
            Optional[T], project_config_model.model if project_config_model else None
        )
        self._config = cast(T, merged_config_model.model)

        return self._config

    @property
    def config(self) -> T:
        """
        Get the current merged configuration

        Returns:
            Current effective configuration object
        """
        if self._config is None:
            return self.load()
        return self._config

    def get_default_config(self) -> T:
        """
        Get the default configuration

        Returns:
            Default configuration object based on schema
        """
        if self._default_config is None:
            self._default_config_model = ConfigModel.from_schema(self.config_schema)
            self._default_config = cast(T, self._default_config_model.model)
        return self._default_config

    def _load_config_from_path(
        self, config_path: Optional[Path]
    ) -> Tuple[Optional[ConfigModel], Optional[T]]:
        """
        Helper method to load configuration from a path

        Args:
            config_path: Path to the configuration file

        Returns:
            Tuple of (ConfigModel, typed config object) or (None, None) if not available
        """
        if not config_path or not config_path.exists():
            return None, None

        try:
            with open(config_path, "r") as f:
                config_dict = yaml.safe_load(f) or {}

            if not config_dict:
                return None, None

            config_model: ConfigModel = ConfigModel.from_schema(
                self.config_schema, config_dict
            )
            typed_config: T = cast(T, config_model.model)
            return config_model, typed_config
        except Exception as e:
            logger.error(f"Warning: Failed to load config from {config_path}: {e}")
            return None, None

    def get_user_config(self) -> Optional[T]:
        """
        Get the user-level configuration

        Returns:
            User configuration object or None if not available
        """
        if self._user_config is None and self._user_config_model is None:
            self._user_config_model, self._user_config = self._load_config_from_path(
                self.user_config_path
            )
        return self._user_config

    def get_project_config(self) -> Optional[T]:
        """
        Get the project-level configuration

        Returns:
            Project configuration object or None if not available
        """
        if self._project_config is None and self._project_config_model is None:
            self._project_config_model, self._project_config = (
                self._load_config_from_path(self.project_config_path)
            )
        return self._project_config

    def _update_config_file(
        self,
        config_path: Path,
        config_model: Optional[ConfigModel],
        config_update: Dict[str, Any],
    ) -> Tuple[ConfigModel, T]:
        """
        Helper method for updating configuration files

        Args:
            config_path: Path to the configuration file
            config_model: Existing config model or None
            config_update: Updates to apply to the configuration

        Returns:
            Tuple of (updated ConfigModel, typed config object)
        """
        # Load or create config model if not provided
        if config_model is None:
            existing_config: Dict[str, Any] = {}
            if config_path.exists():
                try:
                    with open(config_path, "r") as f:
                        existing_config = yaml.safe_load(f) or {}
                except Exception as e:
                    logger.error(f"Warning: Failed to load config for update: {e}")

            config_model = ConfigModel.from_schema(self.config_schema, existing_config)

        config_model.update(config_update)

        updated_config: Dict = config_model.to_dict()
        os.makedirs(os.path.dirname(str(config_path)), exist_ok=True)
        with open(config_path, "w") as f:
            yaml.dump(updated_config, f, default_flow_style=False, sort_keys=False)

        typed_config: T = cast(T, config_model.model)
        return config_model, typed_config

    def update_user_config(self, config_update: Dict[str, Any]) -> None:
        """
        Update the user-level configuration

        Args:
            config_update: Configuration dictionary to update
        """
        self._user_config_model, self._user_config = self._update_config_file(
            self.user_config_path, self._user_config_model, config_update
        )

        # Reset merged config to force reload
        self._config = None
        self._config_model = None

    def update_project_config(self, config_update: Dict[str, Any]) -> None:
        """
        Update the project-level configuration

        Args:
            config_update: Configuration dictionary to update
        """
        if not self.project_config_path:
            raise ValueError(
                "No project root found. Cannot update project configuration."
            )

        self._project_config_model, self._project_config = self._update_config_file(
            self.project_config_path, self._project_config_model, config_update
        )

        # Reset merged config to force reload
        self._config = None
        self._config_model = None

    def create_project_template(self, path: Optional[str] = None) -> str:
        """
        Create a project configuration template

        Args:
            path: Optional project path, defaults to current directory

        Returns:
            Path to the created configuration file
        """
        project_path = Path(path) if path else Path.cwd()
        config_dir = project_path / f".{self.config_name}"
        config_file = config_dir / "config.yaml"

        if not config_dir.exists():
            config_dir.mkdir(parents=True, exist_ok=True)

        if not config_file.exists():
            default_config = self._get_default_dict()
            with open(config_file, "w") as f:
                yaml.dump(default_config, f, default_flow_style=False, sort_keys=False)

        return str(config_file)


def get_user_config_path(config_name: str) -> Path:
    """Get the path to the user-level configuration file"""
    user_config_path = os.path.expanduser(f"~/.zeeland/{config_name}/config.yaml")
    return Path(user_config_path)


def get_project_config_path(
    config_name: str, project_path: Optional[str] = None
) -> Optional[Path]:
    """Get the path to the project-level configuration file"""
    if project_path:
        project_root = Path(project_path)
    else:
        project_root = find_project_root()

    if not project_root:
        return None

    return project_root / f".{config_name}" / "config.yaml"


def find_project_root() -> Optional[Path]:
    """
    Find the project root directory by looking for common project files
    like .git, pyproject.toml, etc.
    """
    cwd = Path.cwd()

    # Common project root indicators
    indicators = [".git", "pyproject.toml", "setup.py", "package.json", "Cargo.toml"]

    # Check current directory and parents until root
    current = cwd
    while current.parent != current:  # Stop at system root
        for indicator in indicators:
            if (current / indicator).exists():
                return current
        current = current.parent

    # No project root found
    return None


def merge_configs_dict(
    default_config: Dict[str, Any],
    user_config: Dict[str, Any],
    project_config: Dict[str, Any],
) -> Dict[str, Any]:
    """
    Merge multiple configuration levels

    Args:
        default_config: Default configuration
        user_config: User-level configuration
        project_config: Project-level configuration

    Returns:
        Merged configuration dictionary
    """
    result = default_config.copy()

    # Apply user config over defaults
    if user_config:
        result = deep_update(result, user_config)

    # Apply project config over previous levels
    if project_config:
        result = deep_update(result, project_config)

    return result


def deep_update(
    base_dict: Dict[str, Any], update_dict: Dict[str, Any]
) -> Dict[str, Any]:
    """
    Recursively update a dictionary

    Args:
        base_dict: The base dictionary to update
        update_dict: The dictionary with updates to apply

    Returns:
        Updated dictionary
    """
    result = base_dict.copy()

    for key, value in update_dict.items():
        if key in result and isinstance(result[key], dict) and isinstance(value, dict):
            # Recursively update nested dictionaries
            result[key] = deep_update(result[key], value)
        else:
            # Direct update for simple values or new keys
            result[key] = value

    return result
