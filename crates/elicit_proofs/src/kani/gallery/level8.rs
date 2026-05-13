//! Gallery level 8: Test the **derived** KaniCompose machinery.
//!
//! Levels 0–7 used hand-written KaniCompose impls. Level 8 switches to
//! `#[derive(KaniCompose)]` so we test what the macro actually generates.
//!
//! Escalation ladder:
//!
//! ### 8a — Derived struct, primitives only
//! Baseline: does the derive produce working d0/d1/d2/any?
//!
//! ### 8b — Derived struct, Vec<prim> field
//! struct kani_any() calls kani_vec_closure(1,3) — how expensive is that alone?
//!
//! ### 8c — Derived struct nested: struct B { inner: Vec<A> }
//! Depth chain A::d0 → B::d2: does chaining stay tractable?
//!
//! ### 8d — Derived enum with struct-valued fields, depth-2 construction
//! Mirrors how ArchivePanelState::kani_any() is built by the derive.
//!
//! ### 8e — Isolate one complex real variant: MonitorView alone vs ColumnDetail

#[allow(unused_imports)]
use elicitation::KaniCompose;

// ── 8a: Derived primitive struct ─────────────────────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
struct D8PrimStruct {
    a: i32,
    b: f64,
    c: bool,
    d: u64,
}

#[cfg(kani)]
#[kani::proof]
fn gallery8a_derived_prim_d0() {
    let s = D8PrimStruct::kani_depth0();
    let _ = s;
}

#[cfg(kani)]
#[kani::proof]
fn gallery8a_derived_prim_d2() {
    let s = D8PrimStruct::kani_depth2();
    let _ = s;
}

#[cfg(kani)]
#[kani::proof]
fn gallery8a_derived_prim_any() {
    let s = D8PrimStruct::kani_any();
    let _ = s;
}

// ── 8b: Derived struct with Vec<prim> field ───────────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
struct D8VecPrim {
    label: String,
    vals: Vec<i32>, // kani_any() → kani_vec_closure(1, 3)
    flag: bool,
}

#[cfg(kani)]
#[kani::proof]
fn gallery8b_vec_prim_d0() {
    // d0: vals = vec![], label = ""
    let s = D8VecPrim::kani_depth0();
    kani::assert(s.vals.is_empty(), "d0 vec empty");
}

#[cfg(kani)]
#[kani::proof]
fn gallery8b_vec_prim_d2() {
    // d2: vals = [i32::d0(), i32::d0()] = [any(), any()]
    let s = D8VecPrim::kani_depth2();
    kani::assert(s.vals.len() == 2, "d2 has 2 elements");
}

#[cfg(kani)]
#[kani::proof]
fn gallery8b_vec_prim_any() {
    // kani_any(): vals = kani_vec_closure(1, 3) — symbolic length 1..3
    let s = D8VecPrim::kani_any();
    kani::assert(!s.vals.is_empty(), "any vec nonempty");
}

// ── 8c: Nested derived structs: B contains Vec<A> ────────────────────────────

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
struct D8Inner {
    x: f64,
    y: f64,
}

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
struct D8Outer {
    name: String,
    items: Vec<D8Inner>, // d2 → [D8Inner::d0(), D8Inner::d0()]
    count: u32,
}

#[cfg(kani)]
#[kani::proof]
fn gallery8c_nested_d2() {
    // items = [D8Inner{any,any}, D8Inner{any,any}] at d2
    let s = D8Outer::kani_depth2();
    kani::assert(s.items.len() == 2, "d2 nested has 2 items");
}

#[cfg(kani)]
#[kani::proof]
fn gallery8c_nested_any() {
    // items = kani_vec_closure(1, 3) with D8Inner::kani_any() elements
    let s = D8Outer::kani_any();
    kani::assert(!s.items.is_empty(), "any nested nonempty");
}

// ── 8d: Derived enum with struct-valued fields (like ArchivePanelState) ───────
//
// Enum kani_any() uses field_exprs.2 (depth-2) for each variant's fields,
// NOT kani_any(). This is what ArchivePanelState::kani_any() does.

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D8Enum {
    Unit,
    WithStr(String),
    WithStruct(D8Outer),
    WithVecInner(Vec<D8Inner>),
}

#[cfg(kani)]
#[kani::proof]
fn gallery8d_enum_any() {
    // All variant arms use depth-2 field construction — no kani_vec_closure here
    let e = D8Enum::kani_any();
    std::mem::forget(e);
}

// ── 8e: Isolate one complex real variant: 2-variant enum with MonitorView ─────
//
// Mini-ArchivePanelState: just ColumnDetail + MonitorView.
// If this is slow, MonitorSnapshot::kani_depth2() is the bottleneck.
// If fast, the cost comes from combining ALL 18 variants.

#[cfg(kani)]
use elicit_server::archive::types::MonitorSnapshot;

#[cfg(kani)]
#[derive(Clone, KaniCompose)]
enum D8TwoVariant {
    Nothing,
    Monitor(MonitorSnapshot),
}

#[cfg(kani)]
#[kani::proof]
fn gallery8e_monitor_depth2() {
    // Test MonitorSnapshot::kani_depth2() in isolation
    let snap = MonitorSnapshot::kani_depth2();
    std::mem::forget(snap);
}

#[cfg(kani)]
#[kani::proof]
fn gallery8e_two_variant_any() {
    // 2-variant enum: Nothing | Monitor(MonitorSnapshot::kani_depth2())
    let e = D8TwoVariant::kani_any();
    std::mem::forget(e);
}
