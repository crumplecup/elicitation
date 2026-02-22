# Prusti Verification Branch - Awaiting Tooling Renaissance

## Status: ⏸️ **Preserved on Branch**

The `prusti-verification` branch exists as a frozen artifact, preserving 427 formal proofs across 19 modules until Prusti tooling updates to support the modern Rust ecosystem.

## The Dream That Lives On

Branch: `prusti-verification` (edition 2021, no CI)  
Status: Complete implementation, blocked by upstream  
Proofs: 427 across 19 modules in `elicitation_prusti`

## The Trifecta of Formalism (Active on main/dev)

While Prusti sleeps, three verifiers actively guard our code:

1. **Kani** ✅ - Model checking with CBMC backend
2. **Verus** ✅ - SMT-based verification with advanced type system
3. **Creusot** ✅ - Deductive verification via Why3

## The Blocker

**Last Prusti Release:** cargo-prusti 0.2.2 (rustc 1.74.0, Oct 2023)  
**Edition 2024 Stabilized:** February 2024  
**Ecosystem Status:** crates.io has moved to edition 2024

Prusti's embedded cargo cannot parse edition 2024 manifests, even in transitive dependencies:

```
error: failed to parse manifest at `time-core-0.1.8/Cargo.toml`
Caused by:
  this version of Cargo is older than the `2024` edition
```

This isn't a configuration issue - it's ecosystem incompatibility.

## What Lives on the Branch

- Edition 2021 throughout workspace
- Let-chain syntax converted to edition 2021 forms
- Cargo.lock version 3 (not 4)
- Full Prusti integration ready to activate
- Comprehensive documentation of the attempt

## When Prusti Returns

Test compatibility:
```bash
git checkout prusti-verification
cargo install cargo-prusti  # When updated
just verify-prusti-tracked
```

If successful, Prusti rejoins the trifecta on main/dev!

---

*"A dream deferred is not a dream denied."*
