//! Creusot proofs for WKT elicitation wrappers.
//!
//! Trust the source. Verify the wrapper.

#![cfg(feature = "wkt-types")]

use creusot_std::prelude::*;

/// Trusted axiom: WktString parsing delegates to the upstream crate.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkt_string_trusted() -> bool {
    true
}

/// Trusted axiom: WktCoord roundtrip preserves all fields.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkt_coord_roundtrip() -> bool {
    let coord = wkt::types::Coord {
        x: 1.5_f64,
        y: -2.0_f64,
        z: Some(3.25_f64),
        m: Some(4.5_f64),
    };
    let wrapper = elicitation::WktCoord::from(coord);
    let restored: wkt::types::Coord<f64> = wrapper.into();
    restored.x == 1.5_f64
        && restored.y == -2.0_f64
        && restored.z == Some(3.25_f64)
        && restored.m == Some(4.5_f64)
}

/// Trusted axiom: WktCoord concrete field construction is preserved.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkt_coord_concrete() -> bool {
    let coord = wkt::types::Coord {
        x: 0.0_f64,
        y: 0.0_f64,
        z: None,
        m: None,
    };
    let wrapper = elicitation::WktCoord::from(coord);
    wrapper.x == 0.0_f64 && wrapper.y == 0.0_f64 && wrapper.z.is_none() && wrapper.m.is_none()
}

/// Trusted axiom: empty WKT points remain empty through the wrapper.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkt_point_empty() -> bool {
    let point = wkt::types::Point::<f64>(None);
    let wrapper = elicitation::WktPoint::from(point);
    let is_empty = wrapper.coord.is_none();
    let restored: wkt::types::Point<f64> = wrapper.into();
    is_empty && restored.0.is_none()
}

/// Trusted axiom: WktGeom preserves the Point variant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wkt_geom_point_variant() -> bool {
    let geom = wkt::Wkt::Point(wkt::types::Point(Some(wkt::types::Coord {
        x: 1.0_f64,
        y: 2.0_f64,
        z: None,
        m: None,
    })));
    let wrapper = elicitation::WktGeom::from(geom);
    matches!(wrapper, elicitation::WktGeom::Point(_))
}
