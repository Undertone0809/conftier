"""
Unit tests for the ConfigManager class
"""

import os
import shutil
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, Optional

import pytest
import yaml

from conftier import ConfigManager
from conftier.core import (
    find_project_root,
    get_project_config_path,
    get_user_config_path,
)


# Test Data Models
@dataclass
class NestedConfig:
    name: str = "nested"
    value: int = 42


@dataclass
class TestConfig:
    title: str = "test"
    enabled: bool = True
    number: int = 100
    nested: NestedConfig = NestedConfig()


try:
    from pydantic import BaseModel, Field

    class LLMConfig(BaseModel):
        name: str = Field(default="gpt-4")
        api_key: str = Field(default="")
        api_base: Optional[str] = None

    class PydanticTestConfig(BaseModel):
        llm_config: LLMConfig = Field(default_factory=LLMConfig)
        enabled: bool = Field(default=True)
        number: int = Field(default=100)

    PYDANTIC_AVAILABLE = True
except ImportError:
    PYDANTIC_AVAILABLE = False


class TestConfigManager:
    """Tests for the ConfigManager class"""

    @pytest.fixture
    def temp_home_dir(self):
        """Fixture to create a temporary home directory"""
        original_home = os.environ.get("HOME")
        with tempfile.TemporaryDirectory() as temp_dir:
            os.environ["HOME"] = temp_dir
            yield temp_dir
            if original_home:
                os.environ["HOME"] = original_home
            else:
                del os.environ["HOME"]

    @pytest.fixture
    def temp_project_dir(self):
        """Fixture to create a temporary project directory"""
        with tempfile.TemporaryDirectory() as temp_dir:
            # Create a .git directory to identify as project root
            os.makedirs(os.path.join(temp_dir, ".git"))

            original_cwd = os.getcwd()
            os.chdir(temp_dir)

            yield temp_dir

            os.chdir(original_cwd)

    def test_init_with_dataclass(self, temp_home_dir):
        """Test initializing ConfigManager with a dataclass schema"""
        config_manager = ConfigManager[TestConfig](
            framework_name="test_framework",
            config_schema=TestConfig,
            version="1.0.0",
            auto_create=True,
        )

        assert config_manager.framework_name == "test_framework"
        assert config_manager.schema_type == "dataclass"
        assert config_manager.version == "1.0.0"

        # Check if user config was created
        user_config_path = get_user_config_path("test_framework")
        assert user_config_path.exists()

        # Verify config content
        with open(user_config_path, "r") as f:
            config_dict = yaml.safe_load(f)

        assert config_dict["title"] == "test"
        assert config_dict["enabled"] == True
        assert config_dict["number"] == 100
        assert config_dict["nested"]["name"] == "nested"
        assert config_dict["nested"]["value"] == 42

    def test_init_with_dict(self, temp_home_dir):
        """Test initializing ConfigManager with a dict schema"""
        test_dict = {"name": "test_dict", "value": 123, "nested": {"key": "value"}}

        config_manager = ConfigManager[Dict](
            framework_name="test_dict_framework",
            config_schema=test_dict,
            version="1.0.0",
            auto_create=True,
        )

        assert config_manager.framework_name == "test_dict_framework"
        assert config_manager.schema_type == "dict"

        # Check if user config was created
        user_config_path = get_user_config_path("test_dict_framework")
        assert user_config_path.exists()

        # Verify config content
        with open(user_config_path, "r") as f:
            config_dict = yaml.safe_load(f)

        assert config_dict["name"] == "test_dict"
        assert config_dict["value"] == 123
        assert config_dict["nested"]["key"] == "value"

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_init_with_pydantic(self, temp_home_dir):
        """Test initializing ConfigManager with a Pydantic schema"""
        config_manager = ConfigManager[PydanticTestConfig](
            framework_name="test_pydantic",
            config_schema=PydanticTestConfig,
            version="1.0.0",
            auto_create=True,
        )

        assert config_manager.framework_name == "test_pydantic"
        assert config_manager.schema_type == "pydantic"
        assert config_manager.version == "1.0.0"

        # Check if user config was created
        user_config_path = get_user_config_path("test_pydantic")
        assert user_config_path.exists()

        # Verify config content
        with open(user_config_path, "r") as f:
            config_dict = yaml.safe_load(f)

        assert config_dict["llm_config"]["name"] == "gpt-4"
        assert config_dict["llm_config"]["api_key"] == ""
        assert config_dict["enabled"] == True
        assert config_dict["number"] == 100

    def test_load_default_only(self, temp_home_dir):
        """Test loading configuration with only default values"""
        # Delete any existing config files
        framework_name = "test_load_default"
        user_config_path = get_user_config_path(framework_name)
        if user_config_path.exists():
            os.remove(user_config_path)

        # Initialize with auto_create=False
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=False
        )

        # Load configuration
        config = config_manager.load()

        # Verify default values
        assert config.title == "test"
        assert config.enabled == True
        assert config.number == 100
        assert config.nested.name == "nested"
        assert config.nested.value == 42

        # Verify no user config created
        assert not user_config_path.exists()

    def test_load_with_user_config(self, temp_home_dir):
        """Test loading configuration with user config"""
        framework_name = "test_load_user"
        user_config_path = get_user_config_path(framework_name)

        # Ensure directory exists
        os.makedirs(user_config_path.parent, exist_ok=True)

        # Create user config
        user_config = {
            "title": "user_title",
            "number": 200,
            "nested": {"name": "user_nested"},
        }

        with open(user_config_path, "w") as f:
            yaml.dump(user_config, f)

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=False
        )

        # Load configuration
        config = config_manager.load()

        # Verify merged values
        assert config.title == "user_title"  # From user config
        assert config.enabled == True  # From default
        assert config.number == 200  # From user config
        assert config.nested.name == "user_nested"  # From user config
        assert config.nested.value == 42  # From default

    def test_load_with_all_configs(self, temp_home_dir, temp_project_dir):
        """Test loading configuration with default, user, and project configs"""
        framework_name = "test_load_all"

        # Create user config
        user_config_path = get_user_config_path(framework_name)
        os.makedirs(user_config_path.parent, exist_ok=True)

        user_config = {
            "title": "user_title",
            "number": 200,
            "nested": {"name": "user_nested"},
        }

        with open(user_config_path, "w") as f:
            yaml.dump(user_config, f)

        # Create project config
        project_dir = Path(temp_project_dir)
        project_config_dir = project_dir / f".{framework_name}"
        os.makedirs(project_config_dir, exist_ok=True)

        project_config_path = project_config_dir / "config.yaml"
        project_config = {"title": "project_title", "nested": {"value": 999}}

        with open(project_config_path, "w") as f:
            yaml.dump(project_config, f)

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=False
        )

        # Load configuration
        config = config_manager.load()

        # Verify merged values (project > user > default)
        assert config.title == "project_title"  # From project config
        assert config.enabled == True  # From default
        assert config.number == 200  # From user config
        assert config.nested.name == "user_nested"  # From user config
        assert config.nested.value == 999  # From project config

    def test_get_user_config(self, temp_home_dir):
        """Test getting user config"""
        framework_name = "test_get_user"
        user_config_path = get_user_config_path(framework_name)

        # Ensure directory exists
        os.makedirs(user_config_path.parent, exist_ok=True)

        # Create user config
        user_config = {"title": "user_title", "number": 200}

        with open(user_config_path, "w") as f:
            yaml.dump(user_config, f)

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=False
        )

        # Get user config
        user_config_obj = config_manager.get_user_config()

        # Verify values
        assert user_config_obj is not None
        assert user_config_obj.title == "user_title"
        assert user_config_obj.number == 200
        # Default values for unspecified fields
        assert user_config_obj.enabled == True
        assert user_config_obj.nested.name == "nested"
        assert user_config_obj.nested.value == 42

    def test_get_project_config(self, temp_home_dir, temp_project_dir):
        """Test getting project config"""
        framework_name = "test_get_project"

        # Create project config
        project_dir = Path(temp_project_dir)
        project_config_dir = project_dir / f".{framework_name}"
        os.makedirs(project_config_dir, exist_ok=True)

        project_config_path = project_config_dir / "config.yaml"
        project_config = {"title": "project_title", "nested": {"value": 999}}

        with open(project_config_path, "w") as f:
            yaml.dump(project_config, f)

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=False
        )

        # Get project config
        project_config_obj = config_manager.get_project_config()

        # Verify values
        assert project_config_obj is not None
        assert project_config_obj.title == "project_title"
        # Default values for unspecified fields
        assert project_config_obj.enabled == True
        assert project_config_obj.number == 100
        assert project_config_obj.nested.name == "nested"
        assert project_config_obj.nested.value == 999

    def test_update_user_config(self, temp_home_dir):
        """Test updating user config"""
        framework_name = "test_update_user"

        # Initialize manager with auto_create
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=True
        )

        # Update user config
        config_manager.update_user_config(
            {"title": "updated_title", "nested": {"name": "updated_nested"}}
        )

        # Verify file was updated
        user_config_path = get_user_config_path(framework_name)
        with open(user_config_path, "r") as f:
            updated_config = yaml.safe_load(f)

        assert updated_config["title"] == "updated_title"
        assert updated_config["nested"]["name"] == "updated_nested"
        # Other values should be preserved
        assert updated_config["enabled"] == True
        assert updated_config["number"] == 100
        assert updated_config["nested"]["value"] == 42

        # Reload and verify
        reloaded_config = config_manager.load()
        assert reloaded_config.title == "updated_title"
        assert reloaded_config.nested.name == "updated_nested"

    def test_update_project_config(self, temp_home_dir, temp_project_dir):
        """Test updating project config"""
        framework_name = "test_update_project"

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=True
        )

        # Update project config
        config_manager.update_project_config(
            {"title": "updated_project", "nested": {"value": 888}}
        )

        # Verify file was created and updated
        project_config_path = get_project_config_path(framework_name)
        assert project_config_path is not None
        assert project_config_path.exists()

        with open(project_config_path, "r") as f:
            updated_config = yaml.safe_load(f)

        assert updated_config["title"] == "updated_project"
        assert updated_config["nested"]["value"] == 888

        # Reload and verify
        reloaded_config = config_manager.load()
        assert reloaded_config.title == "updated_project"
        assert reloaded_config.nested.value == 888
        # Default values for other fields
        assert reloaded_config.enabled == True
        assert reloaded_config.number == 100
        assert reloaded_config.nested.name == "nested"

    def test_create_project_template(self, temp_home_dir, temp_project_dir):
        """Test creating project template"""
        framework_name = "test_template"

        # Initialize manager
        config_manager = ConfigManager[TestConfig](
            framework_name=framework_name, config_schema=TestConfig, auto_create=True
        )

        # Create template
        template_path = config_manager.create_project_template()

        # Verify file was created
        assert os.path.exists(template_path)

        with open(template_path, "r") as f:
            template_config = yaml.safe_load(f)

        # Verify template content
        assert template_config["title"] == "test"
        assert template_config["enabled"] == True
        assert template_config["number"] == 100
        assert template_config["nested"]["name"] == "nested"
        assert template_config["nested"]["value"] == 42

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_pydantic_full_workflow(self, temp_home_dir, temp_project_dir):
        """Test a full workflow with Pydantic models"""
        framework_name = "test_pydantic_workflow"

        # Initialize manager
        config_manager = ConfigManager[PydanticTestConfig](
            framework_name=framework_name,
            config_schema=PydanticTestConfig,
            auto_create=True,
        )

        # Verify default config
        default_config = config_manager.get_default_config()
        assert default_config.llm_config.name == "gpt-4"
        assert default_config.enabled == True
        assert default_config.number == 100

        # Update user config
        config_manager.update_user_config(
            {
                "llm_config": {"name": "user_model", "api_key": "user_key123"},
                "number": 200,
            }
        )

        # Update project config
        config_manager.update_project_config({"llm_config": {"name": "project_model"}})

        # Reload and verify merged config
        config = config_manager.load()

        # Project config takes precedence over user config over default
        assert config.llm_config.name == "project_model"  # From project
        assert config.llm_config.api_key == "user_key123"  # From user
        assert config.number == 200  # From user
        assert config.enabled == True  # From default

        # Verify individual configs
        user_config = config_manager.get_user_config()
        assert user_config is not None
        assert user_config.llm_config.name == "user_model"

        project_config = config_manager.get_project_config()
        assert project_config is not None
        assert project_config.llm_config.name == "project_model"
