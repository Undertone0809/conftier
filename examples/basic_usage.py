"""
Basic usage example of conftier.
"""

from dataclasses import dataclass
from typing import Optional

from conftier import ConfigManager


# Option 1: Using dataclasses
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


def main_with_dataclass():
    """Example using dataclasses"""
    print("=== USING DATACLASSES ===")

    # Initialize the configuration manager with type hints
    config_manager = ConfigManager(
        framework_name="example",
        config_schema=ExampleConfig,
        version="1.0.0",
        auto_create=True,
    )

    # Load the merged configuration
    config = config_manager.load()

    # Print configuration details
    print(f"Model Name: {config.model_config.model_name}")
    print(
        f"API Key: {'*' * len(config.model_config.api_key) if config.model_config.api_key else 'Not set'}"
    )
    print(f"API Base: {config.model_config.api_base}")
    print(f"Prompt Template: {config.prompt_template}")
    print(f"Feature Enabled: {config.enable_feature}")

    # Show the source of configurations
    print("\nConfiguration Sources:")
    default_config = config_manager.get_default_config()
    print(f"Default config: model_name={default_config.model_config.model_name}")

    user_config = config_manager.get_user_config()
    if user_config:
        print(f"User config: model_name={user_config.model_config.model_name}")
    else:
        print("User config: None")

    project_config = config_manager.get_project_config()
    if project_config:
        print(f"Project config: model_name={project_config.model_config.model_name}")
    else:
        print("Project config: None")


# Option 2: Using pydantic (if available)
try:
    from pydantic import BaseModel, Field

    # Note: In Pydantic, naming a field "model_config" conflicts with Pydantic's own configuration.
    # Instead, use a different name like "llm_config" or "ai_model"
    class LLMConfig(BaseModel):
        model_name: str = Field(default="gpt-4", description="Model name")
        api_key: str = Field(default="", description="API key")
        api_base: Optional[str] = None

    class PydanticExampleConfig(BaseModel):
        # Renamed from "model_config" to "llm_config" to avoid conflicts with Pydantic
        llm_config: LLMConfig = Field(default_factory=LLMConfig)
        prompt_template: str = Field(default="This is default prompt template")
        enable_feature: bool = Field(default=True)

    def main_with_pydantic():
        """Example using pydantic"""
        print("\n=== USING PYDANTIC ===")

        # Initialize the configuration manager with type hints
        config_manager = ConfigManager[PydanticExampleConfig](
            framework_name="example_pydantic",
            config_schema=PydanticExampleConfig,
            version="1.0.0",
            auto_create=True,
        )

        # Load the merged configuration
        config = config_manager.load()

        # Print configuration details
        print(f"Model Name: {config.llm_config.model_name}")
        print(
            f"API Key: {'*' * len(config.llm_config.api_key) if config.llm_config.api_key else 'Not set'}"
        )
        print(f"API Base: {config.llm_config.api_base}")
        print(f"Prompt Template: {config.prompt_template}")
        print(f"Feature Enabled: {config.enable_feature}")

        # Update a configuration value
        config_manager.update_user_config(
            {"llm_config": {"model_name": "gpt-3.5-turbo"}}
        )

        # Reload the configuration
        config = config_manager.load()
        print("\nAfter update:")
        print(f"Model Name: {config.llm_config.model_name}")

except ImportError:

    def main_with_pydantic():
        print("\n=== PYDANTIC NOT AVAILABLE ===")
        print("Install pydantic to run this example: pip install pydantic")


if __name__ == "__main__":
    main_with_dataclass()
    main_with_pydantic()
