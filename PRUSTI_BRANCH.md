# Prusti Verification Branch - Frozen Until Upstream Update

## Status: ⏸️ **Preserved Dream**

This branch exists as a historical artifact and future-ready foundation for when Prusti tooling is updated to support modern Rust ecosystems.

**Current Blocker:** Prusti's embedded cargo (rustc 1.74.0, Oct 2023) cannot parse edition 2024, even in transitive dependencies from crates.io. This is not a configuration issue - it's a fundamental incompatibility with the modern Rust ecosystem as of 2024.

**Last Prusti Release:** cargo-prusti 0.2.2 (using rustc 1.74 from 2023-09-14)  
**Edition 2024 Stabilization:** February 2024  
**Upstream Status:** No releases or significant activity since 2023

## Why This Branch Exists

Rather than delete the Prusti integration work, we preserve it here:
- 427 proofs across 19 modules remain in `elicitation_prusti` crate
- Branch uses edition 2021 throughout (compatible with Prusti's cargo)
- Let-chain syntax converted from edition 2024 to 2021 compatible forms
- Cargo.lock downgraded to version 3
- Ready to activate when Prusti updates

## The Trifecta of Formalism

On `dev`/`main` branches (edition 2024), we support three **active** verifiers:

1. **Kani** - Model checking with CBMC backend
2. **Verus** - SMT-based verification with advanced type system  
3. **Creusot** - Deductive verification via Why3

Prusti will rejoin this lineup when upstream catches up to modern Rust.

## What We Changed for Edition 2021

### Workspace Configuration
- `Cargo.toml`: edition 2024 → 2021
- `crates/elicitation_creusot/Cargo.toml`: edition 2024 → 2021
- `Cargo.lock`: version 4 → version 3

### Let-Chain Syntax (Edition 2024 → 2021)

**elicitation_macros/src/lib.rs:**
```rust
// Edition 2024 (let chains)
if let syn::FnArg::Typed(pat_type) = arg
    && let syn::Pat::Ident(ident) = &*pat_type.pat
{
    return Some(ident.ident.clone());
}

// Edition 2021 (nested if-let)
if let syn::FnArg::Typed(pat_type) = arg {
    if let syn::Pat::Ident(ident) = &*pat_type.pat {
        return Some(ident.ident.clone());
    }
}
```

**elicitation_derive/src/contract_type.rs:**
```rust
// Edition 2024 (let chains)
if let Some(name) = name
    && let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = nv.value
{
    match name.as_str() { ... }
}

// Edition 2021 (nested if-let)
if let Some(name) = name {
    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = nv.value {
        match name.as_str() { ... }
    }
}
```

## The Fatal Blocker: Ecosystem Edition Mismatch

Even with all edition downgrades, Prusti's cargo fails during dependency resolution:

```
error: failed to download replaced source registry `crates-io`
Caused by:
  failed to parse manifest at `time-core-0.1.8/Cargo.toml`
Caused by:
  this version of Cargo is older than the `2024` edition
```

The Rust ecosystem has moved to edition 2024. Prusti's cargo (from Oct 2023) cannot parse any crate.io package manifests that declare edition 2024, **even as transitive dependencies**.

## When Will This Work?

This branch will become viable when:
- Prusti releases an update using rustc 1.82+ (edition 2024 support)
- Or the Prusti project provides a compatibility layer
- Or someone forks Prusti and updates it (massive undertaking)

Until then, this branch serves as documentation of the attempt and a ready foundation for future work.

## Current Verification Status

| Verifier | Status | Location | Branch |
|----------|--------|----------|--------|
| **Kani** | ✅ Active | `elicitation_kani` | `main`/`dev` |
| **Verus** | ✅ Active | `elicitation_verus` | `main`/`dev` |
| **Creusot** | ✅ Active | `elicitation_creusot` | `main`/`dev` |
| **Prusti** | ⏸️ Frozen | `elicitation_prusti` | `prusti-verification` |

## For Future Maintainers

If Prusti updates:

1. Test on this branch first:
   ```bash
   git checkout prusti-verification
   cargo install cargo-prusti  # New version
   just verify-prusti-tracked
   ```

2. If successful, merge strategy to main:
   ```bash
   git checkout dev
   git merge prusti-verification  # Resolve edition conflicts
   # Keep edition 2024 on dev, test Prusti compatibility
   ```

3. If Prusti now supports edition 2024, delete this branch and move Prusti to main workflow.

## Acknowledgments

The integration work represents 427 formal proofs across 19 modules - a significant verification effort that deserves preservation, not deletion. This branch honors that work and keeps the door open for Prusti's return.

---

*"Some dreams are worth preserving, even when the tools aren't ready."*
