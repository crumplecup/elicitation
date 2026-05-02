//! Gallery level 6: cumulative ITE density — does the SAT cost scale with
//! the number of arms that each carry symbolic heap?
//!
//! Prior findings (L5d): one Vec<GNode{4 sym}> arm out of 4 variants = 7.7 s.
//! The question: does CBMC scale linearly, or is there a phase transition when
//! EVERY arm carries symbolic heap?
//!
//! GNode = {f64, f64, i64, i32} = 4 sym primitives per element.
//! Vec at depth-0 = 2 elements, so each arm carrying Vec<GNode> contributes
//! 8 symbolic primitives to the formula.
//!
//! ### 6a — 2 heavy arms (16 sym total ITE)
//! Both variants carry Vec<GNode>.
//!
//! ### 6b — 4 heavy arms (32 sym total ITE)
//! All 4 variants carry Vec<GNode>.
//!
//! ### 6c — 8 heavy arms (64 sym total ITE)
//!
//! ### 6d — 16 heavy arms (128 sym total ITE) — near ArchivePanelState density
//!
//! ### 6e — drop transition on 4-heavy-arm enum
//! Same as 6b but the transition drops the input (like `column_detail` does).
//!
//! Run each:
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery6a_2heavy
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery6b_4heavy
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery6c_8heavy
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery6d_16heavy
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery6e_4heavy_drop
//! ```

use std::mem::forget;

// ── Shared GNode (4 sym primitives per element, same as Level 5) ──────────────

#[cfg(kani)]
#[derive(Clone)]
struct GNode6 {
    startup_cost: f64,
    total_cost: f64,
    plan_rows: i64,
    plan_width: i32,
}

#[cfg(kani)]
impl elicitation::KaniCompose for GNode6 {
    fn kani_depth0() -> Self {
        GNode6 {
            startup_cost: kani::any::<f64>(),
            total_cost: kani::any::<f64>(),
            plan_rows: kani::any::<i64>(),
            plan_width: kani::any::<i32>(),
        }
    }
    fn kani_depth1() -> Self {
        Self::kani_depth0()
    }
    fn kani_depth2() -> Self {
        Self::kani_depth0()
    }
    fn kani_any() -> Self {
        Self::kani_depth0()
    }
}

// ── 6a: 2 heavy arms ─────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G2Heavy {
    A(Vec<GNode6>),
    B(Vec<GNode6>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G2Heavy {
    fn kani_depth0() -> Self {
        G2Heavy::A(vec![])
    }
    fn kani_depth1() -> Self {
        G2Heavy::B(vec![])
    }
    fn kani_depth2() -> Self {
        G2Heavy::A(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 2);
        match v {
            0 => G2Heavy::A(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            _ => G2Heavy::B(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
        }
    }
}

/// 2 variants each with Vec<GNode>, identity transition, forget output.
/// 16 sym primitives total across both arms.
#[cfg(kani)]
#[kani::proof]
fn gallery6a_2heavy() {
    let s = <G2Heavy as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "6a: invariant");
    forget(r);
}

// ── 6b: 4 heavy arms ─────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G4Heavy {
    A(Vec<GNode6>),
    B(Vec<GNode6>),
    C(Vec<GNode6>),
    D(Vec<GNode6>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4Heavy {
    fn kani_depth0() -> Self {
        G4Heavy::A(vec![])
    }
    fn kani_depth1() -> Self {
        G4Heavy::B(vec![])
    }
    fn kani_depth2() -> Self {
        G4Heavy::C(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4Heavy::A(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            1 => G4Heavy::B(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            2 => G4Heavy::C(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            _ => G4Heavy::D(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
        }
    }
}

/// 4 variants each with Vec<GNode>; 32 sym total.
#[cfg(kani)]
#[kani::proof]
fn gallery6b_4heavy() {
    let s = <G4Heavy as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "6b: invariant");
    forget(r);
}

// ── 6c: 8 heavy arms ─────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G8Heavy {
    A(Vec<GNode6>),
    B(Vec<GNode6>),
    C(Vec<GNode6>),
    D(Vec<GNode6>),
    E(Vec<GNode6>),
    F(Vec<GNode6>),
    G(Vec<GNode6>),
    H(Vec<GNode6>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G8Heavy {
    fn kani_depth0() -> Self {
        G8Heavy::A(vec![])
    }
    fn kani_depth1() -> Self {
        G8Heavy::B(vec![])
    }
    fn kani_depth2() -> Self {
        G8Heavy::C(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 8);
        match v {
            0 => G8Heavy::A(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            1 => G8Heavy::B(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            2 => G8Heavy::C(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            3 => G8Heavy::D(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            4 => G8Heavy::E(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            5 => G8Heavy::F(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            6 => G8Heavy::G(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
            _ => G8Heavy::H(vec![GNode6::kani_depth0(), GNode6::kani_depth0()]),
        }
    }
}

/// 8 variants each with Vec<GNode>; 64 sym total.
#[cfg(kani)]
#[kani::proof]
fn gallery6c_8heavy() {
    let s = <G8Heavy as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "6c: invariant");
    forget(r);
}

// ── 6d: 16 heavy arms ────────────────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G16Heavy {
    V0(Vec<GNode6>),
    V1(Vec<GNode6>),
    V2(Vec<GNode6>),
    V3(Vec<GNode6>),
    V4(Vec<GNode6>),
    V5(Vec<GNode6>),
    V6(Vec<GNode6>),
    V7(Vec<GNode6>),
    V8(Vec<GNode6>),
    V9(Vec<GNode6>),
    V10(Vec<GNode6>),
    V11(Vec<GNode6>),
    V12(Vec<GNode6>),
    V13(Vec<GNode6>),
    V14(Vec<GNode6>),
    V15(Vec<GNode6>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G16Heavy {
    fn kani_depth0() -> Self {
        G16Heavy::V0(vec![])
    }
    fn kani_depth1() -> Self {
        G16Heavy::V1(vec![])
    }
    fn kani_depth2() -> Self {
        G16Heavy::V2(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 16);
        let nodes = || vec![GNode6::kani_depth0(), GNode6::kani_depth0()];
        match v {
            0 => G16Heavy::V0(nodes()),
            1 => G16Heavy::V1(nodes()),
            2 => G16Heavy::V2(nodes()),
            3 => G16Heavy::V3(nodes()),
            4 => G16Heavy::V4(nodes()),
            5 => G16Heavy::V5(nodes()),
            6 => G16Heavy::V6(nodes()),
            7 => G16Heavy::V7(nodes()),
            8 => G16Heavy::V8(nodes()),
            9 => G16Heavy::V9(nodes()),
            10 => G16Heavy::V10(nodes()),
            11 => G16Heavy::V11(nodes()),
            12 => G16Heavy::V12(nodes()),
            13 => G16Heavy::V13(nodes()),
            14 => G16Heavy::V14(nodes()),
            _ => G16Heavy::V15(nodes()),
        }
    }
}

/// 16 variants each with Vec<GNode>; 128 sym total — near ArchivePanelState density.
#[cfg(kani)]
#[kani::proof]
fn gallery6d_16heavy() {
    let s = <G16Heavy as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "6d: invariant");
    forget(r);
}

// ── 6e: 4 heavy arms with DROP transition ────────────────────────────────────

/// Like 6b but transition drops input, returns unit — mirrors `column_detail`.
#[cfg(kani)]
fn drop_to_unit(_s: G4Heavy) -> G4Heavy {
    G4Heavy::A(vec![]) // input is dropped here
}

#[cfg(kani)]
#[kani::proof]
fn gallery6e_4heavy_drop() {
    let s = <G4Heavy as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = drop_to_unit(s);
    kani::assert(true, "6e: drop invariant");
    forget(r);
}
