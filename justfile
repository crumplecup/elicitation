# Elicitation Development Justfile
#
# Common tasks for building, testing, and maintaining the Elicitation project.
# Run `just` or `just --list` to see all available commands.

# Default recipe to display help
default:
    @just --list

# Development Setup
# ================

# Install all required development tools and verification tools
setup:
    @echo "🔧 Installing development tools..."
    cargo install just || true
    cargo install cargo-audit || true
    cargo install cargo-dist || true
    cargo install cargo-release || true
    cargo install git-cliff || true
    cargo install omnibor-cli || true
    cargo install cargo-hack || true
    cargo install cargo-nextest || true
    @echo "✅ All development tools installed"
    @echo ""
    @just setup-verifiers

# Install formal verification tools
setup-verifiers install_dir="~/repos":
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔬 Installing formal verification tools to {{install_dir}}..."
    echo ""
    
    # Kani
    if command -v kani &> /dev/null; then
        echo "✅ Kani already installed"
    else
        echo "📦 Installing Kani..."
        cargo install --locked kani-verifier
        cargo kani setup
    fi
    echo ""
    
    # Creusot (requires opam)
    if command -v cargo-creusot &> /dev/null; then
        echo "✅ Creusot already installed"
    else
        echo "📦 Installing Creusot..."
        # Check for opam
        if ! command -v opam &> /dev/null; then
            echo "  📦 Installing opam (OCaml package manager)..."
            sudo apt-get update && sudo apt-get install -y opam
        fi
        mkdir -p {{install_dir}}
        cd {{install_dir}} && git clone https://github.com/creusot-rs/creusot.git || true
        cd {{install_dir}}/creusot && ./INSTALL
    fi
    echo ""
    
    # Prusti (requires Java)
    if command -v cargo-prusti &> /dev/null; then
        echo "✅ Prusti already installed"
    else
        echo "📦 Installing Prusti..."
        # Check for Java
        if ! command -v java &> /dev/null; then
            echo "  ⚠️  Java not found. Install with:"
            echo "      Arch/Manjaro: sudo pacman -S jdk-openjdk"
            echo "      Ubuntu/Debian: sudo apt-get install default-jdk"
            echo "      Then set JAVA_HOME and re-run"
            exit 1
        fi
        # Set JAVA_HOME if not set
        if [ -z "${JAVA_HOME:-}" ]; then
            export JAVA_HOME=$(dirname $(dirname $(readlink -f $(which java))))
        fi
        mkdir -p {{install_dir}}
        cd {{install_dir}} && git clone https://github.com/viperproject/prusti-dev.git || true
        cd {{install_dir}}/prusti-dev && ./x.py setup && ./x.py build --release
    fi
    echo ""
    
    # Verus
    if command -v verus &> /dev/null; then
        echo "✅ Verus already installed"
    else
        echo "📦 Installing Verus..."
        mkdir -p {{install_dir}}
        cd {{install_dir}} && git clone https://github.com/verus-lang/verus.git || true
        cd {{install_dir}}/verus && ./tools/get-z3.sh && source ./tools/activate && vargo build --release
    fi
    echo ""
    
    echo "✅ All verifiers ready"

# Building and Checking
# ======================

# Check a single package or entire workspace
check package="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        echo "🔍 Checking entire workspace..."
        cargo check --workspace
    else
        echo "🔍 Checking package: {{package}}"
        cargo check -p {{package}}
    fi

# Build specific package or entire workspace
build package="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        cargo build --release
    else
        cargo build --release --package {{package}}
    fi

# Clean build artifacts
clean:
    cargo clean

# Clean and rebuild
rebuild: clean build

# Testing
# =======

# Run tests: just test [package] [test_name]
test package="" test_name="":
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        # No package specified - run all tests
        cargo test --workspace --lib --tests
    elif [ -z "{{test_name}}" ]; then
        # Package specified, no test - run all tests for package
        cargo test --package {{package}} --lib --tests
    else
        # Package and test specified - run specific test
        cargo test --package {{package}} --lib --tests {{test_name}} -- --nocapture
    fi

# Run tests with verbose output
test-verbose:
    cargo test --workspace --lib --tests -- --nocapture

# Run doctests
test-doc:
    cargo test --workspace --doc

# Run tests for a specific package
test-package package test_name="":
    #!/usr/bin/env bash
    echo "📦 Testing {{package}}"
    if [ -n "{{test_name}}" ]; then
        cargo test -p {{package}} --lib --tests {{test_name}} -- --nocapture
    else
        cargo test -p {{package}} --lib --tests
    fi

# Run API tests (rate-limited, expensive)
test-api package="" test_name="":
    #!/usr/bin/env bash
    LOG_FILE="/tmp/elicitation-test-api.log"
    rm -f "$LOG_FILE"

    PACKAGE_FLAG=""
    if [ -n "{{package}}" ]; then
        PACKAGE_FLAG="-p {{package}}"
    fi

    TEST_NAME=""
    if [ -n "{{test_name}}" ]; then
        TEST_NAME="{{test_name}}"
    fi

    if cargo test $PACKAGE_FLAG --features api $TEST_NAME -- --nocapture --show-output 2>&1 | tee "$LOG_FILE"; then
        if [ -s "$LOG_FILE" ] && grep -qE "^(warning:|error:|\s+\^|error\[|test result:.*FAILED)" "$LOG_FILE"; then
            echo "⚠️  API tests completed with warnings/errors. See: $LOG_FILE"
            exit 1
        else
            echo "✅ All API tests passed!"
            rm -f "$LOG_FILE"
        fi
    else
        echo "❌ API tests failed. See: $LOG_FILE"
        exit 1
    fi

# Run local + doc tests (no linting)
test-full: test test-doc

# Code Quality
# ============

# Run clippy linter (no warnings allowed)
lint package='':
    #!/usr/bin/env bash
    if [ -z "{{package}}" ]; then
        echo "🔍 Linting entire workspace"
        cargo clippy --workspace --all-targets -- -D warnings
    else
        echo "🔍 Linting {{package}}"
        cargo clippy -p {{package}} --all-targets -- -D warnings
    fi

# Run clippy and fix issues automatically
lint-fix:
    cargo clippy --workspace --all-targets --fix --allow-dirty --allow-staged

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format all code
fmt:
    cargo fmt --all

# Check markdown files for issues
lint-md:
    @command -v markdownlint-cli2 >/dev/null 2>&1 || (echo "❌ markdownlint-cli2 not installed. Install with: npm install -g markdownlint-cli2" && exit 1)
    markdownlint-cli2 "**/*.md" "#target"

# Test various feature gate combinations (requires cargo-hack)
check-features:
    #!/usr/bin/env bash
    set -e
    command -v cargo-hack >/dev/null 2>&1 || (echo "❌ cargo-hack not installed. Run: cargo install cargo-hack" && exit 1)

    LOG_FILE="/tmp/elicitation-check-features.log"
    rm -f "$LOG_FILE"

    echo "🔍 Checking no-default-features..."
    if ! cargo check --workspace --no-default-features 2>&1 | tee -a "$LOG_FILE"; then
        echo "❌ No-default-features check failed. See: $LOG_FILE"
        exit 1
    fi

    echo "🔍 Checking all-features..."
    if ! cargo check --workspace --all-features 2>&1 | tee -a "$LOG_FILE"; then
        echo "❌ All-features check failed. See: $LOG_FILE"
        exit 1
    fi

    if [ -s "$LOG_FILE" ] && grep -qE "^(warning:|error:|\s+\^|error\[)" "$LOG_FILE"; then
        echo "⚠️  Feature gate checks completed with warnings/errors. See: $LOG_FILE"
        exit 1
    else
        echo "✅ All feature gate checks passed!"
        rm -f "$LOG_FILE"
    fi

# Run all checks (lint, format check, tests)
check-all package='':
    #!/usr/bin/env bash
    set -uo pipefail  # Removed -e so we can capture exit codes
    LOG_FILE="/tmp/elicitation_check_all.log"
    rm -f "$LOG_FILE"
    EXIT_CODE=0

    if [ -z "{{package}}" ]; then
        echo "🔍 Running all checks on entire workspace..."

        # Run fmt (errors only)
        cargo fmt --all

        # Run lint (show output and log warnings/errors)
        echo "🔍 Linting entire workspace"
        if ! cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tee -a "$LOG_FILE"; then
            EXIT_CODE=1
        fi

        # Run tests (show output and log failures)
        if ! cargo test --workspace --lib --tests 2>&1 | tee -a "$LOG_FILE"; then
            EXIT_CODE=1
        fi

        # Report results
        if [ $EXIT_CODE -ne 0 ]; then
            echo ""
            echo "⚠️  Checks completed with warnings/errors. Full log saved to: $LOG_FILE"
            exit 1
        else
            echo ""
            echo "✅ All checks passed!"
            rm -f "$LOG_FILE"
        fi
    else
        echo "🔍 Running all checks on {{package}}..."
        just fmt
        just lint "{{package}}"
        just test-package "{{package}}"
        # Run doc tests for the package
        cargo test -p "{{package}}" --doc
    fi
    echo "✅ All checks passed!"

# Fix all auto-fixable issues
fix-all: fmt lint-fix
    @echo "✅ Auto-fixes applied!"

# Security
# ========

# Check for security vulnerabilities in dependencies
audit:
    @command -v cargo-audit >/dev/null 2>&1 || (echo "❌ cargo-audit not installed. Run: just setup" && exit 1)
    cargo audit

# Update dependencies and check for vulnerabilities
audit-fix:
    cargo update
    cargo audit

# Generate OmniBOR artifact tree for supply chain transparency
omnibor:
    @command -v omnibor >/dev/null 2>&1 || (echo "❌ omnibor-cli not installed. Run: just setup" && exit 1)
    omnibor --help > /dev/null && echo "✅ OmniBOR installed" || echo "❌ OmniBOR not found"

# Run all security checks
security: audit omnibor
    @echo "✅ Security checks completed!"

# Full Workflow (CI/CD)
# ====================

# Run the complete CI pipeline locally
ci: fmt-check lint check-features test-full audit
    @echo "✅ CI pipeline completed successfully!"

# Prepare for commit (format, lint, tests, feature checks)
pre-commit: fix-all check-features test-full
    @echo "✅ Ready to commit!"

# Prepare for merge (all checks including API tests)
pre-merge: pre-commit test-api
    @echo "✅ Ready to merge!"

# Prepare for release (all checks + security + changelog update + release build)
pre-release: ci security
    @echo "📋 Generating changelog preview..."
    git cliff --unreleased
    @echo ""
    @echo "📦 Testing release dry-run..."
    cargo release --workspace --no-publish --no-push --no-tag --allow-branch '*' --execute
    @echo ""
    @echo "🏗️  Building release artifacts..."
    just build
    @echo ""
    @echo "✅ Ready for release!"
    @echo ""
    @echo "Next steps:"
    @echo "  1. Review changelog above"
    @echo "  2. Run: git cliff --unreleased --prepend CHANGELOG.md"
    @echo "  3. Commit changelog updates"
    @echo "  4. Run: cargo release [patch|minor|major] --workspace --execute"

# Release Management (cargo-dist)
# ==================

# Build distribution artifacts for current platform
dist-build:
    @command -v dist >/dev/null 2>&1 || (echo "❌ cargo-dist not installed. Run: just setup" && exit 1)
    dist build

# Build and check distribution artifacts (doesn't upload)
dist-check:
    @command -v dist >/dev/null 2>&1 || (echo "❌ cargo-dist not installed. Run: just setup" && exit 1)
    dist build --check

# Generate release configuration
dist-init:
    @command -v dist >/dev/null 2>&1 || (echo "❌ cargo-dist not installed. Run: just setup" && exit 1)
    dist init

# Plan a release (preview changes)
dist-plan:
    @command -v dist >/dev/null 2>&1 || (echo "❌ cargo-dist not installed. Run: just setup" && exit 1)
    dist plan

# Generate CI workflow files
dist-generate:
    @command -v dist >/dev/null 2>&1 || (echo "❌ cargo-dist not installed. Run: just setup" && exit 1)
    dist generate

# Changelog and Release
# ====================

# Generate changelog for unreleased changes
changelog-preview:
    @command -v git-cliff >/dev/null 2>&1 || (echo "❌ git-cliff not installed. Run: just setup" && exit 1)
    git cliff --unreleased

# Update CHANGELOG.md with unreleased changes
changelog-update:
    @command -v git-cliff >/dev/null 2>&1 || (echo "❌ git-cliff not installed. Run: just setup" && exit 1)
    @echo "📋 Updating CHANGELOG.md..."
    git cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ CHANGELOG.md updated"

# Generate full changelog
changelog-full:
    @command -v git-cliff >/dev/null 2>&1 || (echo "❌ git-cliff not installed. Run: just setup" && exit 1)
    git cliff --output CHANGELOG.md

# Dry-run release (shows what would happen)
release-dry-run level="patch":
    @command -v cargo-release >/dev/null 2>&1 || (echo "❌ cargo-release not installed. Run: just setup" && exit 1)
    @echo "🔍 Dry-run release {{level}}..."
    cargo release {{level}} --workspace --no-publish --no-push --no-tag --allow-branch '*'

# Execute release (bumps version, tags, pushes)
release level="patch":
    @command -v cargo-release >/dev/null 2>&1 || (echo "❌ cargo-release not installed. Run: just setup" && exit 1)
    @echo "🚀 Releasing {{level}} version..."
    @echo "⚠️  This will:"
    @echo "   - Bump version in Cargo.toml files"
    @echo "   - Create git tag"
    @echo "   - Push to remote"
    @echo ""
    @read -p "Continue? (y/N) " -n 1 -r; echo; \
    if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
        cargo release {{level}} --workspace --execute; \
    else \
        echo "❌ Release cancelled"; \
        exit 1; \
    fi

# Documentation
# =============

# Build and open documentation
doc:
    cargo doc --workspace --no-deps --open

# Check documentation for issues
doc-check:
    cargo doc --workspace --no-deps

# Build and view documentation for a specific crate
doc-crate crate:
    cargo doc --package {{crate}} --no-deps --open

# Utility
# =======

# Remove generated files and caches
clean-all: clean
    @echo "🧹 Deep cleaning..."
    rm -rf target/
    rm -f Cargo.lock
    @echo "✅ All build artifacts removed"

# Check for outdated dependencies
outdated:
    @command -v cargo-outdated >/dev/null 2>&1 || (echo "Installing cargo-outdated..." && cargo install cargo-outdated)
    cargo outdated

# Update dependencies to latest compatible versions
update-deps:
    cargo update
    @echo "✅ Dependencies updated. Run 'just test' to verify."

# Show project statistics
stats:
    @echo "📊 Project Statistics"
    @echo "===================="
    @echo ""
    @echo "Workspace crates:"
    @ls -1d crates/*/ | wc -l
    @echo ""
    @echo "Lines of Rust code (all crates):"
    @find crates -name '*.rs' -not -path '*/target/*' -exec wc -l {} + 2>/dev/null | tail -1 || echo "  0"
    @echo ""
    @echo "Lines of test code:"
    @find crates/*/tests tests -name '*.rs' 2>/dev/null -exec wc -l {} + 2>/dev/null | tail -1 || echo "  0"
    @echo ""
    @echo "Number of dependencies:"
    @grep -c "^name =" Cargo.lock 2>/dev/null || echo "  0"

# Show environment information
env:
    #!/usr/bin/env bash
    echo "🔧 Environment Information"
    echo "========================="
    echo ""
    echo "Rust version:"
    rustc --version
    echo ""
    echo "Cargo version:"
    cargo --version
    echo ""
    echo "Just version:"
    just --version

# Aliases for common tasks
# ========================

alias b := build
alias t := test
alias l := lint
alias f := fmt
alias c := check
alias d := doc

# Formal Verification
# ====================

# Check status of all verification tools
verify-status:
    @echo "📊 Formal Verification Tools Status"
    @echo "===================================="
    @just _status-kani
    @just _status-creusot
    @just _status-prusti
    @just _status-verus
    @echo ""

# Show Kani status
_status-kani:
    #!/usr/bin/env bash
    if command -v kani &> /dev/null; then
        echo "✅ Kani: $(kani --version)"
    else
        echo "❌ Kani: Not installed (run: just setup-verifiers)"
    fi

# Show Creusot status
_status-creusot:
    #!/usr/bin/env bash
    if command -v creusot &> /dev/null; then
        echo "✅ Creusot: $(creusot --version)"
    else
        echo "❌ Creusot: Not installed"
    fi

# Show Prusti status
_status-prusti:
    #!/usr/bin/env bash
    if command -v cargo-prusti &> /dev/null; then
        echo "✅ Prusti: Installed"
    else
        echo "❌ Prusti: Not installed"
    fi

# Show Verus status
_status-verus:
    #!/usr/bin/env bash
    if command -v verus &> /dev/null; then
        echo "✅ Verus: Installed"
    else
        echo "❌ Verus: Not installed"
    fi

# Run Kani verification
verify-kani harness="":
    #!/usr/bin/env bash
    if [ -z "{{harness}}" ]; then
        echo "🔬 Running all Kani verifications..."
        cargo kani --features verify-kani
    else
        echo "🔬 Running Kani harness: {{harness}}"
        cargo kani --harness {{harness}} --features verify-kani
    fi

# Run Prusti verification
verify-prusti:
    @command -v cargo-prusti >/dev/null 2>&1 || (echo "❌ Prusti not installed. Run: just setup-verifiers" && exit 1)
    @echo "🔬 Running Prusti verification..."
    cargo-prusti --package elicitation --features verify-prusti

# Run Creusot verification
verify-creusot file="":
    #!/usr/bin/env bash
    if ! command -v creusot &> /dev/null; then
        echo "❌ Creusot not installed. Run: just setup-verifiers"
        exit 1
    fi
    if [ -z "{{file}}" ]; then
        echo "❌ Usage: just verify-creusot <file.rs>"
        exit 1
    fi
    echo "🔬 Running Creusot verification on {{file}}"
    creusot verify {{file}}

# Run Verus verification
verify-verus:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Load VERUS_PATH from .env if it exists
    if [ -f .env ]; then
        export $(grep -v '^#' .env | grep VERUS_PATH | xargs)
    fi
    
    # Expand ~ in path
    VERUS_BIN="${VERUS_PATH/#\~/$HOME}"
    
    if [ ! -f "$VERUS_BIN" ]; then
        echo "❌ Verus not found at: $VERUS_BIN"
        echo "   Set VERUS_PATH in .env (see .env.example)"
        exit 1
    fi
    
    echo "🔬 Running Verus verification..."
    "$VERUS_BIN" --crate-type=lib crates/elicitation/src/lib.rs

# Run all formal verification tools
verify-all: verify-kani verify-prusti verify-creusot verify-verus
    @echo "✅ All verification completed!"

# Run all verification examples
verify-examples:
    @echo "🔬 Running verification examples..."
    @echo ""
    @echo "Kani example:"
    cargo run --example kani_example --features verify-kani
    @echo ""
    @echo "Creusot example:"
    cargo run --example creusot_example --features verify-creusot
    @echo ""
    @echo "✅ All examples passed"
