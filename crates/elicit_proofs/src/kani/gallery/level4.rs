//! Gallery level 4: N-variant enum scaling and per-variant harness strategy.
//!
//! **Hypothesis**: The 18-variant `ArchivePanelState` closure times out not
//! because of field complexity (L2/L3 passed with depth-2 fields) but because
//! 18 symbolic primitive fields (u32, usize, bool) across 18 variant arms
//! create an intractable SAT formula when combined as one symbolic selector.
//!
//! ## Experiments
//!
//! ### 4a — Pure variant-count scaling (unit enum, no fields)
//!
//! - `G4Unit` (4 variants), `G8Unit` (8 variants), `G16Unit` (16 variants)
//! - Hypothesis: all closures fast — variant count alone is not the bottleneck.
//!
//! ### 4b — Variant-count × symbolic primitives
//!
//! - `G4Prim` (4 variants, each with a `u32` symbolic field)
//! - `G8Prim` (8 variants, each with a `u32` symbolic field)
//! - `G16Prim` (16 variants, each with a `u32` symbolic field)
//! - Hypothesis: starts timing out around 8–16 variants, since CBMC must
//!   solve `N × 32` symbolic bits at once.
//!
//! ### 4c — Per-variant harnesses (the proposed fix)
//!
//! - Same `G4Prim` but with one harness per variant instead of one closure.
//! - Hypothesis: each per-variant harness is fast (only ~32 symbolic bits).
//! - If confirmed, we adopt per-variant closures for ArchivePanelState.
//!
//! Run a single harness:
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery4a_unit_4_closure
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery4b_prim_4_closure
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery4c_prim_v0
//! ```

use std::mem::forget;

// ── 4a: N-variant unit enum (no fields) ──────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G4Unit {
    A,
    B,
    C,
    D,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4Unit {
    fn kani_depth0() -> Self {
        G4Unit::A
    }
    fn kani_depth1() -> Self {
        G4Unit::B
    }
    fn kani_depth2() -> Self {
        G4Unit::C
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4Unit::A,
            1 => G4Unit::B,
            2 => G4Unit::C,
            _ => G4Unit::D,
        }
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G8Unit {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G8Unit {
    fn kani_depth0() -> Self {
        G8Unit::A
    }
    fn kani_depth1() -> Self {
        G8Unit::B
    }
    fn kani_depth2() -> Self {
        G8Unit::C
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 8);
        match v {
            0 => G8Unit::A,
            1 => G8Unit::B,
            2 => G8Unit::C,
            3 => G8Unit::D,
            4 => G8Unit::E,
            5 => G8Unit::F,
            6 => G8Unit::G,
            _ => G8Unit::H,
        }
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G16Unit {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G16Unit {
    fn kani_depth0() -> Self {
        G16Unit::V0
    }
    fn kani_depth1() -> Self {
        G16Unit::V1
    }
    fn kani_depth2() -> Self {
        G16Unit::V2
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 16);
        match v {
            0 => G16Unit::V0,
            1 => G16Unit::V1,
            2 => G16Unit::V2,
            3 => G16Unit::V3,
            4 => G16Unit::V4,
            5 => G16Unit::V5,
            6 => G16Unit::V6,
            7 => G16Unit::V7,
            8 => G16Unit::V8,
            9 => G16Unit::V9,
            10 => G16Unit::V10,
            11 => G16Unit::V11,
            12 => G16Unit::V12,
            13 => G16Unit::V13,
            14 => G16Unit::V14,
            _ => G16Unit::V15,
        }
    }
}

#[cfg(kani)]
fn unit_consistent(_: &G4Unit) -> bool {
    true
}
#[cfg(kani)]
fn unit_consistent8(_: &G8Unit) -> bool {
    true
}
#[cfg(kani)]
fn unit_consistent16(_: &G16Unit) -> bool {
    true
}
#[cfg(kani)]
fn unit_id4(s: G4Unit) -> G4Unit {
    s
}
#[cfg(kani)]
fn unit_id8(s: G8Unit) -> G8Unit {
    s
}
#[cfg(kani)]
fn unit_id16(s: G16Unit) -> G16Unit {
    s
}

#[cfg(kani)]
#[kani::proof]
fn gallery4a_unit_4_closure() {
    let s = <G4Unit as elicitation::KaniCompose>::kani_any();
    kani::assume(unit_consistent(&s));
    let r = unit_id4(s);
    kani::assert(unit_consistent(&r), "4a-4: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4a_unit_8_closure() {
    let s = <G8Unit as elicitation::KaniCompose>::kani_any();
    kani::assume(unit_consistent8(&s));
    let r = unit_id8(s);
    kani::assert(unit_consistent8(&r), "4a-8: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4a_unit_16_closure() {
    let s = <G16Unit as elicitation::KaniCompose>::kani_any();
    kani::assume(unit_consistent16(&s));
    let r = unit_id16(s);
    kani::assert(unit_consistent16(&r), "4a-16: invariant preserved");
    forget(r);
}

// ── 4b: N-variant enum with one symbolic u32 per variant ─────────────────────
//
// Each variant carries exactly one `val: u32` so CBMC sees N*32 symbolic bits.

#[cfg(kani)]
#[derive(Clone)]
enum G4Prim {
    A { val: u32 },
    B { val: u32 },
    C { val: u32 },
    D { val: u32 },
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4Prim {
    fn kani_depth0() -> Self {
        G4Prim::A { val: 0 }
    }
    fn kani_depth1() -> Self {
        G4Prim::B { val: 1 }
    }
    fn kani_depth2() -> Self {
        G4Prim::C { val: 2 }
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4Prim::A { val: kani::any() },
            1 => G4Prim::B { val: kani::any() },
            2 => G4Prim::C { val: kani::any() },
            _ => G4Prim::D { val: kani::any() },
        }
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G8Prim {
    V0 { val: u32 },
    V1 { val: u32 },
    V2 { val: u32 },
    V3 { val: u32 },
    V4 { val: u32 },
    V5 { val: u32 },
    V6 { val: u32 },
    V7 { val: u32 },
}

#[cfg(kani)]
impl elicitation::KaniCompose for G8Prim {
    fn kani_depth0() -> Self {
        G8Prim::V0 { val: 0 }
    }
    fn kani_depth1() -> Self {
        G8Prim::V1 { val: 1 }
    }
    fn kani_depth2() -> Self {
        G8Prim::V2 { val: 2 }
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 8);
        match v {
            0 => G8Prim::V0 { val: kani::any() },
            1 => G8Prim::V1 { val: kani::any() },
            2 => G8Prim::V2 { val: kani::any() },
            3 => G8Prim::V3 { val: kani::any() },
            4 => G8Prim::V4 { val: kani::any() },
            5 => G8Prim::V5 { val: kani::any() },
            6 => G8Prim::V6 { val: kani::any() },
            _ => G8Prim::V7 { val: kani::any() },
        }
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G16Prim {
    V0 { val: u32 },
    V1 { val: u32 },
    V2 { val: u32 },
    V3 { val: u32 },
    V4 { val: u32 },
    V5 { val: u32 },
    V6 { val: u32 },
    V7 { val: u32 },
    V8 { val: u32 },
    V9 { val: u32 },
    V10 { val: u32 },
    V11 { val: u32 },
    V12 { val: u32 },
    V13 { val: u32 },
    V14 { val: u32 },
    V15 { val: u32 },
}

#[cfg(kani)]
impl elicitation::KaniCompose for G16Prim {
    fn kani_depth0() -> Self {
        G16Prim::V0 { val: 0 }
    }
    fn kani_depth1() -> Self {
        G16Prim::V1 { val: 1 }
    }
    fn kani_depth2() -> Self {
        G16Prim::V2 { val: 2 }
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 16);
        match v {
            0 => G16Prim::V0 { val: kani::any() },
            1 => G16Prim::V1 { val: kani::any() },
            2 => G16Prim::V2 { val: kani::any() },
            3 => G16Prim::V3 { val: kani::any() },
            4 => G16Prim::V4 { val: kani::any() },
            5 => G16Prim::V5 { val: kani::any() },
            6 => G16Prim::V6 { val: kani::any() },
            7 => G16Prim::V7 { val: kani::any() },
            8 => G16Prim::V8 { val: kani::any() },
            9 => G16Prim::V9 { val: kani::any() },
            10 => G16Prim::V10 { val: kani::any() },
            11 => G16Prim::V11 { val: kani::any() },
            12 => G16Prim::V12 { val: kani::any() },
            13 => G16Prim::V13 { val: kani::any() },
            14 => G16Prim::V14 { val: kani::any() },
            _ => G16Prim::V15 { val: kani::any() },
        }
    }
}

#[cfg(kani)]
fn prim_consistent4(s: &G4Prim) -> bool {
    match s {
        G4Prim::A { val } | G4Prim::B { val } | G4Prim::C { val } | G4Prim::D { val } => {
            *val < 1000
        }
    }
}
#[cfg(kani)]
fn prim_consistent8(s: &G8Prim) -> bool {
    match s {
        G8Prim::V0 { val }
        | G8Prim::V1 { val }
        | G8Prim::V2 { val }
        | G8Prim::V3 { val }
        | G8Prim::V4 { val }
        | G8Prim::V5 { val }
        | G8Prim::V6 { val }
        | G8Prim::V7 { val } => *val < 1000,
    }
}
#[cfg(kani)]
fn prim_consistent16(s: &G16Prim) -> bool {
    match s {
        G16Prim::V0 { val }
        | G16Prim::V1 { val }
        | G16Prim::V2 { val }
        | G16Prim::V3 { val }
        | G16Prim::V4 { val }
        | G16Prim::V5 { val }
        | G16Prim::V6 { val }
        | G16Prim::V7 { val }
        | G16Prim::V8 { val }
        | G16Prim::V9 { val }
        | G16Prim::V10 { val }
        | G16Prim::V11 { val }
        | G16Prim::V12 { val }
        | G16Prim::V13 { val }
        | G16Prim::V14 { val }
        | G16Prim::V15 { val } => *val < 1000,
    }
}

#[cfg(kani)]
fn prim_id4(s: G4Prim) -> G4Prim {
    s
}
#[cfg(kani)]
fn prim_id8(s: G8Prim) -> G8Prim {
    s
}
#[cfg(kani)]
fn prim_id16(s: G16Prim) -> G16Prim {
    s
}

#[cfg(kani)]
#[kani::proof]
fn gallery4b_prim_4_closure() {
    let s = <G4Prim as elicitation::KaniCompose>::kani_any();
    kani::assume(prim_consistent4(&s));
    let r = prim_id4(s);
    kani::assert(prim_consistent4(&r), "4b-4: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4b_prim_8_closure() {
    let s = <G8Prim as elicitation::KaniCompose>::kani_any();
    kani::assume(prim_consistent8(&s));
    let r = prim_id8(s);
    kani::assert(prim_consistent8(&r), "4b-8: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4b_prim_16_closure() {
    let s = <G16Prim as elicitation::KaniCompose>::kani_any();
    kani::assume(prim_consistent16(&s));
    let r = prim_id16(s);
    kani::assert(prim_consistent16(&r), "4b-16: invariant preserved");
}

// ── 4c: Per-variant harnesses for G4Prim ─────────────────────────────────────
//
// Instead of one closure over all variants, each harness pins a single variant.
// This is the proposed decomposition strategy for ArchivePanelState:
//   `column_detail__kani_closure` becomes 18 harnesses, one per input variant.
//
// Inductive argument: "for every possible input variant (verified by exhaustive
// enumeration over 18 fixed-variant harnesses), the transition preserves the
// invariant" — equivalent to the one-shot closure but tractable per-harness.

#[cfg(kani)]
#[kani::proof]
fn gallery4c_prim_v0() {
    // Pin to variant A: only ~32 symbolic bits.
    let s = G4Prim::A { val: kani::any() };
    kani::assume(prim_consistent4(&s));
    let r = prim_id4(s);
    kani::assert(prim_consistent4(&r), "4c V0: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4c_prim_v1() {
    let s = G4Prim::B { val: kani::any() };
    kani::assume(prim_consistent4(&s));
    let r = prim_id4(s);
    kani::assert(prim_consistent4(&r), "4c V1: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4c_prim_v2() {
    let s = G4Prim::C { val: kani::any() };
    kani::assume(prim_consistent4(&s));
    let r = prim_id4(s);
    kani::assert(prim_consistent4(&r), "4c V2: invariant preserved");
}

#[cfg(kani)]
#[kani::proof]
fn gallery4c_prim_v3() {
    let s = G4Prim::D { val: kani::any() };
    kani::assume(prim_consistent4(&s));
    let r = prim_id4(s);
    kani::assert(prim_consistent4(&r), "4c V3: invariant preserved");
}
