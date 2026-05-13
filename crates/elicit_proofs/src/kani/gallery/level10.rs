//! Gallery level 10: Isolate the drop-glue cost in the real ArchivePanelState harness.
//!
//! Level 9d showed 18 uniform-MonitorSnapshot variants finish in ~31s when the value
//! is `mem::forget`'d.  The real `column_detail__kani_closure` times out even though
//! its body is `(ArchivePanelState::ColumnDetail, proof)` — trivially cheap.
//!
//! Hypothesis: the difference is DROP GLUE.  In gallery9d the enum is forgotten.
//! In the real harness, `_state` is moved into `column_detail` and dropped there.
//! CBMC must model the destructor over all 18 symbolic variants, each with nested
//! Vec/String fields, producing an unbounded SAT formula.
//!
//! This level:
//! - 10a: `ArchivePanelState::kani_any()` + `mem::forget` — should be fast (no drop)
//! - 10b: `ArchivePanelState::kani_any()` + move into identity fn + drop there — should
//!         be slow if drop glue is the bottleneck
//! - 10c: same as real harness shape (kani_any + assume + call + assert + forget result)
//!         but with extra `mem::forget(_state)` before the call — should be fast
//!
//! If 10a is fast, 10b is slow, and 10c is fast: drop glue IS the bottleneck, and the
//! fix is to `mem::forget` the input state in the generated closure harness.

use elicit_server::archive::vsm::ArchivePanelState;
use elicitation::KaniCompose;

// ── helpers ──────────────────────────────────────────────────────────────────

/// Identity function that consumes and drops its argument — like a real transition.
#[cfg(kani)]
fn drop_state(state: ArchivePanelState) {
    let _ = state;
}

// ── 10a: kani_any + forget — baseline (should be fast like gallery9d) ─────────

#[cfg(kani)]
#[kani::proof]
fn gallery10a_any_forget() {
    let s = ArchivePanelState::kani_any();
    std::mem::forget(s);
}

// ── 10b: kani_any + move-into-fn (drop inside fn) — tests drop-glue cost ────

#[cfg(kani)]
#[kani::proof]
fn gallery10b_any_drop_in_fn() {
    let s = ArchivePanelState::kani_any();
    drop_state(s); // s is dropped inside drop_state
}

// ── 10c: real harness shape but with forget of input — proposed fix ──────────

#[cfg(kani)]
fn trivial_transition(state: ArchivePanelState, _proof: ()) -> (ArchivePanelState, ()) {
    std::mem::forget(state); // prevent drop of input
    (ArchivePanelState::ColumnDetail, ())
}

#[cfg(kani)]
#[kani::proof]
fn gallery10c_any_forget_input() {
    let s = ArchivePanelState::kani_any();
    let proof = ();
    let _result = trivial_transition(s, proof);
    std::mem::forget(_result);
}
