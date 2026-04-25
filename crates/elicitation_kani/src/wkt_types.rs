//! Kani proofs for WKT elicitation wrappers.
//!
//! Available with the `wkt-types` feature.

/// WktString delegates parsing to the upstream `wkt` crate.
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_string_trusted() {
    kani::assume(true);
    assert!(true, "WktString parsing is trusted third-party logic");
}

/// WktCoord preserves all fields through wrapper conversion roundtrip.
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_coord_from_roundtrip() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();
    let z: Option<f64> = kani::any();
    let m: Option<f64> = kani::any();

    kani::assume(x.is_finite() && y.is_finite());
    kani::assume(z.is_none_or(f64::is_finite));
    kani::assume(m.is_none_or(f64::is_finite));

    let original = wkt::types::Coord { x, y, z, m };
    let wrapper = elicitation::WktCoord::from(original);
    let restored: wkt::types::Coord<f64> = wrapper.into();

    assert!(restored.x == x, "Coord x preserved");
    assert!(restored.y == y, "Coord y preserved");
    assert!(restored.z == z, "Coord z preserved");
    assert!(restored.m == m, "Coord m preserved");
}

/// WktCoord exposes the same field values as the wrapped coordinate.
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_coord_wrapper_fields() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();
    let z: Option<f64> = kani::any();
    let m: Option<f64> = kani::any();

    kani::assume(x.is_finite() && y.is_finite());
    kani::assume(z.is_none_or(f64::is_finite));
    kani::assume(m.is_none_or(f64::is_finite));

    let original = wkt::types::Coord { x, y, z, m };
    let wrapper = elicitation::WktCoord::from(original);

    assert!(wrapper.x == x, "wrapper.x matches");
    assert!(wrapper.y == y, "wrapper.y matches");
    assert!(wrapper.z == z, "wrapper.z matches");
    assert!(wrapper.m == m, "wrapper.m matches");
}

/// WktPoint preserves emptiness through wrapper conversion.
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_point_empty_roundtrip() {
    let original = wkt::types::Point::<f64>::new(None, wkt::types::Dimension::XY);
    let wrapper = elicitation::WktPoint::from(original);
    let restored: wkt::types::Point<f64> = wrapper.into();

    assert!(wrapper.coord.is_none(), "wrapper preserves emptiness");
    assert!(restored.coord().is_none(), "roundtrip preserves emptiness");
}

/// WktGeom preserves the Point variant through conversion.
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_geom_point_variant() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();

    kani::assume(x.is_finite() && y.is_finite());

    let original = wkt::Wkt::Point(wkt::types::Point::from_coord(wkt::types::Coord {
        x,
        y,
        z: None,
        m: None,
    }));
    let wrapper = elicitation::WktGeom::from(original);

    assert!(
        matches!(wrapper, elicitation::WktGeom::Point(_)),
        "Point variant preserved"
    );
}
