<p align="center">
   <img src="./docs/public/banner.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

<div align="center">

[![Build status](https://github.com/Undertone0809/conftier/workflows/build/badge.svg?branch=main&event=push)](https://github.com/Undertone0809/conftier/actions?query=workflow%3Abuild)
[![Python Version](https://img.shields.io/pypi/pyversions/conftier.svg)](https://pypi.org/project/conftier/)
[![Pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/Undertone0809/conftier/blob/main/.pre-commit-config.yaml)
[![Semantic Versions](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--versions-e10079.svg)](https://github.com/Undertone0809/conftier/releases)
[![License](https://img.shields.io/github/license/Undertone0809/conftier)](https://github.com/Undertone0809/conftier/blob/main/LICENSE)
![Coverage Report](assets/images/coverage.svg)

Multi-level configuration framework

</div>


# Conftier

Conftier is a powerful multi-tier configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications. It provides intelligent merging of user preferences and project settings.

## Features

- **User-level Configuration**: Store user preferences that apply across projects
- **Project-level Configuration**: Store project-specific settings for team collaboration
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

#### Using Pydantic Models

```python
from conftier import ConfigManager
from pydantic import BaseModel, Field
from typing import Optional

# 1. Define your configuration schema
# Note: Avoid using "model_config" as a field name in Pydantic models
# as it conflicts with Pydantic's internal configuration mechanism
class LLMConfig(BaseModel):
    model_name: str = Field(default="gpt-4", description="LLM model name")
    api_key: str = Field(default="", description="API key")
    api_base: Optional[str] = None

class MyFrameworkConfig(BaseModel):
    # Use "llm_config" instead of "model_config" to avoid conflicts
    llm_config: LLMConfig = Field(default_factory=LLMConfig)
    prompt_template: str = Field(default="Default prompt template")
    enable_feature: bool = Field(default=True)

# 2. Initialize the configuration manager with type parameter
config_manager = ConfigManager[MyFrameworkConfig](
    config_name="myframework",
    config_schema=MyFrameworkConfig,
    version="1.0.0",
    auto_create=True
)

# 3. Use the configuration in your application
def initialize_app():
    # Load the merged configuration (MyFrameworkConfig type is inferred)
    config = config_manager.load()
    
    # Access configuration values with full IDE autocompletion
    model_name = config.llm_config.model_name
    api_key = config.llm_config.api_key
    
    print(f"Using model: {model_name}")
    print(f"Feature enabled: {config.enable_feature}")
    
    # Get individual config levels - can be None if not available
    user_config = config_manager.get_user_config()
    if user_config:
        print(f"Using user-configured model: {user_config.llm_config.model_name}")
```

### Using Dataclasses

```python
from conftier import ConfigManager
from dataclasses import dataclass
from typing import Optional

# Define your configuration with dataclasses
@dataclass
class ModelConfig:
    model_name: str = "gpt-4"
    api_key: str = ""
    api_base: Optional[str] = None

@dataclass
class MyConfig:
    model_config: ModelConfig = ModelConfig()
    prompt_template: str = "Default template"
    enable_feature: bool = True

# Initialize with type parameter
config_manager = ConfigManager[MyConfig](
    config_name="myframework",
    config_schema=MyConfig,
    version="1.0.0",
    auto_create=True
)

# The IDE will now recognize that config is of type MyConfig
config = config_manager.load()

# Full IDE autocompletion and type checking
print(f"Using model: {config.model_config.model_name}")
print(f"Feature enabled: {config.enable_feature}")
```

### Advanced: Using ConfigModel Directly

For advanced use cases, you can use the ConfigModel class directly to work with configuration objects in a unified way, regardless of their underlying type (Pydantic, dataclass, or dict):

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

### For Framework Users

Users can create configuration files in these locations:

1. User-level config (applies to all projects): `~/.zeeland/myframework/config.yaml`
2. Project-level config (project-specific): `/.myframework/config.yaml` (in project root)

Example user configuration with Pydantic model:

```yaml
# ~/.zeeland/myframework/config.yaml
llm_config:
  model_name: "gpt-4-turbo"
  api_key: "user-api-key-123"
```

Example project configuration with Pydantic model:

```yaml
# /.myframework/config.yaml
llm_config:
  model_name: "gpt-3.5-turbo"  # Overrides user configuration
prompt_template: "Project specific prompt template"
```

Example configuration with dataclass model:

```yaml
# ~/.zeeland/myframework/config.yaml
model_config:
  model_name: "gpt-4-turbo"
  api_key: "user-api-key-123"
```

### Command Line Interface

Conftier provides a CLI for managing configurations:

```bash
# Initialize a project configuration template
conftier init-project myframework

# Show current configuration and sources
conftier show-config myframework

# Set a configuration value (user-level)
conftier set-config myframework --key llm_config.api_key --value "my-api-key"

# Set a configuration value (project-level)
conftier set-config myframework --key llm_config.model_name --value "gpt-3.5-turbo" --project
```

## Configuration Priority

Conftier follows a clear priority order when merging configurations:

1. Project-level configuration (highest priority)
2. User-level configuration
3. Default configuration (from schema)
