//! Gallery level 13: callee `#[instrument]` and `goto-instrument` hang.
//!
//! Level 12 established `proof_for_contract` (~32 s) as the leaf-proof baseline
//! when the contracted function has no instrumented callees.  Level 13 isolates
//! the cost of bare (ungated) `#[instrument]` on **callee** functions that lie
//! in the call tree under `proof_for_contract`.
//!
//! § 6.3 of `KANI_FOR_VSMS.md` documents that `#[instrument]` causes SAT
//! explosion even on trivial one-line functions.  Level 13 confirms this
//! extends to *callees* (not just the contracted function itself):
//! `goto-instrument --dfcc` inlines callee bodies, so any callee that carries
//! tracing spans drags in thread-local storage access, atomic operations, and
//! event-dispatch logic — inflating the CBMC formula beyond tractable limits.
//!
//! The fix is to gate `#[instrument]` with `#[cfg_attr(not(kani), instrument(…))]`
//! on every function in the call tree, not just the transition entry point.
//!
//! ## Experiment table
//!
//! | ID  | Callee instrumentation                        | Expected result     |
//! |-----|-----------------------------------------------|---------------------|
//! | 13a | bare `#[tracing::instrument(skip_all)]`       | goto-instrument hang |
//! | 13b | `#[cfg_attr(not(kani), tracing::instrument(…))]` | ~32 s (baseline) |
//!
//! ## Run commands
//!
//! ```bash
//! # 13a: expect hang — send SIGINT after ~2 minutes to confirm
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery13a_ungated_instrument
//!
//! # 13b: expect ~32 s (same cost class as Level 12 baseline)
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery13b_gated_instrument
//! ```

// ── State type ────────────────────────────────────────────────────────────────

/// Minimal two-variant unit enum — zero heap, trivially-true invariant.
/// Keeps the state-machine cost identical across both experiments so the only
/// variable is callee instrumentation.
#[cfg(kani)]
#[derive(Clone, Copy)]
enum G13State {
    Idle,
    Active,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G13State {
    fn kani_depth0() -> Self {
        G13State::Idle
    }
    fn kani_depth1() -> Self {
        G13State::Active
    }
    fn kani_depth2() -> Self {
        G13State::Idle
    }
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            G13State::Idle
        } else {
            G13State::Active
        }
    }
}

/// Trivially-true invariant — evaluates to `true` for all `G13State` values.
#[cfg(kani)]
fn g13_consistent(_: &G13State) -> bool {
    true
}

// ── L13a: callee with bare (ungated) #[tracing::instrument] ──────────────────
//
// `g13a_callee` carries a plain `#[tracing::instrument(skip_all)]` — not gated
// under `cfg_attr(not(kani), …)`.  When `proof_for_contract` is run on
// `g13a_contracted_fn`, `goto-instrument --dfcc` inlines `g13a_callee` and
// encounters the full tracing span machinery:
//
//   • `tracing::Span::new()` — thread-local dispatcher lookup
//   • `__tracing_attr_span.enter()` — atomic reference count bump
//   • drop-glue on `__tracing_attr_guard` — atomic release + potential flush
//
// Each of these is a symbolic branch over unbounded internal state in the
// tracing crate, inflating the CBMC formula to an intractable size.
// `goto-instrument` runs at 99 % CPU and never terminates.

#[cfg(kani)]
#[tracing::instrument(skip_all)]
fn g13a_callee(_state: G13State) -> G13State {
    G13State::Active
}

/// Contracted wrapper: forget input, delegate to `g13a_callee`.
/// The contract is trivial (invariant is `true`) so any cost comes from DFCC
/// inlining `g13a_callee` with its ungated tracing span.
#[cfg(kani)]
#[kani::requires(g13_consistent(&_state))]
#[kani::ensures(|result: &G13State| g13_consistent(result))]
fn g13a_contracted_fn(_state: G13State) -> G13State {
    std::mem::forget(_state);
    g13a_callee(G13State::Idle)
}

// ── L13b: callee with gated #[instrument] ─────────────────────────────────────
//
// `g13b_callee` uses `#[cfg_attr(not(kani), tracing::instrument(skip_all))]`.
// Under kani `not(kani)` is `false`, so the `instrument` attribute is absent.
// DFCC inlines only the trivial callee body (one enum constructor).
//
// Expected: ~32 s — the same cost class as Level 12 baselines 12a–12c, where
// the contracted functions also had no callee overhead.

#[cfg(kani)]
#[cfg_attr(not(kani), tracing::instrument(skip_all))]
fn g13b_callee(_state: G13State) -> G13State {
    G13State::Active
}

/// Contracted wrapper — identical logic to `g13a_contracted_fn`.
/// The ONLY difference from the 13a pair is the `#[instrument]` guard on the callee.
#[cfg(kani)]
#[kani::requires(g13_consistent(&_state))]
#[kani::ensures(|result: &G13State| g13_consistent(result))]
fn g13b_contracted_fn(_state: G13State) -> G13State {
    std::mem::forget(_state);
    g13b_callee(G13State::Idle)
}

// ── Harnesses ─────────────────────────────────────────────────────────────────

// ── 13a: ungated callee instrument → goto-instrument hang ────────────────────
//
// Forgive-and-forget pattern: kani_any() → forget → kani_depth0() → call.
// The contracted fn calls g13a_callee, which has bare #[tracing::instrument].
// goto-instrument --dfcc inlines the callee and hits the tracing closure.
//
// RESULT (expected): goto-instrument runs at 99 % CPU and never terminates.
// Confirmed by: running with `timeout 120 cargo kani …` and observing no output.

#[cfg(kani)]
#[kani::proof_for_contract(g13a_contracted_fn)]
fn gallery13a_ungated_instrument() {
    let _state = G13State::kani_any();
    std::mem::forget(_state);
    let _state = G13State::kani_depth0();
    let _ = g13a_contracted_fn(_state);
}

// ── 13b: gated callee instrument → ~32 s baseline ────────────────────────────
//
// Identical harness; the ONLY change is that g13b_callee has instrument gated.
// Under kani the callee body is: return G13State::Active.  DFCC sees one
// enum constructor — negligible symbolic cost.
//
// RESULT (expected): completes in ~32 s — matches Level 12 leaf-proof baseline.

#[cfg(kani)]
#[kani::proof_for_contract(g13b_contracted_fn)]
fn gallery13b_gated_instrument() {
    let _state = G13State::kani_any();
    std::mem::forget(_state);
    let _state = G13State::kani_depth0();
    let _ = g13b_contracted_fn(_state);
}
