//! [`ElicitSpec`](crate::ElicitSpec) implementations for wgpu 29 GPU descriptor types.
//!
//! Available with the `wgpu-types` feature.

#[cfg(feature = "wgpu-types")]
mod wgpu_impls {
    use crate::{
        ElicitComplete, ElicitSpec, Select, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey, WgpuAddressMode, WgpuBackend, WgpuBlendFactor,
        WgpuBlendOperation, WgpuBufferUsages, WgpuColor, WgpuColorWrites,
        WgpuCompareFunctionSelect, WgpuCompositeAlphaMode, WgpuExtent3d, WgpuFace, WgpuFilterMode,
        WgpuFrontFace, WgpuIndexFormat, WgpuOrigin3d, WgpuPolygonMode, WgpuPowerPreference,
        WgpuPresentMode, WgpuPrimitiveTopology, WgpuSamplerBorderColor, WgpuShaderStages,
        WgpuStencilOperation, WgpuTextureDimension, WgpuTextureFormat, WgpuTextureUsages,
        WgpuTextureViewDimension, WgpuVertexFormat, WgpuVertexStepMode,
    };

    // -------------------------------------------------------------------------
    // Helper: build a Select-pattern TypeSpec from Select::labels()
    // -------------------------------------------------------------------------

    fn _wgpu_select_type_spec<T: Select>(name: &str, summary: &str) -> TypeSpec {
        let variants = SpecCategoryBuilder::default()
            .name("variants".to_string())
            .entries(
                T::labels()
                    .into_iter()
                    .map(|label| {
                        SpecEntryBuilder::default()
                            .label(label.clone())
                            .description(label)
                            .build()
                            .expect("valid SpecEntry")
                    })
                    .collect(),
            )
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("crate".to_string())
                    .description("wgpu v29 — cross-platform GPU API".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description("Select — choose one variant from the list".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name(name.to_string())
            .summary(summary.to_string())
            .categories(vec![variants, source])
            .build()
            .expect("valid TypeSpec")
    }

    // -------------------------------------------------------------------------
    // Macro: impl_wgpu_select_spec!
    // -------------------------------------------------------------------------

    macro_rules! impl_wgpu_select_spec {
        (
            type    = $ty:ty,
            wrapper = $wrapper:ty,
            name    = $name:literal,
            summary = $summary:literal
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _wgpu_select_type_spec::<$ty>($name, $summary)
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    <$ty as ElicitSpec>::type_spec()
                }
            }

            impl ElicitComplete for $wrapper {}
        };
    }

    // ── Select enum specs ─────────────────────────────────────────────────────

    impl_wgpu_select_spec!(
        type    = wgpu::TextureFormat,
        wrapper = WgpuTextureFormat,
        name    = "wgpu::TextureFormat",
        summary = "GPU texture pixel format — 49 core formats covering 8/16/32-bit \
                   single/multi-channel, packed, and depth/stencil variants."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::PresentMode,
        wrapper = WgpuPresentMode,
        name    = "wgpu::PresentMode",
        summary = "Surface present mode: AutoVsync, Fifo (vsync), Mailbox, Immediate, etc."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::PowerPreference,
        wrapper = WgpuPowerPreference,
        name    = "wgpu::PowerPreference",
        summary = "GPU power preference for adapter selection: None, LowPower, or HighPerformance."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::TextureDimension,
        wrapper = WgpuTextureDimension,
        name    = "wgpu::TextureDimension",
        summary = "Texture dimensionality: D1, D2, or D3."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::TextureViewDimension,
        wrapper = WgpuTextureViewDimension,
        name    = "wgpu::TextureViewDimension",
        summary = "Texture view dimensionality: D1, D2, D2Array, Cube, CubeArray, or D3."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::PrimitiveTopology,
        wrapper = WgpuPrimitiveTopology,
        name    = "wgpu::PrimitiveTopology",
        summary = "Primitive assembly topology: PointList, LineList/Strip, TriangleList/Strip."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::FrontFace,
        wrapper = WgpuFrontFace,
        name    = "wgpu::FrontFace",
        summary = "Front-face winding order: Ccw (counter-clockwise) or Cw."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::Face,
        wrapper = WgpuFace,
        name    = "wgpu::Face",
        summary = "Face culling target: Front or Back."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::PolygonMode,
        wrapper = WgpuPolygonMode,
        name    = "wgpu::PolygonMode",
        summary = "Polygon rasterization fill mode: Fill, Line, or Point."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::CompareFunction,
        wrapper = WgpuCompareFunctionSelect,
        name    = "wgpu::CompareFunction",
        summary = "Depth/stencil comparison function: Never, Less, Equal, LessEqual, etc."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::BlendFactor,
        wrapper = WgpuBlendFactor,
        name    = "wgpu::BlendFactor",
        summary = "Blend factor controlling source/destination weight in blend operations."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::BlendOperation,
        wrapper = WgpuBlendOperation,
        name    = "wgpu::BlendOperation",
        summary = "Blend arithmetic operation: Add, Subtract, ReverseSubtract, Min, or Max."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::IndexFormat,
        wrapper = WgpuIndexFormat,
        name    = "wgpu::IndexFormat",
        summary = "Index buffer element format: Uint16 or Uint32."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::StencilOperation,
        wrapper = WgpuStencilOperation,
        name    = "wgpu::StencilOperation",
        summary = "Stencil buffer operation on pass/fail: Keep, Zero, Replace, Invert, etc."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::VertexStepMode,
        wrapper = WgpuVertexStepMode,
        name    = "wgpu::VertexStepMode",
        summary = "Vertex buffer stepping mode: Vertex (per-vertex) or Instance (per-instance)."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::VertexFormat,
        wrapper = WgpuVertexFormat,
        name    = "wgpu::VertexFormat",
        summary = "Vertex attribute data format — 45 formats covering uint/sint/unorm/snorm/float \
                   in 8/16/32/64-bit widths and 1–4 component counts."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::AddressMode,
        wrapper = WgpuAddressMode,
        name    = "wgpu::AddressMode",
        summary = "Texture address (wrap) mode: ClampToEdge, Repeat, MirrorRepeat, ClampToBorder."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::FilterMode,
        wrapper = WgpuFilterMode,
        name    = "wgpu::FilterMode",
        summary = "Texture filter mode: Nearest (point sampling) or Linear (bilinear)."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::SamplerBorderColor,
        wrapper = WgpuSamplerBorderColor,
        name    = "wgpu::SamplerBorderColor",
        summary = "Border color used with ClampToBorder address mode."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::CompositeAlphaMode,
        wrapper = WgpuCompositeAlphaMode,
        name    = "wgpu::CompositeAlphaMode",
        summary = "Surface composite alpha mode controlling how the OS composites the window."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::Backend,
        wrapper = WgpuBackend,
        name    = "wgpu::Backend",
        summary = "GPU backend: Vulkan, Metal, Dx12, Gl, or BrowserWebGpu."
    );

    // ── Bitflag specs ─────────────────────────────────────────────────────────

    impl_wgpu_select_spec!(
        type    = wgpu::BufferUsages,
        wrapper = WgpuBufferUsages,
        name    = "wgpu::BufferUsages",
        summary = "Buffer usage flags: MapRead/Write, CopySrc/Dst, Index, Vertex, \
                   Uniform, Storage, Indirect, QueryResolve."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::TextureUsages,
        wrapper = WgpuTextureUsages,
        name    = "wgpu::TextureUsages",
        summary = "Texture usage flags: CopySrc/Dst, TextureBinding, StorageBinding, \
                   RenderAttachment."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::ShaderStages,
        wrapper = WgpuShaderStages,
        name    = "wgpu::ShaderStages",
        summary = "Shader stage visibility flags: None, Vertex, Fragment, Compute, \
                   or VertexFragment."
    );

    impl_wgpu_select_spec!(
        type    = wgpu::ColorWrites,
        wrapper = WgpuColorWrites,
        name    = "wgpu::ColorWrites",
        summary = "Color write mask flags: Red, Green, Blue, Alpha, Color (RGB), or All."
    );

    // ── Struct specs ──────────────────────────────────────────────────────────

    fn _wgpu_struct_fields_spec(name: &str, summary: &str, fields: Vec<(&str, &str)>) -> TypeSpec {
        let fields_cat = SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(
                fields
                    .into_iter()
                    .map(|(fname, fdesc)| {
                        SpecEntryBuilder::default()
                            .label(fname.to_string())
                            .description(fdesc.to_string())
                            .build()
                            .expect("valid SpecEntry")
                    })
                    .collect(),
            )
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name(name.to_string())
            .summary(summary.to_string())
            .categories(vec![fields_cat])
            .build()
            .expect("valid TypeSpec")
    }

    impl ElicitSpec for WgpuExtent3d {
        fn type_spec() -> TypeSpec {
            _wgpu_struct_fields_spec(
                "WgpuExtent3d",
                "3-D extent (width, height, depth/layers) for textures and copy regions.",
                vec![
                    ("width", "Width in texels (u32)"),
                    ("height", "Height in texels (u32)"),
                    ("depth_or_array_layers", "Depth or array layer count (u32)"),
                ],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WgpuExtent3d",
        <WgpuExtent3d as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WgpuExtent3d>
    ));

    impl ElicitComplete for WgpuExtent3d {}

    impl ElicitSpec for WgpuColor {
        fn type_spec() -> TypeSpec {
            _wgpu_struct_fields_spec(
                "WgpuColor",
                "RGBA color with f64 channels in [0.0, 1.0] for clear values and blend constants.",
                vec![
                    ("r", "Red channel (f64, 0.0–1.0)"),
                    ("g", "Green channel (f64, 0.0–1.0)"),
                    ("b", "Blue channel (f64, 0.0–1.0)"),
                    ("a", "Alpha channel (f64, 0.0=transparent, 1.0=opaque)"),
                ],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WgpuColor",
        <WgpuColor as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WgpuColor>
    ));

    impl ElicitComplete for WgpuColor {}

    impl ElicitSpec for WgpuOrigin3d {
        fn type_spec() -> TypeSpec {
            _wgpu_struct_fields_spec(
                "WgpuOrigin3d",
                "3-D texel origin (x, y, z offsets) for copy operations.",
                vec![
                    ("x", "X offset in texels (u32)"),
                    ("y", "Y offset in texels (u32)"),
                    ("z", "Z offset in texels (u32)"),
                ],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WgpuOrigin3d",
        <WgpuOrigin3d as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WgpuOrigin3d>
    ));

    impl ElicitComplete for WgpuOrigin3d {}
}
