# Sync uv if any changes
uv sync

# Rebuild mixed workspaces
Get-ChildItem -Path mixed/ -Recurse -Depth 0 | Where-Object { $_.PSIsContainer } | ForEach-Object { 
    maturin develop --release --uv -m mixed/$_/Cargo.toml 
}

# Check all rust code (also mixed workspaces)
cargo check
cargo fmt
cargo test

# Check python code (also mixed workspaces)
uv run mypy .
uv run ruff check
uv run pytest