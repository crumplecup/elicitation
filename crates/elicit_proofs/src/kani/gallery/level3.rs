//! Gallery level 3: Vec<u32> field, length invariant.
//!
//! **Hypothesis**: `Vec` drop-glue is the second major cost source.
//!
//! - `d0`: empty Vec — should be fast.
//! - `d1`: `vec![0u32]` — one concrete element.
//! - `d2`: `vec![0u32, 0u32]` — two concrete elements.
//! - `closure`: `kani_vec_closure(1, 3)` — symbolic length ∈ 0..=3, elements concrete.
//!
//! The invariant checks only `v.len() <= 3`, matching the `kani_vec_closure`
//! bound.  `std::mem::forget` is used to suppress drop analysis.
//!
//! A companion `gallery3b` uses `Vec<String>` to stack both Vec and String cost.

use std::mem::forget;

/// Two-variant enum where both arms carry a `Vec<u32>`.
#[cfg(kani)]
#[derive(Clone)]
enum GVec {
    A(Vec<u32>),
    B(Vec<u32>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GVec {
    fn kani_depth0() -> Self {
        GVec::A(Vec::new())
    }
    fn kani_depth1() -> Self {
        GVec::B(vec![0u32])
    }
    fn kani_depth2() -> Self {
        GVec::A(vec![0u32, 0u32])
    }
    /// Symbolic variant selector over depth-2 field constructions.
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            GVec::A(vec![0u32, 0u32])
        } else {
            GVec::B(vec![0u32, 0u32])
        }
    }
}

/// Invariant: Vec length must be no more than 3.
#[cfg(kani)]
fn g_vec_consistent(s: &GVec) -> bool {
    match s {
        GVec::A(v) | GVec::B(v) => v.len() <= 3,
    }
}

/// Identity transition.
#[cfg(kani)]
fn g_vec_identity(s: GVec) -> GVec {
    s
}

// ── L3 harnesses (Vec<u32>) ────────────────────────────────────────────────────

#[cfg(kani)]
#[kani::proof]
fn gallery3_vec_d0() {
    let state = <GVec as elicitation::KaniCompose>::kani_depth0();
    kani::assume(g_vec_consistent(&state));
    let result = g_vec_identity(state);
    kani::assert(g_vec_consistent(&result), "L3 d0: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery3_vec_d1() {
    let state = <GVec as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_vec_consistent(&state));
    let result = g_vec_identity(state);
    kani::assert(g_vec_consistent(&result), "L3 d1: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery3_vec_d2() {
    let state = <GVec as elicitation::KaniCompose>::kani_depth2();
    kani::assume(g_vec_consistent(&state));
    let result = g_vec_identity(state);
    kani::assert(g_vec_consistent(&result), "L3 d2: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery3_vec_closure() {
    let state = <GVec as elicitation::KaniCompose>::kani_any();
    kani::assume(g_vec_consistent(&state));
    let result = g_vec_identity(state);
    kani::assert(g_vec_consistent(&result), "L3 closure: invariant preserved");
    forget(result);
}

// ── L3b: Vec<String> — stacked cost ──────────────────────────────────────────

/// Enum carrying `Vec<String>` — combines Vec and String symbolic cost.
#[cfg(kani)]
#[derive(Clone)]
enum GVecStr {
    A(Vec<String>),
    B(Vec<String>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GVecStr {
    fn kani_depth0() -> Self {
        GVecStr::A(Vec::new())
    }
    fn kani_depth1() -> Self {
        GVecStr::B(vec![String::new()])
    }
    fn kani_depth2() -> Self {
        GVecStr::A(vec![String::new(), String::new()])
    }
    /// Symbolic variant selector over depth-2 field constructions.
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            GVecStr::A(vec![String::new(), String::new()])
        } else {
            GVecStr::B(vec![String::new(), String::new()])
        }
    }
}

#[cfg(kani)]
fn g_vec_str_consistent(s: &GVecStr) -> bool {
    match s {
        GVecStr::A(v) | GVecStr::B(v) => v.len() <= 2,
    }
}

#[cfg(kani)]
fn g_vec_str_identity(s: GVecStr) -> GVecStr {
    s
}

#[cfg(kani)]
#[kani::proof]
fn gallery3b_vec_str_d0() {
    let state = <GVecStr as elicitation::KaniCompose>::kani_depth0();
    kani::assume(g_vec_str_consistent(&state));
    let result = g_vec_str_identity(state);
    kani::assert(g_vec_str_consistent(&result), "L3b d0: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery3b_vec_str_d1() {
    let state = <GVecStr as elicitation::KaniCompose>::kani_depth1();
    kani::assume(g_vec_str_consistent(&state));
    let result = g_vec_str_identity(state);
    kani::assert(g_vec_str_consistent(&result), "L3b d1: invariant preserved");
    forget(result);
}

#[cfg(kani)]
#[kani::proof]
fn gallery3b_vec_str_closure() {
    let state = <GVecStr as elicitation::KaniCompose>::kani_any();
    kani::assume(g_vec_str_consistent(&state));
    let result = g_vec_str_identity(state);
    kani::assert(
        g_vec_str_consistent(&result),
        "L3b closure: invariant preserved",
    );
    forget(result);
}
