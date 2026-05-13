//! Kani proofs for winit elicitation support.
//!
//! Available with the `winit-types` feature.

/// `WinitPhysicalSize` stores width and height without modification.
#[cfg(feature = "winit-types")]
#[kani::proof]
fn verify_winit_physical_size_fields() {
    let size = elicitation::WinitPhysicalSize {
        width: 1920_u32,
        height: 1080_u32,
    };
    assert!(size.width == 1920_u32, "width preserved");
    assert!(size.height == 1080_u32, "height preserved");
}

/// `WinitLogicalSize` stores width and height without modification.
#[cfg(feature = "winit-types")]
#[kani::proof]
fn verify_winit_logical_size_fields() {
    let size = elicitation::WinitLogicalSize {
        width: 1280_f64,
        height: 720_f64,
    };
    assert!(size.width == 1280_f64, "width preserved");
    assert!(size.height == 720_f64, "height preserved");
}

/// `WinitLogicalPosition` stores x and y coordinates without modification.
#[cfg(feature = "winit-types")]
#[kani::proof]
fn verify_winit_logical_position_fields() {
    let pos = elicitation::WinitLogicalPosition {
        x: 100_f64,
        y: 200_f64,
    };
    assert!(pos.x == 100_f64, "x preserved");
    assert!(pos.y == 200_f64, "y preserved");
}

/// Zero-sized `WinitPhysicalSize` is representable.
#[cfg(feature = "winit-types")]
#[kani::proof]
fn verify_winit_physical_size_zero() {
    let size = elicitation::WinitPhysicalSize {
        width: 0_u32,
        height: 0_u32,
    };
    assert!(size.width == 0_u32, "zero width");
    assert!(size.height == 0_u32, "zero height");
}
