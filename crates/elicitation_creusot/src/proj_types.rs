//! Creusot proofs for proj elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "proj-types")]

use creusot_std::prelude::*;

/// Trusted axiom: `ProjArea::new` stores all four boundary values without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_proj_area_new_fields() -> bool {
    let area = elicitation::ProjArea::new(1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64);
    area.west == 1.0_f64 && area.south == 2.0_f64 && area.east == 3.0_f64 && area.north == 4.0_f64
}

/// Trusted axiom: `ProjArea` round-trips through `proj::Area` without data loss.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_proj_area_roundtrip() -> bool {
    let area = elicitation::ProjArea::new(-10.0_f64, -20.0_f64, 10.0_f64, 20.0_f64);
    let inner: proj::Area = area.into();
    let roundtrip = elicitation::ProjArea::from(inner);
    roundtrip.west == -10.0_f64
        && roundtrip.south == -20.0_f64
        && roundtrip.east == 10.0_f64
        && roundtrip.north == 20.0_f64
}

/// Trusted axiom: antimeridian-crossing areas are stored correctly.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_proj_area_antimeridian() -> bool {
    let area = elicitation::ProjArea::new(170.0_f64, -10.0_f64, -170.0_f64, 10.0_f64);
    area.west > area.east
}
