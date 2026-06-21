//! Bevy 0.19 atmosphere type elicitation.
//!
//! Covers:
//! - [`BevyFalloff`] — owned trenchcoat for `bevy::light::atmosphere::Falloff`.
//! - [`BevyPhaseFunction`] — owned trenchcoat for `bevy::light::atmosphere::PhaseFunction`.
//! - [`BevyScatteringTerm`] — owned local struct for `bevy::light::atmosphere::ScatteringTerm`.
//! - [`BevyAtmosphere`] — owned local struct for the serializable fields of
//!   `bevy::light::Atmosphere` (the `Handle<ScatteringMedium>` field is excluded).

use super::vec::BevyVec3;
use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, Survey, TypeMetadata,
    VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── BevyFalloff ───────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::light::atmosphere::Falloff`].
///
/// Covers `Linear`, `Exponential`, and `Tent` variants; the `Curve` variant
/// (which holds an `Arc<dyn Curve<f32>>`) is not serializable and is excluded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "variant", rename_all = "snake_case")]
pub enum BevyFalloff {
    /// Linear falloff: f(p) = p.
    Linear,
    /// Exponential falloff parametrized by a proportional scale.
    Exponential {
        /// The scale of the exponential falloff.
        scale: f32,
    },
    /// Tent-shaped falloff with a triangular peak at `center`.
    Tent {
        /// Centre of the tent peak [0, 1].
        center: f32,
        /// Total width of the tent peak [0, 1].
        width: f32,
    },
}

impl From<BevyFalloff> for bevy::light::atmosphere::Falloff {
    fn from(f: BevyFalloff) -> Self {
        match f {
            BevyFalloff::Linear => Self::Linear,
            BevyFalloff::Exponential { scale } => Self::Exponential { scale },
            BevyFalloff::Tent { center, width } => Self::Tent { center, width },
        }
    }
}

impl From<&bevy::light::atmosphere::Falloff> for BevyFalloff {
    fn from(f: &bevy::light::atmosphere::Falloff) -> Self {
        match f {
            bevy::light::atmosphere::Falloff::Linear => Self::Linear,
            bevy::light::atmosphere::Falloff::Exponential { scale } => {
                Self::Exponential { scale: *scale }
            }
            bevy::light::atmosphere::Falloff::Tent { center, width } => Self::Tent {
                center: *center,
                width: *width,
            },
            // Curve variant is not serializable; fall back to Linear.
            bevy::light::atmosphere::Falloff::Curve(_) => Self::Linear,
        }
    }
}

impl Prompt for BevyFalloff {
    fn prompt() -> Option<&'static str> {
        Some("Select atmosphere falloff distribution:")
    }
}

// Internal kind enum for BevyFalloff variant selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BevyFalloffKind {
    Linear,
    Exponential,
    Tent,
}

impl Prompt for BevyFalloffKind {
    fn prompt() -> Option<&'static str> {
        Some("Falloff variant:")
    }
}

impl Select for BevyFalloffKind {
    fn options() -> Vec<Self> {
        vec![Self::Linear, Self::Exponential, Self::Tent]
    }

    fn labels() -> Vec<String> {
        vec!["Linear".into(), "Exponential".into(), "Tent".into()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Linear" => Some(Self::Linear),
            "Exponential" => Some(Self::Exponential),
            "Tent" => Some(Self::Tent),
            _ => None,
        }
    }
}

crate::default_style!(BevyFalloff => BevyFalloffStyle);

impl Elicitation for BevyFalloff {
    type Style = BevyFalloffStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyFalloff"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let kind_params = mcp::select_params(
            BevyFalloffKind::prompt().unwrap_or("Falloff variant:"),
            &BevyFalloffKind::labels(),
        );
        let kind_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(kind_params),
            )
            .await?;
        let kind_value = mcp::extract_value(kind_result)?;
        let kind_label = mcp::parse_string(kind_value)?;
        let kind = BevyFalloffKind::from_label(&kind_label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyFalloff variant: {kind_label}"
            )))
        })?;
        match kind {
            BevyFalloffKind::Linear => Ok(Self::Linear),
            BevyFalloffKind::Exponential => {
                let scale = f32::elicit(communicator).await?;
                Ok(Self::Exponential { scale })
            }
            BevyFalloffKind::Tent => {
                let center = f32::elicit(communicator).await?;
                let width = f32::elicit(communicator).await?;
                Ok(Self::Tent { center, width })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyFalloff",
            "BevyFalloff::Linear",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyFalloff",
            "BevyFalloff::Linear",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyFalloff",
            "BevyFalloff::Linear",
        )
    }
}

impl ElicitIntrospect for BevyFalloff {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyFalloff",
            description: Some(
                "Atmosphere density falloff distribution (Linear, Exponential, Tent)",
            ),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Linear".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Exponential".to_string(),
                        fields: vec![FieldInfo {
                            name: "scale",
                            prompt: Some("Exponential scale factor:"),
                            type_name: "f32",
                        }],
                    },
                    VariantMetadata {
                        label: "Tent".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "center",
                                prompt: Some("Center of tent peak [0, 1]:"),
                                type_name: "f32",
                            },
                            FieldInfo {
                                name: "width",
                                prompt: Some("Total width of tent peak [0, 1]:"),
                                type_name: "f32",
                            },
                        ],
                    },
                ],
            },
        }
    }
}

// ── BevyPhaseFunction ─────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::light::atmosphere::PhaseFunction`].
///
/// Covers `Isotropic`, `Rayleigh`, and `Mie`; the `Curve` variant is excluded.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "variant", rename_all = "snake_case")]
pub enum BevyPhaseFunction {
    /// Uniform scattering in all directions.
    Isotropic,
    /// Rayleigh scattering (wavelength-dependent, small gas molecules).
    Rayleigh,
    /// Henyey-Greenstein approximation of Mie scattering (aerosol particles).
    Mie {
        /// Forward/backward scattering bias [-1, 1].
        asymmetry: f32,
    },
}

impl From<BevyPhaseFunction> for bevy::light::atmosphere::PhaseFunction {
    fn from(f: BevyPhaseFunction) -> Self {
        match f {
            BevyPhaseFunction::Isotropic => Self::Isotropic,
            BevyPhaseFunction::Rayleigh => Self::Rayleigh,
            BevyPhaseFunction::Mie { asymmetry } => Self::Mie { asymmetry },
        }
    }
}

impl From<&bevy::light::atmosphere::PhaseFunction> for BevyPhaseFunction {
    fn from(f: &bevy::light::atmosphere::PhaseFunction) -> Self {
        match f {
            bevy::light::atmosphere::PhaseFunction::Isotropic => Self::Isotropic,
            bevy::light::atmosphere::PhaseFunction::Rayleigh => Self::Rayleigh,
            bevy::light::atmosphere::PhaseFunction::Mie { asymmetry } => Self::Mie {
                asymmetry: *asymmetry,
            },
            // Non-serializable variants fall back to Isotropic.
            bevy::light::atmosphere::PhaseFunction::Curve(_)
            | bevy::light::atmosphere::PhaseFunction::ChromaticCurve(_)
            | bevy::light::atmosphere::PhaseFunction::ChromaticTexture(_) => Self::Isotropic,
        }
    }
}

impl Prompt for BevyPhaseFunction {
    fn prompt() -> Option<&'static str> {
        Some("Select atmosphere phase function:")
    }
}

// Internal kind enum for BevyPhaseFunction variant selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BevyPhaseFunctionKind {
    Isotropic,
    Rayleigh,
    Mie,
}

impl Prompt for BevyPhaseFunctionKind {
    fn prompt() -> Option<&'static str> {
        Some("Phase function variant:")
    }
}

impl Select for BevyPhaseFunctionKind {
    fn options() -> Vec<Self> {
        vec![Self::Isotropic, Self::Rayleigh, Self::Mie]
    }

    fn labels() -> Vec<String> {
        vec!["Isotropic".into(), "Rayleigh".into(), "Mie".into()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Isotropic" => Some(Self::Isotropic),
            "Rayleigh" => Some(Self::Rayleigh),
            "Mie" => Some(Self::Mie),
            _ => None,
        }
    }
}

crate::default_style!(BevyPhaseFunction => BevyPhaseFunctionStyle);

impl Elicitation for BevyPhaseFunction {
    type Style = BevyPhaseFunctionStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyPhaseFunction"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let kind_params = mcp::select_params(
            BevyPhaseFunctionKind::prompt().unwrap_or("Phase function:"),
            &BevyPhaseFunctionKind::labels(),
        );
        let kind_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(kind_params),
            )
            .await?;
        let kind_value = mcp::extract_value(kind_result)?;
        let kind_label = mcp::parse_string(kind_value)?;
        let kind = BevyPhaseFunctionKind::from_label(&kind_label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyPhaseFunction variant: {kind_label}"
            )))
        })?;
        match kind {
            BevyPhaseFunctionKind::Isotropic => Ok(Self::Isotropic),
            BevyPhaseFunctionKind::Rayleigh => Ok(Self::Rayleigh),
            BevyPhaseFunctionKind::Mie => {
                let asymmetry = f32::elicit(communicator).await?;
                Ok(Self::Mie { asymmetry })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "BevyPhaseFunction",
            "BevyPhaseFunction::Isotropic",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "BevyPhaseFunction",
            "BevyPhaseFunction::Isotropic",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "BevyPhaseFunction",
            "BevyPhaseFunction::Isotropic",
        )
    }
}

impl ElicitIntrospect for BevyPhaseFunction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyPhaseFunction",
            description: Some("Light scattering phase function (Isotropic, Rayleigh, Mie)"),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Isotropic".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Rayleigh".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Mie".to_string(),
                        fields: vec![FieldInfo {
                            name: "asymmetry",
                            prompt: Some("Mie asymmetry parameter [-1, 1]:"),
                            type_name: "f32",
                        }],
                    },
                ],
            },
        }
    }
}

// ── BevyScatteringTerm ────────────────────────────────────────────────────────

/// Owned Survey for [`bevy::light::atmosphere::ScatteringTerm`].
///
/// Represents one optical element (e.g. Rayleigh gas or Mie aerosol) that
/// composes a [`bevy::light::atmosphere::ScatteringMedium`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyScatteringTerm {
    /// Optical absorption density per metre (RGB ≈ wavelength channels).
    pub absorption: BevyVec3,
    /// Optical scattering density per metre (RGB ≈ wavelength channels).
    pub scattering: BevyVec3,
    /// Falloff distribution for this term.
    pub falloff: BevyFalloff,
    /// Phase function for this term.
    pub phase: BevyPhaseFunction,
}

impl From<BevyScatteringTerm> for bevy::light::atmosphere::ScatteringTerm {
    fn from(t: BevyScatteringTerm) -> Self {
        Self {
            absorption: t.absorption.into_inner(),
            scattering: t.scattering.into_inner(),
            falloff: t.falloff.into(),
            phase: t.phase.into(),
        }
    }
}

impl From<&bevy::light::atmosphere::ScatteringTerm> for BevyScatteringTerm {
    fn from(t: &bevy::light::atmosphere::ScatteringTerm) -> Self {
        Self {
            absorption: t.absorption.into(),
            scattering: t.scattering.into(),
            falloff: (&t.falloff).into(),
            phase: (&t.phase).into(),
        }
    }
}

impl Prompt for BevyScatteringTerm {
    fn prompt() -> Option<&'static str> {
        Some("Enter scattering term parameters:")
    }
}

impl Survey for BevyScatteringTerm {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "absorption",
                prompt: Some("Absorption density (Vec3, units: m⁻¹):"),
                type_name: "BevyVec3",
            },
            FieldInfo {
                name: "scattering",
                prompt: Some("Scattering density (Vec3, units: m⁻¹):"),
                type_name: "BevyVec3",
            },
            FieldInfo {
                name: "falloff",
                prompt: Some("Falloff distribution:"),
                type_name: "BevyFalloff",
            },
            FieldInfo {
                name: "phase",
                prompt: Some("Phase function:"),
                type_name: "BevyPhaseFunction",
            },
        ]
    }
}

crate::default_style!(BevyScatteringTerm => BevyScatteringTermStyle);

impl Elicitation for BevyScatteringTerm {
    type Style = BevyScatteringTermStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyScatteringTerm"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let absorption = BevyVec3::elicit(communicator).await?;
        let scattering = BevyVec3::elicit(communicator).await?;
        let falloff = BevyFalloff::elicit(communicator).await?;
        let phase = BevyPhaseFunction::elicit(communicator).await?;
        Ok(Self {
            absorption,
            scattering,
            falloff,
            phase,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVec3 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyScatteringTerm {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyScatteringTerm",
            description: Some(
                "Individual optical element of an atmosphere medium (absorption, scattering, falloff, phase)",
            ),
            details: PatternDetails::Survey {
                fields: Self::fields(),
            },
        }
    }
}

// ── BevyAtmosphere ────────────────────────────────────────────────────────────

/// Owned Survey for the serializable fields of [`bevy::light::Atmosphere`].
///
/// The `medium: Handle<ScatteringMedium>` field is excluded; code generation
/// must supply the handle — e.g. via `Atmosphere::earthlike(medium)` — or
/// construct the component manually.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyAtmosphere {
    /// Radius of the planet surface in metres.
    pub bottom_radius: f32,
    /// Radius of the outer atmosphere boundary in metres.
    pub top_radius: f32,
    /// Average albedo (colour) of the planet surface.
    pub ground_albedo: BevyVec3,
}

impl Prompt for BevyAtmosphere {
    fn prompt() -> Option<&'static str> {
        Some("Enter atmosphere parameters:")
    }
}

impl Survey for BevyAtmosphere {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "bottom_radius",
                prompt: Some("Planet radius [m] (Earth ≈ 6 360 000):"),
                type_name: "f32",
            },
            FieldInfo {
                name: "top_radius",
                prompt: Some("Outer atmosphere radius [m] (Earth ≈ 6 460 000):"),
                type_name: "f32",
            },
            FieldInfo {
                name: "ground_albedo",
                prompt: Some("Average planet surface albedo (Vec3):"),
                type_name: "BevyVec3",
            },
        ]
    }
}

crate::default_style!(BevyAtmosphere => BevyAtmosphereStyle);

impl Elicitation for BevyAtmosphere {
    type Style = BevyAtmosphereStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyAtmosphere"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let bottom_radius = f32::elicit(communicator).await?;
        let top_radius = f32::elicit(communicator).await?;
        let ground_albedo = BevyVec3::elicit(communicator).await?;
        Ok(Self {
            bottom_radius,
            top_radius,
            ground_albedo,
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

impl ElicitIntrospect for BevyAtmosphere {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyAtmosphere",
            description: Some("Atmospheric scattering parameters for an HDR camera component"),
            details: PatternDetails::Survey {
                fields: Self::fields(),
            },
        }
    }
}

// ── ElicitPromptTree + ToCodeLiteral ──────────────────────────────────────────

impl crate::ElicitPromptTree for BevyFalloff {
    fn prompt_tree() -> crate::PromptTree {
        let f_leaf = crate::PromptTree::Leaf {
            prompt: "Value:".to_string(),
            type_name: "f32".to_string(),
        };
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Falloff distribution:")
                .to_string(),
            type_name: "BevyFalloff".to_string(),
            options: vec!["Linear".into(), "Exponential".into(), "Tent".into()],
            branches: vec![
                None,
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyFalloff::Exponential".to_string(),
                    fields: vec![("scale".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyFalloff::Tent".to_string(),
                    fields: vec![
                        ("center".to_string(), Box::new(f_leaf.clone())),
                        ("width".to_string(), Box::new(f_leaf)),
                    ],
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyFalloff {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Linear => {
                quote::quote! { bevy::light::atmosphere::Falloff::Linear }
            }
            Self::Exponential { scale } => {
                quote::quote! { bevy::light::atmosphere::Falloff::Exponential { scale: #scale } }
            }
            Self::Tent { center, width } => {
                quote::quote! {
                    bevy::light::atmosphere::Falloff::Tent { center: #center, width: #width }
                }
            }
        }
    }
}

impl crate::ElicitPromptTree for BevyPhaseFunction {
    fn prompt_tree() -> crate::PromptTree {
        let f_leaf = crate::PromptTree::Leaf {
            prompt: "Asymmetry factor [-1, 1]:".to_string(),
            type_name: "f32".to_string(),
        };
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Phase function:").to_string(),
            type_name: "BevyPhaseFunction".to_string(),
            options: vec!["Isotropic".into(), "Rayleigh".into(), "Mie".into()],
            branches: vec![
                None,
                None,
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyPhaseFunction::Mie".to_string(),
                    fields: vec![("asymmetry".to_string(), Box::new(f_leaf))],
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPhaseFunction {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Isotropic => {
                quote::quote! { bevy::light::atmosphere::PhaseFunction::Isotropic }
            }
            Self::Rayleigh => {
                quote::quote! { bevy::light::atmosphere::PhaseFunction::Rayleigh }
            }
            Self::Mie { asymmetry } => {
                quote::quote! {
                    bevy::light::atmosphere::PhaseFunction::Mie { asymmetry: #asymmetry }
                }
            }
        }
    }
}

impl crate::ElicitPromptTree for BevyScatteringTerm {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: None,
            type_name: "BevyScatteringTerm".to_string(),
            fields: vec![
                ("absorption".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("scattering".to_string(), Box::new(BevyVec3::prompt_tree())),
                ("falloff".to_string(), Box::new(BevyFalloff::prompt_tree())),
                (
                    "phase".to_string(),
                    Box::new(BevyPhaseFunction::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyScatteringTerm {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let absorption = crate::emit_code::ToCodeLiteral::to_code_literal(&self.absorption);
        let scattering = crate::emit_code::ToCodeLiteral::to_code_literal(&self.scattering);
        let falloff = crate::emit_code::ToCodeLiteral::to_code_literal(&self.falloff);
        let phase = crate::emit_code::ToCodeLiteral::to_code_literal(&self.phase);
        quote::quote! {
            bevy::light::atmosphere::ScatteringTerm {
                absorption: #absorption,
                scattering: #scattering,
                falloff: #falloff,
                phase: #phase,
            }
        }
    }
}

impl crate::ElicitPromptTree for BevyAtmosphere {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyAtmosphere".to_string(),
            fields: vec![
                ("bottom_radius".to_string(), Box::new(f32::prompt_tree())),
                ("top_radius".to_string(), Box::new(f32::prompt_tree())),
                (
                    "ground_albedo".to_string(),
                    Box::new(BevyVec3::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAtmosphere {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let bottom_radius = self.bottom_radius;
        let top_radius = self.top_radius;
        let ground_albedo = crate::emit_code::ToCodeLiteral::to_code_literal(&self.ground_albedo);
        quote::quote! {
            bevy::light::Atmosphere {
                bottom_radius: #bottom_radius,
                top_radius: #top_radius,
                ground_albedo: #ground_albedo,
                ..Default::default()
            }
        }
    }
}
