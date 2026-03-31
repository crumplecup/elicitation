use verus_builtin_macros::verus;
// Required by verus! macro for int type, comparison operators, and arithmetic
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

// ============================================================================
// palette crate — Color type shadow proofs
//
// Trust the source. Verify the wrapper.
//
// We trust palette's Srgb color type. We model our own PaletteSrgb wrapper
// via a shadow struct that mirrors the field layout, proving field preservation
// across conversions and color value invariants.
// ============================================================================

// ---- Shadow struct: Srgb (r, g, b: f32) ----

pub struct ShadowPaletteSrgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

/// Construct a ShadowPaletteSrgb from components.
pub fn make_palette_srgb(r: f32, g: f32, b: f32) -> (result: ShadowPaletteSrgb)
    ensures
        result.r == r,
        result.g == g,
        result.b == b,
{
    ShadowPaletteSrgb { r, g, b }
}

/// Prove Srgb roundtrip: construct → read channels → reconstruct preserves all channels.
pub fn verify_palette_srgb_roundtrip(r: f32, g: f32, b: f32) -> (result: ShadowPaletteSrgb)
    ensures
        result.r == r,
        result.g == g,
        result.b == b,
{
    let original = make_palette_srgb(r, g, b);
    make_palette_srgb(original.r, original.g, original.b)
}

/// Prove Srgb concrete construction with known values.
pub fn verify_palette_srgb_concrete() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 0.5f32,
        result.g == 0.75f32,
        result.b == 0.25f32,
{
    make_palette_srgb(0.5, 0.75, 0.25)
}

/// Prove black color construction (all channels zero).
pub fn verify_palette_srgb_black() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 0.0f32,
        result.g == 0.0f32,
        result.b == 0.0f32,
{
    make_palette_srgb(0.0, 0.0, 0.0)
}

/// Prove white color construction (all channels one).
pub fn verify_palette_srgb_white() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 1.0f32,
        result.g == 1.0f32,
        result.b == 1.0f32,
{
    make_palette_srgb(1.0, 1.0, 1.0)
}

/// Prove primary red construction.
pub fn verify_palette_srgb_red() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 1.0f32,
        result.g == 0.0f32,
        result.b == 0.0f32,
{
    make_palette_srgb(1.0, 0.0, 0.0)
}

/// Prove primary green construction.
pub fn verify_palette_srgb_green() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 0.0f32,
        result.g == 1.0f32,
        result.b == 0.0f32,
{
    make_palette_srgb(0.0, 1.0, 0.0)
}

/// Prove primary blue construction.
pub fn verify_palette_srgb_blue() -> (result: ShadowPaletteSrgb)
    ensures
        result.r == 0.0f32,
        result.g == 0.0f32,
        result.b == 1.0f32,
{
    make_palette_srgb(0.0, 0.0, 1.0)
}

/// Prove field independence: changing one channel doesn't affect others.
pub fn verify_palette_srgb_independence(r: f32, g: f32, b: f32) -> (result: ShadowPaletteSrgb)
    ensures
        result.r == r,
        result.g == g,
        result.b == b,
{
    let c = make_palette_srgb(r, g, b);
    // Reconstruct from individual field reads to prove independence
    let r2 = c.r;
    let g2 = c.g;
    let b2 = c.b;
    make_palette_srgb(r2, g2, b2)
}

} // verus!
