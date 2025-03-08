# Template for a new project

## Create main directory

### Python

Create `.python-version` file to init python version:

```text
3.10
```

Create `pyproject.toml` file:

```toml
[project]
name = "Add name of main directory"
version = "0.1.0"
description = "Python code"
readme = "README.md"
requires-python = ">=3.10"
authors = [{name = "Krystian Krakowski", email = "kkrakowski22@gmail.com"}]
dependencies = []

[tool.uv.workspace]
members = ["python/*", "mixed/*"]
```

### Rust

Create `rust-toolchain.toml` file to init rust version:

```toml
[toolchain]
channel = "1.75.0"
```

Create `Cargo.toml` file:

```toml
[workspace]
members = ["rust/*", "mixed/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
description = "Rust code"
authors = ["Krystian Krakowski <kkrakowski22@gmail.com>"]
readme = "README.md"
rust-version = "1.75"
```

## Create project workspaces

### Rust workspace

```bash
cargo new --vcs none --bin rust/<name>
```

### Python workspace

```bash
uv init --vcs none --package python/<name>
```

## Mixed (Rust libray + Python package) workspace

```bash
uv init --vcs none --package --build-backend maturin mixed/<name>
```

## Run project

### Python package

```bash
uv run --package <name> <name>
```

### Rust bin

```bash
cargo run --bin <name>
```

## Check code

```bash
./check_code.sh
```

## Check specific tests

### Python tests

```bash
uv run pytest python/<name>
```

### Rust tests (not working for libraries)

```bash
cargo test --bin <name>
```

### Mixed (Rust library + Python package)

> [!IMPORTANT]  
> You need to manually develop release for maturin project. More info at [notes](#update-mixed-workspace)

```bash
uv run pytest mixed/<name>
cargo test
```

> [!NOTE]
> You cannot test rust libraries directly. You can only test whole project.

## Notes

### Update mixed workspace

To get newest python version of maturin/pyo3 project you need to run:

```bash
maturin develop --release --uv -m mixed/<name>/Cargo.toml
```
