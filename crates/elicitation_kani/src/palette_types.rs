//! Kani proofs for palette elicitation wrappers.
//!
//! Available with the `palette` feature.
//!
//! # Proof Strategy
//!
//! For PaletteSrgb we verify:
//! 1. **From roundtrip**: `PaletteSrgb::from(srgb)` then back preserves channels
//! 2. **Field preservation**: wrapper fields match original channel values
//! 3. **Channel clamping**: wrapper preserves extreme channel values (0.0 and 1.0)

// ── Srgb proofs ──────────────────────────────────────────────────────────────

#[cfg(feature = "palette")]
#[kani::proof]
fn verify_palette_srgb_from_roundtrip() {
    let r: f32 = kani::any();
    let g: f32 = kani::any();
    let b: f32 = kani::any();

    kani::assume!(r.is_finite());
    kani::assume!(g.is_finite());
    kani::assume!(b.is_finite());

    let original = palette::Srgb::new(r, g, b);
    let wrapper = elicitation::PaletteSrgb::from(original);
    let restored: palette::Srgb<f32> = wrapper.into();

    assert!(restored.red == r, "Srgb red preserved");
    assert!(restored.green == g, "Srgb green preserved");
    assert!(restored.blue == b, "Srgb blue preserved");
}

#[cfg(feature = "palette")]
#[kani::proof]
fn verify_palette_srgb_wrapper_fields() {
    let r: f32 = kani::any();
    let g: f32 = kani::any();
    let b: f32 = kani::any();

    kani::assume!(r.is_finite());
    kani::assume!(g.is_finite());
    kani::assume!(b.is_finite());

    let original = palette::Srgb::new(r, g, b);
    let wrapper = elicitation::PaletteSrgb::from(original);

    assert!(wrapper.r == r, "Srgb wrapper.r matches");
    assert!(wrapper.g == g, "Srgb wrapper.g matches");
    assert!(wrapper.b == b, "Srgb wrapper.b matches");
}

#[cfg(feature = "palette")]
#[kani::proof]
fn verify_palette_srgb_black_white() {
    // Black: all channels zero
    let black = palette::Srgb::new(0.0_f32, 0.0_f32, 0.0_f32);
    let bw = elicitation::PaletteSrgb::from(black);
    assert!(bw.r == 0.0_f32, "Black red is 0");
    assert!(bw.g == 0.0_f32, "Black green is 0");
    assert!(bw.b == 0.0_f32, "Black blue is 0");

    // White: all channels one
    let white = palette::Srgb::new(1.0_f32, 1.0_f32, 1.0_f32);
    let ww = elicitation::PaletteSrgb::from(white);
    assert!(ww.r == 1.0_f32, "White red is 1");
    assert!(ww.g == 1.0_f32, "White green is 1");
    assert!(ww.b == 1.0_f32, "White blue is 1");
}
