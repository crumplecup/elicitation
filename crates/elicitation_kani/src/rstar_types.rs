//! Kani proofs for rstar elicitation support.
//!
//! Available with the `rstar-types` feature.

/// `RstarAabb` round-trips lower and upper corners through the upstream `AABB`.
#[cfg(feature = "rstar-types")]
#[kani::proof]
fn verify_rstar_aabb_roundtrip() {
    let wrapper = elicitation::RstarAabb {
        lower: [1.0_f64, 2.0_f64],
        upper: [3.0_f64, 4.0_f64],
    };
    let inner: rstar::AABB<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarAabb::from(inner);

    assert!(
        roundtrip.lower == [1.0_f64, 2.0_f64],
        "lower corner preserved"
    );
    assert!(
        roundtrip.upper == [3.0_f64, 4.0_f64],
        "upper corner preserved"
    );
}

/// `RstarRectangle` round-trips both corners through the upstream primitive.
#[cfg(feature = "rstar-types")]
#[kani::proof]
fn verify_rstar_rectangle_roundtrip() {
    let wrapper = elicitation::RstarRectangle {
        lower: [0.0_f64, 1.0_f64],
        upper: [2.0_f64, 3.0_f64],
    };
    let inner: rstar::primitives::Rectangle<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarRectangle::from(inner);

    assert!(
        roundtrip.lower == [0.0_f64, 1.0_f64],
        "lower corner preserved"
    );
    assert!(
        roundtrip.upper == [2.0_f64, 3.0_f64],
        "upper corner preserved"
    );
}

/// Rectangle envelopes preserve the same bounding corners as the wrapper.
#[cfg(feature = "rstar-types")]
#[kani::proof]
fn verify_rstar_rectangle_envelope_bounds() {
    let wrapper = elicitation::RstarRectangle {
        lower: [1.0_f64, 2.0_f64],
        upper: [4.0_f64, 6.0_f64],
    };
    let envelope = rstar::RTreeObject::envelope(&wrapper);

    assert!(
        envelope.lower() == [1.0_f64, 2.0_f64],
        "envelope lower preserved"
    );
    assert!(
        envelope.upper() == [4.0_f64, 6.0_f64],
        "envelope upper preserved"
    );
}

/// `RstarLine` round-trips endpoints through the upstream primitive.
#[cfg(feature = "rstar-types")]
#[kani::proof]
fn verify_rstar_line_roundtrip() {
    let wrapper = elicitation::RstarLine {
        from: [5.0_f64, 1.0_f64],
        to: [2.0_f64, 4.0_f64],
    };
    let inner: rstar::primitives::Line<[f64; 2]> = wrapper.into();
    let roundtrip = elicitation::RstarLine::from(inner);

    assert!(
        roundtrip.from == [5.0_f64, 1.0_f64],
        "start point preserved"
    );
    assert!(roundtrip.to == [2.0_f64, 4.0_f64], "end point preserved");
}

/// Line envelopes normalize endpoint order into the expected bounding box.
#[cfg(feature = "rstar-types")]
#[kani::proof]
fn verify_rstar_line_envelope_bounds() {
    let wrapper = elicitation::RstarLine {
        from: [5.0_f64, 1.0_f64],
        to: [2.0_f64, 4.0_f64],
    };
    let envelope = rstar::RTreeObject::envelope(&wrapper);

    assert!(
        envelope.lower() == [2.0_f64, 1.0_f64],
        "envelope lower normalized"
    );
    assert!(
        envelope.upper() == [5.0_f64, 4.0_f64],
        "envelope upper normalized"
    );
}
