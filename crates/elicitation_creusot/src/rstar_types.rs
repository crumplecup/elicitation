//! Creusot proofs for rstar elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "rstar-types")]

use creusot_std::prelude::*;

/// Trusted axiom: `RstarAabb` round-trips through `rstar::AABB` without losing corners.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_rstar_aabb_roundtrip() -> bool {
    let wrapper = elicitation::RstarAabb {
        lower: [1.0_f64, 2.0_f64],
        upper: [3.0_f64, 4.0_f64],
    };
    let inner: rstar::AABB<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarAabb::from(inner);

    roundtrip.lower == [1.0_f64, 2.0_f64] && roundtrip.upper == [3.0_f64, 4.0_f64]
}

/// Trusted axiom: `RstarRectangle` round-trips through the upstream primitive.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_rstar_rectangle_roundtrip() -> bool {
    let wrapper = elicitation::RstarRectangle {
        lower: [0.0_f64, 1.0_f64],
        upper: [2.0_f64, 3.0_f64],
    };
    let inner: rstar::primitives::Rectangle<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarRectangle::from(inner);

    roundtrip.lower == [0.0_f64, 1.0_f64] && roundtrip.upper == [2.0_f64, 3.0_f64]
}

/// Trusted axiom: rectangle envelopes preserve the wrapper's corners.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_rstar_rectangle_envelope_bounds() -> bool {
    let wrapper = elicitation::RstarRectangle {
        lower: [1.0_f64, 2.0_f64],
        upper: [4.0_f64, 6.0_f64],
    };
    let envelope = rstar::RTreeObject::envelope(&wrapper);

    envelope.lower() == [1.0_f64, 2.0_f64] && envelope.upper() == [4.0_f64, 6.0_f64]
}

/// Trusted axiom: `RstarLine` round-trips through the upstream primitive.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_rstar_line_roundtrip() -> bool {
    let wrapper = elicitation::RstarLine {
        from: [5.0_f64, 1.0_f64],
        to: [2.0_f64, 4.0_f64],
    };
    let inner: rstar::primitives::Line<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarLine::from(inner);

    roundtrip.from == [5.0_f64, 1.0_f64] && roundtrip.to == [2.0_f64, 4.0_f64]
}

/// Trusted axiom: line envelopes normalize endpoint order into bounding corners.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_rstar_line_envelope_bounds() -> bool {
    let wrapper = elicitation::RstarLine {
        from: [5.0_f64, 1.0_f64],
        to: [2.0_f64, 4.0_f64],
    };
    let envelope = rstar::RTreeObject::envelope(&wrapper);

    envelope.lower() == [2.0_f64, 1.0_f64] && envelope.upper() == [5.0_f64, 4.0_f64]
}
