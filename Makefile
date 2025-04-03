#* Variables
SHELL := /usr/bin/env bash
PYTHON := python
OS := $(shell python -c "import sys; print(sys.platform)")

ifeq ($(OS),win32)
	PYTHONPATH := $(shell python -c "import os; print(os.getcwd())")
    TEST_COMMAND := set PYTHONPATH=$(PYTHONPATH) && poetry run pytest -c pyproject.toml --cov-report=html --cov=conftier tests/
else
	PYTHONPATH := `pwd`
    TEST_COMMAND := PYTHONPATH=$(PYTHONPATH) poetry run pytest -c pyproject.toml --cov-report=html --cov=conftier tests/
endif

#* Docker variables
IMAGE := conftier
VERSION := latest

.PHONY: lock install  formatting test check-codestyle lint docker-build docker-remove cleanup help install-docs start-docs

lock:
	poetry lock -n && poetry export --without-hashes > requirements.txt

install:
	poetry install -n

format:
	poetry run ruff format --config pyproject.toml .
	poetry run ruff check --fix --config pyproject.toml .

test:
	$(TEST_COMMAND)
	poetry run coverage-badge -o assets/images/coverage.svg -f

check-codestyle:
	poetry run ruff format --check --config pyproject.toml .
	poetry run ruff check --config pyproject.toml .

lint: test check-codestyle 

install-docs:
	cd docs && pnpm install

start-docs:
	cd docs && npm run docs:dev

help:
	@echo "lock                                      Lock the dependencies."
	@echo "install                                   Install the project dependencies."
	@echo "format                                    Format the codebase."
	@echo "test                                      Run the tests."
	@echo "check-codestyle                           Check the codebase for style issues."
	@echo "lint                                      Run the tests and check the codebase for style issues."
	@echo "docker-build                              Build the docker image."
	@echo "docker-remove                             Remove the docker image."
	@echo "cleanup                                   Clean the project directory."
	@echo "help                                      Display this help message."
	@echo "install-docs                              Install the documentation dependencies."
	@echo "start-docs                                Start the documentation server."
