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
        echo "🔬 Running all Kani verifications with default unwind bound..."
        cargo kani --features verify-kani --default-unwind 20
    else
        echo "🔬 Running Kani harness: {{harness}}"
        cargo kani --harness {{harness}} --features verify-kani --default-unwind 20
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

# Kani UTF-8 Long-Running Proofs
# ===============================

# Benchmark Kani verification scaling (measure marginal costs)
# Runs micro-benchmarks to calculate cost per symbolic combination
kani-benchmark:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "🔬 Kani Marginal Cost Benchmark"
    echo "================================"
    echo ""
    echo "This measures verification cost scaling by running"
    echo "progressively larger symbolic proofs (4-16 combinations)."
    echo ""
    echo "Expected time: 1-2 hours"
    echo "Output: kani_marginal_benchmark.log"
    echo ""
    read -p "Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Cancelled."
        exit 1
    fi
    ./scripts/kani_marginal_cost.sh

# Benchmark all verification types (component proofs)
# Fast proofs: UUID, IP, MAC, SocketAddr, PathBuf, URL components, Regex layers
benchmark-verification:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "🔬 Verification Benchmark Suite"
    echo "================================="
    echo ""
    echo "Benchmarks tractable Kani proofs across all validation types."
    echo "Each proof completes in seconds (0.04s - 8s)."
    echo ""
    echo "Expected total time: 2-5 minutes"
    echo "Output: verification_benchmark.csv"
    echo ""
    
    OUTPUT="verification_benchmark.csv"
    
    # CSV header
    echo "Type,Harness,Time_Seconds,Status" > "$OUTPUT"
    
    # UUID proofs (14 proofs, ~2s each)
    echo "📦 UUID proofs..."
    for harness in verify_valid_variant_accepted verify_v4_valid_construction verify_v4_roundtrip; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 20 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "UUID,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "UUID,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # IP address proofs (21 proofs, ~2-3s each)
    echo "📦 IP address proofs..."
    for harness in verify_ipv4_10_network_is_private verify_ipv4_public_valid verify_ipv6_fc00_is_private; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 20 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "IP,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "IP,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # MAC address proofs (18 proofs, 0.07s - 8s)
    echo "📦 MAC address proofs..."
    for harness in verify_unicast_detection verify_multicast_detection verify_universal_detection; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 20 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "MAC,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "MAC,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # SocketAddr proofs (19 proofs, ~2s each)
    echo "📦 SocketAddr proofs..."
    for harness in verify_well_known_port_range verify_socketaddrv4_nonzero_valid; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 20 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "SocketAddr,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "SocketAddr,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # PathBuf proofs (2 proofs, ~0.04s each)
    echo "📦 PathBuf proofs..."
    for harness in verify_valid_ascii_no_null_accepted verify_absolute_path_starts_with_slash; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 20 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "PathBuf,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "PathBuf,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # URL component proofs (5 proofs, ~6s each)
    # Note: URL components use small buffers (8-16 bytes) but need higher unwind
    echo "📦 URL component proofs..."
    for harness in verify_scheme_http verify_scheme_https verify_authority_simple; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 128 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "URL,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "URL,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    # Regex layer proofs (23 proofs, 1.6s - 8s)
    # Note: Regex uses 16-byte buffers, needs unwind=16
    echo "📦 Regex layer proofs..."
    for harness in verify_balanced_simple verify_escape_digit verify_quantifier_range verify_charclass_range verify_regex_literal; do
        echo -n "  $harness... "
        START=$(date +%s.%N)
        if cargo kani --harness "$harness" --features verify-kani --default-unwind 128 &>/dev/null; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ ${TIME}s"
            echo "Regex,$harness,$TIME,SUCCESS" >> "$OUTPUT"
        else
            echo "❌ FAILED"
            echo "Regex,$harness,0,FAILED" >> "$OUTPUT"
        fi
    done
    
    echo ""
    echo "✅ Benchmark complete!"
    echo "Results: $OUTPUT"
    echo ""
    echo "Summary:"
    awk -F, 'NR>1 {sum[$1]+=$3; count[$1]++} END {for (type in sum) printf "  %s: %.2fs avg (%d proofs)\n", type, sum[type]/count[type], count[type]}' "$OUTPUT" | sort

# Benchmark Kani marginal cost with increasing buffer sizes
benchmark-kani-marginal count="1":
    #!/usr/bin/env bash
    set -euo pipefail
    
    OUTPUT="kani_marginal_benchmark.csv"
    COUNT={{count}}
    
    echo "Benchmarking Kani verification marginal cost..."
    echo "This measures verification time vs buffer size for curve fitting."
    echo "Iterations per proof: $COUNT"
    echo "Results append to: $OUTPUT"
    echo ""
    
    # Create header if file doesn't exist
    if [ ! -f "$OUTPUT" ]; then
        echo "Timestamp,Type,Size,Iteration,Time_Seconds,Status" > "$OUTPUT"
    fi
    
    TIMESTAMP=$(date -Iseconds)
    
    # UTF-8 validation: 2, 4, 8, 16, 32 bytes (fast)
    echo "📊 UTF-8 Validation Marginal Cost (Fast: < 5 min)"
    for size in 2 4 8 16 32; do
        harness="bench_utf8_${size}byte"
        echo "  ${size} bytes (${COUNT} iterations):"
        
        for ((iter=1; iter<=COUNT; iter++)); do
            echo -n "    Run $iter/$COUNT... "
            START=$(date +%s.%N)
            if cargo kani --harness "$harness" --features verify-kani &>/dev/null; then
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                echo "✅ ${TIME}s"
                echo "$TIMESTAMP,UTF8,$size,$iter,$TIME,SUCCESS" >> "$OUTPUT"
            else
                echo "❌ FAILED"
                echo "$TIMESTAMP,UTF8,$size,$iter,0,FAILED" >> "$OUTPUT"
            fi
        done
    done
    
    # 64-256 bytes - moderate (minutes)
    echo ""
    echo "📊 UTF-8 Validation (Moderate: 5-30 min)"
    for size in 64 128 256; do
        harness="bench_utf8_${size}byte"
        echo "  ${size} bytes (${COUNT} iterations):"
        
        for ((iter=1; iter<=COUNT; iter++)); do
            echo -n "    Run $iter/$COUNT... "
            START=$(date +%s.%N)
            if cargo kani --harness "$harness" --features verify-kani &>/dev/null; then
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                MINS=$(echo "$TIME / 60" | bc)
                echo "✅ ${TIME}s (${MINS}m)"
                echo "$TIMESTAMP,UTF8,$size,$iter,$TIME,SUCCESS" >> "$OUTPUT"
            else
                echo "❌ FAILED"
                echo "$TIMESTAMP,UTF8,$size,$iter,0,FAILED" >> "$OUTPUT"
            fi
        done
    done
    
    # 512+ bytes - long running (hours to days)
    echo ""
    echo "⚠️  Large buffer proofs (expected: 1-24 hours)"
    echo "   Run with: just benchmark-kani-long [count]"
    for size in 512 1024 2048 4096; do
        echo "$TIMESTAMP,UTF8,$size,0,TBD,SKIPPED" >> "$OUTPUT"
    done
    
    echo ""
    echo "📊 UUID Format Marginal Cost"
    for harness in bench_uuid_variant_2byte bench_uuid_variant_4byte bench_uuid_full_16byte; do
        size=${harness##*_}
        size=${size%byte}
        echo "  $size (${COUNT} iterations):"
        
        for ((iter=1; iter<=COUNT; iter++)); do
            echo -n "    Run $iter/$COUNT... "
            START=$(date +%s.%N)
            if cargo kani --harness "$harness" --features verify-kani &>/dev/null; then
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                echo "✅ ${TIME}s"
                echo "$TIMESTAMP,UUID,$size,$iter,$TIME,SUCCESS" >> "$OUTPUT"
            else
                echo "❌ FAILED"
                echo "$TIMESTAMP,UUID,$size,$iter,0,FAILED" >> "$OUTPUT"
            fi
        done
    done
    
    echo ""
    echo "📊 MAC Address Marginal Cost"
    for harness in bench_mac_multicast_1byte bench_mac_local_1byte bench_mac_full_6byte; do
        size=${harness##*_}
        size=${size%byte}
        echo "  $size (${COUNT} iterations):"
        
        for ((iter=1; iter<=COUNT; iter++)); do
            echo -n "    Run $iter/$COUNT... "
            START=$(date +%s.%N)
            if cargo kani --harness "$harness" --features verify-kani &>/dev/null; then
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                echo "✅ ${TIME}s"
                echo "$TIMESTAMP,MAC,$size,$iter,$TIME,SUCCESS" >> "$OUTPUT"
            else
                echo "❌ FAILED"
                echo "$TIMESTAMP,MAC,$size,$iter,0,FAILED" >> "$OUTPUT"
            fi
        done
    done
    
    echo ""
    echo "✅ Marginal cost benchmark complete!"
    echo "Results appended to: $OUTPUT"
    echo ""
    echo "Curve Fitting Analysis (latest run):"
    python3 << 'PYTHON'
    import csv
    import sys
    from collections import defaultdict
    
    # Get latest timestamp
    latest_ts = None
    with open('kani_marginal_benchmark.csv', 'r') as f:
        reader = csv.DictReader(f)
        for row in reader:
            if row['Timestamp']:
                latest_ts = row['Timestamp']
    
    # Load data from latest run and calculate averages per size
    raw_data = defaultdict(lambda: defaultdict(list))
    
    try:
        with open('kani_marginal_benchmark.csv', 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                if row['Status'] == 'SUCCESS' and row['Timestamp'] == latest_ts:
                    typ = row['Type']
                    size = int(row['Size'])
                    time = float(row['Time_Seconds'])
                    raw_data[typ][size].append(time)
    except FileNotFoundError:
        print("No data file found")
        sys.exit(0)
    
    # Calculate averages
    data = defaultdict(list)
    for typ, size_dict in raw_data.items():
        for size, times in sorted(size_dict.items()):
            avg_time = sum(times) / len(times)
            stddev = (sum((t - avg_time)**2 for t in times) / len(times))**0.5 if len(times) > 1 else 0
            data[typ].append((size, avg_time, stddev, len(times)))
    
    for typ, points in sorted(data.items()):
        if len(points) < 2:
            continue
            
        print(f"\n{typ}:")
        print("  Size → Avg Time (±StdDev) [n]")
        for size, avg_time, stddev, count in sorted(points):
            if count > 1:
                print(f"  {size:3d}  → {avg_time:7.2f}s (±{stddev:5.2f}s) [n={count}]")
            else:
                print(f"  {size:3d}  → {avg_time:7.2f}s")
        
        # Simple growth rate analysis
        if len(points) >= 2:
            points_simple = [(size, avg) for size, avg, _, _ in sorted(points)]
            ratios = []
            for i in range(1, len(points_simple)):
                size_ratio = points_simple[i][0] / points_simple[i-1][0]
                time_ratio = points_simple[i][1] / points_simple[i-1][1]
                ratios.append(time_ratio / size_ratio)
            
            avg_ratio = sum(ratios) / len(ratios)
            print(f"\n  Growth rate: ~{avg_ratio:.2f}x per doubling")
            if avg_ratio < 1.5:
                print("  → Approximately LINEAR")
            elif avg_ratio < 3:
                print("  → Approximately QUADRATIC")
            elif avg_ratio < 10:
                print("  → Approximately CUBIC")
            else:
                print("  → EXPONENTIAL (verification intractable)")
                
            # Extrapolate
            if points_simple[-1][1] < 100:  # Only extrapolate if last point is reasonable
                last_size, last_time = points_simple[-1]
                next_size = last_size * 2
                est_time = last_time * (avg_ratio ** 1)  # One doubling
                
                if est_time < 60:
                    print(f"  Estimated {next_size}-byte: {est_time:.1f}s")
                elif est_time < 3600:
                    print(f"  Estimated {next_size}-byte: {est_time/60:.1f}m")
                elif est_time < 86400:
                    print(f"  Estimated {next_size}-byte: {est_time/3600:.1f}h")
                else:
                    print(f"  Estimated {next_size}-byte: {est_time/86400:.1f}d")
    PYTHON

# Benchmark long-running Kani proofs (512-4096 bytes, hours to days)
benchmark-kani-long count="1":
    #!/usr/bin/env bash
    set -euo pipefail
    
    OUTPUT="kani_marginal_benchmark.csv"
    COUNT={{count}}
    
    # Create header if file doesn't exist
    if [ ! -f "$OUTPUT" ]; then
        echo "Timestamp,Type,Size,Iteration,Time_Seconds,Status" > "$OUTPUT"
    fi
    
    TIMESTAMP=$(date -Iseconds)
    
    echo "🐢 Long-Running Kani Proofs (512-4096 bytes)"
    echo "=============================================="
    echo "Expected times: 512b=1h, 1024b=3h, 2048b=8h, 4096b=24h"
    echo "Iterations per proof: $COUNT"
    echo ""
    read -p "This will run for HOURS/DAYS. Continue? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Cancelled."
        exit 0
    fi
    
    for size in 512 1024 2048 4096; do
        harness="bench_utf8_${size}byte"
        echo ""
        echo "📊 ${size}-byte proof (${COUNT} iterations)"
        
        for ((iter=1; iter<=COUNT; iter++)); do
            echo ""
            echo "  Run $iter/$COUNT started at $(date)"
            
            START=$(date +%s.%N)
            if cargo kani --harness "$harness" --features verify-kani 2>&1 | tee "/tmp/kani_${size}_iter${iter}.log"; then
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                HOURS=$(echo "$TIME / 3600" | bc)
                echo "  ✅ VERIFIED in ${TIME}s (${HOURS}h)"
                echo "$TIMESTAMP,UTF8,$size,$iter,$TIME,SUCCESS" >> "$OUTPUT"
            else
                END=$(date +%s.%N)
                TIME=$(echo "$END - $START" | bc)
                echo "  ❌ FAILED after ${TIME}s"
                echo "$TIMESTAMP,UTF8,$size,$iter,$TIME,FAILED" >> "$OUTPUT"
            fi
            
            echo "  Completed iteration $iter at $(date)"
        done
    done
    
    echo ""
    echo "✅ All long proofs complete!"
    echo "Results in: $OUTPUT"

# Run expensive Kani UTF-8 symbolic proofs (days to weeks)
# WARNING: These proofs explore 3,968 to 786,432 symbolic combinations
kani-long-proofs proof="2byte":
    #!/usr/bin/env bash
    set -euo pipefail
    
    case "{{proof}}" in
        2byte)
            echo "🔬 Kani UTF-8 2-Byte Symbolic Proof"
            echo "===================================="
            echo ""
            echo "Problem space: 3,968 combinations (62 × 64)"
            echo "Expected time: Hours to days (hardware dependent)"
            echo ""
            read -p "Run proof? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                echo "Cancelled."
                exit 1
            fi
            echo ""
            echo "Starting 2-byte proof (output: utf8_2byte_proof.log)..."
            cargo kani --features verify-kani --harness verify_valid_two_byte_accepted 2>&1 | tee utf8_2byte_proof.log
            ;;
        3byte)
            echo "🔬 Kani UTF-8 3-Byte Symbolic Proof"
            echo "===================================="
            echo ""
            echo "Problem space: 49,152 combinations (12 × 64 × 64)"
            echo "Expected time: Days (hardware dependent)"
            echo ""
            read -p "Run proof? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                echo "Cancelled."
                exit 1
            fi
            echo ""
            echo "Starting 3-byte proof (output: utf8_3byte_proof.log)..."
            cargo kani --features verify-kani --harness verify_valid_three_byte_accepted 2>&1 | tee utf8_3byte_proof.log
            ;;
        4byte)
            echo "🔬 Kani UTF-8 4-Byte Symbolic Proof"
            echo "===================================="
            echo ""
            echo "Problem space: 786,432 combinations (3 × 64³)"
            echo "Expected time: Days to weeks (hardware dependent)"
            echo ""
            read -p "Run proof? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                echo "Cancelled."
                exit 1
            fi
            echo ""
            echo "Starting 4-byte proof (output: utf8_4byte_proof.log)..."
            cargo kani --features verify-kani --harness verify_valid_four_byte_accepted 2>&1 | tee utf8_4byte_proof.log
            ;;
        all)
            echo "🔬 All Kani UTF-8 Symbolic Proofs"
            echo "=================================="
            echo ""
            echo "This will run ALL symbolic UTF-8 proofs sequentially:"
            echo "  - 2-byte: 3,968 combinations (hours-days)"
            echo "  - 3-byte: 49,152 combinations (days)"
            echo "  - 4-byte: 786,432 combinations (days-weeks)"
            echo ""
            echo "Total expected time: Weeks to months"
            echo ""
            read -p "Run all proofs? (y/N) " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                echo "Cancelled."
                exit 1
            fi
            echo ""
            just kani-long-proofs 2byte
            just kani-long-proofs 3byte
            just kani-long-proofs 4byte
            ;;
        *)
            echo "❌ Invalid proof: {{proof}}"
            echo ""
            echo "Usage: just kani-long-proofs <proof>"
            echo ""
            echo "Available proofs:"
            echo "  2byte  - 2-byte UTF-8 sequences (3,968 combos, hours-days)"
            echo "  3byte  - 3-byte UTF-8 sequences (49K combos, days)"
            echo "  4byte  - 4-byte UTF-8 sequences (786K combos, days-weeks)"
            echo "  all    - All symbolic proofs (weeks-months)"
            echo ""
            echo "Tip: Run in screen/tmux or use nohup for background:"
            echo "  nohup just kani-long-proofs 2byte &"
            exit 1
            ;;
    esac

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

# Run chunked Kani proofs with checkpoint/resume (uses existing harnesses in library)
kani-chunked proof_type num_chunks:
    #!/usr/bin/env bash
    set -euo pipefail
    
    CSV="kani_proof_record_{{proof_type}}_{{num_chunks}}.csv"
    
    # Create CSV header if doesn't exist
    if [ ! -f "$CSV" ]; then
        echo "Timestamp,Proof_Type,Harness,Chunk_ID,Total_Chunks,Status,Time_Seconds" > "$CSV"
        echo "📝 Created checkpoint file: $CSV"
    fi
    
    echo "🔬 Kani Chunked Proof: {{proof_type}} / {{num_chunks}} chunks"
    echo "=========================================="
    echo ""
    
    # Run each chunk
    for i in $(seq 0 $(({{num_chunks}} - 1))); do
        HARNESS="verify_{{proof_type}}_{{num_chunks}}chunks_${i}"
        
        # Skip if already completed
        if grep -q "${HARNESS}.*SUCCESS" "$CSV" 2>/dev/null; then
            echo "✅ Chunk $i/$(({{num_chunks}}-1)): $HARNESS (cached)"
            continue
        fi
        
        echo "🔬 Chunk $i/$(({{num_chunks}}-1)): $HARNESS"
        START=$(date +%s)
        
        if cargo kani --features verify-kani --harness "$HARNESS" 2>&1 | tee "kani_${HARNESS}.log"; then
            END=$(date +%s)
            ELAPSED=$((END - START))
            TIMESTAMP=$(date -Iseconds)
            echo "$TIMESTAMP,{{proof_type}},$HARNESS,$i,{{num_chunks}},SUCCESS,$ELAPSED" >> "$CSV"
            echo "✅ Chunk $i completed in ${ELAPSED}s"
        else
            END=$(date +%s)
            ELAPSED=$((END - START))
            TIMESTAMP=$(date -Iseconds)
            echo "$TIMESTAMP,{{proof_type}},$HARNESS,$i,{{num_chunks}},FAILED,$ELAPSED" >> "$CSV"
            echo "❌ Chunk $i failed after ${ELAPSED}s"
            echo "See: kani_${HARNESS}.log"
            exit 1
        fi
        echo ""
    done
    
    echo "✅ All chunks completed for {{proof_type}} / {{num_chunks}} chunks"
    echo "📊 Results: $CSV"

# Show status of chunked proof progress
kani-chunked-status proof_type num_chunks:
    #!/usr/bin/env bash
    CSV="kani_proof_record_{{proof_type}}_{{num_chunks}}.csv"
    
    if [ ! -f "$CSV" ]; then
        echo "❌ No record found: $CSV"
        echo ""
        echo "Available configurations:"
        echo "  just kani-chunked 2byte 2"
        echo "  just kani-chunked 2byte 4"
        echo "  just kani-chunked 3byte 4"
        echo "  just kani-chunked 3byte 12"
        echo "  just kani-chunked 4byte 3"
        exit 0
    fi
    
    echo "📊 Chunked Proof Status: {{proof_type}} / {{num_chunks}} chunks"
    echo "========================================"
    echo ""
    
    # Count completed chunks
    TOTAL=$(tail -n +2 "$CSV" | wc -l)
    SUCCESS=$(tail -n +2 "$CSV" | grep -c SUCCESS || echo 0)
    FAILED=$(tail -n +2 "$CSV" | grep -cE "FAILED|ERROR" || echo 0)
    
    echo "Total runs: $TOTAL"
    echo "Successful: $SUCCESS"
    echo "Failed: $FAILED"
    echo ""
    
    if [ $SUCCESS -gt 0 ]; then
        echo "Recent completions:"
        tail -n +2 "$CSV" | grep SUCCESS | tail -5 | \
            awk -F, '{printf "  ✅ Chunk %s (%s combos) in %ss\n", $4, $7, $8}'
    fi
    
    if [ $FAILED -gt 0 ]; then
        echo ""
        echo "Failed chunks:"
        tail -n +2 "$CSV" | grep -E "FAILED|TIMEOUT|ERROR" | \
            awk -F, '{printf "  ❌ Chunk %s: %s\n", $4, $9}'
    fi
