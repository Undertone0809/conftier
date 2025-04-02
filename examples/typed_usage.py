"""
Example demonstrating the use of type hints with conftier
"""

from dataclasses import dataclass
from typing import Optional

from conftier import ConfigManager, ConfigModel


# Example using dataclasses with proper type hints
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
    """Example using dataclasses with proper type hints"""
    print("=== USING DATACLASSES WITH TYPE HINTS ===")

    # Use ConfigManager with type parameter
    config_manager = ConfigManager[ExampleConfig](
        framework_name="example_typed",
        config_schema=ExampleConfig,
        version="1.0.0",
        auto_create=True,
    )

    # The IDE will now properly recognize the type
    config = config_manager.load()

    # Now IDE will properly recognize the type
    print(f"Model Name: {config.model_config.model_name}")
    print(
        f"API Key: {'*' * len(config.model_config.api_key) if config.model_config.api_key else 'Not set'}"
    )
    print(f"API Base: {config.model_config.api_base}")
    print(f"Prompt Template: {config.prompt_template}")
    print(f"Feature Enabled: {config.enable_feature}")

    # Using ConfigModel directly
    config_model = ConfigModel.from_schema(ExampleConfig)
    print("\nUsing ConfigModel directly:")
    model_name = config_model.get_value("model_config.model_name")
    print(f"Model Name from ConfigModel: {model_name}")


# Using pydantic with type hints
try:
    from pydantic import BaseModel, Field

    # Note: In Pydantic v2, "model_config" is a reserved name for model configuration
    # Use a different name like "llm_config" to avoid conflicts
    class LLMConfig(BaseModel):
        model_name: str = Field(default="gpt-4", description="Model name")
        api_key: str = Field(default="", description="API key")
        api_base: Optional[str] = None

    class PydanticExampleConfig(BaseModel):
        # Using "llm_config" instead of "model_config" to avoid conflicts
        llm_config: LLMConfig = Field(default_factory=LLMConfig)
        prompt_template: str = Field(default="This is default prompt template")
        enable_feature: bool = Field(default=True)

    def main_with_pydantic():
        """Example using pydantic with proper type hints"""
        print("\n=== USING PYDANTIC WITH TYPE HINTS ===")

        # Initialize with type annotation - directly use the generic ConfigManager
        config_manager = ConfigManager[PydanticExampleConfig](
            framework_name="example_pydantic_typed",
            config_schema=PydanticExampleConfig,
            version="1.0.0",
            auto_create=True,
        )

        # Load with proper type inference
        config = config_manager.load()

        # Now IDE will properly recognize all properties
        print(f"Model Name: {config.llm_config.model_name}")
        print(
            f"API Key: {'*' * len(config.llm_config.api_key) if config.llm_config.api_key else 'Not set'}"
        )
        print(f"API Base: {config.llm_config.api_base}")
        print(f"Prompt Template: {config.prompt_template}")
        print(f"Feature Enabled: {config.enable_feature}")

        # Show configuration sources handling
        user_config = config_manager.get_user_config()
        if user_config:
            print(f"\nUser config exists: {user_config.llm_config.model_name}")
        else:
            print("\nNo user config found")

        project_config = config_manager.get_project_config()
        if project_config:
            print(f"Project config exists: {project_config.llm_config.model_name}")
        else:
            print("No project config found")

        # Using ConfigModel directly with Pydantic
        config_model = ConfigModel.from_schema(PydanticExampleConfig)
        print("\nUsing ConfigModel directly with Pydantic:")
        config_dict = config_model.to_dict()
        print(f"ConfigModel as dict: {config_dict['llm_config']['model_name']}")

except ImportError:

    def main_with_pydantic():
        print("\n=== PYDANTIC NOT AVAILABLE ===")
        print("Install pydantic to run this example: pip install pydantic")


if __name__ == "__main__":
    main_with_dataclass()
    main_with_pydantic()
