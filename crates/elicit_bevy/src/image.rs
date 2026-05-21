//! Bevy image sampler configuration shadow types.
//!
//! Covers [`ImageSampler`], [`ImageSamplerDescriptor`], [`ImageAddressMode`],
//! [`ImageFilterMode`], [`ImageCompareFunction`], and [`ImageSamplerBorderColor`].
//!
//! All upstream types carry `Serialize`/`Deserialize`, so `From` conversions
//! round-trip through their serde representations. Local shadow enums/structs
//! carry full `JsonSchema` coverage via derive.

// ── shadow_elicitation macro ──────────────────────────────────────────────────

macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }
        impl elicitation::Elicitation for $name {
            type Style = ();
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }
            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }
            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }
            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }
        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }
            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }
        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }
        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── ImageAddressMode ──────────────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageAddressMode`].
#[derive(
    Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum ImageAddressMode {
    /// Clamp to the edge of the texture.
    #[default]
    ClampToEdge,
    /// Tile the texture.
    Repeat,
    /// Mirror-repeat the texture.
    MirrorRepeat,
    /// Clamp to the border color.
    ClampToBorder,
}

impl From<ImageAddressMode> for bevy::image::ImageAddressMode {
    fn from(v: ImageAddressMode) -> Self {
        match v {
            ImageAddressMode::ClampToEdge => Self::ClampToEdge,
            ImageAddressMode::Repeat => Self::Repeat,
            ImageAddressMode::MirrorRepeat => Self::MirrorRepeat,
            ImageAddressMode::ClampToBorder => Self::ClampToBorder,
        }
    }
}

impl From<bevy::image::ImageAddressMode> for ImageAddressMode {
    fn from(v: bevy::image::ImageAddressMode) -> Self {
        match v {
            bevy::image::ImageAddressMode::ClampToEdge => Self::ClampToEdge,
            bevy::image::ImageAddressMode::Repeat => Self::Repeat,
            bevy::image::ImageAddressMode::MirrorRepeat => Self::MirrorRepeat,
            bevy::image::ImageAddressMode::ClampToBorder => Self::ClampToBorder,
        }
    }
}

mod emit_image_address_mode {
    use super::ImageAddressMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageAddressMode {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ImageAddressMode::ClampToEdge => {
                    quote::quote! { ::bevy::image::ImageAddressMode::ClampToEdge }
                }
                ImageAddressMode::Repeat => {
                    quote::quote! { ::bevy::image::ImageAddressMode::Repeat }
                }
                ImageAddressMode::MirrorRepeat => {
                    quote::quote! { ::bevy::image::ImageAddressMode::MirrorRepeat }
                }
                ImageAddressMode::ClampToBorder => {
                    quote::quote! { ::bevy::image::ImageAddressMode::ClampToBorder }
                }
            }
        }
    }
}

shadow_elicitation!(ImageAddressMode);

// ── ImageFilterMode ───────────────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageFilterMode`].
#[derive(
    Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum ImageFilterMode {
    /// Nearest-neighbor (pixelated).
    #[default]
    Nearest,
    /// Bilinear interpolation (smooth).
    Linear,
}

impl From<ImageFilterMode> for bevy::image::ImageFilterMode {
    fn from(v: ImageFilterMode) -> Self {
        match v {
            ImageFilterMode::Nearest => Self::Nearest,
            ImageFilterMode::Linear => Self::Linear,
        }
    }
}

impl From<bevy::image::ImageFilterMode> for ImageFilterMode {
    fn from(v: bevy::image::ImageFilterMode) -> Self {
        match v {
            bevy::image::ImageFilterMode::Nearest => Self::Nearest,
            bevy::image::ImageFilterMode::Linear => Self::Linear,
        }
    }
}

mod emit_image_filter_mode {
    use super::ImageFilterMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageFilterMode {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ImageFilterMode::Nearest => {
                    quote::quote! { ::bevy::image::ImageFilterMode::Nearest }
                }
                ImageFilterMode::Linear => quote::quote! { ::bevy::image::ImageFilterMode::Linear },
            }
        }
    }
}

shadow_elicitation!(ImageFilterMode);

// ── ImageCompareFunction ──────────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageCompareFunction`].
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum ImageCompareFunction {
    /// Always fails.
    Never,
    /// Passes if new < existing.
    Less,
    /// Passes if new == existing.
    Equal,
    /// Passes if new <= existing.
    LessEqual,
    /// Passes if new > existing.
    Greater,
    /// Passes if new != existing.
    NotEqual,
    /// Passes if new >= existing.
    GreaterEqual,
    /// Always passes.
    Always,
}

impl From<ImageCompareFunction> for bevy::image::ImageCompareFunction {
    fn from(v: ImageCompareFunction) -> Self {
        match v {
            ImageCompareFunction::Never => Self::Never,
            ImageCompareFunction::Less => Self::Less,
            ImageCompareFunction::Equal => Self::Equal,
            ImageCompareFunction::LessEqual => Self::LessEqual,
            ImageCompareFunction::Greater => Self::Greater,
            ImageCompareFunction::NotEqual => Self::NotEqual,
            ImageCompareFunction::GreaterEqual => Self::GreaterEqual,
            ImageCompareFunction::Always => Self::Always,
        }
    }
}

impl From<bevy::image::ImageCompareFunction> for ImageCompareFunction {
    fn from(v: bevy::image::ImageCompareFunction) -> Self {
        match v {
            bevy::image::ImageCompareFunction::Never => Self::Never,
            bevy::image::ImageCompareFunction::Less => Self::Less,
            bevy::image::ImageCompareFunction::Equal => Self::Equal,
            bevy::image::ImageCompareFunction::LessEqual => Self::LessEqual,
            bevy::image::ImageCompareFunction::Greater => Self::Greater,
            bevy::image::ImageCompareFunction::NotEqual => Self::NotEqual,
            bevy::image::ImageCompareFunction::GreaterEqual => Self::GreaterEqual,
            bevy::image::ImageCompareFunction::Always => Self::Always,
        }
    }
}

mod emit_image_compare_function {
    use super::ImageCompareFunction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageCompareFunction {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ImageCompareFunction::Never => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::Never }
                }
                ImageCompareFunction::Less => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::Less }
                }
                ImageCompareFunction::Equal => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::Equal }
                }
                ImageCompareFunction::LessEqual => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::LessEqual }
                }
                ImageCompareFunction::Greater => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::Greater }
                }
                ImageCompareFunction::NotEqual => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::NotEqual }
                }
                ImageCompareFunction::GreaterEqual => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::GreaterEqual }
                }
                ImageCompareFunction::Always => {
                    quote::quote! { ::bevy::image::ImageCompareFunction::Always }
                }
            }
        }
    }
}

shadow_elicitation!(ImageCompareFunction);

// ── ImageSamplerBorderColor ───────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageSamplerBorderColor`].
#[derive(
    Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum ImageSamplerBorderColor {
    /// RGBA `[0, 0, 0, 0]`.
    TransparentBlack,
    /// RGBA `[0, 0, 0, 1]`.
    OpaqueBlack,
    /// RGBA `[1, 1, 1, 1]`.
    OpaqueWhite,
    /// RGBA `[0, 0, 0, 0]` (Metal-specific behavior).
    Zero,
}

impl From<ImageSamplerBorderColor> for bevy::image::ImageSamplerBorderColor {
    fn from(v: ImageSamplerBorderColor) -> Self {
        match v {
            ImageSamplerBorderColor::TransparentBlack => Self::TransparentBlack,
            ImageSamplerBorderColor::OpaqueBlack => Self::OpaqueBlack,
            ImageSamplerBorderColor::OpaqueWhite => Self::OpaqueWhite,
            ImageSamplerBorderColor::Zero => Self::Zero,
        }
    }
}

impl From<bevy::image::ImageSamplerBorderColor> for ImageSamplerBorderColor {
    fn from(v: bevy::image::ImageSamplerBorderColor) -> Self {
        match v {
            bevy::image::ImageSamplerBorderColor::TransparentBlack => Self::TransparentBlack,
            bevy::image::ImageSamplerBorderColor::OpaqueBlack => Self::OpaqueBlack,
            bevy::image::ImageSamplerBorderColor::OpaqueWhite => Self::OpaqueWhite,
            bevy::image::ImageSamplerBorderColor::Zero => Self::Zero,
        }
    }
}

mod emit_image_sampler_border_color {
    use super::ImageSamplerBorderColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageSamplerBorderColor {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ImageSamplerBorderColor::TransparentBlack => {
                    quote::quote! { ::bevy::image::ImageSamplerBorderColor::TransparentBlack }
                }
                ImageSamplerBorderColor::OpaqueBlack => {
                    quote::quote! { ::bevy::image::ImageSamplerBorderColor::OpaqueBlack }
                }
                ImageSamplerBorderColor::OpaqueWhite => {
                    quote::quote! { ::bevy::image::ImageSamplerBorderColor::OpaqueWhite }
                }
                ImageSamplerBorderColor::Zero => {
                    quote::quote! { ::bevy::image::ImageSamplerBorderColor::Zero }
                }
            }
        }
    }
}

shadow_elicitation!(ImageSamplerBorderColor);

// ── ImageSamplerDescriptor ────────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageSamplerDescriptor`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct ImageSamplerDescriptor {
    /// Optional debug label.
    pub label: Option<String>,
    /// Address mode for U axis.
    pub address_mode_u: ImageAddressMode,
    /// Address mode for V axis.
    pub address_mode_v: ImageAddressMode,
    /// Address mode for W axis (3D textures).
    pub address_mode_w: ImageAddressMode,
    /// Filter when magnifying.
    pub mag_filter: ImageFilterMode,
    /// Filter when minifying.
    pub min_filter: ImageFilterMode,
    /// Filter between mip levels.
    pub mipmap_filter: ImageFilterMode,
    /// Minimum mip level.
    pub lod_min_clamp: f32,
    /// Maximum mip level.
    pub lod_max_clamp: f32,
    /// Depth comparison function (for shadow maps).
    pub compare: Option<ImageCompareFunction>,
    /// Anisotropy clamp (must be ≥ 1; if > 1, all filters must be linear).
    pub anisotropy_clamp: u16,
    /// Border color when `address_mode` is `ClampToBorder`.
    pub border_color: Option<ImageSamplerBorderColor>,
}

impl Default for ImageSamplerDescriptor {
    fn default() -> Self {
        let u = bevy::image::ImageSamplerDescriptor::default();
        Self {
            label: u.label,
            address_mode_u: u.address_mode_u.into(),
            address_mode_v: u.address_mode_v.into(),
            address_mode_w: u.address_mode_w.into(),
            mag_filter: u.mag_filter.into(),
            min_filter: u.min_filter.into(),
            mipmap_filter: u.mipmap_filter.into(),
            lod_min_clamp: u.lod_min_clamp,
            lod_max_clamp: u.lod_max_clamp,
            compare: u.compare.map(Into::into),
            anisotropy_clamp: u.anisotropy_clamp,
            border_color: u.border_color.map(Into::into),
        }
    }
}

impl From<ImageSamplerDescriptor> for bevy::image::ImageSamplerDescriptor {
    fn from(v: ImageSamplerDescriptor) -> Self {
        Self {
            label: v.label,
            address_mode_u: v.address_mode_u.into(),
            address_mode_v: v.address_mode_v.into(),
            address_mode_w: v.address_mode_w.into(),
            mag_filter: v.mag_filter.into(),
            min_filter: v.min_filter.into(),
            mipmap_filter: v.mipmap_filter.into(),
            lod_min_clamp: v.lod_min_clamp,
            lod_max_clamp: v.lod_max_clamp,
            compare: v.compare.map(Into::into),
            anisotropy_clamp: v.anisotropy_clamp,
            border_color: v.border_color.map(Into::into),
        }
    }
}

mod emit_image_sampler_descriptor {
    use super::ImageSamplerDescriptor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageSamplerDescriptor {
        fn to_code_literal(&self) -> TokenStream {
            let label = match &self.label {
                None => quote::quote! { None },
                Some(s) => quote::quote! { Some(#s.to_string()) },
            };
            let addr_u = self.address_mode_u.to_code_literal();
            let addr_v = self.address_mode_v.to_code_literal();
            let addr_w = self.address_mode_w.to_code_literal();
            let mag = self.mag_filter.to_code_literal();
            let min = self.min_filter.to_code_literal();
            let mip = self.mipmap_filter.to_code_literal();
            let lod_min = self.lod_min_clamp;
            let lod_max = self.lod_max_clamp;
            let compare = match &self.compare {
                None => quote::quote! { None },
                Some(c) => {
                    let t = c.to_code_literal();
                    quote::quote! { Some(#t) }
                }
            };
            let aniso = self.anisotropy_clamp;
            let border = match &self.border_color {
                None => quote::quote! { None },
                Some(b) => {
                    let t = b.to_code_literal();
                    quote::quote! { Some(#t) }
                }
            };
            quote::quote! {
                ::bevy::image::ImageSamplerDescriptor {
                    label: #label,
                    address_mode_u: #addr_u,
                    address_mode_v: #addr_v,
                    address_mode_w: #addr_w,
                    mag_filter: #mag,
                    min_filter: #min,
                    mipmap_filter: #mip,
                    lod_min_clamp: #lod_min,
                    lod_max_clamp: #lod_max,
                    compare: #compare,
                    anisotropy_clamp: #aniso,
                    border_color: #border,
                }
            }
        }
    }
}

shadow_elicitation!(ImageSamplerDescriptor);

// ── ImageSampler ──────────────────────────────────────────────────────────────

/// Shadow for [`bevy::image::ImageSampler`].
///
/// Set on an `Image` asset to override the default sampler from `ImagePlugin`.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum ImageSampler {
    /// Use the default sampler from `ImagePlugin` configuration.
    #[default]
    Default,
    /// Use a custom sampler descriptor.
    Descriptor(ImageSamplerDescriptor),
}

impl From<ImageSampler> for bevy::image::ImageSampler {
    fn from(v: ImageSampler) -> Self {
        match v {
            ImageSampler::Default => Self::Default,
            ImageSampler::Descriptor(d) => Self::Descriptor(d.into()),
        }
    }
}

impl From<bevy::image::ImageSampler> for ImageSampler {
    fn from(v: bevy::image::ImageSampler) -> Self {
        match v {
            bevy::image::ImageSampler::Default => Self::Default,
            bevy::image::ImageSampler::Descriptor(d) => Self::Descriptor(ImageSamplerDescriptor {
                label: d.label,
                address_mode_u: d.address_mode_u.into(),
                address_mode_v: d.address_mode_v.into(),
                address_mode_w: d.address_mode_w.into(),
                mag_filter: d.mag_filter.into(),
                min_filter: d.min_filter.into(),
                mipmap_filter: d.mipmap_filter.into(),
                lod_min_clamp: d.lod_min_clamp,
                lod_max_clamp: d.lod_max_clamp,
                compare: d.compare.map(Into::into),
                anisotropy_clamp: d.anisotropy_clamp,
                border_color: d.border_color.map(Into::into),
            }),
        }
    }
}

mod emit_image_sampler {
    use super::ImageSampler;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ImageSampler {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ImageSampler::Default => {
                    quote::quote! { ::bevy::image::ImageSampler::Default }
                }
                ImageSampler::Descriptor(d) => {
                    let desc = d.to_code_literal();
                    quote::quote! { ::bevy::image::ImageSampler::Descriptor(#desc) }
                }
            }
        }
    }
}

shadow_elicitation!(ImageSampler);
