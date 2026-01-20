# Verification Implementation Plan

**Goal:** Implement Contract trait for all Elicitation types across all 4 verifiers (Kani, Creusot, Prusti, Verus) to prove the multi-verifier contract system works.

**Vision:** Users can swap verification backends and refine contracts as their understanding evolves, just like with the style system.

---

## Phase 1: Proof of Concept - Email Validation

**Objective:** Implement Email contracts in all 4 verifiers to prove the pattern works.

**Timeline:** Days 1-5

### Tasks

- [ ] 1.1 Extend Kani Email contract (already partially done)
  - File: `src/verification/kani.rs`
  - Add verification harness for Email
  - Test: `cargo kani --harness verify_email_kani`

- [ ] 1.2 Create Creusot Email contract
  - File: `src/verification/creusot.rs`
  - Add `creusot-contracts` dependency
  - Implement with `#[requires]`/`#[ensures]` attributes
  - Test: `cargo creusot` in elicitation package

- [ ] 1.3 Create Prusti Email contract
  - File: `src/verification/prusti.rs` (new)
  - Add `prusti-contracts` dependency
  - Implement with `#[pure]`, `#[requires]`, `#[ensures]`
  - Test: `cargo prusti --features verify-prusti`

- [ ] 1.4 Create Verus Email contract
  - File: `src/verification/verus.rs` (new)
  - Implement with Verus syntax (`requires`, `ensures`)
  - Test: `verus src/verification/verus.rs`

- [ ] 1.5 Test all 4 implementations
  - Verify each produces correct proofs
  - Document differences in approach
  - Confirm Contract trait works for all

**Success Criteria:**
- ✅ Email validated by all 4 verifiers
- ✅ Each verifier produces successful proof
- ✅ Contract trait interface works for all

---

## Phase 2: Infrastructure - Contract Swapping

**Objective:** Build the infrastructure to swap contracts at compile-time and runtime.

**Timeline:** Days 6-10

### Tasks

- [ ] 2.1 Add verifier dependencies
  - Update `Cargo.toml` with optional dependencies
  - `creusot-contracts = { version = "0.2", optional = true }`
  - `prusti-contracts = { version = "0.2", optional = true }`
  - Feature flags for each verifier

- [ ] 2.2 Create contract registry pattern
  - File: `src/verification/mod.rs`
  - `enum VerifierBackend` with variants for each
  - Unified interface for verification dispatch

- [ ] 2.3 Implement `.with_contract()` method
  - Trait extension for `Elicitation`
  - Allow runtime contract swapping
  - Example:
    ```rust
    Email::with_contract(CreusotEmailContract)
        .elicit(client)
        .await?
    ```

- [ ] 2.4 Add compile-time contract selection
  - Feature-gated default contracts
  - Users choose via features: `--features verify-creusot`
  - Fallback to Kani if no feature specified

- [ ] 2.5 Update justfile recipes
  - `verify-email-kani`
  - `verify-email-creusot`
  - `verify-email-prusti`
  - `verify-email-verus`
  - `verify-email-all` (runs all)

**Success Criteria:**
- ✅ Can swap verifiers via features
- ✅ Can swap contracts at runtime
- ✅ Justfile recipes work for all verifiers
- ✅ Default contracts provided for all

---

## Phase 3: Rollout - Primitive Types

**Objective:** Implement contracts for all primitive types with Elicitation impls.

**Timeline:** Days 11-20

### 3.1 String Type (Days 11-12)

- [ ] Kani contract: non-empty, length bounds
- [ ] Creusot contract: length invariants
- [ ] Prusti contract: ownership checks
- [ ] Verus contract: string properties
- [ ] Test suite for all verifiers
- [ ] Documentation: when to use which verifier

### 3.2 Integer Types (Days 13-15)

- [ ] i32: range checks, overflow protection
- [ ] u32: positive bounds
- [ ] i64, u64: large number properties
- [ ] i128, u128: full range verification
- [ ] isize, usize: platform-dependent bounds
- [ ] Test all verifiers on each type

### 3.3 Boolean Type (Day 16)

- [ ] Trivial contracts (always valid)
- [ ] Completes primitive coverage
- [ ] Test as sanity check

### 3.4 Floating Point Types (Days 17-18)

- [ ] f32, f64: NaN/Infinity checks
- [ ] Range validation
- [ ] Precision bounds
- [ ] Document limitations per verifier

**Success Criteria:**
- ✅ All primitives have 4 contract impls
- ✅ Test suite passes for all verifiers
- ✅ Documentation explains tradeoffs

---

## Phase 4: Rollout - Complex Types

**Objective:** Implement contracts for complex/composite types.

**Timeline:** Days 21-30

### 4.1 Vec\<T\> (Days 21-23)

- [ ] Non-empty vectors
- [ ] Length bounds (min/max)
- [ ] Element contracts (recursive)
- [ ] Bounded verification (Kani)
- [ ] Inductive proofs (Creusot)

### 4.2 Option\<T\> (Days 24-25)

- [ ] Some variant checks
- [ ] Inner type contracts
- [ ] Composition with other contracts

### 4.3 Result\<T, E\> (Days 26-27)

- [ ] Ok/Err invariants
- [ ] Error type contracts
- [ ] Success type contracts

### 4.4 Custom Enums (Days 28-30)

- [ ] Unit variants (simple)
- [ ] Tuple variants (with data)
- [ ] Struct variants (complex)
- [ ] Derive macro support (future)

**Success Criteria:**
- ✅ Complex types verified by all tools
- ✅ Recursive contracts work
- ✅ Composition patterns documented

---

## Phase 5: Examples & Documentation

**Objective:** Comprehensive examples and user guide for verification system.

**Timeline:** Days 31-35

### Tasks

- [ ] 5.1 Create per-verifier examples
  - `examples/verification_kani_example.rs`
  - `examples/verification_creusot_example.rs`
  - `examples/verification_prusti_example.rs`
  - `examples/verification_verus_example.rs`

- [ ] 5.2 Create multi-verifier example
  - `examples/verification_multi_example.rs`
  - Shows swapping contracts
  - Demonstrates refinement workflow

- [ ] 5.3 Write verification guide
  - Update `VERIFICATION_FRAMEWORK_DESIGN.md`
  - Add "Choosing a Verifier" section
  - Document refinement patterns

- [ ] 5.4 Create migration guide
  - How to start with defaults
  - When to switch verifiers
  - How to write custom contracts

- [ ] 5.5 Document limitations
  - What each verifier can/can't do
  - Performance characteristics
  - Soundness vs completeness

**Success Criteria:**
- ✅ Working example for each verifier
- ✅ Clear documentation of tradeoffs
- ✅ Migration path documented

---

## Phase 6: Polish & Release

**Objective:** Integration testing, CI/CD, and crates.io release.

**Timeline:** Days 36-40

### Tasks

- [ ] 6.1 Integration test suite
  - Test contract composition
  - Test verifier swapping
  - Test all type combinations

- [ ] 6.2 CI/CD integration
  - Add verification to GitHub Actions
  - Run Kani in CI (others optional)
  - Cache verification artifacts

- [ ] 6.3 Performance benchmarks
  - Measure verification time per verifier
  - Document build time impact
  - Optimize hot paths

- [ ] 6.4 Update CHANGELOG
  - Document verification system
  - Breaking changes (if any)
  - Migration guide reference

- [ ] 6.5 Release to crates.io
  - Version bump (0.5.0?)
  - Publish with all features
  - Announce in community

**Success Criteria:**
- ✅ All tests pass
- ✅ CI/CD running verification
- ✅ Published to crates.io
- ✅ Documentation complete

---

## Current Status

### Completed

- ✅ Generic Contract trait created
- ✅ Kani adapter implemented and working
- ✅ Creusot adapter (runtime only)
- ✅ All verifiers installed
- ✅ Justfile recipes for setup
- ✅ Basic examples (Kani working)

### In Progress

- 🔄 Phase 1: Email validation across all verifiers

### Not Started

- ⬜ Phase 2: Contract swapping infrastructure
- ⬜ Phase 3: Primitive types rollout
- ⬜ Phase 4: Complex types rollout
- ⬜ Phase 5: Examples & documentation
- ⬜ Phase 6: Polish & release

---

## Success Criteria (Overall)

1. **Proof of Concept:** Email validated by all 4 verifiers
2. **Infrastructure:** Users can swap contracts at compile-time and runtime
3. **Coverage:** All Elicitation types have contracts
4. **Multi-Verifier:** Each type works with all 4 verifiers
5. **Documentation:** Users know when/how to use each verifier
6. **Refinement:** Clear path from defaults to custom contracts

---

## Risks & Mitigations

| Risk | Impact | Mitigation | Status |
|------|--------|------------|--------|
| Creusot requires code rewrite | High | Separate files per verifier | Planned |
| Prusti doesn't support all Rust | Medium | Document limitations | Planned |
| Verus has different syntax | High | Use macro generation | Planned |
| Contracts diverge over time | High | Single Contract trait interface | Done |
| Build time increases | Medium | Feature flags, optional deps | Done |
| Verification too slow | Low | Cache proofs, CI optimization | Planned |

---

## Open Questions

1. Should derive macro generate contracts automatically?
2. How to handle async code in verification?
3. Should contracts be stored in metadata for introspection?
4. Can we provide contract composition operators?
5. How to visualize proof obligations?

---

## Notes

- Kani is the "default" verifier (works everywhere)
- Other verifiers are opt-in for critical paths
- Contract trait provides unified interface
- Users refine contracts as understanding evolves
- Similar pattern to ElicitationStyle system
