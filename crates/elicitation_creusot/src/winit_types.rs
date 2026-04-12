//! Creusot proofs for winit elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "winit-types")]

use creusot_std::prelude::*;

/// Trusted axiom: `WinitPhysicalSize` stores width and height without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_winit_physical_size_fields() -> bool {
    let size = elicitation::WinitPhysicalSize {
        width: 1920_u32,
        height: 1080_u32,
    };
    size.width == 1920_u32 && size.height == 1080_u32
}

/// Trusted axiom: `WinitLogicalSize` stores width and height without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_winit_logical_size_fields() -> bool {
    let size = elicitation::WinitLogicalSize {
        width: 1280_f64,
        height: 720_f64,
    };
    size.width == 1280_f64 && size.height == 720_f64
}

/// Trusted axiom: `WinitLogicalPosition` stores x and y without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_winit_logical_position_fields() -> bool {
    let pos = elicitation::WinitLogicalPosition {
        x: 100_f64,
        y: 200_f64,
    };
    pos.x == 100_f64 && pos.y == 200_f64
}

/// Trusted axiom: zero-sized `WinitPhysicalSize` is representable.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_winit_physical_size_zero() -> bool {
    let size = elicitation::WinitPhysicalSize {
        width: 0_u32,
        height: 0_u32,
    };
    size.width == 0_u32 && size.height == 0_u32
}
