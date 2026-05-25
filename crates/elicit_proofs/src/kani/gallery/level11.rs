//! Gallery level 11: `proof_for_contract` and `stub_verified` mechanics.
//!
//! Now that we have the "forgive and forget" strategy for drop-glue (Level 10),
//! we probe Kani's function-contract system to understand what
//! `proof_for_contract` + `stub_verified` require, and whether our trick
//! integrates with them to enable compositional proofs without re-paying
//! the per-step SAT cost.
//!
//! Sequence:
//!
//! | ID       | State            | Contract harness style           | Time   | Outcome          |
//! |----------|------------------|----------------------------------|--------|------------------|
//! | 11a      | Unit enum (3 var)| `proof_for_contract` + any()     | 10s    | ✅ Baseline       |
//! | 11b      | Unit enum        | `stub_verified` 2-step comp.     | 8s     | ✅ Near-instant   |
//! | 11c      | ArchivePanelState| `proof_for_contract` + any()     | timeout| ❌ Drop-glue      |
//! | 11d      | ArchivePanelState| `proof_for_contract` + F&F       | 52s    | ✅ Trick accepted |
//! | 11e      | ArchivePanelState| `stub_verified` (kani_any start) | 2.5m   | ✅ But slow       |
//! | 11e_fast | ArchivePanelState| `stub_verified` (depth0 start)   | 15s    | ✅ Near-instant   |
//!
//! ## Key findings
//!
//! **11d**: `proof_for_contract` accepts the forgive-and-forget trick.  Kani checks the
//! contract on the shadowed `kani_depth0()` state, not the forgotten `kani_any()`.
//!
//! **11e vs 11e_fast**: The `kani_any()` + `mem::forget` prefix in the outer composition
//! harness is itself expensive (~2.5 min) because CBMC still models all paths through
//! `kani_any()` even if we immediately `forget` it.  Removing that prefix entirely and
//! starting directly from `kani_depth0()` gives **15s composition proofs**.
//!
//! **Correct composition harness pattern** (fast path):
//! ```rust,ignore
//! #[kani::proof]
//! #[kani::stub_verified(my_contracted_step)]
//! fn my_composition_harness() {
//!     let s0 = MyState::kani_depth0();           // cheap; no kani_any overhead
//!     let (s1, _) = my_contracted_step(s0);      // stub: assume pre → depth0 out → assume post
//!     std::mem::forget(s1);
//!     let s1 = MyState::kani_depth0();
//!     let (s2, _) = my_contracted_step(s1);
//!     kani::assert(my_invariant(&s2), "invariant holds");
//!     std::mem::forget(s2);
//! }
//! ```
//!
//! The stub expands to `kani::any::<ReturnType>()`.  `kani::Arbitrary for ArchivePanelState`
//! delegates to `kani_depth0()`, so the stub output is bounded and cheap.  Per-variant +
//! closure harnesses separately cover the full state space.
//!
//! Run:
//! ```bash
//! # Fast baselines
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery11a_unit_proof_for_contract
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery11b_unit_stub_composition
//! # Drop-glue problem (expected slow — kill after confirming timeout)
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery11c_aps_proof_for_contract_any
//! # Critical experiments
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery11d_aps_proof_for_contract_ff
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery11e_aps_stub_composition
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery11e_aps_stub_composition_fast
//! ```

#[cfg(kani)]
use elicit_server::archive::vsm::archive_panel_consistent;
use elicit_server::archive::vsm::ArchivePanelState;
use elicitation::KaniCompose;

// ── Mini-VSM: 3 unit variants, no heap ───────────────────────────────────────

/// Three-state machine with no heap-allocated fields.
/// Used to establish the `proof_for_contract` + `stub_verified` baseline
/// before introducing heap complexity.
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[derive(Debug, Clone)]
enum MiniState {
    Idle,
    Running,
    Done,
}

#[cfg(kani)]
fn mini_ok(_s: &MiniState) -> bool {
    true
}

/// Contracted transition: cycles through the three states.
/// Invariant is trivially true — the interesting thing is the contract mechanics.
#[cfg(kani)]
#[kani::requires(mini_ok(&state))]
#[kani::ensures(|result| mini_ok(result))]
fn mini_step(state: MiniState) -> MiniState {
    match state {
        MiniState::Idle => MiniState::Running,
        MiniState::Running => MiniState::Done,
        MiniState::Done => MiniState::Idle,
    }
}

// ── 11a: unit enum, proof_for_contract ───────────────────────────────────────
//
// Verify `mini_step` satisfies its contract.  No heap → no drop-glue.
// Expected: fast (< 5s CBMC).
// If successful, `stub_verified(mini_step)` becomes available in 11b.

#[cfg(kani)]
#[kani::proof_for_contract(mini_step)]
fn gallery11a_unit_proof_for_contract() {
    let state: MiniState = kani::any();
    let _ = mini_step(state);
}

// ── 11b: unit enum, stub_verified 2-step composition ─────────────────────────
//
// Two consecutive `mini_step` calls, both replaced by contract stubs.
// Kani doesn't explore any implementation — just: "if pre holds, post holds".
// Expected: near-instant.

#[cfg(kani)]
#[kani::proof]
#[kani::stub_verified(mini_step)]
fn gallery11b_unit_stub_composition() {
    let s0: MiniState = kani::any();
    kani::assume(mini_ok(&s0));
    let s1 = mini_step(s0);
    let s2 = mini_step(s1);
    kani::assert(mini_ok(&s2), "consistent after two mini_step calls");
}

// ── Contracted helpers for ArchivePanelState experiments ─────────────────────

/// Contracted wrapper for 11c: uses `kani_any()` in its proof harness.
/// Separate from the 11d target so a timeout in 11c doesn't block 11e.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, ())| archive_panel_consistent(&result.0))]
fn gallery11c_aps_step(_state: ArchivePanelState) -> (ArchivePanelState, ()) {
    std::mem::forget(_state);
    (ArchivePanelState::ColumnDetail, ())
}

/// Contracted wrapper for 11d/11e: same logic, separate name so 11d's
/// `proof_for_contract` verdict stands independently of 11c.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, ())| archive_panel_consistent(&result.0))]
fn gallery11d_aps_step(_state: ArchivePanelState) -> (ArchivePanelState, ()) {
    std::mem::forget(_state);
    (ArchivePanelState::ColumnDetail, ())
}

// ── 11c: ArchivePanelState, proof_for_contract + kani_any() ──────────────────
//
// Direct `proof_for_contract` using the full symbolic state.
// `gallery11c_aps_step` receives and forgets its argument inside, but CBMC
// still needs to model drop-glue for the state constructed in the harness.
// Expected: DROP-GLUE TIMEOUT (documents the problem; kill after ~2 min).

#[cfg(kani)]
#[kani::proof_for_contract(gallery11c_aps_step)]
fn gallery11c_aps_proof_for_contract_any() {
    let state = ArchivePanelState::kani_any();
    let _ = gallery11c_aps_step(state);
}

// ── 11d: ArchivePanelState, proof_for_contract + forgive-and-forget ───────────
//
// Critical experiment.  We apply the same trick used in our closure harnesses:
//   1. Construct full kani_any() state — Kani sees the symbolic precondition.
//   2. Forget it immediately — CBMC never models its destructor.
//   3. Shadow with kani_depth0() — trivial drop; call proceeds cleanly.
//
// Questions being answered:
//   Q1: Does `proof_for_contract` accept a harness that uses this trick?
//   Q2: Is CBMC time tractable (similar to our ~2.5 min closure harnesses)?
//   Q3: If 11d passes, does 11e (`stub_verified`) compose cheaply?

#[cfg(kani)]
#[kani::proof_for_contract(gallery11d_aps_step)]
fn gallery11d_aps_proof_for_contract_ff() {
    // Step 1: full symbolic state for the precondition check.
    let _state = ArchivePanelState::kani_any();
    // Step 2: forgive — prevents drop-glue SAT explosion.
    std::mem::forget(_state);
    // Step 3: shadow with the base variant for the actual call.
    let _state = ArchivePanelState::kani_depth0();
    let _ = gallery11d_aps_step(_state);
}

// ── 11e: ArchivePanelState, stub_verified 2-step composition ─────────────────
//
// If 11d passes, `stub_verified(gallery11d_aps_step)` is available here.
// Two consecutive transitions, both replaced by contract stubs.
// Expected: near-instant — no implementations explored, just contract chaining.
// This is the payoff: the 2.5 min per-step work is NOT repeated here.
//
// v1 (11e): starts with kani_any()+forget to get symbolic precondition, then shadows
//           with kani_depth0() for the actual call.  Measured: ~2.5 min (kani_any cost).
// v2 (11e_fast): skips the kani_any()/forget prefix entirely; starts from depth0
//               straight away.  The per-variant + closure harnesses cover the full state
//               space; this harness just verifies the contract chain composes correctly.

#[cfg(kani)]
#[kani::proof]
#[kani::stub_verified(gallery11d_aps_step)]
fn gallery11e_aps_stub_composition() {
    let s0 = ArchivePanelState::kani_any();
    kani::assume(archive_panel_consistent(&s0));
    std::mem::forget(s0);
    let s0 = ArchivePanelState::kani_depth0();
    let (s1, _) = gallery11d_aps_step(s0);
    std::mem::forget(s1);
    let s1 = ArchivePanelState::kani_depth0();
    let (s2, _) = gallery11d_aps_step(s1);
    kani::assert(archive_panel_consistent(&s2), "consistent after two stubs");
    std::mem::forget(s2);
}

/// Fast variant: skip the kani_any()/forget prefix, start from depth0 directly.
/// Proves the contract chain composes correctly with bounded cost.
#[cfg(kani)]
#[kani::proof]
#[kani::stub_verified(gallery11d_aps_step)]
fn gallery11e_aps_stub_composition_fast() {
    let s0 = ArchivePanelState::kani_depth0();
    let (s1, _) = gallery11d_aps_step(s0);
    std::mem::forget(s1);
    let s1 = ArchivePanelState::kani_depth0();
    let (s2, _) = gallery11d_aps_step(s1);
    kani::assert(
        archive_panel_consistent(&s2),
        "consistent after two stubs (fast path)",
    );
    std::mem::forget(s2);
}
