_default:
    @just --list --justfile {{ justfile() }}

# Test code, formatting and best practices
test *args:
    cargo nextest run {{ args }}
    cargo fmt --check
    cargo clippy -- -D warnings

# Clean cargo artifacts
@clean:
    cargo clean
