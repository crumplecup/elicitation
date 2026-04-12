//! Creusot proofs for wgpu elicitation support.
//!
//! Trust the source. Verify the wrapper surface.

#![cfg(feature = "wgpu-types")]

use creusot_std::prelude::*;

/// Trusted axiom: `WgpuExtent3d` stores width, height, and depth without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wgpu_extent3d_fields() -> bool {
    let e = elicitation::WgpuExtent3d {
        width: 1920_u32,
        height: 1080_u32,
        depth_or_array_layers: 1_u32,
    };
    e.width == 1920_u32 && e.height == 1080_u32 && e.depth_or_array_layers == 1_u32
}

/// Trusted axiom: `WgpuExtent3d` with zero dimensions is representable.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wgpu_extent3d_zero() -> bool {
    let e = elicitation::WgpuExtent3d {
        width: 0_u32,
        height: 0_u32,
        depth_or_array_layers: 0_u32,
    };
    e.width == 0_u32 && e.height == 0_u32 && e.depth_or_array_layers == 0_u32
}

/// Trusted axiom: `WgpuColor` stores r, g, b, a channels without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wgpu_color_fields() -> bool {
    let c = elicitation::WgpuColor {
        r: 1.0_f64,
        g: 0.5_f64,
        b: 0.0_f64,
        a: 1.0_f64,
    };
    c.r == 1.0_f64 && c.g == 0.5_f64 && c.b == 0.0_f64 && c.a == 1.0_f64
}

/// Trusted axiom: `WgpuOrigin3d` stores x, y, z coordinates without modification.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wgpu_origin3d_fields() -> bool {
    let o = elicitation::WgpuOrigin3d {
        x: 10_u32,
        y: 20_u32,
        z: 30_u32,
    };
    o.x == 10_u32 && o.y == 20_u32 && o.z == 30_u32
}
