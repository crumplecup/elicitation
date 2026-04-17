//! Bevy 0.18 render-related enum elicitation.
//!
//! Covers:
//! - [`BevyAlphaMode`] — owned trenchcoat for `bevy::render::alpha::AlphaMode`
//!   (has a data variant `Mask(f32)`, so requires the full owned-enum pattern).
//! - [`BevyTonemapping`] — select-trenchcoat wrapper for
//!   `bevy::core_pipeline::tonemapping::Tonemapping`.
//! - [`BevyFace`], [`BevyFrontFace`], [`BevyPrimitiveTopology`] — select-trenchcoat
//!   wrappers for the three wgpu render primitives re-exported by Bevy.
//!   These are only compiled when the `wgpu-types` feature is **not** enabled;
//!   when it is, the identical implementations from `wgpu_types::enums` are used.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyAlphaMode ─────────────────────────────────────────────────────────────

/// Internal variant-selection enum for [`BevyAlphaMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BevyAlphaModeKind {
    Opaque,
    Mask,
    Blend,
    Premultiplied,
    AlphaToCoverage,
    Add,
    Multiply,
}

impl Prompt for BevyAlphaModeKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose an alpha blending mode:")
    }
}

impl Select for BevyAlphaModeKind {
    fn options() -> Vec<Self> {
        vec![
            Self::Opaque,
            Self::Mask,
            Self::Blend,
            Self::Premultiplied,
            Self::AlphaToCoverage,
            Self::Add,
            Self::Multiply,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Opaque".to_string(),
            "Mask".to_string(),
            "Blend".to_string(),
            "Premultiplied".to_string(),
            "AlphaToCoverage".to_string(),
            "Add".to_string(),
            "Multiply".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Opaque" => Some(Self::Opaque),
            "Mask" => Some(Self::Mask),
            "Blend" => Some(Self::Blend),
            "Premultiplied" => Some(Self::Premultiplied),
            "AlphaToCoverage" => Some(Self::AlphaToCoverage),
            "Add" => Some(Self::Add),
            "Multiply" => Some(Self::Multiply),
            _ => None,
        }
    }
}

/// Owned trenchcoat for [`bevy::render::alpha::AlphaMode`].
///
/// `AlphaMode::Mask(f32)` carries a threshold value, so this uses the full
/// owned-enum pattern rather than a simple `select_trenchcoat!` wrapper.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "mode", content = "value")]
pub enum BevyAlphaMode {
    /// All fragments are fully opaque.
    Opaque,
    /// Fragments below the threshold are discarded; above are fully opaque.
    Mask(f32),
    /// Standard alpha blending.
    Blend,
    /// Pre-multiplied alpha blending.
    Premultiplied,
    /// Alpha to coverage (MSAA).
    AlphaToCoverage,
    /// Additive blending.
    Add,
    /// Multiplicative blending.
    Multiply,
}

impl From<bevy::render::alpha::AlphaMode> for BevyAlphaMode {
    fn from(a: bevy::render::alpha::AlphaMode) -> Self {
        match a {
            bevy::render::alpha::AlphaMode::Opaque => Self::Opaque,
            bevy::render::alpha::AlphaMode::Mask(t) => Self::Mask(t),
            bevy::render::alpha::AlphaMode::Blend => Self::Blend,
            bevy::render::alpha::AlphaMode::Premultiplied => Self::Premultiplied,
            bevy::render::alpha::AlphaMode::AlphaToCoverage => Self::AlphaToCoverage,
            bevy::render::alpha::AlphaMode::Add => Self::Add,
            bevy::render::alpha::AlphaMode::Multiply => Self::Multiply,
        }
    }
}

impl From<BevyAlphaMode> for bevy::render::alpha::AlphaMode {
    fn from(a: BevyAlphaMode) -> Self {
        match a {
            BevyAlphaMode::Opaque => Self::Opaque,
            BevyAlphaMode::Mask(t) => Self::Mask(t),
            BevyAlphaMode::Blend => Self::Blend,
            BevyAlphaMode::Premultiplied => Self::Premultiplied,
            BevyAlphaMode::AlphaToCoverage => Self::AlphaToCoverage,
            BevyAlphaMode::Add => Self::Add,
            BevyAlphaMode::Multiply => Self::Multiply,
        }
    }
}

impl BevyAlphaMode {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::render::alpha::AlphaMode {
        self.into()
    }
}

crate::default_style!(BevyAlphaMode => BevyAlphaModeStyle);

impl Prompt for BevyAlphaMode {
    fn prompt() -> Option<&'static str> {
        Some("Choose an alpha blending mode:")
    }
}

impl Elicitation for BevyAlphaMode {
    type Style = BevyAlphaModeStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::render::alpha::AlphaMode")
    )]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(mcp::select_params(
                            BevyAlphaModeKind::prompt().unwrap_or("Choose an alpha mode:"),
                            &BevyAlphaModeKind::labels(),
                        )),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            match BevyAlphaModeKind::from_label(&label) {
                Some(BevyAlphaModeKind::Opaque) => Ok(Self::Opaque),
                Some(BevyAlphaModeKind::Mask) => Ok(Self::Mask(f32::elicit(communicator).await?)),
                Some(BevyAlphaModeKind::Blend) => Ok(Self::Blend),
                Some(BevyAlphaModeKind::Premultiplied) => Ok(Self::Premultiplied),
                Some(BevyAlphaModeKind::AlphaToCoverage) => Ok(Self::AlphaToCoverage),
                Some(BevyAlphaModeKind::Add) => Ok(Self::Add),
                Some(BevyAlphaModeKind::Multiply) => Ok(Self::Multiply),
                None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyAlphaMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::render::alpha::AlphaMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Opaque".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Mask".to_string(),
                        fields: vec![crate::FieldInfo {
                            name: "threshold",
                            type_name: "f32",
                            prompt: Some("Alpha threshold [0,1]:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Blend".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Premultiplied".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "AlphaToCoverage".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Add".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Multiply".to_string(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyAlphaMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose an alpha mode:")
                .to_string(),
            type_name: "bevy::render::alpha::AlphaMode".to_string(),
            options: BevyAlphaModeKind::labels(),
            branches: vec![
                None,                               // Opaque
                Some(Box::new(f32::prompt_tree())), // Mask
                None,                               // Blend
                None,                               // Premultiplied
                None,                               // AlphaToCoverage
                None,                               // Add
                None,                               // Multiply
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAlphaMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            BevyAlphaMode::Opaque => {
                quote::quote! { bevy::render::alpha::AlphaMode::Opaque }
            }
            BevyAlphaMode::Mask(t) => {
                let t_lit = crate::emit_code::ToCodeLiteral::to_code_literal(t);
                quote::quote! { bevy::render::alpha::AlphaMode::Mask(#t_lit) }
            }
            BevyAlphaMode::Blend => {
                quote::quote! { bevy::render::alpha::AlphaMode::Blend }
            }
            BevyAlphaMode::Premultiplied => {
                quote::quote! { bevy::render::alpha::AlphaMode::Premultiplied }
            }
            BevyAlphaMode::AlphaToCoverage => {
                quote::quote! { bevy::render::alpha::AlphaMode::AlphaToCoverage }
            }
            BevyAlphaMode::Add => {
                quote::quote! { bevy::render::alpha::AlphaMode::Add }
            }
            BevyAlphaMode::Multiply => {
                quote::quote! { bevy::render::alpha::AlphaMode::Multiply }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::render::alpha::AlphaMode }
    }
}

// ── BevyTonemapping ───────────────────────────────────────────────────────────
//
// All Tonemapping variants are unit (no data), so we use a simple owned enum
// with Serialize + Deserialize + JsonSchema rather than select_trenchcoat!,
// which avoids macro type-inference issues with the long bevy path.

/// Owned trenchcoat for [`bevy::core_pipeline::tonemapping::Tonemapping`].
///
/// Implements `Prompt`, `Select`, `Elicitation`, `ElicitIntrospect`,
/// `ElicitPromptTree`, and `ToCodeLiteral` directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum BevyTonemapping {
    /// Bypass tonemapping.
    None,
    /// Reinhard tonemapping (hue shifting, low saturation at brights).
    Reinhard,
    /// Reinhard luminance tonemapping.
    ReinhardLuminance,
    /// ACES fitted tonemapping (dramatic, high contrast, hue shifts).
    AcesFitted,
    /// AgX tonemapping — very neutral (requires `tonemapping_luts`).
    AgX,
    /// Somewhat boring display transform — compromise between Reinhard variants.
    SomewhatBoringDisplayTransform,
    /// Tony Mc Mapface — Bevy default, very neutral (requires `tonemapping_luts`).
    TonyMcMapface,
    /// Blender filmic display transform (requires `tonemapping_luts`).
    BlenderFilmic,
}

impl From<bevy::core_pipeline::tonemapping::Tonemapping> for BevyTonemapping {
    fn from(t: bevy::core_pipeline::tonemapping::Tonemapping) -> Self {
        use bevy::core_pipeline::tonemapping::Tonemapping as T;
        match t {
            T::None => Self::None,
            T::Reinhard => Self::Reinhard,
            T::ReinhardLuminance => Self::ReinhardLuminance,
            T::AcesFitted => Self::AcesFitted,
            T::AgX => Self::AgX,
            T::SomewhatBoringDisplayTransform => Self::SomewhatBoringDisplayTransform,
            T::TonyMcMapface => Self::TonyMcMapface,
            T::BlenderFilmic => Self::BlenderFilmic,
        }
    }
}

impl From<BevyTonemapping> for bevy::core_pipeline::tonemapping::Tonemapping {
    fn from(b: BevyTonemapping) -> Self {
        use bevy::core_pipeline::tonemapping::Tonemapping as T;
        match b {
            BevyTonemapping::None => T::None,
            BevyTonemapping::Reinhard => T::Reinhard,
            BevyTonemapping::ReinhardLuminance => T::ReinhardLuminance,
            BevyTonemapping::AcesFitted => T::AcesFitted,
            BevyTonemapping::AgX => T::AgX,
            BevyTonemapping::SomewhatBoringDisplayTransform => T::SomewhatBoringDisplayTransform,
            BevyTonemapping::TonyMcMapface => T::TonyMcMapface,
            BevyTonemapping::BlenderFilmic => T::BlenderFilmic,
        }
    }
}

impl BevyTonemapping {
    /// Converts this wrapper into the upstream type.
    pub fn into_inner(self) -> bevy::core_pipeline::tonemapping::Tonemapping {
        self.into()
    }
}

crate::default_style!(BevyTonemapping => BevyTonemappingStyle);

impl Prompt for BevyTonemapping {
    fn prompt() -> Option<&'static str> {
        Some("Choose a tonemapping algorithm:")
    }
}

impl Select for BevyTonemapping {
    fn options() -> Vec<Self> {
        vec![
            Self::None,
            Self::Reinhard,
            Self::ReinhardLuminance,
            Self::AcesFitted,
            Self::AgX,
            Self::SomewhatBoringDisplayTransform,
            Self::TonyMcMapface,
            Self::BlenderFilmic,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "None".to_string(),
            "Reinhard".to_string(),
            "ReinhardLuminance".to_string(),
            "AcesFitted".to_string(),
            "AgX".to_string(),
            "SomewhatBoringDisplayTransform".to_string(),
            "TonyMcMapface".to_string(),
            "BlenderFilmic".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "None" => Some(Self::None),
            "Reinhard" => Some(Self::Reinhard),
            "ReinhardLuminance" => Some(Self::ReinhardLuminance),
            "AcesFitted" => Some(Self::AcesFitted),
            "AgX" => Some(Self::AgX),
            "SomewhatBoringDisplayTransform" => Some(Self::SomewhatBoringDisplayTransform),
            "TonyMcMapface" => Some(Self::TonyMcMapface),
            "BlenderFilmic" => Some(Self::BlenderFilmic),
            _ => None,
        }
    }
}

impl Elicitation for BevyTonemapping {
    type Style = BevyTonemappingStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::core_pipeline::tonemapping::Tonemapping")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        Self::prompt().unwrap_or("Choose a tonemapping algorithm:"),
                        &Self::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid Tonemapping: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyTonemapping",
            "BevyTonemapping::TonyMcMapface",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyTonemapping",
            "BevyTonemapping::TonyMcMapface",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyTonemapping",
            "BevyTonemapping::TonyMcMapface",
        )
    }
}

impl ElicitIntrospect for BevyTonemapping {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::core_pipeline::tonemapping::Tonemapping",
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

impl crate::ElicitPromptTree for BevyTonemapping {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a tonemapping algorithm:")
                .to_string(),
            type_name: "bevy::core_pipeline::tonemapping::Tonemapping".to_string(),
            options: Self::labels(),
            branches: vec![None; 8],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyTonemapping {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            BevyTonemapping::None => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::None
            },
            BevyTonemapping::Reinhard => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::Reinhard
            },
            BevyTonemapping::ReinhardLuminance => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::ReinhardLuminance
            },
            BevyTonemapping::AcesFitted => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::AcesFitted
            },
            BevyTonemapping::AgX => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::AgX
            },
            BevyTonemapping::SomewhatBoringDisplayTransform => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::SomewhatBoringDisplayTransform
            },
            BevyTonemapping::TonyMcMapface => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface
            },
            BevyTonemapping::BlenderFilmic => quote::quote! {
                bevy::core_pipeline::tonemapping::Tonemapping::BlenderFilmic
            },
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { bevy::core_pipeline::tonemapping::Tonemapping }
    }
}

// ── Face / FrontFace / PrimitiveTopology ──────────────────────────────────────
//
// These types are `wgpu` types re-exported by Bevy.  When the `wgpu-types`
// feature is active the identical implementations already exist in
// `wgpu_types::enums`; compiling them again would create duplicate trait impls.
// The cfg guard ensures we only emit these when `wgpu-types` is absent.

#[cfg(not(feature = "wgpu-types"))]
mod render_primitives {
    use super::*;

    // ── Face (cull mode) ──────────────────────────────────────────────────────

    impl Prompt for bevy::render::render_resource::Face {
        fn prompt() -> Option<&'static str> {
            Some("Choose which face to cull:")
        }
    }

    impl Select for bevy::render::render_resource::Face {
        fn options() -> Vec<Self> {
            vec![Self::Front, Self::Back]
        }

        fn labels() -> Vec<String> {
            vec!["Front".to_string(), "Back".to_string()]
        }

        fn from_label(label: &str) -> Option<Self> {
            match label {
                "Front" => Some(Self::Front),
                "Back" => Some(Self::Back),
                _ => None,
            }
        }
    }

    crate::default_style!(bevy::render::render_resource::Face => FaceStyle);

    impl Elicitation for bevy::render::render_resource::Face {
        type Style = FaceStyle;

        #[tracing::instrument(skip(communicator))]
        async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose a cull face:"),
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
                    "Invalid Face: {label}"
                )))
            })
        }

        fn kani_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::kani_select_wrapper(
                "bevy::render::render_resource::Face",
                "bevy::render::render_resource::Face::Back",
            )
        }

        fn verus_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::verus_select_wrapper(
                "bevy::render::render_resource::Face",
                "bevy::render::render_resource::Face::Back",
            )
        }

        fn creusot_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::creusot_select_wrapper(
                "bevy::render::render_resource::Face",
                "bevy::render::render_resource::Face::Back",
            )
        }
    }

    impl ElicitIntrospect for bevy::render::render_resource::Face {
        fn pattern() -> ElicitationPattern {
            ElicitationPattern::Select
        }

        fn metadata() -> TypeMetadata {
            TypeMetadata {
                type_name: "bevy::render::render_resource::Face",
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

    crate::select_trenchcoat!(bevy::render::render_resource::Face, as BevyFace);

    // ── FrontFace ─────────────────────────────────────────────────────────────

    impl Prompt for bevy::render::render_resource::FrontFace {
        fn prompt() -> Option<&'static str> {
            Some("Choose front-face winding order (Ccw = counter-clockwise):")
        }
    }

    impl Select for bevy::render::render_resource::FrontFace {
        fn options() -> Vec<Self> {
            vec![Self::Ccw, Self::Cw]
        }

        fn labels() -> Vec<String> {
            vec!["Ccw".to_string(), "Cw".to_string()]
        }

        fn from_label(label: &str) -> Option<Self> {
            match label {
                "Ccw" => Some(Self::Ccw),
                "Cw" => Some(Self::Cw),
                _ => None,
            }
        }
    }

    crate::default_style!(bevy::render::render_resource::FrontFace => FrontFaceStyle);

    impl Elicitation for bevy::render::render_resource::FrontFace {
        type Style = FrontFaceStyle;

        #[tracing::instrument(skip(communicator))]
        async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose front-face winding:"),
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
                    "Invalid FrontFace: {label}"
                )))
            })
        }

        fn kani_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::kani_select_wrapper(
                "bevy::render::render_resource::FrontFace",
                "bevy::render::render_resource::FrontFace::Ccw",
            )
        }

        fn verus_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::verus_select_wrapper(
                "bevy::render::render_resource::FrontFace",
                "bevy::render::render_resource::FrontFace::Ccw",
            )
        }

        fn creusot_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::creusot_select_wrapper(
                "bevy::render::render_resource::FrontFace",
                "bevy::render::render_resource::FrontFace::Ccw",
            )
        }
    }

    impl ElicitIntrospect for bevy::render::render_resource::FrontFace {
        fn pattern() -> ElicitationPattern {
            ElicitationPattern::Select
        }

        fn metadata() -> TypeMetadata {
            TypeMetadata {
                type_name: "bevy::render::render_resource::FrontFace",
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

    crate::select_trenchcoat!(bevy::render::render_resource::FrontFace, as BevyFrontFace);

    // ── PrimitiveTopology ─────────────────────────────────────────────────────

    impl Prompt for bevy::mesh::PrimitiveTopology {
        fn prompt() -> Option<&'static str> {
            Some("Choose a primitive topology:")
        }
    }

    impl Select for bevy::mesh::PrimitiveTopology {
        fn options() -> Vec<Self> {
            vec![
                Self::PointList,
                Self::LineList,
                Self::LineStrip,
                Self::TriangleList,
                Self::TriangleStrip,
            ]
        }

        fn labels() -> Vec<String> {
            vec![
                "PointList".to_string(),
                "LineList".to_string(),
                "LineStrip".to_string(),
                "TriangleList".to_string(),
                "TriangleStrip".to_string(),
            ]
        }

        fn from_label(label: &str) -> Option<Self> {
            match label {
                "PointList" => Some(Self::PointList),
                "LineList" => Some(Self::LineList),
                "LineStrip" => Some(Self::LineStrip),
                "TriangleList" => Some(Self::TriangleList),
                "TriangleStrip" => Some(Self::TriangleStrip),
                _ => None,
            }
        }
    }

    crate::default_style!(bevy::mesh::PrimitiveTopology => PrimitiveTopologyStyle);

    impl Elicitation for bevy::mesh::PrimitiveTopology {
        type Style = PrimitiveTopologyStyle;

        #[tracing::instrument(skip(communicator))]
        async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
            let params = mcp::select_params(
                Self::prompt().unwrap_or("Choose a primitive topology:"),
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
                    "Invalid PrimitiveTopology: {label}"
                )))
            })
        }

        fn kani_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::kani_select_wrapper(
                "bevy::mesh::PrimitiveTopology",
                "bevy::mesh::PrimitiveTopology::TriangleList",
            )
        }

        fn verus_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::verus_select_wrapper(
                "bevy::mesh::PrimitiveTopology",
                "bevy::mesh::PrimitiveTopology::TriangleList",
            )
        }

        fn creusot_proof() -> proc_macro2::TokenStream {
            crate::verification::proof_helpers::creusot_select_wrapper(
                "bevy::mesh::PrimitiveTopology",
                "bevy::mesh::PrimitiveTopology::TriangleList",
            )
        }
    }

    impl ElicitIntrospect for bevy::mesh::PrimitiveTopology {
        fn pattern() -> ElicitationPattern {
            ElicitationPattern::Select
        }

        fn metadata() -> TypeMetadata {
            TypeMetadata {
                type_name: "bevy::mesh::PrimitiveTopology",
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

    crate::select_trenchcoat!(bevy::mesh::PrimitiveTopology, as BevyPrimitiveTopology);
}

#[cfg(not(feature = "wgpu-types"))]
pub use render_primitives::{BevyFace, BevyFrontFace, BevyPrimitiveTopology};
