# Prusti Verification Branch Strategy

## Problem

Prusti uses an old toolchain (rustc 1.74 from 2023) that doesn't support Rust edition 2024. The main development branch uses edition 2024, making Prusti incompatible.

## Solution: Compatibility Branch

We maintain a `prusti-verification` branch that:
- Uses edition 2021 (compatible with Prusti)
- Contains identical code to main branch
- Automatically runs Prusti verification via CI
- Periodically syncs from main branch

## Branch Management

### Creating the Prusti Branch

```bash
# Create prusti branch from current dev
git checkout dev
git checkout -b prusti-verification

# Downgrade editions to 2021
sed -i 's/edition = "2024"/edition = "2021"/' Cargo.toml
sed -i 's/edition.workspace = true/edition = "2021"/' crates/elicitation/Cargo.toml

# Commit edition changes
git add Cargo.toml crates/elicitation/Cargo.toml
git commit -m "chore(prusti): downgrade to edition 2021 for Prusti compatibility"

# Push to remote
git push -u origin prusti-verification
```

### Syncing from Main

Periodically merge changes from dev/main:

```bash
# On prusti-verification branch
git checkout prusti-verification
git merge dev

# Resolve any conflicts (usually just edition fields)
# Keep edition = "2021" in Cargo.toml files

git push origin prusti-verification
```

### Running Prusti Locally

```bash
# Switch to prusti branch
git checkout prusti-verification

# Run verification
cd crates/elicitation_prusti
cargo prusti --all-features

# Or use justfile
just verify-prusti
```

## CI Integration

GitHub Actions automatically runs Prusti verification on the `prusti-verification` branch. Check `.github/workflows/prusti-verify.yml`.

## Justfile Integration

The prusti runner in the main branch documents this approach:

```rust
// prusti_runner.rs
pub fn run_all(output: &Path, timeout: u64) -> Result<()> {
    println!("⚠️  Prusti verification requires edition 2021 compatibility.");
    println!("   Switch to prusti-verification branch to run verification:");
    println!("   $ git checkout prusti-verification");
    println!("   $ cd crates/elicitation_prusti && cargo prusti --all-features");
    println!();
    println!("   See PRUSTI_BRANCH.md for details.");
    // ... rest of implementation
}
```

## Benefits

1. **No compromise**: Main branch stays on edition 2024
2. **Full verification**: Prusti works on compatibility branch  
3. **Automated**: CI runs verification automatically
4. **Synchronized**: Easy to keep code in sync via merges
5. **Standard practice**: Common pattern for tool compatibility

## Maintenance

- **Weekly**: Merge dev → prusti-verification
- **On release**: Verify prusti branch before tagging
- **Monitor**: Watch for Prusti toolchain updates (can eventually eliminate branch)
