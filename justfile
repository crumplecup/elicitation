# Elicitation workspace justfile

default:
    @just --list

# Check a single package or entire workspace
check package="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        cargo check --workspace
    else
        cargo check -p {{package}}
    fi

# Test a single package or entire workspace
test package="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        cargo test --workspace
    else
        cargo test -p {{package}}
    fi

# Run all checks (clippy, fmt, test)
check-all package="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        cargo clippy --workspace -- -D warnings
        cargo fmt --all -- --check
        cargo test --workspace
    else
        cargo clippy -p {{package}} -- -D warnings
        cargo fmt -p {{package}} -- --check
        cargo test -p {{package}}
    fi

# Run API tests (rate-limited)
test-api:
    cargo test --workspace --features api

# Check all feature combinations
check-features:
    cargo check --workspace --no-default-features
    cargo check --workspace --all-features

# Update dependencies
update-deps:
    cargo update

# Security audit
audit:
    cargo audit

# Build documentation
doc:
    cargo doc --workspace --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Format all code
fmt:
    cargo fmt --all

# Run clippy
clippy:
    cargo clippy --workspace -- -D warnings
