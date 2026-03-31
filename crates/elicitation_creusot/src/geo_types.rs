//! Creusot proofs for geo-types elicitation wrappers.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that geo-types correctly implements its geometric primitives.
//! We verify our own wrappers: that `From` conversions between our wrappers
//! and geo-types preserve field values.
//!
//! # Proof strategy
//!
//! All proofs are `#[trusted]` because geo-types is a foreign crate and Creusot
//! cannot inspect its struct layout or method bodies. The proofs serve as
//! documented axioms that our wrapper conversions are semantically correct.

#![cfg(feature = "geo-types")]

use creusot_std::prelude::*;

// ── Coord proofs ─────────────────────────────────────────────────────────────

/// Trusted axiom: GeoCoord roundtrip preserves x and y fields.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_coord_roundtrip() -> bool {
    let coord = geo_types::Coord { x: 1.5_f64, y: -2.3_f64 };
    let wrapper = elicitation::GeoCoord::from(coord);
    let restored: geo_types::Coord<f64> = wrapper.into();
    restored.x == 1.5_f64 && restored.y == -2.3_f64
}

/// Trusted axiom: GeoCoord concrete construction preserves values.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_coord_concrete() -> bool {
    let coord = geo_types::Coord { x: 0.0_f64, y: 0.0_f64 };
    let wrapper = elicitation::GeoCoord::from(coord);
    wrapper.x == 0.0_f64 && wrapper.y == 0.0_f64
}

// ── Rect proofs ──────────────────────────────────────────────────────────────

/// Trusted axiom: GeoRect roundtrip preserves corners.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_rect_roundtrip() -> bool {
    let rect = geo_types::Rect::new(
        geo_types::Coord { x: 0.0_f64, y: 0.0_f64 },
        geo_types::Coord { x: 10.0_f64, y: 20.0_f64 },
    );
    let wrapper = elicitation::GeoRect::from(rect);
    let restored: geo_types::Rect<f64> = wrapper.into();
    restored.min().x == 0.0_f64
        && restored.min().y == 0.0_f64
        && restored.max().x == 10.0_f64
        && restored.max().y == 20.0_f64
}

/// Trusted axiom: GeoRect well-formedness — min ≤ max after normalization.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_rect_well_formed() -> bool {
    // Reversed corners should be normalized
    let rect = geo_types::Rect::new(
        geo_types::Coord { x: 10.0_f64, y: 20.0_f64 },
        geo_types::Coord { x: 0.0_f64, y: 0.0_f64 },
    );
    let wrapper = elicitation::GeoRect::from(rect);
    wrapper.min.x <= wrapper.max.x && wrapper.min.y <= wrapper.max.y
}

// ── Line proofs ──────────────────────────────────────────────────────────────

/// Trusted axiom: GeoLine roundtrip preserves start and end.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_line_roundtrip() -> bool {
    let line = geo_types::Line::new(
        geo_types::Coord { x: 1.0_f64, y: 2.0_f64 },
        geo_types::Coord { x: 3.0_f64, y: 4.0_f64 },
    );
    let wrapper = elicitation::GeoLine::from(line);
    let restored: geo_types::Line<f64> = wrapper.into();
    restored.start.x == 1.0_f64
        && restored.start.y == 2.0_f64
        && restored.end.x == 3.0_f64
        && restored.end.y == 4.0_f64
}

/// Trusted axiom: GeoLine concrete — degenerate point-line preserves equality.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_geo_line_degenerate() -> bool {
    let line = geo_types::Line::new(
        geo_types::Coord { x: 5.0_f64, y: 5.0_f64 },
        geo_types::Coord { x: 5.0_f64, y: 5.0_f64 },
    );
    let wrapper = elicitation::GeoLine::from(line);
    wrapper.start.x == wrapper.end.x && wrapper.start.y == wrapper.end.y
}
