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

# Conftier

A powerful multi-tier configuration management framework that simplifies the definition, access, and synchronization of layered configurations in Python applications.

Think of VSCode's configuration system: you have user settings that apply globally and workspace settings that override them for specific projects. Conftier brings this same intuitive model to your Python frameworks and applications.

## Documentation

For comprehensive guides, examples, and API reference, visit our documentation:

- [Introduction](https://conftier.zeeland.top/)
- [Quick Start Guide](https://conftier.zeeland.top/guide/quick-start.html)
- [Contributing](https://conftier.zeeland.top/other/contributing.html)

## Overview

Conftier helps you manage configurations across multiple levels:

- **User-level settings**: Global preferences that apply across all projects (~/.zeeland/{config_name}/config.yaml)
- **Project-level settings**: Local configurations specific to a project (./.{config_name}/config.yaml)
- **Default values**: Fallback values defined in your configuration schema

Conftier automatically merges these configurations based on priority (project > user > default).

<p align="center">
   <img src="./docs/public/multi-config.png" alt="conftier Banner" style="border-radius: 15px;">
</p>

## Key Features

- **Multi-level Configuration Management**: Like VSCode's user/workspace settings pattern
- **Flexible Schema Definition**: Use Pydantic models or dataclasses to define and validate configurations
- **Type Safety**: No more string/int confusion or missing required fields
- **Smart Merging**: Only override what's specified, preserving other values
- **CLI Integration**: Built-in command-line tools for configuration management
- **IDE Autocompletion**: Full type hints for a great developer experience

## Why Conftier?

| Without Conftier | With Conftier |
|------------------|---------------|
| Manual parsing of multiple config files | Automatic loading and merging |
| Type errors discovered at runtime | Validation at load time |
| Custom code for merging configs | Smart merging built-in |
| Documentation struggles | Schema serves as documentation |
| Repetitive boilerplate | Consistent, reusable pattern |

## Installation

```bash
# Basic installation
pip install conftier

# With Pydantic support (recommended)
pip install conftier[pydantic]
```

## Quick Example

```python
from pydantic import BaseModel, Field
from conftier import ConfigManager

class AppConfig(BaseModel):
    app_name: str = "MyApp"
    debug: bool = False

config_manager = ConfigManager(
    config_name="myapp",
    config_schema=AppConfig,
    auto_create=True
)

# Load the merged configuration
config: AppConfig = config_manager.load()
```

## When to Use Conftier

Conftier shines when:

1. **You're building a framework or library**: Give your users a consistent way to configure your tool
2. **Your app has both user and project settings**: Like VSCode's personal vs. project-specific settings
3. **You need schema validation**: Ensure configuration values have the correct types and valid ranges
4. **You want to reduce boilerplate**: Stop writing the same configuration loading code in every project

## 🛡 License

[![License](https://img.shields.io/github/license/Undertone0809/conftier)](https://github.com/Undertone0809/conftier/blob/main/LICENSE)

This project is licensed under the terms of the `MIT` license.
See [LICENSE](https://github.com/Undertone0809/conftier/blob/main/LICENSE) for more details.

## 🤝 Support

For more information, please
contact: [zeeland4work@gmail.com](mailto:zeeland4work@gmail.com)

## Credits [![🚀 Your next Python package needs a bleeding-edge project structure.](https://img.shields.io/badge/P3G-%F0%9F%9A%80-brightgreen)](https://github.com/Undertone0809/python-package-template)

This project was generated with [P3G](https://github.com/Undertone0809/P3G)
