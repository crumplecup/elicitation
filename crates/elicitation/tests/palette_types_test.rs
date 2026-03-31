//! Tests for palette type elicitation wrappers.
//!
//! Covers: conversion round-trips, introspection, specs, serde,
//! style types, ElicitComplete, and proof coverage.

#![cfg(feature = "palette")]

use elicitation::{
    ElicitComplete, ElicitIntrospect, ElicitSpec, ElicitationPattern, PaletteSrgb, PaletteSrgbStyle,
};
use palette::Srgb;

// ══════════════════════════════════════════════════════════════════════
// Conversion round-trips
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_from_roundtrip() {
    let original = Srgb::new(0.5_f32, 0.75_f32, 0.25_f32);
    let wrapper = PaletteSrgb::from(original);
    let restored: Srgb<f32> = wrapper.into();
    assert_eq!(restored.red, 0.5);
    assert_eq!(restored.green, 0.75);
    assert_eq!(restored.blue, 0.25);
}

#[test]
fn srgb_wrapper_fields() {
    let original = Srgb::new(0.1_f32, 0.2_f32, 0.3_f32);
    let wrapper = PaletteSrgb::from(original);
    assert_eq!(wrapper.r, 0.1);
    assert_eq!(wrapper.g, 0.2);
    assert_eq!(wrapper.b, 0.3);
}

#[test]
fn srgb_black_roundtrip() {
    let black = Srgb::new(0.0_f32, 0.0_f32, 0.0_f32);
    let wrapper = PaletteSrgb::from(black);
    assert_eq!(wrapper.r, 0.0);
    assert_eq!(wrapper.g, 0.0);
    assert_eq!(wrapper.b, 0.0);
    let restored: Srgb<f32> = wrapper.into();
    assert_eq!(restored.red, 0.0);
}

#[test]
fn srgb_white_roundtrip() {
    let white = Srgb::new(1.0_f32, 1.0_f32, 1.0_f32);
    let wrapper = PaletteSrgb::from(white);
    assert_eq!(wrapper.r, 1.0);
    assert_eq!(wrapper.g, 1.0);
    assert_eq!(wrapper.b, 1.0);
    let restored: Srgb<f32> = wrapper.into();
    assert_eq!(restored.red, 1.0);
}

#[test]
fn srgb_primary_colors() {
    let red = PaletteSrgb::from(Srgb::new(1.0_f32, 0.0_f32, 0.0_f32));
    assert_eq!(red.r, 1.0);
    assert_eq!(red.g, 0.0);
    assert_eq!(red.b, 0.0);

    let green = PaletteSrgb::from(Srgb::new(0.0_f32, 1.0_f32, 0.0_f32));
    assert_eq!(green.r, 0.0);
    assert_eq!(green.g, 1.0);
    assert_eq!(green.b, 0.0);

    let blue = PaletteSrgb::from(Srgb::new(0.0_f32, 0.0_f32, 1.0_f32));
    assert_eq!(blue.r, 0.0);
    assert_eq!(blue.g, 0.0);
    assert_eq!(blue.b, 1.0);
}

// ══════════════════════════════════════════════════════════════════════
// ElicitIntrospect
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_introspect_pattern() {
    assert_eq!(PaletteSrgb::pattern(), ElicitationPattern::Survey);
}

#[test]
fn srgb_introspect_metadata() {
    let meta = PaletteSrgb::metadata();
    assert_eq!(meta.type_name, "palette::Srgb<f32>");
    assert!(meta.description.is_some());
}

#[test]
fn srgb_introspect_fields() {
    let meta = PaletteSrgb::metadata();
    match &meta.details {
        elicitation::PatternDetails::Survey { fields } => {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0].name, "r");
            assert_eq!(fields[1].name, "g");
            assert_eq!(fields[2].name, "b");
        }
        other => panic!("Expected Survey, got {other:?}"),
    }
}

// ══════════════════════════════════════════════════════════════════════
// ElicitSpec
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_spec_has_type_name() {
    let spec = PaletteSrgb::type_spec();
    assert_eq!(spec.type_name(), "palette::Srgb<f32>");
}

#[test]
fn srgb_spec_has_categories() {
    let spec = PaletteSrgb::type_spec();
    let cats = spec.categories();
    assert!(cats.len() >= 2, "Expected fields + source categories");
}

#[test]
fn srgb_spec_fields_category() {
    let spec = PaletteSrgb::type_spec();
    let fields_cat = spec.categories().iter().find(|c| c.name() == "fields");
    assert!(fields_cat.is_some(), "Missing 'fields' category");
    let entries = fields_cat.unwrap().entries();
    assert_eq!(entries.len(), 3);
}

// ══════════════════════════════════════════════════════════════════════
// Serde round-trips
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_serde_roundtrip() {
    let wrapper = PaletteSrgb { r: 0.5, g: 0.75, b: 0.25 };
    let json = serde_json::to_string(&wrapper).expect("serialize");
    let restored: PaletteSrgb = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(wrapper, restored);
}

#[test]
fn srgb_serde_default_fields() {
    let json = "{}";
    let wrapper: PaletteSrgb = serde_json::from_str(json).expect("deserialize with defaults");
    assert_eq!(wrapper.r, 0.0);
    assert_eq!(wrapper.g, 0.0);
    assert_eq!(wrapper.b, 0.0);
}

// ══════════════════════════════════════════════════════════════════════
// Style types
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_style_is_unit() {
    let _style = PaletteSrgbStyle::Default;
    let debug = format!("{_style:?}");
    assert!(!debug.is_empty());
}

// ══════════════════════════════════════════════════════════════════════
// ElicitComplete
// ══════════════════════════════════════════════════════════════════════

#[test]
fn srgb_is_elicit_complete() {
    fn assert_complete<T: ElicitComplete>() {}
    assert_complete::<PaletteSrgb>();
}

// ══════════════════════════════════════════════════════════════════════
// Proof coverage
// ══════════════════════════════════════════════════════════════════════

#[cfg(feature = "proofs")]
mod proof_coverage {
    use elicitation::{Elicitation, PaletteSrgb};

    #[test]
    fn srgb_kani_proof_emits() {
        let tokens = PaletteSrgb::kani_proof();
        assert!(!tokens.is_empty(), "Kani proof should emit tokens");
    }

    #[test]
    fn srgb_verus_proof_emits() {
        let tokens = PaletteSrgb::verus_proof();
        assert!(!tokens.is_empty(), "Verus proof should emit tokens");
    }

    #[test]
    fn srgb_creusot_proof_emits() {
        let tokens = PaletteSrgb::creusot_proof();
        assert!(!tokens.is_empty(), "Creusot proof should emit tokens");
    }
}
