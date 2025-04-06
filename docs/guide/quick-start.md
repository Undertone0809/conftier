---
title: "Getting Started with Conftier: A Multi-Level Configuration Framework"
description: "Learn how to install and use Conftier, a powerful multi-level configuration framework for Python applications. Quick guide for framework and application developers."
head:
  - - meta
    - property: og:title
      content: "Getting Started with Conftier: A Multi-Level Configuration Framework"
  - - meta
    - property: og:description
      content: "Learn how to install and use Conftier, a powerful multi-level configuration framework for Python applications."
  - - meta
    - name: keywords
      content: "conftier, python configuration, multi-level config, configuration framework, pydantic config"
---

# Getting Started with Conftier

Conftier is a powerful multi-level configuration framework designed to manage configurations across different scopes. This quick start guide will help you understand which approach best suits your needs.

## Installation

You can install conftier using pip:

```bash
pip install conftier
```

### With Pydantic Support (Recommended)

Conftier recommends using Pydantic for defining configuration schemas as it provides robust validation, serialization, and documentation features.

```bash
# With Pydantic support
pip install conftier[pydantic]
```

### Using Poetry

```bash
# Basic installation
poetry add conftier

# With Pydantic support
poetry add conftier[pydantic]
```

## Choosing Your Path

Conftier supports two primary user journeys:

### 1. Framework Developer & Framework User Journey

This approach is ideal if you're developing a framework or library that will be used by others and needs to manage configurations at multiple levels (user-level and project-level).

**Key characteristics:**

- You're building a framework/library for others to use
- Users need to set global preferences and project-specific settings
- Multiple configuration levels (default → user → project)
- Your framework needs to provide CLI tools for config management

**Example scenarios:**

- Building an AI assistant framework where users set API keys globally but configure model parameters per project
- Creating a data processing framework with user-specific credentials and project-specific processing rules

[**View the Framework Developer & User Journey Guide →**](/guide/framework-journey)

### 2. Application Developer Journey

This approach is ideal if you're building a standalone application that needs configuration management, primarily at the project level.

**Key characteristics:**

- You're building a standalone application (not a framework)
- You primarily need project-level configuration
- You want to provide a structured way to manage application settings
- You need validation and schema enforcement

**Example scenarios:**

- Building a FastAPI backend with AI features that needs configuration for API parameters and AI settings
- Creating a data pipeline with configurable processing steps
- Developing a web application with environment-specific settings

[**View the Application Developer Journey Guide →**](/guide/application-journey)

## Basic Concepts

Regardless of your chosen journey, here are some key concepts to understand:

### Configuration Levels

Conftier supports multiple configuration levels:

1. **Default values** - Defined in your schema
2. **User-level configuration** - Global settings at `~/.zeeland/[config_name]/config.yaml`
3. **Project-level configuration** - Local settings at `./.[config_name]/config.yaml`

### Configuration Merging

When loading configuration, values are merged in this order (higher priority overrides lower):

1. Default schema values (lowest priority)
2. User-level configuration
3. Project-level configuration (highest priority)

## Next Steps

Choose the journey that best matches your use case from the links above. Each guide provides a comprehensive walkthrough with examples and best practices.

If you encounter any issues or have questions, please [open an issue](https://github.com/Undertone0809/conftier/issues) on our GitHub repository.
