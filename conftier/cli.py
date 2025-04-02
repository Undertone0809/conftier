from pathlib import Path
from typing import Optional

import click
import yaml

from .core import (
    ConfigManager,
    find_project_root,
    get_project_config_path,
    get_user_config_path,
)


@click.group()
def conftier():
    """Conftier configuration management tool"""
    pass


@conftier.command()
@click.argument("framework_name")
@click.option("--path", "-p", help="Project path")
def init_project(framework_name: str, path: Optional[str] = None):
    """Initialize project configuration template"""
    project_path = Path(path) if path else Path.cwd()
    config_dir = project_path / f".{framework_name}"
    config_file = config_dir / "config.yaml"

    # Create directory if it doesn't exist
    if not config_dir.exists():
        config_dir.mkdir(parents=True, exist_ok=True)
        click.echo(f"Created directory: {config_dir}")

    # Create template file if it doesn't exist
    if not config_file.exists():
        # Simple empty config as template
        with open(config_file, "w") as f:
            yaml.dump({}, f, default_flow_style=False)
        click.echo(f"Created project config template: {config_file}")
    else:
        click.echo(f"Project config already exists: {config_file}")


@conftier.command()
@click.argument("framework_name")
def show_config(framework_name: str):
    """Show current effective configuration and its sources"""
    # Get paths
    user_path = get_user_config_path(framework_name)
    project_root = find_project_root()
    project_path = get_project_config_path(
        framework_name, str(project_root) if project_root else None
    )

    # Check if any config files exist
    if not user_path.exists() and (not project_path or not project_path.exists()):
        click.echo(f"No configuration files found for framework '{framework_name}'")
        return

    # Show user config
    if user_path.exists():
        click.echo(f"User config ({user_path}):")
        with open(user_path, "r") as f:
            user_config = yaml.safe_load(f) or {}
        click.echo(yaml.dump(user_config, default_flow_style=False))
    else:
        click.echo(f"No user config found at {user_path}")

    # Show project config
    if project_path and project_path.exists():
        click.echo(f"Project config ({project_path}):")
        with open(project_path, "r") as f:
            project_config = yaml.safe_load(f) or {}
        click.echo(yaml.dump(project_config, default_flow_style=False))
    else:
        click.echo("No project config found")


@conftier.command()
@click.argument("framework_name")
@click.option("--key", "-k", help="Config key to set (dot notation)")
@click.option("--value", "-v", help="Value to set")
@click.option(
    "--project", "-p", is_flag=True, help="Update project config instead of user config"
)
def set_config(framework_name: str, key: str, value: str, project: bool = False):
    """Set a configuration value"""
    if project:
        project_root = find_project_root()
        if not project_root:
            click.echo("No project root found. Cannot update project configuration.")
            return

        config_path = get_project_config_path(framework_name, str(project_root))
        if not config_path:
            click.echo("No project configuration path could be determined.")
            return

        # Ensure directory exists
        if not config_path.parent.exists():
            config_path.parent.mkdir(parents=True, exist_ok=True)
    else:
        config_path = get_user_config_path(framework_name)
        # Ensure directory exists
        if not config_path.parent.exists():
            config_path.parent.mkdir(parents=True, exist_ok=True)

    # Load existing config
    existing_config = {}
    if config_path.exists():
        with open(config_path, "r") as f:
            existing_config = yaml.safe_load(f) or {}

    # Parse key path
    key_parts = key.split(".")

    # Navigate to the nested dict position
    current = existing_config
    for i, part in enumerate(key_parts[:-1]):
        if part not in current:
            current[part] = {}
        elif not isinstance(current[part], dict):
            current[part] = {}
        current = current[part]

    # Set the value
    try:
        # Try to parse as int, float, or bool
        if value.lower() == "true":
            parsed_value = True
        elif value.lower() == "false":
            parsed_value = False
        elif value.isdigit():
            parsed_value = int(value)
        else:
            try:
                parsed_value = float(value)
            except ValueError:
                parsed_value = value
    except:
        parsed_value = value

    current[key_parts[-1]] = parsed_value

    # Write back to file
    with open(config_path, "w") as f:
        yaml.dump(existing_config, f, default_flow_style=False, sort_keys=False)

    config_type = "project" if project else "user"
    click.echo(f"Updated {config_type} config: {key} = {value}")


if __name__ == "__main__":
    conftier()
