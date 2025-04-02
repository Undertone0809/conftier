# Installation

This guide provides detailed instructions for installing conftier in various environments.

## Requirements

- Python 3.8 or higher
- pip (Python package manager) or Poetry

## Installing with pip

The simplest way to install conftier is using pip:

```bash
pip install conftier
```

To install a specific version:

```bash
pip install conftier==1.0.0
```

To upgrade an existing installation:

```bash
pip install --upgrade conftier
```

## Installing with Poetry

If you're using Poetry for dependency management:

```bash
poetry add conftier
```

## Installing from Source

If you want the latest development version or need to customize the installation, you can install directly from the source code:

```bash
git clone https://github.com/Undertone0809/conftier.git
cd conftier
pip install -e .
```

With Poetry:

```bash
git clone https://github.com/Undertone0809/conftier.git
cd conftier
poetry install
```

## Verifying Installation

You can verify that conftier is correctly installed by running:

```bash
python -c "import conftier; print(conftier.__version__)"
```

This should print the version number of the installed package.

## Configuration Directory Setup

Upon first use, conftier will automatically create the necessary configuration directories:

- User-level configuration: `~/.zeeland/{config_name}/config.yaml`
- Project-level configuration: `/.{config_name}/config.yaml` (in the project root)

You don't need to create these directories manually - conftier will handle this for you when you initialize the `ConfigManager` with `auto_create=True`.

## Next Steps

Once installation is complete, you can:

- Continue to the [Getting Started](/guide/) guide to learn the basics
- Check the [API Reference](/api/) for detailed documentation
- Explore [Examples](/api/examples) to see common usage patterns

## Troubleshooting

If you encounter any issues during installation, try the following:

1. Make sure you have the latest version of pip:

   ```bash
   pip install --upgrade pip
   ```

2. If you're behind a proxy, configure pip to use it:

   ```bash
   pip install --proxy http://user:password@proxyserver:port conftier
   ```

3. If you're having dependency conflicts, consider using a virtual environment:

   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   pip install conftier
   ```

If you still have issues, please [open an issue](<https://github.com/Undertone0809/conftier/issues>) on our GitHub repository.
