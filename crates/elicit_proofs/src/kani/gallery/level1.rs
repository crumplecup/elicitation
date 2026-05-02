//! Gallery level 1: numeric fields, arithmetic invariant.
//!
//! **Hypothesis**: Numeric SAT is cheap even when symbolic.  All four harnesses
//! should complete in a few seconds at most.
//!
//! Passing this level shows the invariant-evaluation path works for scalar fields.

/// Two-variant enum with a `u32` field in each arm.
#[cfg(kani)]
#[derive(Clone)]
enum GInt {
    A(u32),
    B(u32),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GInt {
    fn kani_depth0() -> Self {
        GInt::A(0)
    }
    fn kani_depth1() -> Self {
        GInt::B(0)
    }
    fn kani_depth2() -> Self {
        GInt::A(1)
    }
    fn kani_any() -> Self {
        let v: u32 = kani::any();
        if kani::any::<bool>() {
            GInt::A(v)
        } else {
            GInt::B(v)
        }
    }
}

/// Invariant: the numeric payload must be strictly less than 1000.
#[cfg(kani)]
fn g_int_consistent(s: &GInt) -> bool {
    match s {
        GInt::A(v) | GInt::B(v) => *v < 1000,
    }
}

/// Identity transition — passes state through.
#[cfg(kani)]
fn g_int_identity(s: GInt) -> GInt {
    s
}

// ── Harnesses ─────────────────────────────────────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn gallery1_int_d0() {
    let state = <GInt as elicitation::KaniCompose>::kani_depth0();
    kani::assume(g_int_consistent(&state));
    let result = g_int_identity(state);
    kani::assert(g_int_consistent(&result), "L1 d0: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery1_int_d1() {
    let state = <GInt as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_int_consistent(&state));
    let result = g_int_identity(state);
    kani::assert(g_int_consistent(&result), "L1 d1: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery1_int_d2() {
    let state = <GInt as elicitation::KaniCompose>::kani_depth2();
    kani::assume(g_int_consistent(&state));
    let result = g_int_identity(state);
    kani::assert(g_int_consistent(&result), "L1 d2: invariant preserved");
}

/// Closure: symbolic `v` drawn from the full `u32` space, constrained by the
/// invariant (`v < 1000`) via `kani::assume`.  Tests that CBMC can discharge
/// a numeric range assumption efficiently.
#[cfg(kani)]
#[kani::proof]
fn gallery1_int_closure() {
    let state = <GInt as elicitation::KaniCompose>::kani_any();
    kani::assume(g_int_consistent(&state));
    let result = g_int_identity(state);
    kani::assert(g_int_consistent(&result), "L1 closure: invariant preserved");
}
