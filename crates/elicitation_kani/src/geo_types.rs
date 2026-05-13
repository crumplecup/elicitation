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

// ── Point proofs ─────────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_point_from_roundtrip() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    kani::assume(x.is_finite() && y.is_finite());

    let original = geo_types::Point::new(x, y);
    let wrapper = elicitation::GeoPoint::from(original);
    let restored: geo_types::Point<f64> = wrapper.into();

    assert!(restored.x() == x, "Point x preserved");
    assert!(restored.y() == y, "Point y preserved");
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_point_wrapper_fields() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    kani::assume(x.is_finite() && y.is_finite());

    let original = geo_types::Point::new(x, y);
    let wrapper = elicitation::GeoPoint::from(original);

    assert!(wrapper.coord.x == x, "Point wrapper.coord.x matches");
    assert!(wrapper.coord.y == y, "Point wrapper.coord.y matches");
}

// ── Triangle proofs ──────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_triangle_from_roundtrip() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();
    let x3: f64 = kani::any();
    let y3: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());
    kani::assume(x3.is_finite() && y3.is_finite());
    // Bound to prevent cross-product intermediate overflow (finite * finite can
    // exceed f64::MAX, yielding ±infinity, whose subtraction produces NaN).
    kani::assume(x1.abs() < 1e150 && y1.abs() < 1e150);
    kani::assume(x2.abs() < 1e150 && y2.abs() < 1e150);
    kani::assume(x3.abs() < 1e150 && y3.abs() < 1e150);

    let original = geo_types::Triangle::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
        geo_types::Coord { x: x3, y: y3 },
    );
    let wrapper = elicitation::GeoTriangle::from(original);
    let restored: geo_types::Triangle<f64> = wrapper.into();

    // geo_types::Triangle::new reorders vertices to CCW orientation, so the
    // roundtrip may permute v1↔v3. Assert vertex-set preservation instead of
    // positional equality.
    let orig = [original.0, original.1, original.2];
    let r0_ok = orig
        .iter()
        .any(|v| v.x == restored.0.x && v.y == restored.0.y);
    let r1_ok = orig
        .iter()
        .any(|v| v.x == restored.1.x && v.y == restored.1.y);
    let r2_ok = orig
        .iter()
        .any(|v| v.x == restored.2.x && v.y == restored.2.y);
    assert!(
        r0_ok && r1_ok && r2_ok,
        "Triangle vertex set preserved through roundtrip"
    );
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_triangle_wrapper_fields() {
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();
    let x2: f64 = kani::any();
    let y2: f64 = kani::any();
    let x3: f64 = kani::any();
    let y3: f64 = kani::any();

    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x2.is_finite() && y2.is_finite());
    kani::assume(x3.is_finite() && y3.is_finite());
    // Bound to prevent cross-product intermediate overflow.
    kani::assume(x1.abs() < 1e150 && y1.abs() < 1e150);
    kani::assume(x2.abs() < 1e150 && y2.abs() < 1e150);
    kani::assume(x3.abs() < 1e150 && y3.abs() < 1e150);

    let original = geo_types::Triangle::new(
        geo_types::Coord { x: x1, y: y1 },
        geo_types::Coord { x: x2, y: y2 },
        geo_types::Coord { x: x3, y: y3 },
    );
    let wrapper = elicitation::GeoTriangle::from(original);

    // geo_types::Triangle::new may reorder to CCW — the wrapper faithfully
    // mirrors whatever order Triangle stored, so assert against original.0/.1/.2
    // rather than against the raw x1/x2/x3 inputs.
    assert!(
        wrapper.v1.x == original.0.x && wrapper.v1.y == original.0.y,
        "Triangle wrapper.v1 mirrors original.0"
    );
    assert!(
        wrapper.v2.x == original.1.x && wrapper.v2.y == original.1.y,
        "Triangle wrapper.v2 mirrors original.1"
    );
    assert!(
        wrapper.v3.x == original.2.x && wrapper.v3.y == original.2.y,
        "Triangle wrapper.v3 mirrors original.2"
    );
}

// ── LineString proofs ────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_line_string_length_preserved() {
    // Two-coord concrete case (arbitrary-length Vecs are beyond Kani's scope)
    let coord_a = geo_types::Coord {
        x: 1.0_f64,
        y: 2.0_f64,
    };
    let coord_b = geo_types::Coord {
        x: 3.0_f64,
        y: 4.0_f64,
    };
    let original = geo_types::LineString::new(vec![coord_a, coord_b]);
    let wrapper = elicitation::GeoLineString::from(original);

    assert!(wrapper.0.len() == 2, "LineString length preserved");
    assert!(
        wrapper.0[0].x == 1.0_f64,
        "LineString first coord x preserved"
    );
    assert!(
        wrapper.0[1].y == 4.0_f64,
        "LineString second coord y preserved"
    );
}

// ── MultiPoint proofs ────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_multi_point_length_preserved() {
    let p1 = geo_types::Point::new(1.0_f64, 2.0_f64);
    let p2 = geo_types::Point::new(3.0_f64, 4.0_f64);
    let original = geo_types::MultiPoint::new(vec![p1, p2]);
    let wrapper = elicitation::GeoMultiPoint::from(original);

    assert!(wrapper.0.len() == 2, "MultiPoint length preserved");
    assert!(
        wrapper.0[0].coord.x == 1.0_f64,
        "MultiPoint first point x preserved"
    );
}

// ── Geometry proofs ──────────────────────────────────────────────────────────

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_geometry_point_variant() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    kani::assume(x.is_finite() && y.is_finite());

    let original = geo_types::Geometry::Point(geo_types::Point::new(x, y));
    let wrapper = elicitation::GeoGeometry::from(original);

    assert!(
        matches!(wrapper, elicitation::GeoGeometry::Point(_)),
        "Point geometry variant preserved"
    );
}

#[cfg(feature = "geo-types")]
#[kani::proof]
fn verify_geo_geometry_rect_variant() {
    let original = geo_types::Geometry::Rect(geo_types::Rect::new(
        geo_types::Coord {
            x: 0.0_f64,
            y: 0.0_f64,
        },
        geo_types::Coord {
            x: 1.0_f64,
            y: 1.0_f64,
        },
    ));
    let wrapper = elicitation::GeoGeometry::from(original);

    assert!(
        matches!(wrapper, elicitation::GeoGeometry::Rect(_)),
        "Rect geometry variant preserved"
    );
}
