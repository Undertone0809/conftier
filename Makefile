.PHONY: build test lint format

build:
	cargo build

test:
	cargo test

lint:
	cargo clippy --fix --allow-dirty --allow-staged

format:
	cargo fmt
