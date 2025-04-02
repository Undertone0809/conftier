---
title: Introduction
description: Introduction to conftier(A powerful multi-level configuration management framework)
head:
  - - meta
    - property: og:title
      content: Introduction to conftier(A powerful multi-level configuration management framework)
    - property: og:description
      content: Introduction to conftier(A powerful multi-level configuration management framework)
---

<p align="center">
   <img src="/banner.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

# Getting Started

Welcome to the conftier documentation! This guide will help you get started with conftier and show you how to use its features effectively.

## Overview

Conftier is a powerful multi-level configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications. It supports the intelligent merging of user preferences and project settings, providing a complete mechanism for framework developers to build user-level and project-level configuration management systems.

Key features include:

- **User-level configuration**: Store personal preferences effective across projects (~/.zeeland/{config_name}/config.yaml)
- **Project-level configuration**: Store project-specific settings for team collaboration (/.{config_name}/config.yaml)
- **Priority mechanism**: Clear rules where project-level overrides user-level, which overrides defaults
- **Multiple schema formats**: Support for dataclasses, Pydantic models, and dictionaries
- **Automatic file generation**: Auto-creation of configuration files and directories

## Prerequisites

Before you begin, make sure you have:

- Python 3.8 or higher installed
- pip or Poetry for package management

## Installation

You can install conftier using pip:

```bash
pip install conftier
```

Or if you prefer using Poetry:

```bash
poetry add conftier
```

## Basic Usage

Here's a simple example using dataclasses:

```python
from dataclasses import dataclass
from typing import Optional
from conftier import ConfigManager

# Define your configuration schema with dataclasses
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

# Initialize the configuration manager
config_manager = ConfigManager(
    config_name="example",
    config_schema=ExampleConfig,
    version="1.0.0",
    auto_create=True,
)

# Load the merged configuration
config = config_manager.load()

# Use the configuration
print(f"Model Name: {config.model_config.model_name}")
print(f"Prompt Template: {config.prompt_template}")
```

## Using Pydantic Models

If you prefer Pydantic for schema definition:

```python
from typing import Optional
from pydantic import BaseModel, Field
from conftier import ConfigManager

# Define your configuration schema with Pydantic
class LLMConfig(BaseModel):
    model_name: str = Field(default="gpt-4", description="Model name")
    api_key: str = Field(default="", description="API key")
    api_base: Optional[str] = None

class PydanticExampleConfig(BaseModel):
    llm_config: LLMConfig = Field(default_factory=LLMConfig)
    prompt_template: str = Field(default="This is default prompt template")
    enable_feature: bool = Field(default=True)

# Initialize the configuration manager
config_manager = ConfigManager(
    config_name="example_pydantic",
    config_schema=PydanticExampleConfig,
    version="1.0.0",
    auto_create=True,
)

# Load the merged configuration
config = config_manager.load()

# Update a configuration value
config_manager.update_user_config(
    {"llm_config": {"model_name": "gpt-3.5-turbo"}}
)
```

## Next Steps

Now that you have conftier installed, you can:

1. Check out the [Installation](./installation.md) guide for more detailed setup instructions
2. Explore the [API Reference](/api/) for detailed information about all available functions and classes
3. Look at the [Examples](/api/examples) to see conftier in action

If you encounter any issues or have questions, please [open an issue](https://github.com/Undertone0809/conftier/issues) on our GitHub repository.
