---
title: "Contributing to Conftier: Developer Guidelines"
description: "Learn how to contribute to Conftier, the multi-level configuration framework for Python. Guide for submitting improvements, bug fixes, and new features."
head:
  - - meta
    - property: og:title
      content: "Contributing to Conftier: Developer Guidelines"
  - - meta
    - property: og:description
      content: "Learn how to contribute to Conftier, the multi-level configuration framework for Python."
  - - meta
    - name: keywords
      content: "conftier contribution, open source, python development, pull requests, code guidelines"
---

# Contributing to conftier

We welcome contributions to conftier! This document provides guidelines for contributing to the project.

## Getting Started

1. Click [here](https://github.com/undertone0809/conftier/fork) to fork the repository on GitHub.
2. Clone your fork locally:

   ```bash
   git clone https://github.com/your-username/conftier.git
   cd conftier
   ```

3. Create a virtual environment and install dependencies:

   ```bash
   conda create -n conftier python==3.10
   conda activate conftier

   pip install poetry
   poetry install
   ```

## Development Workflow

Here are the steps to contribute to the project and make a pull request on GitHub:

1. Create a new branch for your feature or bugfix:

   ```bash
   git checkout -b feature-or-fix-name
   ```

2. Make your changes and commit them after testing:

   ```bash
   make test
   git commit -m "Your detailed commit message"
   ```

3. Push your changes to your fork:

   ```bash
   git push origin feature-or-fix-name
   ```

4. Submit a pull request through the GitHub website.

## Coding Standards

- We use `ruff` for code formatting, run `make format` and `make lint before committing.
- Use pytest for testing, run `make test` before committing.
- Write clear, commented code.
- Include unit tests and related documentation for new features.

## Run Tests

Run the test suite using:

```bash
make test
```

## Run Documentation

Run the documentation server locally, you can edit them in `./docs`

```bash
make install-docs
make start-docs
```
