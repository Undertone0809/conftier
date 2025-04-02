# type: ignore[attr-defined]
"""Multi-level configuration framework"""

import sys
from typing import Any, Dict, Generic, Optional, Type, TypeVar, cast

if sys.version_info >= (3, 8):
    from importlib import metadata as importlib_metadata
else:
    import importlib_metadata

from .core import (
    ConfigManager,
    ConfigModel,
    deep_update,
    find_project_root,
    get_project_config_path,
    get_user_config_path,
    merge_configs_dict,
)

__version__ = "0.1.0"


def get_version() -> str:
    try:
        return importlib_metadata.version(__name__)
    except importlib_metadata.PackageNotFoundError:  # pragma: no cover
        return "unknown"


version: str = get_version()

__all__ = [
    "ConfigManager",
    "ConfigModel",
    "deep_update",
    "find_project_root",
    "get_project_config_path",
    "get_user_config_path",
    "merge_configs_dict",
    "TypedConfigManager",
    "__version__",
    "version",
]
