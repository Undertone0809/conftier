from zeeland import get_default_storage_path as _get_default_storage_path


def get_default_storage_path(module_name: str = "") -> str:
    return _get_default_storage_path("conftier", module_name)
