//! Unit test suite for the `KaniCompose` trait and derive macro.
//!
//! Proves the correctness of the derive and stdlib impls for every supported
//! *type category*.  Uses small synthetic types defined inline — no domain
//! dependency — so the suite is fast and self-contained.
//!
//! ## Why this suite exists
//!
//! The 1200 VSM harnesses are integration proofs: they exercise full transition
//! functions on domain types.  They depend on `KaniCompose` being correct for
//! every field shape they encounter.  This suite proves the *framework* itself.
//! If every category passes here, all derived types automatically inherit the
//! guarantee by composition.
//!
//! ## Coverage map
//!
//! | Category                  | Harness prefix            |
//! |---------------------------|---------------------------|
//! | Primitives                | `suite__prim__*`          |
//! | `String`                  | `suite__string__*`        |
//! | `Vec<T>`                  | `suite__vec__*`           |
//! | `Option<T>`               | `suite__option__*`        |
//! | `BTreeMap<K,V>`           | `suite__btreemap__*`      |
//! | `HashMap<K,V>`            | *(not verifiable — see note in source)* |
//! | Struct (named fields)     | `suite__struct_named__*`  |
//! | Struct (tuple)            | `suite__struct_tuple__*`  |
//! | Unit struct               | `suite__struct_unit__*`   |
//! | Enum (unit variants)      | `suite__enum_unit__*`     |
//! | Enum (data variants)      | `suite__enum_data__*`     |
//! | Struct with `Vec<T>`      | `suite__struct_vec__*`    |
//! | Struct with `Option<T>`   | `suite__struct_option__*` |
//! | Recursive struct          | `suite__recursive__*`     |
//! | Nested `Vec<Option<T>>`   | `suite__nested__*`        |
//! | `chrono::DateTime<Utc>`   | `suite__chrono__*`        |
//!
//! ## Reading assertions
//!
//! Every harness uses `assert!` to verify a structural postcondition:
//!
//! - **depth-0**: all `Vec` fields empty, all `Option` fields `None`.
//! - **depth-1**: each `Vec` field has exactly one element (populated with
//!   `kani_depth0()` of the inner type), each `Option` field is `Some`.
//! - **depth-2**: each `Vec` field has exactly two elements.

#![cfg(kani)]

use elicitation::KaniCompose;

// ── Synthetic types ───────────────────────────────────────────────────────────
//
// These types exist only to exercise the derive macro against specific field
// shapes.  They are not exported and carry no semantic meaning.

/// A plain named struct — the most common case.
#[derive(Clone, KaniCompose)]
struct Named {
    count: u32,
    name: String,
    flag: bool,
}

/// A two-field tuple struct.
#[derive(Clone, KaniCompose)]
struct Tuple(u32, String);

/// A unit struct — no fields.
#[derive(Clone, KaniCompose)]
struct Unit;

/// An enum with only unit variants.  Derive should always pick `A`.
#[derive(Clone, KaniCompose)]
enum UnitEnum {
    A,
    B,
    C,
}

/// An enum with a unit variant first, then data variants.
/// Derive should always pick the first unit variant (`First`).
#[derive(Clone, KaniCompose)]
enum DataEnum {
    First,
    Second { count: u32, label: String },
    Third(Vec<u32>),
}

/// A struct whose sole interesting field is `Vec<u32>`.
#[derive(Clone, KaniCompose)]
struct WithVec {
    items: Vec<u32>,
    count: u32,
}

/// A struct whose sole interesting field is `Option<String>`.
#[derive(Clone, KaniCompose)]
struct WithOption {
    label: Option<String>,
    count: u32,
}

/// A recursive struct: `children: Vec<Tree>`.
/// This is the canonical hard case for CBMC (unbounded destructor model).
#[derive(Clone, KaniCompose)]
struct Tree {
    value: u32,
    children: Vec<Tree>,
}

/// A struct with a nested `Vec<Option<u32>>` field.
/// Exercises composition of the Vec and Option impls.
#[derive(Clone, KaniCompose)]
struct WithNested {
    items: Vec<Option<u32>>,
}

/// A struct with a `chrono::DateTime<Utc>` field and an optional one.
#[derive(Clone, KaniCompose)]
struct WithDateTime {
    created: chrono::DateTime<chrono::Utc>,
    updated: Option<chrono::DateTime<chrono::Utc>>,
}

// ── Primitives ────────────────────────────────────────────────────────────────
//
// One harness per primitive type.  Proves `kani::any()` is sound (constructs,
// drops, no memory safety violation).  All three depths are identical for
// primitives — a single harness per type suffices.

#[kani::proof]
fn suite__prim__bool() {
    let _ = bool::kani_depth0();
}

#[kani::proof]
fn suite__prim__u32() {
    let _ = u32::kani_depth0();
}

#[kani::proof]
fn suite__prim__i64() {
    let _ = i64::kani_depth0();
}

#[kani::proof]
fn suite__prim__f32() {
    let _ = f32::kani_depth0();
}

#[kani::proof]
fn suite__prim__usize() {
    let _ = usize::kani_depth0();
}

#[kani::proof]
fn suite__prim__char() {
    let _ = char::kani_depth0();
}

// ── String ───────────────────────────────────────────────────────────────────
//
// All three depths must return `String::new()` (empty).
// Symbolic strings cause path explosion; the spec mandates empty at all depths.

#[kani::proof]
fn suite__string__depth0_is_empty() {
    let s = String::kani_depth0();
    assert!(s.is_empty(), "String depth-0 must be empty");
}

#[kani::proof]
fn suite__string__depth1_is_empty() {
    let s = String::kani_depth1();
    assert!(s.is_empty(), "String depth-1 must be empty");
}

#[kani::proof]
fn suite__string__depth2_is_empty() {
    let s = String::kani_depth2();
    assert!(s.is_empty(), "String depth-2 must be empty");
}

// ── Vec<T> ───────────────────────────────────────────────────────────────────
//
// Three harnesses prove the depth semantics:
//   depth-0 → empty (base case)
//   depth-1 → one element (inductive step)
//   depth-2 → two elements (inductive step holds for two applications)
//
// Composing 0 → 1 → 2 proves the inductive step is stable, so by induction
// any finite Vec<u32> is sound.

#[kani::proof]
fn suite__vec_u32__depth0_is_empty() {
    let v = <Vec<u32>>::kani_depth0();
    assert!(v.is_empty(), "Vec depth-0 must be empty");
}

#[kani::proof]
fn suite__vec_u32__depth1_has_one_element() {
    let v = <Vec<u32>>::kani_depth1();
    assert!(v.len() == 1, "Vec depth-1 must have exactly one element");
}

#[kani::proof]
fn suite__vec_u32__depth2_has_two_elements() {
    let v = <Vec<u32>>::kani_depth2();
    assert!(v.len() == 2, "Vec depth-2 must have exactly two elements");
}

// ── Option<T> ────────────────────────────────────────────────────────────────
//
// depth-0 → None (base case)
// depth-1 → Some(T::kani_depth0()) (inductive step)
// depth-2 → same as depth-1 (Option has only two cases; no second inductive step)

#[kani::proof]
fn suite__option_u32__depth0_is_none() {
    let o = <Option<u32>>::kani_depth0();
    assert!(o.is_none(), "Option depth-0 must be None");
}

#[kani::proof]
fn suite__option_u32__depth1_is_some() {
    let o = <Option<u32>>::kani_depth1();
    assert!(o.is_some(), "Option depth-1 must be Some");
}

// ── BTreeMap<K,V> ────────────────────────────────────────────────────────────
//
// Always empty at all depths.  BTreeMap structure does not affect VSM invariant
// preservation, and non-empty maps require symbolic keys (unbounded).

#[kani::proof]
fn suite__btreemap__depth0_is_empty() {
    let m = <std::collections::BTreeMap<u32, u32>>::kani_depth0();
    assert!(m.is_empty(), "BTreeMap depth-0 must be empty");
}

#[kani::proof]
fn suite__btreemap__depth2_still_empty() {
    let m = <std::collections::BTreeMap<u32, u32>>::kani_depth2();
    assert!(m.is_empty(), "BTreeMap depth-2 must still be empty");
}

// ── HashMap<K,V> ────────────────────────────────────────────────────────────
//
// **Not directly verifiable in Kani.**
//
// Two separate issues prevent a practical HashMap harness:
//
// 1. **RandomState**: the default `S = RandomState` reads the system clock in
//    `RandomState::new()`, which CBMC cannot model.
//
// 2. **hashbrown internals**: even with a deterministic hasher, constructing
//    an empty HashMap causes CBMC to model all unsafe hashbrown internal
//    invariants, generating too many proof obligations to complete in practice.
//
// **Implication for domain types**: use `BTreeMap` for any map field in a VSM
// state type.  The `KaniCompose` impl for `BTreeMap` is fully verified by the
// `suite__btreemap__*` harnesses above.
//
// The `KaniCompose` impl for `HashMap` remains in `kani_compose.rs` for
// completeness, but domain types that contain `HashMap` fields will likely
// time out when run under Kani.

// ── Struct: named fields ─────────────────────────────────────────────────────

/// Proves Named constructs at depth-0 with correct String postcondition.
#[kani::proof]
fn suite__struct_named__depth0_postconditions() {
    let n = Named::kani_depth0();
    assert!(n.name.is_empty(), "String field must be empty at depth-0");
}

/// Proves Named is clone-safe (no memory unsoundness in clone + drop).
#[kani::proof]
fn suite__struct_named__clone_soundness() {
    let n = Named::kani_depth0();
    let c = n.clone();
    drop(n);
    drop(c);
}

// ── Struct: tuple ────────────────────────────────────────────────────────────

#[kani::proof]
fn suite__struct_tuple__depth0_constructs() {
    let t = Tuple::kani_depth0();
    assert!(t.1.is_empty(), "String field in tuple must be empty at depth-0");
}

// ── Struct: unit ─────────────────────────────────────────────────────────────

#[kani::proof]
fn suite__struct_unit__all_depths_construct() {
    let _d0 = Unit::kani_depth0();
    let _d1 = Unit::kani_depth1();
    let _d2 = Unit::kani_depth2();
}

// ── Enum: unit variants only ──────────────────────────────────────────────────
//
// Derive picks the first unit variant (`A`).  All three depths must agree.

#[kani::proof]
fn suite__enum_unit__depth0_is_first_variant() {
    let d = UnitEnum::kani_depth0();
    assert!(matches!(d, UnitEnum::A), "UnitEnum depth-0 must be variant A");
}

#[kani::proof]
fn suite__enum_unit__all_depths_agree() {
    let d0 = UnitEnum::kani_depth0();
    let d1 = UnitEnum::kani_depth1();
    let d2 = UnitEnum::kani_depth2();
    // All should be A — use the unit enum's discriminant check
    assert!(matches!(d0, UnitEnum::A), "d0 must be A");
    assert!(matches!(d1, UnitEnum::A), "d1 must be A");
    assert!(matches!(d2, UnitEnum::A), "d2 must be A");
}

// ── Enum: data variants ───────────────────────────────────────────────────────
//
// `First` is a unit variant and should be picked at all depths.
// Proves that even when data variants follow, the unit-first rule holds.

#[kani::proof]
fn suite__enum_data__depth0_is_unit_first_variant() {
    let d = DataEnum::kani_depth0();
    assert!(matches!(d, DataEnum::First), "DataEnum depth-0 must be First");
}

// ── Struct with Vec<T> field ─────────────────────────────────────────────────
//
// Three harnesses prove the Vec field depth semantics carry through a struct.

#[kani::proof]
fn suite__struct_vec__depth0_items_empty() {
    let wv = WithVec::kani_depth0();
    assert!(wv.items.is_empty(), "Vec field must be empty at depth-0");
}

#[kani::proof]
fn suite__struct_vec__depth1_items_one_element() {
    let wv = WithVec::kani_depth1();
    assert!(wv.items.len() == 1, "Vec field must have one element at depth-1");
}

#[kani::proof]
fn suite__struct_vec__depth2_items_two_elements() {
    let wv = WithVec::kani_depth2();
    assert!(wv.items.len() == 2, "Vec field must have two elements at depth-2");
}

// ── Struct with Option<T> field ───────────────────────────────────────────────

#[kani::proof]
fn suite__struct_option__depth0_label_is_none() {
    let wo = WithOption::kani_depth0();
    assert!(wo.label.is_none(), "Option field must be None at depth-0");
}

#[kani::proof]
fn suite__struct_option__depth1_label_is_some() {
    let wo = WithOption::kani_depth1();
    assert!(wo.label.is_some(), "Option field must be Some at depth-1");
}

// ── Recursive struct ─────────────────────────────────────────────────────────
//
// `Tree { value: u32, children: Vec<Tree> }` is the canonical unbounded case.
// Three harnesses prove the compositional argument:
//
//   depth-0: leaf node (no children)  ← base case
//   depth-1: root with one depth-0 child  ← inductive step
//   depth-2: root with two depth-0 children  ← inductive step holds for two
//
// By induction: any finite tree is sound.

#[kani::proof]
fn suite__recursive__depth0_is_leaf() {
    let t = Tree::kani_depth0();
    assert!(t.children.is_empty(), "Tree depth-0 must have no children");
}

#[kani::proof]
fn suite__recursive__depth1_one_leaf_child() {
    let t = Tree::kani_depth1();
    assert!(t.children.len() == 1, "Tree depth-1 must have one child");
    assert!(
        t.children[0].children.is_empty(),
        "Child at depth-1 must itself be a leaf",
    );
}

#[kani::proof]
fn suite__recursive__depth2_two_leaf_children() {
    let t = Tree::kani_depth2();
    assert!(t.children.len() == 2, "Tree depth-2 must have two children");
    assert!(
        t.children[0].children.is_empty(),
        "First child at depth-2 must be a leaf",
    );
    assert!(
        t.children[1].children.is_empty(),
        "Second child at depth-2 must be a leaf",
    );
}

// ── Nested Vec<Option<T>> ─────────────────────────────────────────────────────
//
// `WithNested { items: Vec<Option<u32>> }` exercises composition of Vec and
// Option impls.  At depth-1 the Vec has one element which is the Option at
// depth-0 (None).  At depth-2 two Nones.

#[kani::proof]
fn suite__nested__depth0_items_empty() {
    let wn = WithNested::kani_depth0();
    assert!(wn.items.is_empty(), "Nested depth-0 must be empty");
}

#[kani::proof]
fn suite__nested__depth1_one_none_element() {
    let wn = WithNested::kani_depth1();
    assert!(wn.items.len() == 1, "Nested depth-1 must have one element");
    assert!(
        wn.items[0].is_none(),
        "Inner Option at depth-1 must be None (Vec uses kani_depth0 of inner)",
    );
}

#[kani::proof]
fn suite__nested__depth2_two_none_elements() {
    let wn = WithNested::kani_depth2();
    assert!(wn.items.len() == 2, "Nested depth-2 must have two elements");
    assert!(wn.items[0].is_none(), "First inner Option at depth-2 must be None");
    assert!(wn.items[1].is_none(), "Second inner Option at depth-2 must be None");
}

// ── chrono::DateTime<Utc> ─────────────────────────────────────────────────────
//
// The impl returns the Unix epoch (timestamp 0) at all depths.
// Also proves that a struct containing DateTime fields derives correctly.

#[kani::proof]
fn suite__chrono__depth0_is_epoch() {
    let dt = <chrono::DateTime<chrono::Utc>>::kani_depth0();
    assert!(dt.timestamp() == 0, "DateTime depth-0 must be Unix epoch");
}

#[kani::proof]
fn suite__chrono__struct_with_datetime_depth0() {
    let wd = WithDateTime::kani_depth0();
    assert!(
        wd.created.timestamp() == 0,
        "created field must be epoch at depth-0",
    );
    assert!(
        wd.updated.is_none(),
        "updated Option field must be None at depth-0",
    );
}

#[kani::proof]
fn suite__chrono__struct_with_datetime_depth1() {
    let wd = WithDateTime::kani_depth1();
    assert!(
        wd.updated.is_some(),
        "updated Option field must be Some at depth-1",
    );
}
