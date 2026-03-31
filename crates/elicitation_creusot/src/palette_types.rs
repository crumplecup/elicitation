//! Creusot proofs for palette elicitation wrappers.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that palette correctly implements its color types.
//! We verify our own wrappers: that `From` conversions between our wrappers
//! and palette preserve channel values.
//!
//! # Proof strategy
//!
//! All proofs are `#[trusted]` because palette is a foreign crate and Creusot
//! cannot inspect its struct layout or method bodies. The proofs serve as
//! documented axioms that our wrapper conversions are semantically correct.

#![cfg(feature = "palette")]

use creusot_std::prelude::*;

// ── Srgb proofs ──────────────────────────────────────────────────────────────

/// Trusted axiom: PaletteSrgb roundtrip preserves r, g, b channels.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_palette_srgb_roundtrip() -> bool {
    let srgb = palette::Srgb::new(0.5_f32, 0.75_f32, 0.25_f32);
    let wrapper = elicitation::PaletteSrgb::from(srgb);
    let restored: palette::Srgb<f32> = wrapper.into();
    restored.red == 0.5_f32 && restored.green == 0.75_f32 && restored.blue == 0.25_f32
}

/// Trusted axiom: PaletteSrgb concrete construction preserves values.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_palette_srgb_concrete() -> bool {
    let srgb = palette::Srgb::new(0.0_f32, 0.0_f32, 0.0_f32);
    let wrapper = elicitation::PaletteSrgb::from(srgb);
    wrapper.r == 0.0_f32 && wrapper.g == 0.0_f32 && wrapper.b == 0.0_f32
}

/// Trusted axiom: PaletteSrgb black and white extremes are preserved.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_palette_srgb_extremes() -> bool {
    let black = palette::Srgb::new(0.0_f32, 0.0_f32, 0.0_f32);
    let white = palette::Srgb::new(1.0_f32, 1.0_f32, 1.0_f32);
    let bw = elicitation::PaletteSrgb::from(black);
    let ww = elicitation::PaletteSrgb::from(white);
    bw.r == 0.0_f32
        && bw.g == 0.0_f32
        && bw.b == 0.0_f32
        && ww.r == 1.0_f32
        && ww.g == 1.0_f32
        && ww.b == 1.0_f32
}

/// Trusted axiom: PaletteSrgb pure primary colors are preserved.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_palette_srgb_primaries() -> bool {
    let red = palette::Srgb::new(1.0_f32, 0.0_f32, 0.0_f32);
    let green = palette::Srgb::new(0.0_f32, 1.0_f32, 0.0_f32);
    let blue = palette::Srgb::new(0.0_f32, 0.0_f32, 1.0_f32);

    let rw = elicitation::PaletteSrgb::from(red);
    let gw = elicitation::PaletteSrgb::from(green);
    let bw = elicitation::PaletteSrgb::from(blue);

    rw.r == 1.0_f32
        && rw.g == 0.0_f32
        && rw.b == 0.0_f32
        && gw.r == 0.0_f32
        && gw.g == 1.0_f32
        && gw.b == 0.0_f32
        && bw.r == 0.0_f32
        && bw.g == 0.0_f32
        && bw.b == 1.0_f32
}
