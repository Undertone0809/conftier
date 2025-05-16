"""
Configuration and shared fixtures for pytest
"""

import os
import tempfile
from pathlib import Path

import pytest


@pytest.fixture
def temp_home_dir():
    """Create a temporary home directory for testing"""
    original_home = os.environ.get("HOME")
    with tempfile.TemporaryDirectory() as temp_dir:
        os.environ["HOME"] = temp_dir
        yield temp_dir
        if original_home:
            os.environ["HOME"] = original_home
        else:
            del os.environ["HOME"]


@pytest.fixture
def temp_project_dir():
    """Create a temporary project directory with .git marker for testing"""
    with tempfile.TemporaryDirectory() as temp_dir:
        # Create a .git directory to identify as project root
        os.makedirs(os.path.join(temp_dir, ".git"))

        original_cwd = os.getcwd()
        os.chdir(temp_dir)

        yield temp_dir

        os.chdir(original_cwd)


@pytest.fixture
def mock_yaml_file():
    """Create a temporary YAML file for testing"""
    with tempfile.NamedTemporaryFile(suffix=".yaml", delete=False) as temp_file:
        temp_file.write(b"""
title: test_title
number: 42
nested:
  name: nested_name
  value: 100
list_values:
  - item1
  - item2
  - item3
""")
        temp_path = temp_file.name

    yield temp_path

    # Clean up
    os.unlink(temp_path)
