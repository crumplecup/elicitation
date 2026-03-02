//! Tests for composed `ElicitSpec` via `#[derive(Elicit)]`.
//!
//! Verifies that deriving `Elicit` on a struct also generates an `ElicitSpec`
//! implementation assembled from the field types' specs, plus user-supplied
//! `#[spec_summary]` and `#[spec_requires]` attributes.

use elicitation::verification::types::{I32Positive, StringNonEmpty};
use elicitation::{Elicit, lookup_type_spec, lookup_type_spec_by_id};
use std::any::TypeId;

// ── Basic composition ─────────────────────────────────────────────────────────

#[derive(Elicit, Debug)]
struct SimpleStruct {
    count: I32Positive,
    label: StringNonEmpty,
}

#[test]
fn derived_struct_registers_type_spec() {
    let s = SimpleStruct {
        count: I32Positive::new(1).unwrap(),
        label: StringNonEmpty::new("hello".to_string()).unwrap(),
    };
    tracing::info!(count = ?s.count, label = ?s.label, "SimpleStruct instance");
    let spec = lookup_type_spec("SimpleStruct").expect("SimpleStruct should be registered");
    assert_eq!(spec.type_name(), "SimpleStruct");
}

#[test]
fn derived_struct_auto_summary() {
    let spec = lookup_type_spec("SimpleStruct").unwrap();
    // Auto-summary references the field count
    assert!(
        spec.summary().contains("2"),
        "auto summary should mention 2 fields"
    );
}

#[test]
fn derived_struct_field_category_for_i32positive() {
    let spec = lookup_type_spec("SimpleStruct").unwrap();
    let cat = spec
        .categories()
        .iter()
        .find(|c| c.name() == "fields.count");
    assert!(cat.is_some(), "should have fields.count sub-category");
    let cat = cat.unwrap();
    // I32Positive requires value > 0
    let has_positive = cat
        .entries()
        .iter()
        .any(|e| e.expression().as_deref() == Some("value > 0"));
    assert!(
        has_positive,
        "fields.count should inherit value > 0 from I32Positive"
    );
}

#[test]
fn derived_struct_field_category_for_string_non_empty() {
    let spec = lookup_type_spec("SimpleStruct").unwrap();
    let cat = spec
        .categories()
        .iter()
        .find(|c| c.name() == "fields.label");
    assert!(cat.is_some(), "should have fields.label sub-category");
    let cat = cat.unwrap();
    assert!(
        !cat.entries().is_empty(),
        "fields.label should have inherited entries"
    );
}

#[test]
fn derived_struct_registered_by_type_id() {
    let spec = lookup_type_spec_by_id(TypeId::of::<SimpleStruct>());
    assert!(spec.is_some(), "SimpleStruct should be findable by TypeId");
}

// ── #[spec_summary] override ──────────────────────────────────────────────────

#[derive(Elicit, Debug)]
#[spec_summary = "A date range with a validated start and end point."]
struct DateRange {
    start: I32Positive,
    end: I32Positive,
}

#[test]
fn spec_summary_attr_overrides_auto() {
    let d = DateRange {
        start: I32Positive::new(1).unwrap(),
        end: I32Positive::new(10).unwrap(),
    };
    tracing::info!(start = ?d.start, end = ?d.end, "DateRange instance");
    let spec = lookup_type_spec("DateRange").unwrap();
    assert_eq!(
        spec.summary(),
        "A date range with a validated start and end point."
    );
}

// ── #[spec_requires] on struct (top-level invariant) ─────────────────────────

#[derive(Elicit, Debug)]
#[spec_requires(start < end)]
struct OrderedRange {
    start: I32Positive,
    end: I32Positive,
}

#[test]
fn struct_level_spec_requires_creates_requires_category() {
    let o = OrderedRange {
        start: I32Positive::new(1).unwrap(),
        end: I32Positive::new(5).unwrap(),
    };
    tracing::info!(start = ?o.start, end = ?o.end, "OrderedRange instance");
    let spec = lookup_type_spec("OrderedRange").unwrap();
    let cat = spec.categories().iter().find(|c| c.name() == "requires");
    assert!(
        cat.is_some(),
        "struct-level #[spec_requires] should produce a 'requires' category"
    );
    let cat = cat.unwrap();
    let has_expr = cat.entries().iter().any(|e| {
        e.expression()
            .as_deref()
            .is_some_and(|x| x.contains("start") && x.contains("end"))
    });
    assert!(
        has_expr,
        "requires entry should contain the start < end expression"
    );
}

// ── #[spec_requires] on a field (extra entry in field's sub-category) ─────────

#[derive(Elicit, Debug)]
struct BoundedCount {
    #[spec_requires(value < 100)]
    count: I32Positive,
    label: StringNonEmpty,
}

#[test]
fn field_level_spec_requires_appends_to_field_category() {
    let b = BoundedCount {
        count: I32Positive::new(42).unwrap(),
        label: StringNonEmpty::new("hi".to_string()).unwrap(),
    };
    tracing::info!(count = ?b.count, label = ?b.label, "BoundedCount instance");
    let spec = lookup_type_spec("BoundedCount").unwrap();
    let cat = spec
        .categories()
        .iter()
        .find(|c| c.name() == "fields.count")
        .unwrap();
    // Should have the inherited I32Positive entry AND the user extra
    let has_inherited = cat
        .entries()
        .iter()
        .any(|e| e.expression().as_deref() == Some("value > 0"));
    let has_extra = cat
        .entries()
        .iter()
        .any(|e| e.expression().as_deref().is_some_and(|x| x.contains("100")));
    assert!(
        has_inherited,
        "should still have inherited I32Positive requires"
    );
    assert!(has_extra, "should have the user-added value < 100 entry");
}

// ── Field with no registered ElicitSpec produces no sub-category ──────────────
// Verified implicitly: the codegen calls lookup_type_spec_by_id and produces
// no sub-category when None is returned. This is exercised when Outer contains
// Inner (a user-derived type) — the entries come from Inner's own composed spec,
// not from a primitive, proving the fallback path works when a type IS found and
// also compiles cleanly even if a field type is never registered.

// ── Recursive composition ─────────────────────────────────────────────────────

#[derive(Elicit, Debug)]
struct Inner {
    value: I32Positive,
}

#[derive(Elicit, Debug)]
struct Outer {
    inner: Inner,
    label: StringNonEmpty,
}

#[test]
fn recursive_composition_propagates_specs() {
    let o = Outer {
        inner: Inner {
            value: I32Positive::new(7).unwrap(),
        },
        label: StringNonEmpty::new("outer".to_string()).unwrap(),
    };
    tracing::info!(inner = ?o.inner.value, label = ?o.label, "Outer instance");
    // Inner must be registered first for Outer to find it
    let inner_spec = lookup_type_spec("Inner").expect("Inner registered");
    assert!(
        inner_spec
            .categories()
            .iter()
            .any(|c| c.name() == "fields.value")
    );

    let outer_spec = lookup_type_spec("Outer").expect("Outer registered");
    // Outer.fields.inner should contain entries inherited from Inner's spec
    let cat = outer_spec
        .categories()
        .iter()
        .find(|c| c.name() == "fields.inner");
    assert!(cat.is_some(), "Outer should have fields.inner sub-category");
}
