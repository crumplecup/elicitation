//! Gallery level 5: drop cost, f64 symbolic cost, and nested heap.
//!
//! Prior findings (L4): variant count and symbolic u32 fields are NOT the
//! bottleneck (16 variants × u32 ≈ 7.5 s).
//!
//! This level investigates:
//!
//! ### 5a — f64 symbolic cost (baseline)
//! 2-variant f64 enum closure.  Result: ~10.5 s — f64 adds overhead over u32.
//!
//! ### 5b — N-variant f64 enum
//! 8-variant f64 enum closure.  Result: ~13.7 s — still tractable.
//!
//! ### 5c — Vec<Node{4 sym fields}> inside a variant (no drop)
//! Transition is identity (move in → move out → forget).  Result: ~8.3 s.
//! **Key finding**: symbolic Vec contents do NOT explode when we `forget` the output.
//!
//! ### 5d — DROP hypothesis
//! Same structure as 5c but the transition DROPS the input (returns a unit
//! state instead of the moved value).  If this is slow, heap-free modeling
//! of symbolic Vecs is the true bottleneck.
//!
//! Run:
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery5a_f64_closure
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery5b_f64_8_closure
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery5c_node_closure
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery5d_drop_closure
//! ```

use std::mem::forget;

// ── 5a: direct f64 vs u32 symbolic cost ──────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum GF64Two {
    A(f64),
    B(f64),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GF64Two {
    fn kani_depth0() -> Self { GF64Two::A(0.0) }
    fn kani_depth1() -> Self { GF64Two::B(1.0) }
    fn kani_depth2() -> Self { GF64Two::A(2.0) }
    fn kani_any() -> Self {
        if kani::any::<bool>() {
            GF64Two::A(kani::any::<f64>())
        } else {
            GF64Two::B(kani::any::<f64>())
        }
    }
}

#[cfg(kani)]
fn f64_two_consistent(_: &GF64Two) -> bool { true }

#[cfg(kani)]
fn f64_two_id(s: GF64Two) -> GF64Two { s }

/// Baseline: 2-variant enum, one f64 per variant, trivial invariant.
#[cfg(kani)]
#[kani::proof]
fn gallery5a_f64_closure() {
    let s = <GF64Two as elicitation::KaniCompose>::kani_any();
    kani::assume(f64_two_consistent(&s));
    let r = f64_two_id(s);
    kani::assert(f64_two_consistent(&r), "5a: f64 invariant preserved");
    forget(r);
}

// ── 5b: N-variant enum with one f64 per variant ───────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G4F64 {
    V0(f64), V1(f64), V2(f64), V3(f64),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4F64 {
    fn kani_depth0() -> Self { G4F64::V0(0.0) }
    fn kani_depth1() -> Self { G4F64::V1(1.0) }
    fn kani_depth2() -> Self { G4F64::V2(2.0) }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4F64::V0(kani::any::<f64>()),
            1 => G4F64::V1(kani::any::<f64>()),
            2 => G4F64::V2(kani::any::<f64>()),
            _ => G4F64::V3(kani::any::<f64>()),
        }
    }
}

#[cfg(kani)]
fn f64_4_consistent(_: &G4F64) -> bool { true }

#[cfg(kani)]
fn f64_4_id(s: G4F64) -> G4F64 { s }

/// 4-variant f64 enum.
#[cfg(kani)]
#[kani::proof]
fn gallery5b_f64_4_closure() {
    let s = <G4F64 as elicitation::KaniCompose>::kani_any();
    kani::assume(f64_4_consistent(&s));
    let r = f64_4_id(s);
    kani::assert(f64_4_consistent(&r), "5b-4: f64 invariant preserved");
    forget(r);
}

#[cfg(kani)]
#[derive(Clone)]
enum G8F64 {
    V0(f64), V1(f64), V2(f64), V3(f64),
    V4(f64), V5(f64), V6(f64), V7(f64),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G8F64 {
    fn kani_depth0() -> Self { G8F64::V0(0.0) }
    fn kani_depth1() -> Self { G8F64::V1(1.0) }
    fn kani_depth2() -> Self { G8F64::V2(2.0) }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 8);
        match v {
            0 => G8F64::V0(kani::any::<f64>()),
            1 => G8F64::V1(kani::any::<f64>()),
            2 => G8F64::V2(kani::any::<f64>()),
            3 => G8F64::V3(kani::any::<f64>()),
            4 => G8F64::V4(kani::any::<f64>()),
            5 => G8F64::V5(kani::any::<f64>()),
            6 => G8F64::V6(kani::any::<f64>()),
            _ => G8F64::V7(kani::any::<f64>()),
        }
    }
}

#[cfg(kani)]
fn f64_8_consistent(_: &G8F64) -> bool { true }

#[cfg(kani)]
fn f64_8_id(s: G8F64) -> G8F64 { s }

/// 8-variant f64 enum.
#[cfg(kani)]
#[kani::proof]
fn gallery5b_f64_8_closure() {
    let s = <G8F64 as elicitation::KaniCompose>::kani_any();
    kani::assume(f64_8_consistent(&s));
    let r = f64_8_id(s);
    kani::assert(f64_8_consistent(&r), "5b-8: f64 invariant preserved");
    forget(r);
}

// ── 5c: ExplainNode-like struct inside a Vec inside an enum variant ───────────

/// Minimal replica of ExplainNode's symbolic primitive cost.
#[cfg(kani)]
#[derive(Clone)]
struct GNode {
    startup_cost: f64,
    total_cost: f64,
    plan_rows: i64,
    plan_width: i32,
}

#[cfg(kani)]
impl elicitation::KaniCompose for GNode {
    fn kani_depth0() -> Self {
        GNode {
            startup_cost: kani::any::<f64>(),
            total_cost: kani::any::<f64>(),
            plan_rows: kani::any::<i64>(),
            plan_width: kani::any::<i32>(),
        }
    }
    fn kani_depth1() -> Self { Self::kani_depth0() }
    fn kani_depth2() -> Self { Self::kani_depth0() }
    fn kani_any() -> Self { Self::kani_depth0() }
}

/// 4-variant enum where one arm carries Vec<GNode> (mimics ExplainView/ExplainCompare).
#[cfg(kani)]
#[derive(Clone)]
enum GPlan {
    /// Unit variants matching simple ArchivePanelState arms (ColumnDetail, HelpView, etc.)
    Unit0,
    Unit1,
    Unit2,
    /// One arm with Vec<GNode> — mimics ExplainView carrying ExplainPlan.
    WithNodes(Vec<GNode>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for GPlan {
    fn kani_depth0() -> Self { GPlan::Unit0 }
    fn kani_depth1() -> Self { GPlan::Unit1 }
    fn kani_depth2() -> Self { GPlan::Unit2 }
    /// Symbolic variant selector; WithNodes uses depth-2 (two concrete GNodes).
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => GPlan::Unit0,
            1 => GPlan::Unit1,
            2 => GPlan::Unit2,
            _ => GPlan::WithNodes(vec![GNode::kani_depth0(), GNode::kani_depth0()]),
        }
    }
}

#[cfg(kani)]
fn plan_consistent(_: &GPlan) -> bool { true }

#[cfg(kani)]
fn plan_id(s: GPlan) -> GPlan { s }

/// 4-variant enum where one arm has Vec<ExplainNode-like struct> with symbolic f64 fields.
///
/// Transition is identity: moves the state out, never drops Vec, forgets output.
/// Result: ~8.3 s ✅  — symbolic Vec is cheap when there is no DROP.
#[cfg(kani)]
#[kani::proof]
fn gallery5c_node_closure() {
    let s = <GPlan as elicitation::KaniCompose>::kani_any();
    kani::assume(plan_consistent(&s));
    let r = plan_id(s);
    kani::assert(plan_consistent(&r), "5c: node invariant preserved");
    forget(r);
}

// ── 5d: DROP hypothesis ───────────────────────────────────────────────────────

/// Same as `gallery5c` but the transition RETURNS a unit state, so the
/// input GPlan (which may contain Vec<GNode>) is DROPPED inside the fn.
///
/// If CBMC has to model `free()` for a symbolically-constructed Vec, this
/// harness will be significantly slower than 5c even though 5c had the same
/// input construction cost.
#[cfg(kani)]
fn plan_to_unit(_s: GPlan) -> GPlan {
    // _s is dropped here — CBMC must model heap deallocation for Vec<GNode>
    GPlan::Unit0
}

#[cfg(kani)]
#[kani::proof]
fn gallery5d_drop_closure() {
    let s = <GPlan as elicitation::KaniCompose>::kani_any();
    kani::assume(plan_consistent(&s));
    let r = plan_to_unit(s);   // s is dropped inside; WithNodes arm frees Vec
    kani::assert(plan_consistent(&r), "5d: drop closure invariant preserved");
    forget(r);
}
