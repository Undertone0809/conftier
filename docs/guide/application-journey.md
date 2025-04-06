---
title: "Application Developer Guide for Conftier Configuration Management"
description: "Learn how to implement Conftier in your Python applications for robust configuration management across different environments. Step-by-step tutorial with FastAPI example."
head:
  - - meta
    - property: og:title
      content: "Application Developer Guide for Conftier Configuration Management"
  - - meta
    - property: og:description
      content: "Learn how to implement Conftier in your Python applications for robust configuration management across different environments."
  - - meta
    - name: keywords
      content: "conftier application, python app configuration, configuration management, fastapi configuration, environment config"
---

# Application Developer Journey

This guide covers the journey for application developers who want to manage configuration for their applications using conftier. Unlike framework developers who need to support multiple users with different configuration levels, application developers typically focus on project-level configuration for their specific applications.

## Overview

As an application developer, you'll primarily use conftier to:

1. Define a structured configuration schema for your application
2. Load and validate configuration values
3. Manage configuration across different environments (development, production, etc.)

```mermaid
sequenceDiagram
  participant Dev as App Developer
  participant Conftier as Conftier Framework
  participant App as Application
  
  Note over Dev: 1. Configuration Definition
  Dev->>Conftier: Define configuration schema<br/>(Pydantic/Dataclass)
  Note right of Dev: AppConfig with API settings,<br/>database configs, feature flags
  
  Note over Dev: 2. Integration Setup
  Dev->>Conftier: Initialize ConfigManager<br/>(name="myapp", schema=AppConfig)
  Conftier-->>Dev: Configuration manager ready
  
  Note over Dev: 3. Configuration Usage
  Dev->>Conftier: Load configuration
  Conftier-->>Dev: Return configuration (from project or defaults)
  Dev->>App: Initialize app with configuration
  
  Note over Dev: 4. Environment Management
  Dev->>Conftier: Set up different project configs<br/>for different environments
  Note right of Dev: Create templates or CI scripts for<br/>dev/staging/production
```

## Example: Building a FastAPI AI Application

Throughout this guide, we'll use an example of building a FastAPI application with AI features that needs to configure:

- FastAPI server settings
- Database connection details
- AI model parameters
- Logging preferences

### 1. Define Your Configuration Schema

First, let's define the configuration schema using Pydantic models:

```python
from conftier import ConfigManager
from pydantic import BaseModel, Field
from typing import Optional, List

# Define your configuration schema
class ServerConfig(BaseModel):
    host: str = Field(default="0.0.0.0", description="Server host")
    port: int = Field(default=8000, description="Server port")
    workers: int = Field(default=4, description="Number of worker processes")
    debug: bool = Field(default=False, description="Debug mode")

class DatabaseConfig(BaseModel):
    url: str = Field(default="sqlite:///./app.db", description="Database URL")
    echo: bool = Field(default=False, description="Echo SQL commands")
    pool_size: int = Field(default=5, description="Connection pool size")

class AIModelConfig(BaseModel):
    model_name: str = Field(default="gpt-3.5-turbo", description="AI model name")
    temperature: float = Field(default=0.7, description="Model temperature")
    api_key: str = Field(default="", description="API key")
    max_tokens: int = Field(default=2000, description="Maximum number of tokens")
    
class LoggingConfig(BaseModel):
    level: str = Field(default="INFO", description="Logging level")
    file: Optional[str] = Field(default=None, description="Log file path")

class AppConfig(BaseModel):
    server: ServerConfig = Field(default_factory=ServerConfig)
    database: DatabaseConfig = Field(default_factory=DatabaseConfig)
    ai_model: AIModelConfig = Field(default_factory=AIModelConfig)
    logging: LoggingConfig = Field(default_factory=LoggingConfig)
    enable_features: List[str] = Field(default=["basic", "api"], description="Enabled feature flags")
```

This schema defines the structure of your application's configuration. When you create a configuration file, it will follow this structure:

```yaml
# Example YAML structure
server:
  host: 0.0.0.0
  port: 8000
  workers: 4
  debug: false

database:
  url: sqlite:///./app.db
  echo: false
  pool_size: 5

ai_model:
  model_name: gpt-3.5-turbo
  temperature: 0.7
  api_key: ""
  max_tokens: 2000
  
logging:
  level: INFO
  file: null

enable_features:
  - basic
  - api
```

### 2. Initialize the Configuration Manager

Now, let's create a ConfigManager for your application:

```python
# In myapp/config.py
from myapp.schema import AppConfig

config_manager = ConfigManager(
    config_name="myapp",
    config_schema=AppConfig,
    version="1.0.0",
    auto_create_project=True  # Automatically create project config if it doesn't exist
)
```

When `auto_create_project=True`:

- If the project-level config file doesn't exist at `./.myapp/config.yaml`, it will be created with default values
- This ensures your application always has a configuration file to work with

### 3. Loading and Using Configuration

Now let's set up the main application initialization that loads and uses the configuration:

```python
# In myapp/app.py
from fastapi import FastAPI
from myapp.config import config_manager
from myapp.database import setup_database
from myapp.logging import setup_logging
from myapp.ai import setup_ai_model

def create_app():
    # Load the configuration (or use config_manager.config)
    config = config_manager.load()
    
    # Set up logging first
    setup_logging(config.logging)
    
    # Create the FastAPI app with configuration
    app = FastAPI(
        title="MyApp",
        description="FastAPI Application with AI Features",
        debug=config.server.debug
    )
    
    # Set up database
    db = setup_database(config.database)
    
    # Set up AI model client
    ai_client = setup_ai_model(config.ai_model)
    
    # Attach configuration and components to app for use in route handlers
    app.state.config = config
    app.state.db = db
    app.state.ai_client = ai_client
    
    # Enable features based on configuration
    for feature in config.enable_features:
        enable_feature(app, feature)
    
    return app

# In main.py (app entrypoint)
import uvicorn
from myapp.app import create_app
from myapp.config import config_manager

app = create_app()

if __name__ == "__main__":
    # Get configuration for the server
    config = config_manager.load()
    server_config = config.server
    
    # Run the server with the configured settings
    uvicorn.run(
        "main:app",
        host=server_config.host,
        port=server_config.port,
        workers=server_config.workers
    )
```

### 4. Managing Environment-Specific Configurations

For different environments (development, testing, production), simply edit the `./.myapp/config.yaml` file directly with the appropriate settings:

```yaml
# Development environment example (./.myapp/config.yaml)
server:
  debug: true
  port: 8888
database:
  url: sqlite:///./dev.db
  echo: true

# Production environment example (./.myapp/config.yaml)
# server:
#   host: 0.0.0.0
#   port: 80
#   workers: 8
#   debug: false
# database:
#   url: postgresql://user:pass@localhost/prod_db
#   pool_size: 20
# logging:
#   level: WARNING
#   file: /var/log/myapp.log
```

You can maintain different config files for different environments in your project repository (e.g., `config.dev.yaml`, `config.prod.yaml`) and simply copy the appropriate one to `./.myapp/config.yaml` when deploying:

```bash
# For development
cp config.dev.yaml ./.myapp/config.yaml

# For production
cp config.prod.yaml ./.myapp/config.yaml
```

This approach is simple and transparent, allowing developers to easily see and modify configuration for different environments.

### 5. Environment Variables and Secrets

For sensitive values like API keys, it's better to use environment variables. You can add environment variable support:

```python
# In myapp/config.py
import os
from conftier import ConfigManager
from myapp.schema import AppConfig

# Initialize the config manager as before
config_manager = ConfigManager(
    config_name="myapp",
    config_schema=AppConfig,
    version="1.0.0",
    auto_create_project=True
)

def load_config_with_env():
    """Load configuration with environment variable overrides."""
    config = config_manager.load()
    
    # Override with environment variables if they exist
    if os.environ.get("MYAPP_AI_API_KEY"):
        config.ai_model.api_key = os.environ.get("MYAPP_AI_API_KEY")
        
    if os.environ.get("MYAPP_DB_URL"):
        config.database.url = os.environ.get("MYAPP_DB_URL")
        
    if os.environ.get("MYAPP_LOG_LEVEL"):
        config.logging.level = os.environ.get("MYAPP_LOG_LEVEL")
    
    return config
```

Then in your app, use this enhanced loader:

```python
# In myapp/app.py
from myapp.config import load_config_with_env

def create_app():
    # Load config with environment variable overrides
    config = load_config_with_env()
    # ... rest of the app initialization
```

## CLI Integration

You can add configuration management commands to your application's CLI using click:

```python
# In myapp/cli.py
import click
from myapp.config import config_manager

@click.group()
def cli():
    """MyApp CLI"""
    pass

@cli.command()
def init_config():
    """Initialize application configuration."""
    config_path = config_manager.create_project_config_template()
    print(f"Configuration template created at {config_path}")
    print("Edit this file to customize your application settings.")

# Register conftier's built-in config commands
from conftier.cli import register_config_commands
register_config_commands(
    cli,
    config_manager=config_manager,
    command_prefix="config"  # Creates commands like "myapp config show"
)

if __name__ == "__main__":
    cli()
```

Now users of your application can run:

```bash
# Initialize a configuration template
myapp init-config

# Show current configuration
myapp config show

# Set a configuration value
myapp config set --key server.port --value 5000
```

## Best Practices for Application Configuration

1. **Keep secrets out of configuration files**
   - Use environment variables for sensitive values like API keys and credentials
   - Provide placeholders in configuration templates

2. **Version control considerations**
   - Add `./.myapp/config.yaml` to your `.gitignore` file
   - Include a `config.example.yaml` file in version control as a template

3. **Documentation**
   - Document all configuration options
   - Provide example configurations for different environments

4. **Validation**
   - Use Pydantic's validation capabilities to catch configuration errors early
   - Add custom validation hooks for complex rules

5. **Testing**
   - Create test configurations programmatically
   - Test your application with different configuration values

## Next Steps

After setting up your application's configuration system, you might want to explore:

1. **Docker integration** - Using conftier to manage configurations in containerized environments
2. **CI/CD pipelines** - Generating different configurations for different deployment environments
3. **Feature flags** - Using configuration to enable/disable features in your application
