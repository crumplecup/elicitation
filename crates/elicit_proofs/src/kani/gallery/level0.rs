//! Gallery level 0: unit enum, no fields, trivially-true invariant.
//!
//! **Hypothesis**: Harness overhead (assume/assert/forget on a zero-size type)
//! is negligible.  All four harnesses should complete in under 1 second.
//!
//! Passing this level confirms the harness template itself is sound.

/// Two-variant unit enum — no heap allocation, zero symbolic state.
#[cfg(kani)]
#[derive(Clone)]
enum GUnit {
    A,
    B,
}

#[cfg(kani)]
impl elicitation::KaniCompose for GUnit {
    fn kani_depth0() -> Self {
        GUnit::A
    }
    fn kani_depth1() -> Self {
        GUnit::B
    }
    fn kani_depth2() -> Self {
        GUnit::A
    }
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            GUnit::A
        } else {
            GUnit::B
        }
    }
}

/// Trivially-true invariant — never fails, costs nothing to evaluate.
#[cfg(kani)]
fn g_unit_consistent(_: &GUnit) -> bool {
    true
}

/// Identity transition — state passes through unchanged.
#[cfg(kani)]
fn g_unit_identity(s: GUnit) -> GUnit {
    s
}

// ── Harnesses ─────────────────────────────────────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn gallery0_unit_d0() {
    let state = <GUnit as elicitation::KaniCompose>::kani_depth0();
    kani::assume(g_unit_consistent(&state));
    let result = g_unit_identity(state);
    kani::assert(g_unit_consistent(&result), "L0 d0: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery0_unit_d1() {
    let state = <GUnit as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_unit_consistent(&state));
    let result = g_unit_identity(state);
    kani::assert(g_unit_consistent(&result), "L0 d1: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery0_unit_d2() {
    let state = <GUnit as elicitation::KaniCompose>::kani_depth2();
    kani::assume(g_unit_consistent(&state));
    let result = g_unit_identity(state);
    kani::assert(g_unit_consistent(&result), "L0 d2: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery0_unit_closure() {
    let state = <GUnit as elicitation::KaniCompose>::kani_any();
    kani::assume(g_unit_consistent(&state));
    let result = g_unit_identity(state);
    kani::assert(g_unit_consistent(&result), "L0 closure: invariant preserved");
}
