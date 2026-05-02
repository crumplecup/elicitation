//! Gallery level 7: Incrementally add each element that differs between our
//! proven-fast GNode and the actual ExplainNode / ExplainPlan types.
//!
//! GNode baseline (Level 5/6): 4 numeric fields, 16-arm enum → 2.67s ✅
//!
//! ExplainNode actual shape:
//!   node_type: String
//!   relation_name: Option<String>
//!   alias: Option<String>
//!   startup_cost: f64
//!   total_cost: f64
//!   plan_rows: i64
//!   plan_width: i32
//!   actual_startup_time: Option<f64>
//!   actual_total_time: Option<f64>
//!   actual_rows: Option<i64>
//!   actual_loops: Option<i64>
//!   children: Vec<usize>
//!
//! We add fields one category at a time to find which one causes CBMC blowup.
//!
//! ### 7a — GNode + String field (does String change cost?)
//! ### 7b — GNode + Option<f64> fields
//! ### 7c — GNode + Vec<usize> field (like ExplainNode.children)
//! ### 7d — Full ExplainNode replica (all fields), 1 arm in a 4-variant enum
//! ### 7e — Full ExplainPlan replica (Vec<ExplainNodeR>), 1 arm in 4-variant
//! ### 7f — Full ExplainPlan replica, 4 arms, drop transition

use std::mem::forget;

// ── 7a: GNode + one String field ─────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
struct GNodeStr {
    label: String, // ← new: String field
    x: f64,
    y: f64,
    row: i64,
    width: i32,
}

#[cfg(kani)]
impl elicitation::KaniCompose for GNodeStr {
    fn kani_depth0() -> Self {
        GNodeStr {
            label: String::new(),
            x: kani::any(),
            y: kani::any(),
            row: kani::any(),
            width: kani::any(),
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

#[cfg(kani)]
#[derive(Clone)]
enum G4Str {
    A(Vec<GNodeStr>),
    B(Vec<GNodeStr>),
    C(Vec<GNodeStr>),
    D(Vec<GNodeStr>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4Str {
    fn kani_depth0() -> Self {
        G4Str::A(vec![])
    }
    fn kani_depth1() -> Self {
        G4Str::B(vec![])
    }
    fn kani_depth2() -> Self {
        G4Str::C(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4Str::A(vec![GNodeStr::kani_depth0(), GNodeStr::kani_depth0()]),
            1 => G4Str::B(vec![GNodeStr::kani_depth0(), GNodeStr::kani_depth0()]),
            2 => G4Str::C(vec![GNodeStr::kani_depth0(), GNodeStr::kani_depth0()]),
            _ => G4Str::D(vec![GNodeStr::kani_depth0(), GNodeStr::kani_depth0()]),
        }
    }
}

/// 4 arms, each Vec<GNodeStr> (4 prim + 1 String). Identity. Drop output.
#[cfg(kani)]
#[kani::proof]
fn gallery7a_string_field() {
    let s = <G4Str as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "7a");
    forget(r);
}

// ── 7b: GNode + Option<f64> fields ───────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
struct GNodeOpt {
    x: f64,
    y: f64,
    row: i64,
    width: i32,
    opt_a: Option<f64>, // ← new
    opt_b: Option<f64>, // ← new
    opt_c: Option<i64>, // ← new
    opt_d: Option<i64>, // ← new
}

#[cfg(kani)]
impl elicitation::KaniCompose for GNodeOpt {
    fn kani_depth0() -> Self {
        GNodeOpt {
            x: kani::any(),
            y: kani::any(),
            row: kani::any(),
            width: kani::any(),
            opt_a: Some(kani::any()),
            opt_b: Some(kani::any()),
            opt_c: Some(kani::any()),
            opt_d: Some(kani::any()),
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

#[cfg(kani)]
#[derive(Clone)]
enum G4Opt {
    A(Vec<GNodeOpt>),
    B(Vec<GNodeOpt>),
    C(Vec<GNodeOpt>),
    D(Vec<GNodeOpt>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4Opt {
    fn kani_depth0() -> Self {
        G4Opt::A(vec![])
    }
    fn kani_depth1() -> Self {
        G4Opt::B(vec![])
    }
    fn kani_depth2() -> Self {
        G4Opt::C(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4Opt::A(vec![GNodeOpt::kani_depth0(), GNodeOpt::kani_depth0()]),
            1 => G4Opt::B(vec![GNodeOpt::kani_depth0(), GNodeOpt::kani_depth0()]),
            2 => G4Opt::C(vec![GNodeOpt::kani_depth0(), GNodeOpt::kani_depth0()]),
            _ => G4Opt::D(vec![GNodeOpt::kani_depth0(), GNodeOpt::kani_depth0()]),
        }
    }
}

/// 4 arms, each Vec<GNodeOpt> (4 prim + 4 Option<prim>). 16 sym per node.
#[cfg(kani)]
#[kani::proof]
fn gallery7b_option_fields() {
    let s = <G4Opt as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "7b");
    forget(r);
}

// ── 7c: GNode + Vec<usize> field (like ExplainNode.children) ─────────────────

#[cfg(kani)]
#[derive(Clone)]
struct GNodeVecIdx {
    x: f64,
    y: f64,
    row: i64,
    width: i32,
    children: Vec<usize>, // ← new: index vec
}

#[cfg(kani)]
impl elicitation::KaniCompose for GNodeVecIdx {
    fn kani_depth0() -> Self {
        GNodeVecIdx {
            x: kani::any(),
            y: kani::any(),
            row: kani::any(),
            width: kani::any(),
            children: vec![],
        }
    }
    fn kani_depth1() -> Self {
        GNodeVecIdx {
            x: kani::any(),
            y: kani::any(),
            row: kani::any(),
            width: kani::any(),
            children: vec![kani::any()],
        }
    }
    fn kani_depth2() -> Self {
        GNodeVecIdx {
            x: kani::any(),
            y: kani::any(),
            row: kani::any(),
            width: kani::any(),
            children: vec![kani::any(), kani::any()],
        }
    }
    fn kani_any() -> Self {
        Self::kani_depth2()
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G4VecIdx {
    A(Vec<GNodeVecIdx>),
    B(Vec<GNodeVecIdx>),
    C(Vec<GNodeVecIdx>),
    D(Vec<GNodeVecIdx>),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4VecIdx {
    fn kani_depth0() -> Self {
        G4VecIdx::A(vec![])
    }
    fn kani_depth1() -> Self {
        G4VecIdx::B(vec![])
    }
    fn kani_depth2() -> Self {
        G4VecIdx::C(vec![])
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4VecIdx::A(vec![GNodeVecIdx::kani_depth0(), GNodeVecIdx::kani_depth0()]),
            1 => G4VecIdx::B(vec![GNodeVecIdx::kani_depth0(), GNodeVecIdx::kani_depth0()]),
            2 => G4VecIdx::C(vec![GNodeVecIdx::kani_depth0(), GNodeVecIdx::kani_depth0()]),
            _ => G4VecIdx::D(vec![GNodeVecIdx::kani_depth0(), GNodeVecIdx::kani_depth0()]),
        }
    }
}

/// 4 arms, each Vec<GNodeVecIdx> — node has nested Vec<usize> children.
/// At depth-0 the inner children vec is empty; no unbounded symbolic Vec.
#[cfg(kani)]
#[kani::proof]
fn gallery7c_vec_usize_field() {
    let s = <G4VecIdx as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "7c");
    forget(r);
}

// ── 7d: Full ExplainNode replica, 1 arm in 4-variant enum ────────────────────

#[cfg(kani)]
#[derive(Clone)]
struct ExplainNodeR {
    node_type: String,
    relation_name: Option<String>,
    alias: Option<String>,
    startup_cost: f64,
    total_cost: f64,
    plan_rows: i64,
    plan_width: i32,
    actual_startup_time: Option<f64>,
    actual_total_time: Option<f64>,
    actual_rows: Option<i64>,
    actual_loops: Option<i64>,
    children: Vec<usize>,
}

#[cfg(kani)]
impl elicitation::KaniCompose for ExplainNodeR {
    fn kani_depth0() -> Self {
        ExplainNodeR {
            node_type: String::new(),
            relation_name: None,
            alias: None,
            startup_cost: kani::any(),
            total_cost: kani::any(),
            plan_rows: kani::any(),
            plan_width: kani::any(),
            actual_startup_time: None,
            actual_total_time: None,
            actual_rows: None,
            actual_loops: None,
            children: vec![],
        }
    }
    fn kani_depth1() -> Self {
        ExplainNodeR {
            node_type: String::new(),
            relation_name: None,
            alias: None,
            startup_cost: kani::any(),
            total_cost: kani::any(),
            plan_rows: kani::any(),
            plan_width: kani::any(),
            actual_startup_time: Some(kani::any()),
            actual_total_time: Some(kani::any()),
            actual_rows: Some(kani::any()),
            actual_loops: Some(kani::any()),
            children: vec![kani::any()],
        }
    }
    fn kani_depth2() -> Self {
        ExplainNodeR {
            node_type: String::new(),
            relation_name: None,
            alias: None,
            startup_cost: kani::any(),
            total_cost: kani::any(),
            plan_rows: kani::any(),
            plan_width: kani::any(),
            actual_startup_time: Some(kani::any()),
            actual_total_time: Some(kani::any()),
            actual_rows: Some(kani::any()),
            actual_loops: Some(kani::any()),
            children: vec![kani::any(), kani::any()],
        }
    }
    fn kani_any() -> Self {
        Self::kani_depth2()
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G4ExplainNode {
    Heavy(Vec<ExplainNodeR>),
    Unit0,
    Unit1,
    Unit2,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4ExplainNode {
    fn kani_depth0() -> Self {
        G4ExplainNode::Unit0
    }
    fn kani_depth1() -> Self {
        G4ExplainNode::Unit1
    }
    fn kani_depth2() -> Self {
        G4ExplainNode::Unit2
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4ExplainNode::Heavy(vec![
                ExplainNodeR::kani_depth0(),
                ExplainNodeR::kani_depth0(),
            ]),
            1 => G4ExplainNode::Unit0,
            2 => G4ExplainNode::Unit1,
            _ => G4ExplainNode::Unit2,
        }
    }
}

/// 1 of 4 arms has Vec<ExplainNodeR> — full field replica of ExplainNode.
#[cfg(kani)]
#[kani::proof]
fn gallery7d_explain_node_replica() {
    let s = <G4ExplainNode as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "7d");
    forget(r);
}

// ── 7e: Full ExplainPlan replica (Vec<ExplainNodeR>), 1 arm in 4-variant ─────

#[cfg(kani)]
#[derive(Clone)]
struct ExplainPlanR {
    nodes: Vec<ExplainNodeR>,
    root: usize,
}

#[cfg(kani)]
impl elicitation::KaniCompose for ExplainPlanR {
    fn kani_depth0() -> Self {
        ExplainPlanR {
            nodes: vec![],
            root: kani::any(),
        }
    }
    fn kani_depth1() -> Self {
        ExplainPlanR {
            nodes: vec![ExplainNodeR::kani_depth0()],
            root: kani::any(),
        }
    }
    fn kani_depth2() -> Self {
        ExplainPlanR {
            nodes: vec![ExplainNodeR::kani_depth0(), ExplainNodeR::kani_depth0()],
            root: kani::any(),
        }
    }
    fn kani_any() -> Self {
        Self::kani_depth2()
    }
}

#[cfg(kani)]
#[derive(Clone)]
enum G4ExplainPlan {
    Heavy(ExplainPlanR),
    Unit0,
    Unit1,
    Unit2,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4ExplainPlan {
    fn kani_depth0() -> Self {
        G4ExplainPlan::Unit0
    }
    fn kani_depth1() -> Self {
        G4ExplainPlan::Unit1
    }
    fn kani_depth2() -> Self {
        G4ExplainPlan::Unit2
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4ExplainPlan::Heavy(ExplainPlanR::kani_depth2()),
            1 => G4ExplainPlan::Unit0,
            2 => G4ExplainPlan::Unit1,
            _ => G4ExplainPlan::Unit2,
        }
    }
}

/// 1 of 4 arms has ExplainPlanR — nested Vec<ExplainNodeR> structure.
#[cfg(kani)]
#[kani::proof]
fn gallery7e_explain_plan_replica() {
    let s = <G4ExplainPlan as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = s;
    kani::assert(true, "7e");
    forget(r);
}

// ── 7f: 4 ExplainPlan arms, drop transition ───────────────────────────────────

#[cfg(kani)]
#[derive(Clone)]
enum G4ExplainPlanAll {
    A(ExplainPlanR),
    B(ExplainPlanR),
    C(ExplainPlanR),
    D(ExplainPlanR),
}

#[cfg(kani)]
impl elicitation::KaniCompose for G4ExplainPlanAll {
    fn kani_depth0() -> Self {
        G4ExplainPlanAll::A(ExplainPlanR::kani_depth0())
    }
    fn kani_depth1() -> Self {
        G4ExplainPlanAll::B(ExplainPlanR::kani_depth0())
    }
    fn kani_depth2() -> Self {
        G4ExplainPlanAll::C(ExplainPlanR::kani_depth0())
    }
    fn kani_any() -> Self {
        let v: usize = kani::any();
        kani::assume(v < 4);
        match v {
            0 => G4ExplainPlanAll::A(ExplainPlanR::kani_depth2()),
            1 => G4ExplainPlanAll::B(ExplainPlanR::kani_depth2()),
            2 => G4ExplainPlanAll::C(ExplainPlanR::kani_depth2()),
            _ => G4ExplainPlanAll::D(ExplainPlanR::kani_depth2()),
        }
    }
}

#[cfg(kani)]
fn explain_to_unit(_s: G4ExplainPlanAll) -> G4ExplainPlanAll {
    G4ExplainPlanAll::A(ExplainPlanR {
        nodes: vec![],
        root: 0,
    }) // drops input
}

/// 4 ExplainPlan-bearing arms, drop transition. Mirrors column_detail pattern.
#[cfg(kani)]
#[kani::proof]
fn gallery7f_explain_plan_4arms_drop() {
    let s = <G4ExplainPlanAll as elicitation::KaniCompose>::kani_any();
    kani::assume(true);
    let r = explain_to_unit(s);
    kani::assert(true, "7f");
    forget(r);
}
