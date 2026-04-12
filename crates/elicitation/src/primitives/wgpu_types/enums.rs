//! `Prompt`, `Select`, `Elicitation`, and `ElicitIntrospect` impls for wgpu 29
//! enum and bitflag types.
//!
//! All inner types here have `serde` support (enabled via `wgpu/serde`), so
//! `labels()` and `from_label()` delegate to serde round-trips.  Only
//! `options()` enumerates the concrete variants.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

// ── Local helper macro ────────────────────────────────────────────────────────
//
// `impl_wgpu_select!` generates all four traits for a wgpu enum that has serde.
//
// Parameters:
//   $ty         — full path to the wgpu type (e.g. `wgpu::TextureFormat`)
//   $style      — identifier for the generated style enum (e.g. `TextureFormatStyle`)
//   $prompt     — string literal displayed to the user
//   $kani_var   — first-variant path for Kani/Verus/Creusot proofs (e.g. `wgpu::TextureFormat::Rgba8Unorm`)
//   $variants   — comma-separated list of variant expressions
//
// The macro is intentionally placed here (not in a public macro module) because
// it is tightly coupled to this feature gate.

macro_rules! impl_wgpu_select {
    (
        type       = $ty:ty,
        style      = $style:ident,
        prompt     = $prompt:literal,
        kani_var   = $kani_var:expr,
        variants   = [ $($v:expr),+ $(,)? ]
    ) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Select for $ty {
            fn options() -> Vec<Self> {
                vec![$($v),+]
            }

            fn labels() -> Vec<String> {
                Self::options()
                    .iter()
                    .map(|v| {
                        serde_json::to_string(v)
                            .unwrap()
                            .trim_matches('"')
                            .to_string()
                    })
                    .collect()
            }

            fn from_label(label: &str) -> Option<Self> {
                serde_json::from_str(&format!("\"{}\"", label)).ok()
            }
        }

        crate::default_style!($ty => $style);

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(ty = stringify!($ty), "Eliciting wgpu type");
                let params = mcp::select_params(
                    Self::prompt().unwrap_or("Choose a value:"),
                    &Self::labels(),
                );
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                            .with_arguments(params),
                    )
                    .await?;
                let value = mcp::extract_value(result)?;
                let label = mcp::parse_string(value)?;
                Self::from_label(&label).ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid {}: {label}",
                        stringify!($ty)
                    )))
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_select_wrapper(
                    stringify!($ty),
                    stringify!($kani_var),
                )
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_select_wrapper(
                    stringify!($ty),
                    stringify!($kani_var),
                )
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_select_wrapper(
                    stringify!($ty),
                    stringify!($kani_var),
                )
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Select
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($ty),
                    description: Self::prompt(),
                    details: PatternDetails::Select {
                        variants: Self::labels()
                            .into_iter()
                            .map(|label| VariantMetadata {
                                label,
                                fields: vec![],
                            })
                            .collect(),
                    },
                }
            }
        }
    };
}

// ── TextureFormat ─────────────────────────────────────────────────────────────
// Common texture pixel formats.  Only the universally supported core formats
// are included; compressed and HDR formats are omitted for brevity.
impl_wgpu_select! {
    type     = wgpu::TextureFormat,
    style    = TextureFormatStyle,
    prompt   = "Choose a texture pixel format:",
    kani_var = wgpu::TextureFormat::Rgba8Unorm,
    variants = [
        // 8-bit single-channel
        wgpu::TextureFormat::R8Unorm,
        wgpu::TextureFormat::R8Snorm,
        wgpu::TextureFormat::R8Uint,
        wgpu::TextureFormat::R8Sint,
        // 16-bit single-channel
        wgpu::TextureFormat::R16Uint,
        wgpu::TextureFormat::R16Sint,
        wgpu::TextureFormat::R16Unorm,
        wgpu::TextureFormat::R16Snorm,
        wgpu::TextureFormat::R16Float,
        // 8-bit two-channel
        wgpu::TextureFormat::Rg8Unorm,
        wgpu::TextureFormat::Rg8Snorm,
        wgpu::TextureFormat::Rg8Uint,
        wgpu::TextureFormat::Rg8Sint,
        // 32-bit single-channel
        wgpu::TextureFormat::R32Uint,
        wgpu::TextureFormat::R32Sint,
        wgpu::TextureFormat::R32Float,
        // 16-bit two-channel
        wgpu::TextureFormat::Rg16Uint,
        wgpu::TextureFormat::Rg16Sint,
        wgpu::TextureFormat::Rg16Unorm,
        wgpu::TextureFormat::Rg16Snorm,
        wgpu::TextureFormat::Rg16Float,
        // 8-bit four-channel (RGBA)
        wgpu::TextureFormat::Rgba8Unorm,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureFormat::Rgba8Snorm,
        wgpu::TextureFormat::Rgba8Uint,
        wgpu::TextureFormat::Rgba8Sint,
        // 8-bit four-channel (BGRA)
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Bgra8UnormSrgb,
        // Packed
        wgpu::TextureFormat::Rgb10a2Uint,
        wgpu::TextureFormat::Rgb10a2Unorm,
        wgpu::TextureFormat::Rg11b10Ufloat,
        wgpu::TextureFormat::Rgb9e5Ufloat,
        // 32-bit two-channel
        wgpu::TextureFormat::Rg32Uint,
        wgpu::TextureFormat::Rg32Sint,
        wgpu::TextureFormat::Rg32Float,
        // 16-bit four-channel
        wgpu::TextureFormat::Rgba16Uint,
        wgpu::TextureFormat::Rgba16Sint,
        wgpu::TextureFormat::Rgba16Unorm,
        wgpu::TextureFormat::Rgba16Snorm,
        wgpu::TextureFormat::Rgba16Float,
        // 32-bit four-channel
        wgpu::TextureFormat::Rgba32Uint,
        wgpu::TextureFormat::Rgba32Sint,
        wgpu::TextureFormat::Rgba32Float,
        // Depth / stencil
        wgpu::TextureFormat::Stencil8,
        wgpu::TextureFormat::Depth16Unorm,
        wgpu::TextureFormat::Depth24Plus,
        wgpu::TextureFormat::Depth24PlusStencil8,
        wgpu::TextureFormat::Depth32Float,
        wgpu::TextureFormat::Depth32FloatStencil8,
    ]
}

// ── PresentMode ───────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::PresentMode,
    style    = PresentModeStyle,
    prompt   = "Choose a surface present mode:",
    kani_var = wgpu::PresentMode::Fifo,
    variants = [
        wgpu::PresentMode::AutoVsync,
        wgpu::PresentMode::AutoNoVsync,
        wgpu::PresentMode::Fifo,
        wgpu::PresentMode::FifoRelaxed,
        wgpu::PresentMode::Immediate,
        wgpu::PresentMode::Mailbox,
    ]
}

// ── PowerPreference ───────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::PowerPreference,
    style    = PowerPreferenceStyle,
    prompt   = "Choose a GPU power preference:",
    kani_var = wgpu::PowerPreference::None,
    variants = [
        wgpu::PowerPreference::None,
        wgpu::PowerPreference::LowPower,
        wgpu::PowerPreference::HighPerformance,
    ]
}

// ── TextureDimension ──────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::TextureDimension,
    style    = TextureDimensionStyle,
    prompt   = "Choose a texture dimensionality:",
    kani_var = wgpu::TextureDimension::D2,
    variants = [
        wgpu::TextureDimension::D1,
        wgpu::TextureDimension::D2,
        wgpu::TextureDimension::D3,
    ]
}

// ── TextureViewDimension ──────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::TextureViewDimension,
    style    = TextureViewDimensionStyle,
    prompt   = "Choose a texture view dimensionality:",
    kani_var = wgpu::TextureViewDimension::D2,
    variants = [
        wgpu::TextureViewDimension::D1,
        wgpu::TextureViewDimension::D2,
        wgpu::TextureViewDimension::D2Array,
        wgpu::TextureViewDimension::Cube,
        wgpu::TextureViewDimension::CubeArray,
        wgpu::TextureViewDimension::D3,
    ]
}

// ── PrimitiveTopology ─────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::PrimitiveTopology,
    style    = PrimitiveTopologyStyle,
    prompt   = "Choose a primitive topology:",
    kani_var = wgpu::PrimitiveTopology::TriangleList,
    variants = [
        wgpu::PrimitiveTopology::PointList,
        wgpu::PrimitiveTopology::LineList,
        wgpu::PrimitiveTopology::LineStrip,
        wgpu::PrimitiveTopology::TriangleList,
        wgpu::PrimitiveTopology::TriangleStrip,
    ]
}

// ── FrontFace ─────────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::FrontFace,
    style    = FrontFaceStyle,
    prompt   = "Choose front-face winding order (Ccw = counter-clockwise):",
    kani_var = wgpu::FrontFace::Ccw,
    variants = [
        wgpu::FrontFace::Ccw,
        wgpu::FrontFace::Cw,
    ]
}

// ── Face (cull mode) ──────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::Face,
    style    = FaceStyle,
    prompt   = "Choose which face to cull:",
    kani_var = wgpu::Face::Back,
    variants = [
        wgpu::Face::Front,
        wgpu::Face::Back,
    ]
}

// ── PolygonMode ───────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::PolygonMode,
    style    = PolygonModeStyle,
    prompt   = "Choose polygon fill mode:",
    kani_var = wgpu::PolygonMode::Fill,
    variants = [
        wgpu::PolygonMode::Fill,
        wgpu::PolygonMode::Line,
        wgpu::PolygonMode::Point,
    ]
}

// ── CompareFunction ───────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::CompareFunction,
    style    = CompareFunctionStyle,
    prompt   = "Choose a depth/stencil comparison function:",
    kani_var = wgpu::CompareFunction::Less,
    variants = [
        wgpu::CompareFunction::Never,
        wgpu::CompareFunction::Less,
        wgpu::CompareFunction::Equal,
        wgpu::CompareFunction::LessEqual,
        wgpu::CompareFunction::Greater,
        wgpu::CompareFunction::NotEqual,
        wgpu::CompareFunction::GreaterEqual,
        wgpu::CompareFunction::Always,
    ]
}

// ── BlendFactor ───────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::BlendFactor,
    style    = BlendFactorStyle,
    prompt   = "Choose a blend factor:",
    kani_var = wgpu::BlendFactor::One,
    variants = [
        wgpu::BlendFactor::Zero,
        wgpu::BlendFactor::One,
        wgpu::BlendFactor::Src,
        wgpu::BlendFactor::OneMinusSrc,
        wgpu::BlendFactor::SrcAlpha,
        wgpu::BlendFactor::OneMinusSrcAlpha,
        wgpu::BlendFactor::Dst,
        wgpu::BlendFactor::OneMinusDst,
        wgpu::BlendFactor::DstAlpha,
        wgpu::BlendFactor::OneMinusDstAlpha,
        wgpu::BlendFactor::SrcAlphaSaturated,
        wgpu::BlendFactor::Constant,
        wgpu::BlendFactor::OneMinusConstant,
        wgpu::BlendFactor::Src1,
        wgpu::BlendFactor::OneMinusSrc1,
        wgpu::BlendFactor::Src1Alpha,
        wgpu::BlendFactor::OneMinusSrc1Alpha,
    ]
}

// ── BlendOperation ────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::BlendOperation,
    style    = BlendOperationStyle,
    prompt   = "Choose a blend operation:",
    kani_var = wgpu::BlendOperation::Add,
    variants = [
        wgpu::BlendOperation::Add,
        wgpu::BlendOperation::Subtract,
        wgpu::BlendOperation::ReverseSubtract,
        wgpu::BlendOperation::Min,
        wgpu::BlendOperation::Max,
    ]
}

// ── IndexFormat ───────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::IndexFormat,
    style    = IndexFormatStyle,
    prompt   = "Choose an index buffer format:",
    kani_var = wgpu::IndexFormat::Uint16,
    variants = [
        wgpu::IndexFormat::Uint16,
        wgpu::IndexFormat::Uint32,
    ]
}

// ── StencilOperation ─────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::StencilOperation,
    style    = StencilOperationStyle,
    prompt   = "Choose a stencil operation:",
    kani_var = wgpu::StencilOperation::Keep,
    variants = [
        wgpu::StencilOperation::Keep,
        wgpu::StencilOperation::Zero,
        wgpu::StencilOperation::Replace,
        wgpu::StencilOperation::Invert,
        wgpu::StencilOperation::IncrementClamp,
        wgpu::StencilOperation::DecrementClamp,
        wgpu::StencilOperation::IncrementWrap,
        wgpu::StencilOperation::DecrementWrap,
    ]
}

// ── VertexStepMode ────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::VertexStepMode,
    style    = VertexStepModeStyle,
    prompt   = "Choose vertex step mode:",
    kani_var = wgpu::VertexStepMode::Vertex,
    variants = [
        wgpu::VertexStepMode::Vertex,
        wgpu::VertexStepMode::Instance,
    ]
}

// ── VertexFormat ─────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::VertexFormat,
    style    = VertexFormatStyle,
    prompt   = "Choose a vertex attribute format:",
    kani_var = wgpu::VertexFormat::Float32x4,
    variants = [
        wgpu::VertexFormat::Uint8,
        wgpu::VertexFormat::Uint8x2,
        wgpu::VertexFormat::Uint8x4,
        wgpu::VertexFormat::Sint8,
        wgpu::VertexFormat::Sint8x2,
        wgpu::VertexFormat::Sint8x4,
        wgpu::VertexFormat::Unorm8,
        wgpu::VertexFormat::Unorm8x2,
        wgpu::VertexFormat::Unorm8x4,
        wgpu::VertexFormat::Snorm8,
        wgpu::VertexFormat::Snorm8x2,
        wgpu::VertexFormat::Snorm8x4,
        wgpu::VertexFormat::Uint16,
        wgpu::VertexFormat::Uint16x2,
        wgpu::VertexFormat::Uint16x4,
        wgpu::VertexFormat::Sint16,
        wgpu::VertexFormat::Sint16x2,
        wgpu::VertexFormat::Sint16x4,
        wgpu::VertexFormat::Unorm16,
        wgpu::VertexFormat::Unorm16x2,
        wgpu::VertexFormat::Unorm16x4,
        wgpu::VertexFormat::Snorm16,
        wgpu::VertexFormat::Snorm16x2,
        wgpu::VertexFormat::Snorm16x4,
        wgpu::VertexFormat::Float16,
        wgpu::VertexFormat::Float16x2,
        wgpu::VertexFormat::Float16x4,
        wgpu::VertexFormat::Float32,
        wgpu::VertexFormat::Float32x2,
        wgpu::VertexFormat::Float32x3,
        wgpu::VertexFormat::Float32x4,
        wgpu::VertexFormat::Uint32,
        wgpu::VertexFormat::Uint32x2,
        wgpu::VertexFormat::Uint32x3,
        wgpu::VertexFormat::Uint32x4,
        wgpu::VertexFormat::Sint32,
        wgpu::VertexFormat::Sint32x2,
        wgpu::VertexFormat::Sint32x3,
        wgpu::VertexFormat::Sint32x4,
        wgpu::VertexFormat::Float64,
        wgpu::VertexFormat::Float64x2,
        wgpu::VertexFormat::Float64x3,
        wgpu::VertexFormat::Float64x4,
        wgpu::VertexFormat::Unorm10_10_10_2,
        wgpu::VertexFormat::Unorm8x4Bgra,
    ]
}

// ── AddressMode ───────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::AddressMode,
    style    = AddressModeStyle,
    prompt   = "Choose a texture address (wrap) mode:",
    kani_var = wgpu::AddressMode::ClampToEdge,
    variants = [
        wgpu::AddressMode::ClampToEdge,
        wgpu::AddressMode::Repeat,
        wgpu::AddressMode::MirrorRepeat,
        wgpu::AddressMode::ClampToBorder,
    ]
}

// ── FilterMode ────────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::FilterMode,
    style    = FilterModeStyle,
    prompt   = "Choose a texture filter mode:",
    kani_var = wgpu::FilterMode::Nearest,
    variants = [
        wgpu::FilterMode::Nearest,
        wgpu::FilterMode::Linear,
    ]
}

// ── SamplerBorderColor ────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::SamplerBorderColor,
    style    = SamplerBorderColorStyle,
    prompt   = "Choose a sampler border color (used with ClampToBorder address mode):",
    kani_var = wgpu::SamplerBorderColor::TransparentBlack,
    variants = [
        wgpu::SamplerBorderColor::TransparentBlack,
        wgpu::SamplerBorderColor::OpaqueBlack,
        wgpu::SamplerBorderColor::OpaqueWhite,
        wgpu::SamplerBorderColor::Zero,
    ]
}

// ── CompositeAlphaMode ────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::CompositeAlphaMode,
    style    = CompositeAlphaModeStyle,
    prompt   = "Choose a surface composite alpha mode:",
    kani_var = wgpu::CompositeAlphaMode::Auto,
    variants = [
        wgpu::CompositeAlphaMode::Auto,
        wgpu::CompositeAlphaMode::Opaque,
        wgpu::CompositeAlphaMode::PreMultiplied,
        wgpu::CompositeAlphaMode::PostMultiplied,
        wgpu::CompositeAlphaMode::Inherit,
    ]
}

// ── Backend ───────────────────────────────────────────────────────────────────
impl_wgpu_select! {
    type     = wgpu::Backend,
    style    = BackendStyle,
    prompt   = "Choose a GPU backend:",
    kani_var = wgpu::Backend::Vulkan,
    variants = [
        wgpu::Backend::Vulkan,
        wgpu::Backend::Metal,
        wgpu::Backend::Dx12,
        wgpu::Backend::Gl,
        wgpu::Backend::BrowserWebGpu,
    ]
}

// ── Bitflag types ─────────────────────────────────────────────────────────────
//
// wgpu bitflags types serialize as their debug string (e.g. "UNIFORM | COPY_DST").
// We treat them as Select enums at the individual-flag level — users pick one
// flag at a time through the elicitation tool, and OR them together in code.
//
// `from_label` parses the uppercase flag name.

impl_wgpu_select! {
    type     = wgpu::BufferUsages,
    style    = BufferUsagesStyle,
    prompt   = "Choose a buffer usage flag:",
    kani_var = wgpu::BufferUsages::UNIFORM,
    variants = [
        wgpu::BufferUsages::MAP_READ,
        wgpu::BufferUsages::MAP_WRITE,
        wgpu::BufferUsages::COPY_SRC,
        wgpu::BufferUsages::COPY_DST,
        wgpu::BufferUsages::INDEX,
        wgpu::BufferUsages::VERTEX,
        wgpu::BufferUsages::UNIFORM,
        wgpu::BufferUsages::STORAGE,
        wgpu::BufferUsages::INDIRECT,
        wgpu::BufferUsages::QUERY_RESOLVE,
    ]
}

impl_wgpu_select! {
    type     = wgpu::TextureUsages,
    style    = TextureUsagesStyle,
    prompt   = "Choose a texture usage flag:",
    kani_var = wgpu::TextureUsages::TEXTURE_BINDING,
    variants = [
        wgpu::TextureUsages::COPY_SRC,
        wgpu::TextureUsages::COPY_DST,
        wgpu::TextureUsages::TEXTURE_BINDING,
        wgpu::TextureUsages::STORAGE_BINDING,
        wgpu::TextureUsages::RENDER_ATTACHMENT,
    ]
}

impl_wgpu_select! {
    type     = wgpu::ShaderStages,
    style    = ShaderStagesStyle,
    prompt   = "Choose a shader stage flag:",
    kani_var = wgpu::ShaderStages::VERTEX,
    variants = [
        wgpu::ShaderStages::NONE,
        wgpu::ShaderStages::VERTEX,
        wgpu::ShaderStages::FRAGMENT,
        wgpu::ShaderStages::COMPUTE,
        wgpu::ShaderStages::VERTEX_FRAGMENT,
    ]
}

impl_wgpu_select! {
    type     = wgpu::ColorWrites,
    style    = ColorWritesStyle,
    prompt   = "Choose a color write mask flag:",
    kani_var = wgpu::ColorWrites::ALL,
    variants = [
        wgpu::ColorWrites::RED,
        wgpu::ColorWrites::GREEN,
        wgpu::ColorWrites::BLUE,
        wgpu::ColorWrites::ALPHA,
        wgpu::ColorWrites::COLOR,
        wgpu::ColorWrites::ALL,
    ]
}
