# API Overview

This section provides detailed information about the conftier API.

## Introduction

Conftier is a multi-level configuration management framework that simplifies handling layered configurations in Python applications. The API is designed to be intuitive and flexible, supporting different schema definition approaches including dataclasses and Pydantic models.

## Core Components

### ConfigManager

The central class that manages all configuration operations:

```python
from conftier import ConfigManager

config_manager = ConfigManager(
    config_name="your_framework",
    config_schema=YourConfigClass,
    version="1.0.0",
    auto_create=True
)
```

#### Parameters

- `config_name`: Name of your framework/application (used for directory structure)
- `config_schema`: Configuration schema definition class (dataclass or Pydantic model)
- `version`: Configuration schema version
- `auto_create`: Whether to automatically create config directories and files

#### Key Methods

- `load()`: Loads and merges configurations from all levels
- `get_default_config()`: Returns the default configuration
- `get_user_config()`: Returns the user-level configuration
- `get_project_config()`: Returns the project-level configuration
- `update_user_config(config_update)`: Updates the user-level configuration
- `update_project_config(config_update)`: Updates the project-level configuration
- `create_project_template(path)`: Creates a project configuration template

## Configuration Levels

Conftier manages configurations at three levels, with a clear priority order:

1. **Default Configuration**: Defined by the schema's default values
2. **User-level Configuration**: Stored at `~/.zeeland/{config_name}/config.yaml`
3. **Project-level Configuration**: Stored at `/.{config_name}/config.yaml` in the project root

Higher priority configurations only overwrite corresponding keys, without affecting other keys.

## Supported Schema Formats

Conftier supports multiple ways to define your configuration schema:

### Dataclasses

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class ModelConfig:
    model_name: str = "gpt-4"
    api_key: str = ""
    api_base: Optional[str] = None

@dataclass
class ExampleConfig:
    model_config: ModelConfig = ModelConfig()
    prompt_template: str = "This is default prompt template"
    enable_feature: bool = True
```

### Pydantic Models

```python
from typing import Optional
from pydantic import BaseModel, Field

class LLMConfig(BaseModel):
    model_name: str = Field(default="gpt-4", description="Model name")
    api_key: str = Field(default="", description="API key")
    api_base: Optional[str] = None

class PydanticExampleConfig(BaseModel):
    llm_config: LLMConfig = Field(default_factory=LLMConfig)
    prompt_template: str = Field(default="This is default prompt template")
    enable_feature: bool = Field(default=True)
```

## Utility Functions

Conftier provides several utility functions for working with configuration files:

- `get_user_config_path(config_name)`: Gets the user-level configuration file path
- `get_project_config_path(config_name, project_path)`: Gets the project-level configuration file path
- `find_project_root()`: Finds the project root directory

## Next Steps

Check out the [Examples](/api/examples) section for detailed code samples demonstrating common usage patterns.
