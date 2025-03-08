# Rebuild mixed workspaces
Get-ChildItem -Path mixed/ -Recurse -Depth 1 | Where-Object { $_.PSIsContainer } | ForEach-Object {
    maturin develop --release --uv -m $_.FullName/Cargo.toml
}

# Check all rust code (also mixed workspaces)
cargo check
cargo fmt
cargo test

# Check python code (also mixed workspaces)
uv run mypy .
uv run ruff check
uv run pytest