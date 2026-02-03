# Elicitation Core Refactor Plan

## Goal

Extract pure contract types into `elicitation_core` crate so Creusot can verify them without encountering async code.

## Critical Success Criteria

1. **Zero loss**: Every type, trait, impl, test, and doc must be preserved
2. **Verification**: Automated checks at each step to ensure completeness
3. **Rollback**: Git commits after each phase for easy revert
4. **Compilation**: Main crate compiles at every step

## Pre-Refactor Inventory

### Step 0: Document Current State

- [ ] Count all public types in `elicitation/src/verification/types/*.rs`
- [ ] List all public traits (Contract, etc.)
- [ ] List all error types (ValidationError, etc.)
- [ ] Count all verification proof modules
- [ ] Record all re-exports in lib.rs
- [ ] Save inventory to `elicitation_core_inventory.json`

**Verification**: Run `cargo check` and `cargo test` - record baseline

## Phase 1: Create Core Crate (No Code Movement)

### Step 1.1: Create Crate Structure

- [ ] Create `crates/elicitation_core/` directory
- [ ] Create `crates/elicitation_core/Cargo.toml` with minimal deps
- [ ] Create `crates/elicitation_core/src/lib.rs` (empty, just module structure)
- [ ] Add to workspace members in root Cargo.toml
- [ ] Verify: `cargo check -p elicitation_core` passes

### Step 1.2: Set Up Dependencies

Copy ONLY these dependencies from elicitation to core:
- [ ] derive_more (for Display/Error)
- [ ] derive-getters (for field access)
- [ ] serde (for serialization)
- [ ] Verification deps (optional): creusot-std, prusti-contracts
- [ ] NO async deps (tokio, rmcp, etc.)
- [ ] Verify: `cargo check -p elicitation_core` passes

## Phase 2: Move Error Types (Foundation)

### Step 2.1: Move ValidationError

- [ ] Copy `error.rs` → `elicitation_core/src/error.rs`
- [ ] Update imports in error.rs (remove async-related)
- [ ] Add `pub mod error;` to core lib.rs
- [ ] Add `pub use error::*;` exports
- [ ] Verify: `cargo check -p elicitation_core` passes
- [ ] **COUNT CHECK**: Grep for "pub struct.*Error\|pub enum.*Error" - must match original count

### Step 2.2: Update Main Crate

- [ ] Add `elicitation_core` dependency to elicitation Cargo.toml
- [ ] Replace error module with re-export: `pub use elicitation_core::ValidationError;`
- [ ] Verify: `cargo check -p elicitation` passes
- [ ] Run `grep -r "ValidationError" crates/elicitation/src | wc -l` - usage count must match
- [ ] Commit: "refactor: Move ValidationError to elicitation_core"

## Phase 3: Move Contract Types (Bulk Movement)

### Step 3.1: Move Integer Types

For each file in order:
1. [ ] `verification/types/integers.rs` → `elicitation_core/src/integers.rs`
2. [ ] Update imports (remove crate::, add elicitation_core::)
3. [ ] Add module to core lib.rs
4. [ ] Add re-exports to core lib.rs
5. [ ] Replace in main crate with re-export
6. [ ] **COUNT CHECK**: 
   - Grep "pub struct.*Positive\|pub struct.*NonNegative\|pub struct.*NonZero\|pub struct.*Range" 
   - Count before/after must match
7. [ ] Verify: `cargo check` on both crates passes
8. [ ] Commit: "refactor: Move integer types to elicitation_core"

### Step 3.2: Move String Types

- [ ] `verification/types/strings.rs` → `elicitation_core/src/strings.rs`
- [ ] Same process as integers
- [ ] **COUNT CHECK**: Grep "pub struct String" - count must match
- [ ] Commit: "refactor: Move string types to elicitation_core"

### Step 3.3: Move Boolean Types

- [ ] `verification/types/bools.rs` → `elicitation_core/src/bools.rs`
- [ ] **COUNT CHECK**: Should be 2 types (BoolTrue, BoolFalse)
- [ ] Commit: "refactor: Move bool types to elicitation_core"

### Step 3.4: Move Character Types

- [ ] `verification/types/chars.rs` → `elicitation_core/src/chars.rs`
- [ ] **COUNT CHECK**: Grep "pub struct Char" - count must match
- [ ] Commit: "refactor: Move char types to elicitation_core"

### Step 3.5: Move Float Types

- [ ] `verification/types/floats.rs` → `elicitation_core/src/floats.rs`
- [ ] **COUNT CHECK**: Grep "pub struct F32\|pub struct F64"
- [ ] Commit: "refactor: Move float types to elicitation_core"

### Step 3.6: Move Duration Types

- [ ] `verification/types/durations.rs` → `elicitation_core/src/durations.rs`
- [ ] Handle conditional compilation for chrono/jiff/time
- [ ] **COUNT CHECK**: Grep for Duration types
- [ ] Commit: "refactor: Move duration types to elicitation_core"

### Step 3.7: Move Collection Types

- [ ] `verification/types/collections.rs` → `elicitation_core/src/collections.rs`
- [ ] **COUNT CHECK**: Count Vec*, HashMap*, etc. types
- [ ] Commit: "refactor: Move collection types to elicitation_core"

### Step 3.8: Move URL Types (if feature enabled)

- [ ] `verification/types/urls.rs` → `elicitation_core/src/urls.rs`
- [ ] Gate with `#[cfg(feature = "url")]`
- [ ] **COUNT CHECK**: Count Url* types
- [ ] Commit: "refactor: Move URL types to elicitation_core"

### Step 3.9: Move Regex Types (if feature enabled)

- [ ] `verification/types/regexes.rs` → `elicitation_core/src/regexes.rs`
- [ ] Gate with `#[cfg(feature = "regex")]`
- [ ] **COUNT CHECK**: Count Regex* types
- [ ] Commit: "refactor: Move regex types to elicitation_core"

## Phase 4: Move Core Traits

### Step 4.1: Move Contract Trait

- [ ] Copy `verification/contract.rs` → `elicitation_core/src/contract.rs`
- [ ] Update all impl blocks to reference elicitation_core types
- [ ] **COUNT CHECK**: Grep "impl Contract for" - count must match
- [ ] Verify: All contract impls still present
- [ ] Commit: "refactor: Move Contract trait to elicitation_core"

## Phase 5: Clean Up Main Crate

### Step 5.1: Update Re-exports

- [ ] Update `elicitation/src/lib.rs` to re-export from elicitation_core
- [ ] Remove old module declarations
- [ ] Keep: `pub use elicitation_core::*;` or selective exports
- [ ] **Verify**: Run `grep "pub use" crates/elicitation/src/lib.rs` - count public exports
- [ ] Compare with pre-refactor count - must be >= original

### Step 5.2: Update Tests

- [ ] Find all tests importing contract types
- [ ] Update imports to use `elicitation::*` (which re-exports from core)
- [ ] **Verify**: `cargo test -p elicitation` passes with same test count
- [ ] Commit: "refactor: Update test imports for core types"

## Phase 6: Wire Up Creusot Proofs

### Step 6.1: Update Creusot Proofs Crate

- [ ] Update `elicitation_creusot_proofs/Cargo.toml` to depend on `elicitation_core`
- [ ] Remove dependency on `elicitation`
- [ ] Update imports in proof files
- [ ] Verify: `cargo check -p elicitation_creusot_proofs` passes
- [ ] Commit: "refactor: Point Creusot proofs to elicitation_core"

### Step 6.2: Run Creusot Verification

- [ ] `cd crates/elicitation_creusot_proofs && cargo creusot`
- [ ] Verify: Compilation succeeds (no async panic)
- [ ] `cargo creusot prove` - run actual verification
- [ ] Document: How many proofs pass/fail
- [ ] Commit: "feat: Enable Creusot verification via elicitation_core"

## Phase 7: Final Verification

### Step 7.1: Automated Checks

Run these commands and compare to baseline:
- [ ] `cargo check --workspace` - must pass
- [ ] `cargo test --workspace` - test count must match baseline
- [ ] `cargo clippy --workspace` - warning count must be <= baseline
- [ ] `just check-all` - must pass
- [ ] Count public API surface: `cargo doc --no-deps && grep "pub " -r target/doc/elicitation`

### Step 7.2: Compare Inventories

- [ ] Generate post-refactor inventory
- [ ] Diff against pre-refactor inventory
- [ ] Every type, trait, function must be accounted for
- [ ] Document: What moved, what stayed, what changed

### Step 7.3: Documentation

- [ ] Update README with new crate structure
- [ ] Update module docs for elicitation_core
- [ ] Update verification documentation
- [ ] Add migration guide for users (if public API changed)

## Rollback Plan

If anything goes wrong at any step:

```bash
# Revert to last good commit
git log --oneline | head -20  # Find last working commit
git reset --hard <commit-hash>

# Or revert specific phase
git revert <commit-range>
```

## Automated Verification Scripts

### Pre-Refactor Inventory Script

```bash
#!/bin/bash
# Save to scripts/inventory.sh

echo "Collecting pre-refactor inventory..."

# Count types
echo "PUBLIC_STRUCTS=$(grep -r "pub struct" crates/elicitation/src/verification/types | wc -l)" > inventory.txt
echo "PUBLIC_ENUMS=$(grep -r "pub enum" crates/elicitation/src/verification/types | wc -l)" >> inventory.txt
echo "PUBLIC_TRAITS=$(grep -r "pub trait" crates/elicitation/src/verification | wc -l)" >> inventory.txt
echo "IMPL_CONTRACT=$(grep -r "impl Contract for" crates/elicitation/src/verification/types | wc -l)" >> inventory.txt

# List all public types
grep -r "pub struct\|pub enum" crates/elicitation/src/verification/types | \
  sed 's/.*pub \(struct\|enum\) \([A-Za-z0-9_]*\).*/\2/' | \
  sort | uniq > type_list.txt

echo "Saved to inventory.txt and type_list.txt"
```

### Post-Move Verification Script

```bash
#!/bin/bash
# Save to scripts/verify_move.sh

# Check that all types from pre-refactor exist in either core or main crate
while read type; do
  count_core=$(grep -r "pub struct $type\|pub enum $type" crates/elicitation_core/src | wc -l)
  count_main=$(grep -r "pub struct $type\|pub enum $type" crates/elicitation/src | wc -l)
  total=$((count_core + count_main))
  
  if [ $total -eq 0 ]; then
    echo "❌ MISSING: $type"
    exit 1
  fi
done < type_list.txt

echo "✅ All types accounted for"
```

## Safety Guarantees

1. **No files deleted until verified working** - Copy first, verify, then remove
2. **Git commit after each phase** - Easy rollback to any point
3. **Automated counts** - Scripts catch missing types immediately
4. **Compilation gates progress** - Can't proceed if previous step broke build
5. **Test coverage maintained** - Test count must stay same or increase

## Estimated Effort

- Phase 1-2 (Setup + Errors): ~30 minutes
- Phase 3 (Move types): ~2-3 hours (9 type files × 15-20 min each)
- Phase 4 (Traits): ~30 minutes
- Phase 5 (Cleanup): ~30 minutes
- Phase 6 (Creusot): ~30 minutes
- Phase 7 (Verification): ~30 minutes

**Total: 4-5 hours of careful, systematic work**

## Success Metrics

- [ ] Zero compilation errors
- [ ] Zero test failures
- [ ] All types present in inventory
- [ ] Creusot can verify core types without async panic
- [ ] Public API unchanged (or clearly documented changes)
- [ ] Documentation updated
