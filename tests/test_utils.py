"""
Unit tests for utility functions in the conftier package
"""

import os
import tempfile
from pathlib import Path

import pytest

from conftier.core import (
    find_project_root,
    get_project_config_path,
    get_user_config_path,
)


class TestUtilityFunctions:
    """Tests for utility functions in conftier"""

    def test_get_user_config_path(self, monkeypatch):
        """Test that get_user_config_path returns the correct path"""
        # Set up a fake home directory
        with tempfile.TemporaryDirectory() as temp_dir:
            monkeypatch.setenv("HOME", temp_dir)

            # Test with a framework name
            framework_name = "test_framework"
            config_path = get_user_config_path(framework_name)

            # Expected path: ~/.test_framework/config.yaml
            expected_path = Path(temp_dir) / f".{framework_name}" / "config.yaml"
            assert config_path == expected_path

            # Verify with a different framework name
            another_framework = "another_framework"
            another_path = get_user_config_path(another_framework)
            expected_another_path = (
                Path(temp_dir) / f".{another_framework}" / "config.yaml"
            )
            assert another_path == expected_another_path

    def test_find_project_root(self):
        """Test that find_project_root correctly identifies project root directories"""
        # Create a temporary directory structure
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)

            # Create mock project markers
            git_dir = temp_path / ".git"
            os.makedirs(git_dir)

            # Create subdirectories
            subdir1 = temp_path / "subdir1"
            subdir2 = subdir1 / "subdir2"
            os.makedirs(subdir2)

            # Test from root
            result = find_project_root(temp_path)
            assert result == temp_path

            # Test from subdirectory
            result = find_project_root(subdir1)
            assert result == temp_path

            # Test from nested subdirectory
            result = find_project_root(subdir2)
            assert result == temp_path

            # Test when no project root is found
            with tempfile.TemporaryDirectory() as no_marker_dir:
                result = find_project_root(Path(no_marker_dir))
                assert result is None

    def test_get_project_config_path(self, monkeypatch):
        """Test that get_project_config_path returns the correct path"""
        # Create a temporary directory structure with a project root
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)

            # Create mock project marker (.git directory)
            git_dir = temp_path / ".git"
            os.makedirs(git_dir)

            # Create subdirectory
            subdir = temp_path / "subdir"
            os.makedirs(subdir)

            # Use subdir as current working directory
            original_cwd = os.getcwd()
            os.chdir(subdir)

            try:
                # Test with a framework name from a subdirectory
                framework_name = "test_framework"
                config_path = get_project_config_path(framework_name)

                # Expected path: <project_root>/.test_framework/config.yaml
                expected_path = temp_path / f".{framework_name}" / "config.yaml"
                assert config_path == expected_path

                # Verify with a different framework name
                another_framework = "another_framework"
                another_path = get_project_config_path(another_framework)
                expected_another_path = (
                    temp_path / f".{another_framework}" / "config.yaml"
                )
                assert another_path == expected_another_path

                # Test when no project root is found
                with tempfile.TemporaryDirectory() as no_marker_dir:
                    os.chdir(no_marker_dir)
                    result = get_project_config_path(framework_name)
                    assert result is None
            finally:
                # Restore original working directory
                os.chdir(original_cwd)
