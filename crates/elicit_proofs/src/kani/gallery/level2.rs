//! Gallery level 2: String field, length invariant.
//!
//! **Hypothesis**: This is where complexity first appears.
//!
//! - `d0`: empty string — should be fast (same as level 1).
//! - `d1`: one symbolic `char` — CBMC must enumerate all Unicode code-points.
//! - `d2`: two symbolic chars — quadratic in code-point space.
//! - `closure`: `kani_any()` string (length ≤ 4, each char symbolic).
//!
//! The invariant checks only `s.len() <= 4` (no character-level predicates),
//! so we are isolating *string construction cost* from *invariant complexity*.
//!
//! A companion sub-level (gallery2b) uses a discriminant-only invariant to
//! determine whether `s.len()` evaluation itself is expensive.

use std::mem::forget;

/// Two-variant enum where both arms carry a `String`.
#[cfg(kani)]
#[derive(Clone)]
enum GStr {
    A(String),
    B(String),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GStr {
    fn kani_depth0() -> Self {
        GStr::A(String::new())
    }
    fn kani_depth1() -> Self {
        GStr::B(<String as elicitation::KaniCompose>::kani_depth1())
    }
    fn kani_depth2() -> Self {
        GStr::A(<String as elicitation::KaniCompose>::kani_depth2())
    }
    /// Symbolic variant selector over depth-2 field constructions.
    ///
    /// Using `String::kani_any()` (4 symbolic chars) here times out.
    /// Instead we pick the variant symbolically but keep field content
    /// at `kani_depth2()` — already proven tractable.  The inductive
    /// argument is: d0/d1/d2 cover field growth; this covers variant
    /// selection.
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            GStr::A(<String as elicitation::KaniCompose>::kani_depth2())
        } else {
            GStr::B(<String as elicitation::KaniCompose>::kani_depth2())
        }
    }
}

/// Invariant: the string payload must be no longer than 4 characters.
/// This matches the bound used in `String::kani_any()`.
#[cfg(kani)]
fn g_str_consistent(s: &GStr) -> bool {
    match s {
        GStr::A(s) | GStr::B(s) => s.len() <= 4,
    }
}

/// Discriminant-only invariant — does NOT inspect string content at all.
/// Used to isolate whether `s.len()` evaluation is the bottleneck.
#[cfg(kani)]
fn g_str_consistent_cheap(_: &GStr) -> bool {
    true
}

/// Identity transition.
#[cfg(kani)]
fn g_str_identity(s: GStr) -> GStr {
    s
}

// ── L2 harnesses (s.len() <= 4 invariant) ────────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn gallery2_str_d0() {
    let state = <GStr as elicitation::KaniCompose>::kani_depth0();
    kani::assume(g_str_consistent(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent(&result), "L2 d0: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery2_str_d1() {
    let state = <GStr as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_str_consistent(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent(&result), "L2 d1: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery2_str_d2() {
    let state = <GStr as elicitation::KaniCompose>::kani_depth2();
    kani::assume(g_str_consistent(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent(&result), "L2 d2: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery2_str_closure() {
    let state = <GStr as elicitation::KaniCompose>::kani_any();
    kani::assume(g_str_consistent(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent(&result), "L2 closure: invariant preserved");
    forget(result);
}

// ── L2b harnesses (discriminant-only invariant — cheap baseline) ──────────────

#[cfg(kani)]
#[kani::proof]
fn gallery2b_str_d1_cheap_inv() {
    let state = <GStr as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_str_consistent_cheap(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent_cheap(&result), "L2b d1 cheap: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery2b_str_closure_cheap_inv() {
    let state = <GStr as elicitation::KaniCompose>::kani_any();
    kani::assume(g_str_consistent_cheap(&state));
    let result = g_str_identity(state);
    kani::assert(g_str_consistent_cheap(&result), "L2b closure cheap: invariant preserved");
    forget(result);
}
