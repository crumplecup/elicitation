//! Kani proofs for wgpu elicitation support.
//!
//! Available with the `wgpu-types` feature.

/// `WgpuExtent3d` stores width, height, and depth without modification.
#[cfg(feature = "wgpu-types")]
#[kani::proof]
fn verify_wgpu_extent3d_fields() {
    let e = elicitation::WgpuExtent3d {
        width: 1920_u32,
        height: 1080_u32,
        depth_or_array_layers: 1_u32,
    };
    assert!(e.width == 1920_u32, "width preserved");
    assert!(e.height == 1080_u32, "height preserved");
    assert!(e.depth_or_array_layers == 1_u32, "depth preserved");
}

/// `WgpuExtent3d` with zero dimensions is representable.
#[cfg(feature = "wgpu-types")]
#[kani::proof]
fn verify_wgpu_extent3d_zero() {
    let e = elicitation::WgpuExtent3d {
        width: 0_u32,
        height: 0_u32,
        depth_or_array_layers: 0_u32,
    };
    assert!(e.width == 0_u32);
    assert!(e.height == 0_u32);
    assert!(e.depth_or_array_layers == 0_u32);
}

/// `WgpuColor` stores r, g, b, a channels without modification.
#[cfg(feature = "wgpu-types")]
#[kani::proof]
fn verify_wgpu_color_fields() {
    let c = elicitation::WgpuColor {
        r: 1.0_f64,
        g: 0.5_f64,
        b: 0.0_f64,
        a: 1.0_f64,
    };
    assert!(c.r == 1.0_f64, "r preserved");
    assert!(c.g == 0.5_f64, "g preserved");
    assert!(c.b == 0.0_f64, "b preserved");
    assert!(c.a == 1.0_f64, "a preserved");
}

/// `WgpuOrigin3d` stores x, y, z coordinates without modification.
#[cfg(feature = "wgpu-types")]
#[kani::proof]
fn verify_wgpu_origin3d_fields() {
    let o = elicitation::WgpuOrigin3d {
        x: 10_u32,
        y: 20_u32,
        z: 30_u32,
    };
    assert!(o.x == 10_u32, "x preserved");
    assert!(o.y == 20_u32, "y preserved");
    assert!(o.z == 30_u32, "z preserved");
}
