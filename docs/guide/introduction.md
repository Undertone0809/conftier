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

## What is Conftier?

Conftier is a powerful multi-tier configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications.

Think of VSCode's configuration system: you have user settings that apply globally and workspace settings that override them for specific projects. Conftier brings this same intuitive model to your Python frameworks and applications.

```python
# Define your configuration schema with Pydantic
class LLMConfig(BaseModel):
    model_name: str = "gpt-4"
    api_key: str = ""
    temperature: float = 0.7

# Access the merged configuration
config = config_manager.load()
model_name = config.llm_config.model_name  # Automatically picks the highest priority value
```

## Why Conftier?

### The Problem

Have you ever faced these challenges in your Python applications?

- **Global vs. Local Settings**: Users want their preferences everywhere, but projects need specific overrides
- **Configuration Complexity**: Managing multiple YAML files, ENV variables, and command-line arguments
- **Type Safety**: Dealing with string values when you need integers, or missing required settings
- **Documentation**: Explaining to users what configuration options are available and where to set them

Here's a familiar example:

```yaml
# User's global preferences (~/.myapp/config.yaml)
theme: "dark"
editor:
  font_size: 14
  tab_width: 2

# Project-specific settings (./project/.myapp/config.yaml)
editor:
  tab_width: 4  # Override just this value for this project
debug: true     # Project-only setting
```

Conftier makes this pattern easy to implement in your Python applications.

### Real-World Use Cases

#### AI Framework Configuration

```python
# Define schema once
class OpenAIConfig(BaseModel):
    api_key: str = ""
    model: str = "gpt-4"
    temperature: float = 0.7

# Users set their API key globally
# ~/.zeeland/myai/config.yaml
api_key: "user-api-key-123"

# Projects override the model
# ./.myai/config.yaml
model: "gpt-3.5-turbo"
```

#### CLI Tool Settings

```python
# Default tool settings
@dataclass
class ToolConfig:
    output_format: str = "json"
    verbose: bool = False
    cache_dir: str = "~/.cache/mytool"

# User's preferred CLI experience (applies to all projects)
# ~/.zeeland/mytool/config.yaml
output_format: "yaml"
verbose: true

# Project-specific needs
# ./.mytool/config.yaml
output_format: "csv"  # This project requires CSV output
```

#### Web Framework Configuration

```python
# Database connection defaults
class DBConfig(BaseModel):
    host: str = "localhost"
    port: int = 5432
    username: str = ""
    password: str = ""
    pool_size: int = 10

# Development environment (user config)
# ~/.zeeland/mywebapp/config.yaml
host: "localhost"
username: "dev_user"
password: "dev_password"

# Production deployment (project config)
# ./.mywebapp/config.yaml  
host: "production.db"
port: 5433
pool_size: 25
```

<p align="center">
   <img src="/web-config.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

## Key Features

- **Multi-level Configuration Management**: Like VSCode's user/workspace settings, handle default, user-level, and project-level configurations.

- **Flexible Schema Definition**: Use Pydantic models for validation or simpler dataclasses - your choice.

- **Priority Mechanism**: Project settings override user settings, which override defaults.

- **Type Safety**: No more string/int confusion or missing required fields - everything is validated.

- **Smart Merging**: Only override what's specified, preserving all other values.

- **CLI Integration**: Add configuration commands to your CLI tools with a single function call.

- **IDE Autocompletion**: Full type hints for a great developer experience.

<p align="center">
   <img src="/multi-config.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

## When to Use Conftier

Conftier shines when:

1. **You're building a framework or library**: Give your users a consistent way to configure your tool.

2. **Your app has both user preferences and project settings**: Like VSCode, where some settings are personal and others are project-specific.

3. **You need schema validation**: Ensure configuration values are the correct types and within valid ranges.

4. **You want to reduce boilerplate**: Stop writing the same configuration loading code in every project.

## Benefits at a Glance

| Without Conftier | With Conftier |
|------------------|---------------|
| Manual parsing of multiple config files | Automatic loading and merging |
| Type errors discovered at runtime | Validation at load time |
| Custom code for merging configs | Smart merging built-in |
| Documentation struggles | Schema serves as documentation |
| Repetitive boilerplate | Consistent, reusable pattern |

Ready to get started? Head to the [Quick Start Guide](./quick-start.md) to begin using Conftier in your projects.
