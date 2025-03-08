# Template for a new project

## Create project

### Rust

```bash
cargo new --vcs none --bin rust/<name>
```

### Python

```bash
uv init --vcs none --package python/<name>
```

## Mixed (Rust libray + Python package)

```bash
uv init --vcs none --package --build-backend maturin mixed/<name>
```

## Check code

> [!WARNING]  
> You need to manually develop release for maturin project.

```bash
./check_code.sh
```

## Check specific tests

### Python (Pytest)

```bash
uv run pytest python/<name>
```

### Rust (Cargo)

```bash
cargo test --bin <name>
```

## Mixed(Rust libray + Python package)

```bash
maturin develop --release --uv mixed/<name>/Cargo.toml
uv run pytest mixed/<name>
```
