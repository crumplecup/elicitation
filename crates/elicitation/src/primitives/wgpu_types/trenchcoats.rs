//! `select_trenchcoat!` wrappers for wgpu 29 enum and bitflag types.
//!
//! All inner types have `serde` support (via `wgpu/serde`), so the `serde`
//! delegation form is used throughout.  All wgpu enums and bitflags implement
//! `Copy`, `Eq`, and `Hash`, so all wrappers use `[copy, eq, hash]`.

// ── Pure enums ────────────────────────────────────────────────────────────────

crate::select_trenchcoat!(wgpu::TextureFormat, as WgpuTextureFormat, serde);
crate::select_trenchcoat_traits!(WgpuTextureFormat, wgpu::TextureFormat, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::PresentMode, as WgpuPresentMode, serde);
crate::select_trenchcoat_traits!(WgpuPresentMode, wgpu::PresentMode, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::PowerPreference, as WgpuPowerPreference, serde);
crate::select_trenchcoat_traits!(WgpuPowerPreference, wgpu::PowerPreference, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::TextureDimension, as WgpuTextureDimension, serde);
crate::select_trenchcoat_traits!(
    WgpuTextureDimension,
    wgpu::TextureDimension,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::TextureViewDimension, as WgpuTextureViewDimension, serde);
crate::select_trenchcoat_traits!(
    WgpuTextureViewDimension,
    wgpu::TextureViewDimension,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::PrimitiveTopology, as WgpuPrimitiveTopology, serde);
crate::select_trenchcoat_traits!(
    WgpuPrimitiveTopology,
    wgpu::PrimitiveTopology,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::FrontFace, as WgpuFrontFace, serde);
crate::select_trenchcoat_traits!(WgpuFrontFace, wgpu::FrontFace, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::Face, as WgpuFace, serde);
crate::select_trenchcoat_traits!(WgpuFace, wgpu::Face, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::PolygonMode, as WgpuPolygonMode, serde);
crate::select_trenchcoat_traits!(WgpuPolygonMode, wgpu::PolygonMode, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::CompareFunction, as WgpuCompareFunctionSelect, serde);
crate::select_trenchcoat_traits!(
    WgpuCompareFunctionSelect,
    wgpu::CompareFunction,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::BlendFactor, as WgpuBlendFactor, serde);
crate::select_trenchcoat_traits!(WgpuBlendFactor, wgpu::BlendFactor, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::BlendOperation, as WgpuBlendOperation, serde);
crate::select_trenchcoat_traits!(WgpuBlendOperation, wgpu::BlendOperation, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::IndexFormat, as WgpuIndexFormat, serde);
crate::select_trenchcoat_traits!(WgpuIndexFormat, wgpu::IndexFormat, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::StencilOperation, as WgpuStencilOperation, serde);
crate::select_trenchcoat_traits!(
    WgpuStencilOperation,
    wgpu::StencilOperation,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::VertexStepMode, as WgpuVertexStepMode, serde);
crate::select_trenchcoat_traits!(WgpuVertexStepMode, wgpu::VertexStepMode, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::VertexFormat, as WgpuVertexFormat, serde);
crate::select_trenchcoat_traits!(WgpuVertexFormat, wgpu::VertexFormat, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::AddressMode, as WgpuAddressMode, serde);
crate::select_trenchcoat_traits!(WgpuAddressMode, wgpu::AddressMode, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::FilterMode, as WgpuFilterMode, serde);
crate::select_trenchcoat_traits!(WgpuFilterMode, wgpu::FilterMode, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::SamplerBorderColor, as WgpuSamplerBorderColor, serde);
crate::select_trenchcoat_traits!(
    WgpuSamplerBorderColor,
    wgpu::SamplerBorderColor,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::CompositeAlphaMode, as WgpuCompositeAlphaMode, serde);
crate::select_trenchcoat_traits!(
    WgpuCompositeAlphaMode,
    wgpu::CompositeAlphaMode,
    [copy, eq, hash]
);

crate::select_trenchcoat!(wgpu::Backend, as WgpuBackend, serde);
crate::select_trenchcoat_traits!(WgpuBackend, wgpu::Backend, [copy, eq, hash]);

// ── Bitflag types ─────────────────────────────────────────────────────────────
//
// wgpu bitflags also implement Copy/Eq/Hash and have serde support.

crate::select_trenchcoat!(wgpu::BufferUsages, as WgpuBufferUsages, serde);
crate::select_trenchcoat_traits!(WgpuBufferUsages, wgpu::BufferUsages, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::TextureUsages, as WgpuTextureUsages, serde);
crate::select_trenchcoat_traits!(WgpuTextureUsages, wgpu::TextureUsages, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::ShaderStages, as WgpuShaderStages, serde);
crate::select_trenchcoat_traits!(WgpuShaderStages, wgpu::ShaderStages, [copy, eq, hash]);

crate::select_trenchcoat!(wgpu::ColorWrites, as WgpuColorWrites, serde);
crate::select_trenchcoat_traits!(WgpuColorWrites, wgpu::ColorWrites, [copy, eq, hash]);
