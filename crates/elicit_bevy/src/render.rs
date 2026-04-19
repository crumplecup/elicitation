//! Shadow types for user-facing render components.
//!
//! Covers [`Msaa`], [`Hdr`], [`ColorGradingSection`], [`ColorGradingGlobal`],
//! [`ColorGrading`], [`NoAutomaticBatching`], [`MipBias`], and
//! [`OcclusionCulling`].

// ── shadow_elicitation / unit_elicitation macros (module-local) ───────────────

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

macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
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
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

// ── NoAutomaticBatching ───────────────────────────────────────────────────────

/// Shadow of [`bevy::render::batching::NoAutomaticBatching`].
///
/// Marker component that disables automatic draw-call batching for a mesh entity.
/// Useful when per-entity rendering order or draw call isolation is required.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct NoAutomaticBatching;

impl From<NoAutomaticBatching> for bevy::render::batching::NoAutomaticBatching {
    fn from(_: NoAutomaticBatching) -> Self {
        bevy::render::batching::NoAutomaticBatching
    }
}

mod emit_impls_no_automatic_batching {
    use super::NoAutomaticBatching;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for NoAutomaticBatching {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::NoAutomaticBatching }
        }
    }
}

unit_elicitation!(
    NoAutomaticBatching,
    bevy::render::batching::NoAutomaticBatching
);

// ── MipBias ───────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::camera::MipBias`].
///
/// Camera component specifying a mip bias when sampling material textures.
/// Typically used alongside TAA or other temporal upscaling techniques
/// to counteract texture blurriness introduced by jitter.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct MipBias(pub f32);

impl Default for MipBias {
    fn default() -> Self {
        Self(bevy::render::camera::MipBias::default().0)
    }
}

impl From<MipBias> for bevy::render::camera::MipBias {
    fn from(v: MipBias) -> Self {
        bevy::render::camera::MipBias(v.0)
    }
}

impl From<bevy::render::camera::MipBias> for MipBias {
    fn from(v: bevy::render::camera::MipBias) -> Self {
        MipBias(v.0)
    }
}

mod emit_impls_mip_bias {
    use super::MipBias;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for MipBias {
        fn to_code_literal(&self) -> TokenStream {
            let bias = self.0;
            quote::quote! { ::elicit_bevy::MipBias(#bias) }
        }
    }
}

shadow_elicitation!(MipBias);

// ── OcclusionCulling ──────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::experimental::occlusion_culling::OcclusionCulling`].
///
/// Marker component enabling GPU-driven occlusion culling for a camera.
/// Entities occluded by closer geometry are skipped during rendering.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct OcclusionCulling;

impl From<OcclusionCulling> for bevy::render::experimental::occlusion_culling::OcclusionCulling {
    fn from(_: OcclusionCulling) -> Self {
        bevy::render::experimental::occlusion_culling::OcclusionCulling
    }
}

mod emit_impls_occlusion_culling {
    use super::OcclusionCulling;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OcclusionCulling {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::OcclusionCulling }
        }
    }
}

unit_elicitation!(
    OcclusionCulling,
    bevy::render::experimental::occlusion_culling::OcclusionCulling
);

// ── Msaa ─────────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::view::Msaa`].
///
/// Multisample anti-aliasing (MSAA) sample count setting, placed as a
/// component on a camera entity or as a resource for the app.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub enum Msaa {
    /// No MSAA (1 sample).
    Off,
    /// 2 samples.
    Sample2,
    /// 4 samples (default).
    #[default]
    Sample4,
    /// 8 samples.
    Sample8,
}

impl From<Msaa> for bevy::render::view::Msaa {
    fn from(v: Msaa) -> Self {
        match v {
            Msaa::Off => bevy::render::view::Msaa::Off,
            Msaa::Sample2 => bevy::render::view::Msaa::Sample2,
            Msaa::Sample4 => bevy::render::view::Msaa::Sample4,
            Msaa::Sample8 => bevy::render::view::Msaa::Sample8,
        }
    }
}

impl From<bevy::render::view::Msaa> for Msaa {
    fn from(v: bevy::render::view::Msaa) -> Self {
        match v {
            bevy::render::view::Msaa::Off => Msaa::Off,
            bevy::render::view::Msaa::Sample2 => Msaa::Sample2,
            bevy::render::view::Msaa::Sample4 => Msaa::Sample4,
            bevy::render::view::Msaa::Sample8 => Msaa::Sample8,
        }
    }
}

mod emit_impls_msaa {
    use super::Msaa;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Msaa {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                Msaa::Off => quote::quote! { ::bevy::render::view::Msaa::Off },
                Msaa::Sample2 => quote::quote! { ::bevy::render::view::Msaa::Sample2 },
                Msaa::Sample4 => quote::quote! { ::bevy::render::view::Msaa::Sample4 },
                Msaa::Sample8 => quote::quote! { ::bevy::render::view::Msaa::Sample8 },
            }
        }
    }
}

shadow_elicitation!(Msaa);

// ── Hdr ──────────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::view::Hdr`].
///
/// Marker component enabling HDR rendering on a camera entity.
/// Adds support for high dynamic range lighting values in the intermediate
/// render texture (does not affect display output format).
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct Hdr;

impl From<Hdr> for bevy::render::view::Hdr {
    fn from(_: Hdr) -> Self {
        bevy::render::view::Hdr
    }
}

mod emit_impls_hdr {
    use super::Hdr;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Hdr {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::render::view::Hdr }
        }
    }
}

unit_elicitation!(Hdr, bevy::render::view::Hdr);

// ── ColorGradingSection ───────────────────────────────────────────────────────

/// Shadow of [`bevy::render::view::ColorGradingSection`].
///
/// Per-tone-range color grading settings (shadows, midtones, highlights).
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ColorGradingSection {
    /// Saturation multiplier (1.0 = unchanged, 0.0 = grayscale).
    pub saturation: f32,
    /// Contrast adjustment (1.0 = unchanged).
    pub contrast: f32,
    /// Gamma correction exponent (1.0 = unchanged).
    pub gamma: f32,
    /// Linear gain multiplier (1.0 = unchanged).
    pub gain: f32,
    /// Linear lift offset (0.0 = unchanged).
    pub lift: f32,
}

impl Default for ColorGradingSection {
    fn default() -> Self {
        let d = bevy::render::view::ColorGradingSection::default();
        Self {
            saturation: d.saturation,
            contrast: d.contrast,
            gamma: d.gamma,
            gain: d.gain,
            lift: d.lift,
        }
    }
}

impl From<ColorGradingSection> for bevy::render::view::ColorGradingSection {
    fn from(s: ColorGradingSection) -> Self {
        bevy::render::view::ColorGradingSection {
            saturation: s.saturation,
            contrast: s.contrast,
            gamma: s.gamma,
            gain: s.gain,
            lift: s.lift,
        }
    }
}

impl From<bevy::render::view::ColorGradingSection> for ColorGradingSection {
    fn from(s: bevy::render::view::ColorGradingSection) -> Self {
        ColorGradingSection {
            saturation: s.saturation,
            contrast: s.contrast,
            gamma: s.gamma,
            gain: s.gain,
            lift: s.lift,
        }
    }
}

mod emit_impls_color_grading_section {
    use super::ColorGradingSection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ColorGradingSection {
        fn to_code_literal(&self) -> TokenStream {
            let saturation = self.saturation;
            let contrast = self.contrast;
            let gamma = self.gamma;
            let gain = self.gain;
            let lift = self.lift;
            quote::quote! {
                ::bevy::render::view::ColorGradingSection {
                    saturation: #saturation,
                    contrast: #contrast,
                    gamma: #gamma,
                    gain: #gain,
                    lift: #lift,
                }
            }
        }
    }
}

shadow_elicitation!(ColorGradingSection);

// ── ColorGradingGlobal ────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::view::ColorGradingGlobal`].
///
/// Global (whole-image) color grading settings: exposure, white-balance, hue,
/// saturation, and the midtone luminance range.
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ColorGradingGlobal {
    /// Exposure value (EV) offset in stops.
    pub exposure: f32,
    /// White-balance chromaticity X offset (positive = redder).
    pub temperature: f32,
    /// White-balance chromaticity Y offset (positive = more magenta).
    pub tint: f32,
    /// Hue rotation in radians.
    pub hue: f32,
    /// Post-tonemapping saturation (1.0 = unchanged).
    pub post_saturation: f32,
    /// Lower bound of the midtones luminance range (default 0.2).
    pub midtones_range_start: f32,
    /// Upper bound of the midtones luminance range (default 0.7).
    pub midtones_range_end: f32,
}

impl Default for ColorGradingGlobal {
    fn default() -> Self {
        let d = bevy::render::view::ColorGradingGlobal::default();
        Self {
            exposure: d.exposure,
            temperature: d.temperature,
            tint: d.tint,
            hue: d.hue,
            post_saturation: d.post_saturation,
            midtones_range_start: d.midtones_range.start,
            midtones_range_end: d.midtones_range.end,
        }
    }
}

impl From<ColorGradingGlobal> for bevy::render::view::ColorGradingGlobal {
    fn from(g: ColorGradingGlobal) -> Self {
        bevy::render::view::ColorGradingGlobal {
            exposure: g.exposure,
            temperature: g.temperature,
            tint: g.tint,
            hue: g.hue,
            post_saturation: g.post_saturation,
            midtones_range: g.midtones_range_start..g.midtones_range_end,
        }
    }
}

impl From<bevy::render::view::ColorGradingGlobal> for ColorGradingGlobal {
    fn from(g: bevy::render::view::ColorGradingGlobal) -> Self {
        ColorGradingGlobal {
            exposure: g.exposure,
            temperature: g.temperature,
            tint: g.tint,
            hue: g.hue,
            post_saturation: g.post_saturation,
            midtones_range_start: g.midtones_range.start,
            midtones_range_end: g.midtones_range.end,
        }
    }
}

mod emit_impls_color_grading_global {
    use super::ColorGradingGlobal;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ColorGradingGlobal {
        fn to_code_literal(&self) -> TokenStream {
            let exposure = self.exposure;
            let temperature = self.temperature;
            let tint = self.tint;
            let hue = self.hue;
            let post_saturation = self.post_saturation;
            let start = self.midtones_range_start;
            let end = self.midtones_range_end;
            quote::quote! {
                ::bevy::render::view::ColorGradingGlobal {
                    exposure: #exposure,
                    temperature: #temperature,
                    tint: #tint,
                    hue: #hue,
                    post_saturation: #post_saturation,
                    midtones_range: #start..#end,
                }
            }
        }
    }
}

shadow_elicitation!(ColorGradingGlobal);

// ── ColorGrading ──────────────────────────────────────────────────────────────

/// Shadow of [`bevy::render::view::ColorGrading`].
///
/// Camera component enabling filmic color grading with separate controls
/// for shadows, midtones, and highlights.
#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct ColorGrading {
    /// Global settings applied to the whole image.
    pub global: ColorGradingGlobal,
    /// Color grading applied to dark areas.
    pub shadows: ColorGradingSection,
    /// Color grading applied to mid-brightness areas.
    pub midtones: ColorGradingSection,
    /// Color grading applied to bright areas.
    pub highlights: ColorGradingSection,
}

impl From<ColorGrading> for bevy::render::view::ColorGrading {
    fn from(c: ColorGrading) -> Self {
        bevy::render::view::ColorGrading {
            global: c.global.into(),
            shadows: c.shadows.into(),
            midtones: c.midtones.into(),
            highlights: c.highlights.into(),
        }
    }
}

impl From<bevy::render::view::ColorGrading> for ColorGrading {
    fn from(c: bevy::render::view::ColorGrading) -> Self {
        ColorGrading {
            global: c.global.into(),
            shadows: c.shadows.into(),
            midtones: c.midtones.into(),
            highlights: c.highlights.into(),
        }
    }
}

mod emit_impls_color_grading {
    use super::ColorGrading;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ColorGrading {
        fn to_code_literal(&self) -> TokenStream {
            let global = self.global.to_code_literal();
            let shadows = self.shadows.to_code_literal();
            let midtones = self.midtones.to_code_literal();
            let highlights = self.highlights.to_code_literal();
            quote::quote! {
                ::bevy::render::view::ColorGrading {
                    global: #global,
                    shadows: #shadows,
                    midtones: #midtones,
                    highlights: #highlights,
                }
            }
        }
    }
}

shadow_elicitation!(ColorGrading);
