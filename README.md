<p align="center">
   <img src="./docs/public/banner.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

<div align="center">

[![Build status](https://github.com/Undertone0809/conftier/workflows/build/badge.svg?branch=main&event=push)](https://github.com/Undertone0809/conftier/actions?query=workflow%3Abuild)
[![Python Version](https://img.shields.io/pypi/pyversions/conftier.svg)](https://pypi.org/project/conftier/)
[![Code style: ruff](https://img.shields.io/badge/code%20style-ruff-000000.svg)](https://github.com/astral-sh/ruff)
[![License](https://img.shields.io/github/license/Undertone0809/conftier)](https://github.com/Undertone0809/conftier/blob/main/LICENSE)
![Coverage Report](assets/images/coverage.svg)

</div>

Conftier is a powerful multi-tier configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications.

## Features

- **Multi-level Configuration**: User-level preferences and project-specific settings
- **Priority Mechanism**: Clear priority rules (project > user > default)
- **Schema Validation**: Use Pydantic or dataclasses to define and validate configurations
- **Smart Merging**: Intelligently merge different configuration levels
- **CLI Tools**: Command-line tools for managing configurations
- **Type Hints**: Full IDE support with proper type inference
- **Unified Model**: ConfigModel provides a unified interface for different configuration types

## Installation

```bash
# Basic installation
pip install conftier

# With Pydantic support (recommended)
pip install conftier[pydantic]
```

## Usage

### For Framework Developers

As a framework developer, you'll integrate Conftier into your application to manage configurations across different levels. Here's a detailed guide on the journey:

#### 1. Define Your Configuration Schema

First, define your configuration schema using either Pydantic models or dataclasses. This schema acts as the "contract" for your configuration and provides default values.

**Using Pydantic Models:**

```python
from conftier import ConfigManager
from pydantic import BaseModel, Field
from typing import Optional, List

# Define your configuration schema
class LLMConfig(BaseModel):
    model_name: str = Field(default="gpt-4", description="LLM model name")
    api_key: str = Field(default="", description="API key")
    api_base: Optional[str] = None
    temperature: float = Field(default=0.7, description="Sampling temperature")

class LoggingConfig(BaseModel):
    log_level: str = Field(default="INFO", description="Logging level")
    log_file: Optional[str] = Field(default=None, description="Log file path")

class MyFrameworkConfig(BaseModel):
    # Note: Avoid using "model_config" as it conflicts with Pydantic's internals
    llm_config: LLMConfig = Field(default_factory=LLMConfig)
    logging_config: LoggingConfig = Field(default_factory=LoggingConfig)
    prompt_template: str = Field(default="Default prompt template")
    enable_feature: bool = Field(default=True)
    supported_formats: List[str] = Field(default=["json", "yaml"])
```

**Using Dataclasses:**

```python
from conftier import ConfigManager
from dataclasses import dataclass, field
from typing import Optional, List

@dataclass
class LLMConfig:
    model_name: str = "gpt-4"
    api_key: str = ""
    api_base: Optional[str] = None
    temperature: float = 0.7

@dataclass
class LoggingConfig:
    log_level: str = "INFO"
    log_file: Optional[str] = None

@dataclass
class MyFrameworkConfig:
    llm_config: LLMConfig = field(default_factory=LLMConfig)
    logging_config: LoggingConfig = field(default_factory=LoggingConfig)
    prompt_template: str = "Default prompt template"
    enable_feature: bool = True
    supported_formats: List[str] = field(default_factory=lambda: ["json", "yaml"])
```

#### 2. Initialize the Configuration Manager

Create a ConfigManager instance with your schema. This is typically done when your framework initializes:

```python
config_manager = ConfigManager[MyFrameworkConfig](
    config_name="myframework",  # This will be used for config file names
    config_schema=MyFrameworkConfig,
    version="1.0.0",
    auto_create=True,  # Automatically create config files if they don't exist
    description="Configuration for MyFramework"
)
```

**What happens during initialization:**

- When `auto_create=True`:
  - If the user-level config file doesn't exist at `~/.zeeland/myframework/config.yaml`, it will be created with default values
  - No project-level config is automatically created unless explicitly requested

- The `config_schema` provides default values for all configuration options
- The `version` is stored in the configuration files for future compatibility checks

#### 3. Loading and Using Configuration

Load the configuration when your framework starts:

```python
def initialize_framework():
    # Load the merged configuration (combines project, user, and default configs)
    config = config_manager.load()
    
    # Access configuration values with full IDE autocompletion
    model_name = config.llm_config.model_name
    api_key = config.llm_config.api_key
    log_level = config.logging_config.log_level
    
    print(f"Initializing framework with model: {model_name}")
    print(f"Log level: {log_level}")
    
    # Set up your framework components using the configuration
    setup_logging(config.logging_config)
    initialize_llm_client(config.llm_config)
    
    return config
```

**What happens during loading:**

1. Conftier looks for configuration files in this order:
   - Project-level config: `./.myframework/config.yaml` (in the current working directory)
   - User-level config: `~/.zeeland/myframework/config.yaml`
   - Default values from your schema

2. It merges these configurations according to priority:
   - Project-level values override user-level values
   - User-level values override default values
   - Only defined values are overridden; undefined values are preserved from lower priority levels

3. The result is a fully populated configuration object of your schema type

#### 4. Accessing Individual Configuration Levels

You can access specific configuration levels when needed:

```python
# Get individual config levels - these can be None if not available
project_config = config_manager.get_project_config()
user_config = config_manager.get_user_config()

if project_config:
    print(f"Project is using model: {project_config.llm_config.model_name}")
    
if user_config:
    print(f"User has configured API key: {'Yes' if user_config.llm_config.api_key else 'No'}")
```

#### 5. Programmatically Creating or Updating Configurations

You can create or update configuration files programmatically:

```python
# Create or update user-level config
config_manager.save_user_config({
    "llm_config": {
        "api_key": "user-default-key"
    }
})

# Create or update project-level config
config_manager.save_project_config({
    "llm_config": {
        "model_name": "gpt-3.5-turbo"
    }
})
```

**What happens during saving:**

- For user config, the file is created/updated at `~/.zeeland/myframework/config.yaml`
- For project config, the file is created/updated at `./.myframework/config.yaml`
- Only the specified values are updated; other values remain unchanged
- The directory structure is automatically created if it doesn't exist

#### 6. Providing CLI Tools for Your Users

Integrate Conftier's CLI capabilities into your framework's CLI:

```python
import click
from conftier.cli import register_config_commands

@click.group()
def cli():
    """MyFramework CLI"""
    pass

# Register Conftier commands under your CLI
register_config_commands(
    cli,
    config_manager=config_manager,
    command_prefix="config"  # Creates commands like "myframework config show"
)

if __name__ == "__main__":
    cli()
```

### For Framework Users

As a user of a framework that implements Conftier, you have multiple ways to customize configurations:

#### 1. Configuration File Locations

You can create and edit configuration files in two locations:

1. **User-level config** (applies to all your projects):
   - Location: `~/.zeeland/myframework/config.yaml`
   - Purpose: Store your personal preferences that apply across all projects

2. **Project-level config** (applies to a specific project):
   - Location: `./.myframework/config.yaml` (in the project root directory)
   - Purpose: Store project-specific settings that should be shared with your team

#### 2. Creating and Editing Configuration Files

You can create these files manually or use the framework's CLI tools:

**Manually creating configuration files:**

User-level configuration:
```yaml
# ~/.zeeland/myframework/config.yaml
llm_config:
  model_name: "gpt-4-turbo"
  api_key: "user-api-key-123"
  temperature: 0.8

logging_config:
  log_level: "DEBUG"
```

Project-level configuration:
```yaml
# ./.myframework/config.yaml
llm_config:
  model_name: "gpt-3.5-turbo"  # Overrides user configuration
  temperature: 0.5             # Overrides user configuration

prompt_template: "Project specific prompt template"
enable_feature: false
```

**Using CLI tools:**

```bash
# Initialize a project configuration template
myframework config init-project

# Show current configuration and sources
myframework config show

# Set a configuration value (user-level)
myframework config set --key llm_config.api_key --value "my-api-key"

# Set a configuration value (project-level)
myframework config set --key llm_config.model_name --value "gpt-3.5-turbo" --project
```

#### 3. Configuration Merging Behavior

When the framework loads configuration, it follows these rules:

1. Start with default values from the schema
2. Override with any values from user-level config
3. Further override with any values from project-level config

Example of merged configuration:

```
Default schema:
  - llm_config.model_name: "gpt-4"
  - llm_config.temperature: 0.7
  - enable_feature: true
  - prompt_template: "Default prompt template"

User config:
  - llm_config.model_name: "gpt-4-turbo"
  - llm_config.api_key: "user-api-key-123"
  - llm_config.temperature: 0.8

Project config:
  - llm_config.model_name: "gpt-3.5-turbo"
  - llm_config.temperature: 0.5
  - prompt_template: "Project specific prompt template"
  - enable_feature: false

Final merged config:
  - llm_config.model_name: "gpt-3.5-turbo"    (from project)
  - llm_config.api_key: "user-api-key-123"    (from user)
  - llm_config.temperature: 0.5               (from project)
  - prompt_template: "Project specific prompt template" (from project)
  - enable_feature: false                     (from project)
```

### Advanced Usage

#### Using ConfigModel Directly

For advanced use cases, you can use the ConfigModel class to work with configuration objects directly:

```python
from conftier import ConfigModel
from pydantic import BaseModel, Field

# Your existing model
class AppConfig(BaseModel):
    app_name: str = Field(default="MyApp")
    debug: bool = Field(default=False)

# Create a ConfigModel instance from your schema
config_model = ConfigModel.from_schema(AppConfig, {"app_name": "CustomApp"})

# Access values
app_name = config_model.get_value("app_name")  # Returns "CustomApp"

# Convert to dictionary
config_dict = config_model.to_dict()  # {"app_name": "CustomApp", "debug": False}

# Update values
config_model.update({"debug": True})

# Create another config and merge them
default_config = ConfigModel.from_schema(AppConfig)
merged_config = default_config.merge(config_model)
```

#### Custom Configuration Paths

You can customize the configuration file paths:

```python
config_manager = ConfigManager[MyConfig](
    config_name="myframework",
    config_schema=MyConfig,
    version="1.0.0",
    user_config_dir="~/custom/user/path",
    project_config_dir="./custom/project/path"
)
```

#### Validation Hooks

You can add custom validation hooks:

```python
def validate_api_key(config):
    if config.llm_config.model_name != "demo" and not config.llm_config.api_key:
        raise ValueError("API key is required when not using demo model")

config_manager = ConfigManager[MyConfig](
    config_name="myframework",
    config_schema=MyConfig,
    version="1.0.0",
    validation_hooks=[validate_api_key]
)
```

## Configuration Priority

Conftier follows a clear priority order when merging configurations:

1. Project-level configuration (highest priority)
2. User-level configuration
3. Default configuration (from schema)

This allows for flexible configuration management:
- Framework developers provide sensible defaults
- Users set their preferences at the user level
- Project-specific overrides can be applied for team collaboration

## Development

### Quick Start

```bash
# Create and activate conda environment
conda create -n conftier python==3.10
conda activate conftier

# Install poetry and dependencies
pip install poetry
poetry install
```

### Makefile Usage

The project includes a comprehensive Makefile with commands for development:

- `make install` - Install all dependencies
- `make format` - Format code with ruff
- `make lint` - Run all linters
- `make test` - Run tests with pytest
- `make cleanup` - Remove cache files and build artifacts

## License

[![License](https://img.shields.io/github/license/Undertone0809/conftier)](https://github.com/Undertone0809/conftier/blob/main/LICENSE)

This project is licensed under the terms of the `MIT` license.