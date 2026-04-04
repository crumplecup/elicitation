//! Kani proofs for geo-types elicitation wrappers.
//!
//! Available with the `geo-types` feature.
//!
//! # Proof Strategy
//!
//! For each composite wrapper we verify:
//! 1. **From roundtrip**: `Wrapper::from(geo_type)` then back preserves values
//! 2. **Field preservation**: individual fields survive the conversion
//! 3. **Well-formedness**: Rect min ≤ max after normalization

// ── Coord proofs ─────────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_coord_from_roundtrip() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    // Skip NaN (NaN != NaN breaks equality)
    kani::assume(x.is_finite());
    kani::assume(y.is_finite());

    let original = geo_types::Coord { x, y };
    let wrapper = elicitation::GeoCoord::from(original);
    let restored: geo_types::Coord<f64> = wrapper.into();

    assert!(restored.x == x, "Coord x preserved");
    assert!(restored.y == y, "Coord y preserved");
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_coord_wrapper_fields() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    kani::assume(x.is_finite());
    kani::assume(y.is_finite());

    let original = geo_types::Coord { x, y };
    let wrapper = elicitation::GeoCoord::from(original);

    assert!(wrapper.x == x, "Coord wrapper.x matches");
    assert!(wrapper.y == y, "Coord wrapper.y matches");
}

// ── Rect proofs ──────────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_rect_from_roundtrip() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());

    let original = geo_types::Rect::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
    );
    let wrapper = elicitation::GeoRect::from(original);
    let restored: geo_types::Rect<f64> = wrapper.into();

    // geo_types::Rect normalizes min/max, so compare normalized values
    assert!(restored.min().x == original.min().x, "Rect min.x preserved");
    assert!(restored.min().y == original.min().y, "Rect min.y preserved");
    assert!(restored.max().x == original.max().x, "Rect max.x preserved");
    assert!(restored.max().y == original.max().y, "Rect max.y preserved");
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_rect_well_formed() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());

    let rect = geo_types::Rect::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
    );
    let wrapper = elicitation::GeoRect::from(rect);

    // After normalization, min ≤ max on each axis
    assert!(wrapper.min.x <= wrapper.max.x, "Rect min.x <= max.x");
    assert!(wrapper.min.y <= wrapper.max.y, "Rect min.y <= max.y");
}

// ── Line proofs ──────────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_line_from_roundtrip() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());

    let original = geo_types::Line::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
    );
    let wrapper = elicitation::GeoLine::from(original);
    let restored: geo_types::Line<f64> = wrapper.into();

    assert!(restored.start.x == x1, "Line start.x preserved");
    assert!(restored.start.y == y1, "Line start.y preserved");
    assert!(restored.end.x == x2, "Line end.x preserved");
    assert!(restored.end.y == y2, "Line end.y preserved");
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_line_wrapper_fields() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());

    let original = geo_types::Line::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
    );
    let wrapper = elicitation::GeoLine::from(original);

    assert!(wrapper.start.x == x1, "Line wrapper.start.x matches");
    assert!(wrapper.start.y == y1, "Line wrapper.start.y matches");
    assert!(wrapper.end.x == x2, "Line wrapper.end.x matches");
    assert!(wrapper.end.y == y2, "Line wrapper.end.y matches");
}
