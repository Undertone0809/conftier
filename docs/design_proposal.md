# Conftier Design Proposal

Conftier is a powerful multi-level configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications, supporting the intelligent merging of user preferences and project settings. It provides a complete mechanism for framework developers to build user-level (user-based) and project-level (project-based) configuration management systems for their products.

Conftier handles the complex logic of storing, reading, merging, validating, and updating configuration files, allowing framework developers to focus on their core functionality development.

## Core Features

### User-level Configuration (User-based Configuration)

- Storage Path: ~/.zeeland/{config_name}/config.yaml
- Purpose: Store user personal preference settings that are effective across projects
- Characteristics:
  - Globally effective for a single user
  - Not affected by version control
  - Suitable for storing personal authentication information, preference settings, etc.
- Auto-creation: Automatically creates the directory structure and default configuration file on first use

### Project-level Configuration (Project-based Configuration)

- Storage Path: /.{config_name}/config.yaml (located in the project root directory)
- Purpose: Store configurations specific to a project, suitable for team collaboration
- Characteristics:
  - Can be included in version control
  - Shared among team members
  - Suitable for storing project-specific settings, team conventions, etc.
- Template Generation: Provides a command-line tool to generate project configuration templates

### Priority Mechanism

- Clear priority rules: Project-level configuration > User-level configuration > Default configuration
- Partial Overwrite: Higher priority configurations only overwrite corresponding keys, without affecting other keys
- Configuration Merging: Intelligently merges configurations from different levels to form the final effective configuration
- Conflict Resolution: Provides clear logs explaining the source of configuration values and overwrite situations

## Use Case Scenarios

### Framework Developers

#### Initial Integration

1. Install the package with pip install conftier

2. Define the config schema structure, which can be defined using dataclass or pydantic BaseModel, and build the default configuration. The following example shows how a framework developer builds a configuration for an LLM Model.

```python
# schema version=1.0.0, framework name gcop
from typing import Optional
from pydantic import BaseModel, Field

class ModelConfig(BaseModel):
    model_name: str = Field(description="The name of the LLM model")
    api_key: str = Field(description="The API key for the LLM model")
    api_base: Optional[str] = None


class GCOPConfig(BaseModel):
    model_config: ModelConfig
    prompt_template: Optional[str] = "This is default prompt template"
    enable_data_improvement: Optional[bool] = True # Optional fields can be left blank but require default values
```

### Framework Users

#### Parameter Configuration

Users can configure user-level parameters in `~/.zeeland/{config_name}/config.yaml`, and if there are team configuration requirements, they can configure related parameters in `/.{config_name}/config.yaml` in the current project directory, such as `/.gcop/config.yaml` in the example, and then gcop, which uses the conftier framework to build its configuration system, will read the user's configuration and complete its business.

### SDK Design

```python
class ConfigManager:
    """
    Core configuration manager, handling configuration loading, merging, and access
    """
    
    def __init__(
        self, 
        config_name: str,
        config_schema: Any,  # Supports pydantic.BaseModel or dataclass
        version: str = "1.0.0",
        auto_create: bool = True
    ):
        """
        Initializes the configuration manager
        
        Args:
            config_name: Framework name, used to determine the configuration file path
            config_schema: Configuration schema definition, can be pydantic model or dataclass
            version: Configuration schema version
            auto_create: Whether to automatically create the default configuration file
        """
        pass
    
    def load(self) -> Any:
        """
        Loads and merges all levels of configurations
        
        Returns:
            The merged final configuration object (of the same type as the schema)
        """
        pass
    
    @property
    def config(self) -> Any:
        """
        Gets the merged configuration
        
        Returns:
            The current effective configuration object
        """
        pass
    
    def get_default_config(self) -> Any:
        """
        Gets the default configuration
        
        Returns:
            The default configuration object based on the schema
        """
        pass
    
    def get_user_config(self) -> Any:
        """
        Gets the user-level configuration
        
        Returns:
            The user-level configuration object
        """
        pass
    
    def get_project_config(self) -> Any:
        """
        Gets the project-level configuration
        
        Returns:
            The project-level configuration object
        """
        pass
    
    def update_user_config(self, config_update: dict) -> None:
        """
        Updates the user-level configuration
        
        Args:
            config_update: The configuration dictionary to be updated
        """
        pass
    
    def update_project_config(self, config_update: dict) -> None:
        """
        Updates the project-level configuration
        
        Args:
            config_update: The configuration dictionary to be updated
        """
        pass
    
```

```python
def get_user_config_path(config_name: str) -> Path:
    """Gets the user-level configuration file path"""
    pass

def get_project_config_path(config_name: str, project_path: Optional[str] = None) -> Path:
    """Gets the project-level configuration file path"""
    pass

def find_project_root() -> Optional[Path]:
    """Finds the project root directory"""
    pass

```

```python
def merge_configs(
    default_config: Any, 
    user_config: Any, 
    project_config: Any,
    schema: Any
) -> Any:
    """
    Merges multiple levels of configurations
    
    Args:
        default_config: Default configuration
        user_config: User configuration
        project_config: Project configuration
        schema: Configuration schema
        
    Returns:
        The merged configuration object
    """
    pass

```

```python
@click.group()
def conftier():
    """Conftier configuration management tool"""
    pass

@conftier.command()
@click.argument("config_name")
@click.option("--path", "-p", help="Project path")
def init_project(config_name: str, path: Optional[str] = None):
    """Initializes project configuration template"""
    pass

@conftier.command()
@click.argument("config_name")
def show_config(config_name: str):
    """Displays the current effective configuration and its source"""
    pass

```

User Example:

```python
from conftier import ConfigManager
from pydantic import BaseModel, Field
from typing import Optional

# 1. Define the configuration schema
class ModelConfig(BaseModel):
    model_name: str = Field(default="gpt-4", description="LLM model name")
    api_key: str = Field(default="", description="API key")
    api_base: Optional[str] = None

class GCOPConfig(BaseModel):
    model_config: ModelConfig = ModelConfig()
    prompt_template: str = "This is default prompt template"
    enable_data_improvement: bool = True

# 2. Initialize the configuration manager
config_manager = ConfigManager(
    config_name="gcop",
    config_schema=GCOPConfig,
    version="1.0.0",
    auto_create=True
)

# 3. Use the configuration in the application
def initialize_app():
    # Load the merged configuration
    config = config_manager.load()
    
    # Use the configuration values
    model_name = config.model_config.model_name
    api_key = config.model_config.api_key
    
    print(f"Using model: {model_name}")
    print(f"Data improvement enabled: {config.enable_data_improvement}")
```

```yaml
# ~/.zeeland/gcop/config.yaml (user-level configuration)
model_config:
  model_name: "gpt-4-turbo"
  api_key: "user-api-key-123"
```

```yaml
# /.gcop/config.yaml (project-level configuration)
model_config:
  model_name: "gpt-3.5-turbo"  # Will overwrite user configuration
prompt_template: "Project specific prompt template"
```

Configuration merging logic:

- Builds the base configuration starting from default values
- Loads user configuration and merges to overwrite default values
- Loads project configuration and merges to overwrite the previous two levels
- Validates the merged result against the schema

Uses ConfigModel to simultaneously support Pydantic Model, dataclass, dict.

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Pydantic Model │     │    Dataclass    │     │      Dict       │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                        ConfigModel                              │
│                                                                 │
│  - Unified configuration representation                                               │
│  - Supports nested structures                                                 │
│  - Provides validation functionality                                                 │
│  - Supports serialization/deserialization                                          │
└─────────────┬───────────────────────────────────────┬───────────┘
              │                                       │
              ▼                                       ▼
┌─────────────────────────┐             ┌─────────────────────────┐
│      Configuration Merge Engine       │             │      Configuration Storage Engine       │
└─────────────────────────┘             └─────────────────────────┘

```
