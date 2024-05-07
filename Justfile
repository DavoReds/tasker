_default:
    @just --list --justfile {{ justfile() }}

# Test code, formatting and best practices
test *args:
    mold -run cargo nextest run {{ args }}
    cargo fmt --check
    mold -run cargo clippy -- -D warnings

# Clean cargo artifacts
@clean:
    cargo clean
