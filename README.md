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

## Check code

```bash
./check.sh
```

## Check specific tests

### Python

```bash
uv run pytest python/<name>
```

### Rust

```bash
cargo test --bin <name>
```

