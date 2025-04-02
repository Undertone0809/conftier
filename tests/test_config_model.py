"""
Unit tests for the ConfigModel class
"""

import os
import tempfile
from dataclasses import dataclass
from typing import Optional

import pytest

from conftier import ConfigModel


# Test Data Models
@dataclass
class NestedConfig:
    name: str = "nested"
    value: int = 42


@dataclass
class DataclassConfig:
    title: str = "dataclass"
    enabled: bool = True
    number: int = 100
    nested: NestedConfig = NestedConfig()


try:
    from pydantic import BaseModel, Field

    class NestedPydanticConfig(BaseModel):
        name: str = Field(default="nested")
        value: int = Field(default=42)

    class PydanticConfig(BaseModel):
        title: str = Field(default="pydantic")
        enabled: bool = Field(default=True)
        number: int = Field(default=100)
        nested: NestedPydanticConfig = Field(default_factory=NestedPydanticConfig)

    PYDANTIC_AVAILABLE = True
except ImportError:
    PYDANTIC_AVAILABLE = False


class TestConfigModel:
    """Tests for the ConfigModel class"""

    def test_from_schema_dataclass(self):
        """Test creating ConfigModel from a dataclass schema"""
        # Create with default values
        config_model = ConfigModel.from_schema(DataclassConfig)

        assert config_model.schema_type == "dataclass"
        assert isinstance(config_model.model, DataclassConfig)
        assert config_model.model.title == "dataclass"
        assert config_model.model.enabled == True
        assert config_model.model.number == 100
        assert config_model.model.nested.name == "nested"

        # Create with custom values
        custom_data = {
            "title": "custom",
            "enabled": False,
            "number": 200,
            "nested": {"name": "custom_nested", "value": 99},
        }

        config_model = ConfigModel.from_schema(DataclassConfig, custom_data)
        assert config_model.model.title == "custom"
        assert config_model.model.enabled == False
        assert config_model.model.number == 200
        assert config_model.model.nested.name == "custom_nested"
        assert config_model.model.nested.value == 99

    def test_from_schema_dict(self):
        """Test creating ConfigModel from a dict schema"""
        # Default dict schema
        default_dict = {"name": "default", "value": 123}

        # Create with default values
        config_model = ConfigModel.from_schema(default_dict)

        assert config_model.schema_type == "dict"
        assert isinstance(config_model.model, dict)
        assert config_model.model["name"] == "default"
        assert config_model.model["value"] == 123

        # Create with custom values
        custom_data = {"name": "custom", "value": 456, "extra": True}
        config_model = ConfigModel.from_schema(dict, custom_data)

        assert config_model.model["name"] == "custom"
        assert config_model.model["value"] == 456
        assert config_model.model["extra"] == True

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_from_schema_pydantic(self):
        """Test creating ConfigModel from a Pydantic schema"""
        # Create with default values
        config_model = ConfigModel.from_schema(PydanticConfig)

        assert config_model.schema_type == "pydantic"
        assert isinstance(config_model.model, PydanticConfig)
        assert config_model.model.title == "pydantic"
        assert config_model.model.enabled == True
        assert config_model.model.number == 100
        assert config_model.model.nested.name == "nested"

        # Create with custom values
        custom_data = {
            "title": "custom",
            "enabled": False,
            "number": 200,
            "nested": {"name": "custom_nested", "value": 99},
        }

        config_model = ConfigModel.from_schema(PydanticConfig, custom_data)
        assert config_model.model.title == "custom"
        assert config_model.model.enabled == False
        assert config_model.model.number == 200
        assert config_model.model.nested.name == "custom_nested"
        assert config_model.model.nested.value == 99

    def test_to_dict(self):
        """Test converting ConfigModel to dictionary"""
        # Test with dataclass
        config_model = ConfigModel.from_schema(DataclassConfig)
        config_dict = config_model.to_dict()

        assert isinstance(config_dict, dict)
        assert config_dict["title"] == "dataclass"
        assert config_dict["enabled"] == True
        assert config_dict["number"] == 100
        assert config_dict["nested"]["name"] == "nested"
        assert config_dict["nested"]["value"] == 42

        # Test with dict
        dict_model = ConfigModel.from_schema({"a": 1, "b": 2})
        dict_result = dict_model.to_dict()

        assert isinstance(dict_result, dict)
        assert dict_result["a"] == 1
        assert dict_result["b"] == 2

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_to_dict_pydantic(self):
        """Test converting Pydantic ConfigModel to dictionary"""
        config_model = ConfigModel.from_schema(PydanticConfig)
        config_dict = config_model.to_dict()

        assert isinstance(config_dict, dict)
        assert config_dict["title"] == "pydantic"
        assert config_dict["enabled"] == True
        assert config_dict["number"] == 100
        assert config_dict["nested"]["name"] == "nested"
        assert config_dict["nested"]["value"] == 42

    def test_get_value(self):
        """Test getting values from ConfigModel"""
        # Test with dataclass
        config_model = ConfigModel.from_schema(DataclassConfig)

        assert config_model.get_value("title") == "dataclass"
        assert config_model.get_value("enabled") == True
        assert config_model.get_value("nested.name") == "nested"
        assert config_model.get_value("nested.value") == 42
        assert config_model.get_value("nonexistent") is None
        assert config_model.get_value("nested.nonexistent") is None

        # Test with dict
        dict_model = ConfigModel.from_schema({"a": 1, "b": {"c": 2}})

        assert dict_model.get_value("a") == 1
        assert dict_model.get_value("b.c") == 2
        assert dict_model.get_value("nonexistent") is None

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_get_value_pydantic(self):
        """Test getting values from Pydantic ConfigModel"""
        config_model = ConfigModel.from_schema(PydanticConfig)

        assert config_model.get_value("title") == "pydantic"
        assert config_model.get_value("enabled") == True
        assert config_model.get_value("nested.name") == "nested"
        assert config_model.get_value("nested.value") == 42
        assert config_model.get_value("nonexistent") is None

    def test_update(self):
        """Test updating ConfigModel with new values"""
        # Test with dataclass
        config_model = ConfigModel.from_schema(DataclassConfig)

        config_model.update({"title": "updated", "nested": {"name": "updated_nested"}})

        assert config_model.model.title == "updated"
        assert config_model.model.nested.name == "updated_nested"
        # Other values should remain unchanged
        assert config_model.model.enabled == True
        assert config_model.model.nested.value == 42

        # Test with dict
        dict_model = ConfigModel.from_schema({"a": 1, "b": {"c": 2}})
        dict_model.update({"a": 10, "b": {"d": 3}})

        assert dict_model.model["a"] == 10
        assert dict_model.model["b"]["c"] == 2
        assert dict_model.model["b"]["d"] == 3

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_update_pydantic(self):
        """Test updating Pydantic ConfigModel"""
        config_model = ConfigModel.from_schema(PydanticConfig)

        config_model.update({"title": "updated", "nested": {"name": "updated_nested"}})

        assert config_model.model.title == "updated"
        assert config_model.model.nested.name == "updated_nested"
        # Other values should remain unchanged
        assert config_model.model.enabled == True
        assert config_model.model.nested.value == 42

    def test_merge(self):
        """Test merging two ConfigModels"""
        # Base config
        base_config = ConfigModel.from_schema(DataclassConfig)

        # Override config
        override_data = {"title": "override", "nested": {"value": 999}}
        override_config = ConfigModel.from_schema(DataclassConfig, override_data)

        # Merge configs
        merged_config = base_config.merge(override_config)

        # Check merged values
        assert merged_config.model.title == "override"  # Overridden
        assert merged_config.model.enabled == True  # From base
        assert merged_config.model.number == 100  # From base
        assert merged_config.model.nested.name == "nested"  # From base
        assert merged_config.model.nested.value == 999  # Overridden

        # Test with dict
        base_dict = ConfigModel.from_schema({"a": 1, "b": {"c": 2, "d": 3}})
        override_dict = ConfigModel.from_schema({"a": 10, "b": {"d": 30, "e": 4}})

        merged_dict = base_dict.merge(override_dict)

        assert merged_dict.model["a"] == 10
        assert merged_dict.model["b"]["c"] == 2
        assert merged_dict.model["b"]["d"] == 30
        assert merged_dict.model["b"]["e"] == 4

    @pytest.mark.skipif(not PYDANTIC_AVAILABLE, reason="Pydantic not installed")
    def test_merge_pydantic(self):
        """Test merging two Pydantic ConfigModels"""
        # Base config
        base_config = ConfigModel.from_schema(PydanticConfig)

        # Override config
        override_data = {"title": "override", "nested": {"value": 999}}
        override_config = ConfigModel.from_schema(PydanticConfig, override_data)

        # Merge configs
        merged_config = base_config.merge(override_config)

        # Check merged values
        assert merged_config.model.title == "override"  # Overridden
        assert merged_config.model.enabled == True  # From base
        assert merged_config.model.number == 100  # From base
        assert merged_config.model.nested.name == "nested"  # From base
        assert merged_config.model.nested.value == 999  # Overridden
