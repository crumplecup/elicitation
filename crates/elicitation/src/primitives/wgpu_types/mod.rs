//! Elicitation implementations for wgpu 29 GPU descriptor types.
//!
//! Available with the `wgpu-types` feature.

mod enums;
mod structs;
mod trenchcoats;

pub use structs::{WgpuColor, WgpuExtent3d, WgpuOrigin3d};
pub use trenchcoats::{
    WgpuAddressMode, WgpuBackend, WgpuBlendFactor, WgpuBlendOperation, WgpuBufferUsages,
    WgpuColorWrites, WgpuCompareFunctionSelect, WgpuCompositeAlphaMode, WgpuFace, WgpuFilterMode,
    WgpuFrontFace, WgpuIndexFormat, WgpuPolygonMode, WgpuPowerPreference, WgpuPresentMode,
    WgpuPrimitiveTopology, WgpuSamplerBorderColor, WgpuShaderStages, WgpuStencilOperation,
    WgpuTextureDimension, WgpuTextureFormat, WgpuTextureUsages, WgpuTextureViewDimension,
    WgpuVertexFormat, WgpuVertexStepMode,
};
