//! Kani proofs for proj elicitation support.
//!
//! Available with the `proj-types` feature.

/// `ProjArea::new` stores all four boundary values without modification.
#[cfg(feature = "proj-types")]
#[kani::proof]
fn verify_proj_area_new_fields() {
    let area = elicitation::ProjArea::new(1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64);
    assert!(area.west == 1.0_f64, "west preserved");
    assert!(area.south == 2.0_f64, "south preserved");
    assert!(area.east == 3.0_f64, "east preserved");
    assert!(area.north == 4.0_f64, "north preserved");
}

/// `ProjArea` round-trips through `proj::Area` without losing any boundary value.
#[cfg(feature = "proj-types")]
#[kani::proof]
fn verify_proj_area_roundtrip() {
    let area = elicitation::ProjArea::new(-10.0_f64, -20.0_f64, 10.0_f64, 20.0_f64);
    let inner: proj::Area = area.into();
    let roundtrip = elicitation::ProjArea::from(inner);
    assert!(roundtrip.west == -10.0_f64, "west roundtrip");
    assert!(roundtrip.south == -20.0_f64, "south roundtrip");
    assert!(roundtrip.east == 10.0_f64, "east roundtrip");
    assert!(roundtrip.north == 20.0_f64, "north roundtrip");
}

/// Antimeridian-crossing areas are stored correctly (west > east is valid).
#[cfg(feature = "proj-types")]
#[kani::proof]
fn verify_proj_area_antimeridian() {
    let area = elicitation::ProjArea::new(170.0_f64, -10.0_f64, -170.0_f64, 10.0_f64);
    assert!(area.west > area.east, "antimeridian crossing: west > east");
}
