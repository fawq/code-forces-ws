#!/bin/bash

cargo check
cargo fmt
cargo test

uv run mypy .
uv run ruff check
uv run pytest