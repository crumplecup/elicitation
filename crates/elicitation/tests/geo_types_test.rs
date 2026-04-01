//! Tests for geo-types third-party support — Elicitation, ElicitIntrospect,
//! ElicitSpec, ElicitComplete implementations for geo-types wrappers.
//!
//! Covers: GeoCoord, GeoRect, GeoLine.

#![cfg(feature = "geo-types")]

use elicitation::{
    ElicitComplete, ElicitIntrospect, ElicitSpec, Elicitation, ElicitationPattern, GeoCoord,
    GeoCoordStyle, GeoLine, GeoLineStyle, GeoRect, GeoRectStyle,
};

/// Compile-time proof that T satisfies all ElicitComplete bounds.
fn assert_elicit_complete<T: ElicitComplete>() {}

/// Assert all three proof methods return non-empty token streams.
#[cfg(feature = "proofs")]
#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: kani_proof is empty");
    assert!(
        !T::verus_proof().is_empty(),
        "{label}: verus_proof is empty"
    );
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: creusot_proof is empty"
    );
}

// ── Conversions (From round-trips) ───────────────────────────────────────────

mod conversions {
    use super::*;
    use geo_types::{Coord, Line, Rect};

    #[test]
    fn coord_round_trip() {
        let coord = Coord { x: 1.5, y: -2.3 };
        let wrapper = GeoCoord::from(coord);
        assert_eq!(wrapper.x, 1.5);
        assert_eq!(wrapper.y, -2.3);
        let back: Coord<f64> = wrapper.into();
        assert_eq!(back.x, 1.5);
        assert_eq!(back.y, -2.3);
    }

    #[test]
    fn coord_zero() {
        let coord = Coord { x: 0.0, y: 0.0 };
        let wrapper = GeoCoord::from(coord);
        assert_eq!(wrapper.x, 0.0);
        assert_eq!(wrapper.y, 0.0);
    }

    #[test]
    fn rect_round_trip() {
        let rect = Rect::new(Coord { x: 0.0, y: 0.0 }, Coord { x: 10.0, y: 20.0 });
        let wrapper = GeoRect::from(rect);
        assert_eq!(wrapper.min.x, 0.0);
        assert_eq!(wrapper.min.y, 0.0);
        assert_eq!(wrapper.max.x, 10.0);
        assert_eq!(wrapper.max.y, 20.0);
        let back: Rect<f64> = wrapper.into();
        assert_eq!(back.min().x, 0.0);
        assert_eq!(back.max().x, 10.0);
    }

    #[test]
    fn rect_normalizes_corners() {
        // geo_types::Rect normalizes so min ≤ max
        let rect = Rect::new(Coord { x: 10.0, y: 20.0 }, Coord { x: 0.0, y: 0.0 });
        let wrapper = GeoRect::from(rect);
        assert!(wrapper.min.x <= wrapper.max.x);
        assert!(wrapper.min.y <= wrapper.max.y);
    }

    #[test]
    fn line_round_trip() {
        let line = Line::new(Coord { x: 1.0, y: 2.0 }, Coord { x: 3.0, y: 4.0 });
        let wrapper = GeoLine::from(line);
        assert_eq!(wrapper.start.x, 1.0);
        assert_eq!(wrapper.start.y, 2.0);
        assert_eq!(wrapper.end.x, 3.0);
        assert_eq!(wrapper.end.y, 4.0);
        let back: Line<f64> = wrapper.into();
        assert_eq!(back.start.x, 1.0);
        assert_eq!(back.end.y, 4.0);
    }

    #[test]
    fn line_degenerate_point() {
        let line = Line::new(Coord { x: 5.0, y: 5.0 }, Coord { x: 5.0, y: 5.0 });
        let wrapper = GeoLine::from(line);
        assert_eq!(wrapper.start.x, wrapper.end.x);
        assert_eq!(wrapper.start.y, wrapper.end.y);
    }
}

// ── ElicitIntrospect ─────────────────────────────────────────────────────────

mod introspect {
    use super::*;

    #[test]
    fn coord_is_survey() {
        assert_eq!(GeoCoord::pattern(), ElicitationPattern::Survey);
        let meta = GeoCoord::metadata();
        assert_eq!(meta.type_name, "geo_types::Coord<f64>");
    }

    #[test]
    fn rect_is_survey() {
        assert_eq!(GeoRect::pattern(), ElicitationPattern::Survey);
        let meta = GeoRect::metadata();
        assert_eq!(meta.type_name, "geo_types::Rect<f64>");
    }

    #[test]
    fn line_is_survey() {
        assert_eq!(GeoLine::pattern(), ElicitationPattern::Survey);
        let meta = GeoLine::metadata();
        assert_eq!(meta.type_name, "geo_types::Line<f64>");
    }

    #[test]
    fn coord_has_two_fields() {
        let meta = GeoCoord::metadata();
        if let elicitation::PatternDetails::Survey { fields } = &meta.details {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "x");
            assert_eq!(fields[1].name, "y");
        } else {
            panic!("expected Survey pattern");
        }
    }

    #[test]
    fn rect_has_two_fields() {
        let meta = GeoRect::metadata();
        if let elicitation::PatternDetails::Survey { fields } = &meta.details {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "min");
            assert_eq!(fields[1].name, "max");
        } else {
            panic!("expected Survey pattern");
        }
    }

    #[test]
    fn line_has_two_fields() {
        let meta = GeoLine::metadata();
        if let elicitation::PatternDetails::Survey { fields } = &meta.details {
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "start");
            assert_eq!(fields[1].name, "end");
        } else {
            panic!("expected Survey pattern");
        }
    }
}

// ── ElicitSpec (TypeSpec) ────────────────────────────────────────────────────

mod specs {
    use super::*;

    #[test]
    fn coord_spec_has_fields() {
        let spec = <GeoCoord as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "geo_types::Coord<f64>");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 2);
    }

    #[test]
    fn rect_spec_has_fields() {
        let spec = <GeoRect as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "geo_types::Rect<f64>");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 2);
    }

    #[test]
    fn line_spec_has_fields() {
        let spec = <GeoLine as ElicitSpec>::type_spec();
        assert_eq!(spec.type_name(), "geo_types::Line<f64>");
        let fields = spec
            .categories()
            .iter()
            .find(|c| c.name() == "fields")
            .expect("should have fields category");
        assert_eq!(fields.entries().len(), 2);
    }

    #[test]
    fn all_specs_have_source_category() {
        for spec in [
            <GeoCoord as ElicitSpec>::type_spec(),
            <GeoRect as ElicitSpec>::type_spec(),
            <GeoLine as ElicitSpec>::type_spec(),
        ] {
            let source = spec
                .categories()
                .iter()
                .find(|c| c.name() == "source")
                .unwrap_or_else(|| panic!("{} missing source category", spec.type_name()));
            let crate_entry = source
                .entries()
                .iter()
                .find(|e| e.label() == "crate")
                .unwrap_or_else(|| panic!("{} missing crate entry in source", spec.type_name()));
            assert!(
                crate_entry.description().contains("geo-types"),
                "{} source should mention geo-types",
                spec.type_name()
            );
        }
    }
}

// ── Serde round-trips ────────────────────────────────────────────────────────

mod serde_round_trips {
    use super::*;

    #[test]
    fn coord_serde_round_trip() {
        let coord = GeoCoord { x: 1.5, y: -2.3 };
        let json = serde_json::to_string(&coord).expect("serialize");
        let back: GeoCoord = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(coord, back);
    }

    #[test]
    fn rect_serde_round_trip() {
        let rect = GeoRect {
            min: GeoCoord { x: 0.0, y: 0.0 },
            max: GeoCoord { x: 10.0, y: 20.0 },
        };
        let json = serde_json::to_string(&rect).expect("serialize");
        let back: GeoRect = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(rect, back);
    }

    #[test]
    fn line_serde_round_trip() {
        let line = GeoLine {
            start: GeoCoord { x: 1.0, y: 2.0 },
            end: GeoCoord { x: 3.0, y: 4.0 },
        };
        let json = serde_json::to_string(&line).expect("serialize");
        let back: GeoLine = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(line, back);
    }

    #[test]
    fn coord_deserializes_defaults() {
        let json = "{}";
        let coord: GeoCoord = serde_json::from_str(json).expect("deserialize");
        assert_eq!(coord.x, 0.0);
        assert_eq!(coord.y, 0.0);
    }
}

// ── Style type associations ──────────────────────────────────────────────────

mod style_types {
    use super::*;

    #[test]
    fn coord_style_is_unit() {
        assert_eq!(GeoCoordStyle::default(), GeoCoordStyle::Default);
    }

    #[test]
    fn rect_style_is_unit() {
        assert_eq!(GeoRectStyle::default(), GeoRectStyle::Default);
    }

    #[test]
    fn line_style_is_unit() {
        assert_eq!(GeoLineStyle::default(), GeoLineStyle::Default);
    }
}

// ── ElicitComplete (compiler-verified bound satisfaction) ─────────────────────

mod elicit_complete {
    use super::*;

    #[test]
    fn all_geo_wrappers_are_elicit_complete() {
        assert_elicit_complete::<GeoCoord>();
        assert_elicit_complete::<GeoRect>();
        assert_elicit_complete::<GeoLine>();
    }
}

// ── Proof coverage (non-empty token streams) ─────────────────────────────────

#[cfg(feature = "proofs")]
mod proof_coverage {
    use super::*;

    #[test]
    fn coord_proofs_non_empty() {
        assert_proofs_non_empty::<GeoCoord>("GeoCoord");
    }

    #[test]
    fn rect_proofs_non_empty() {
        assert_proofs_non_empty::<GeoRect>("GeoRect");
    }

    #[test]
    fn line_proofs_non_empty() {
        assert_proofs_non_empty::<GeoLine>("GeoLine");
    }
}
