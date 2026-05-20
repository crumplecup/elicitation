//! UOM quantity kind and unit system enums.
//!
//! Available with the `uom-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The 18 registered physical quantity kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UomQuantityKind {
    /// Base: metre (SI).
    Length,
    /// Base: kilogram (SI).
    Mass,
    /// Base: second (SI).
    Time,
    /// Base: kelvin (SI).
    Temperature,
    /// Base: ampere (SI).
    ElectricCurrent,
    /// Base: mole (SI).
    AmountOfSubstance,
    /// Base: candela (SI).
    LuminousIntensity,
    /// Derived: m/s.
    Velocity,
    /// Derived: m/s².
    Acceleration,
    /// Derived: kg⋅m/s² = N.
    Force,
    /// Derived: kg⋅m²/s² = J.
    Energy,
    /// Derived: J/s = W.
    Power,
    /// Derived: N/m² = Pa.
    Pressure,
    /// Derived: 1/s = Hz.
    Frequency,
    /// Derived: m².
    Area,
    /// Derived: m³.
    Volume,
    /// Derived: kg/m³.
    Density,
    /// Derived: rad.
    Angle,
}

/// Unit system used for a quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UomUnitSystem {
    /// SI (International System of Units).
    Si,
    /// Imperial / US customary.
    Imperial,
    /// Natural units.
    Natural,
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
};

// --- UomQuantityKind ---------------------------------------------------------

impl Prompt for UomQuantityKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the physical quantity kind:")
    }
}

impl Select for UomQuantityKind {
    fn options() -> Vec<Self> {
        vec![
            UomQuantityKind::Length, UomQuantityKind::Mass, UomQuantityKind::Time,
            UomQuantityKind::Temperature, UomQuantityKind::ElectricCurrent,
            UomQuantityKind::AmountOfSubstance, UomQuantityKind::LuminousIntensity,
            UomQuantityKind::Velocity, UomQuantityKind::Acceleration, UomQuantityKind::Force,
            UomQuantityKind::Energy, UomQuantityKind::Power, UomQuantityKind::Pressure,
            UomQuantityKind::Frequency, UomQuantityKind::Area, UomQuantityKind::Volume,
            UomQuantityKind::Density, UomQuantityKind::Angle,
        ]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(UomQuantityKind => UomQuantityKindStyle);

impl Elicitation for UomQuantityKind {
    type Style = UomQuantityKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UomQuantityKind");
        let params = crate::mcp::select_params(
            Self::prompt().unwrap_or("Choose quantity kind:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(crate::mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid UomQuantityKind: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "elicitation::UomQuantityKind",
            "length",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::UomQuantityKind")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum("elicitation::UomQuantityKind")
    }
}

impl ElicitIntrospect for UomQuantityKind {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::UomQuantityKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for UomQuantityKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "UomQuantityKind".to_string(),
            type_name: "UomQuantityKind".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

impl crate::emit_code::ToCodeLiteral for UomQuantityKind {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let variant = serde_json::to_string(self).unwrap();
        let variant = variant.trim_matches('"');
        // Convert snake_case label back to PascalCase variant name via serde
        let ident = match self {
            UomQuantityKind::Length           => quote::quote! { elicitation::UomQuantityKind::Length },
            UomQuantityKind::Mass             => quote::quote! { elicitation::UomQuantityKind::Mass },
            UomQuantityKind::Time             => quote::quote! { elicitation::UomQuantityKind::Time },
            UomQuantityKind::Temperature      => quote::quote! { elicitation::UomQuantityKind::Temperature },
            UomQuantityKind::ElectricCurrent  => quote::quote! { elicitation::UomQuantityKind::ElectricCurrent },
            UomQuantityKind::AmountOfSubstance => quote::quote! { elicitation::UomQuantityKind::AmountOfSubstance },
            UomQuantityKind::LuminousIntensity => quote::quote! { elicitation::UomQuantityKind::LuminousIntensity },
            UomQuantityKind::Velocity         => quote::quote! { elicitation::UomQuantityKind::Velocity },
            UomQuantityKind::Acceleration     => quote::quote! { elicitation::UomQuantityKind::Acceleration },
            UomQuantityKind::Force            => quote::quote! { elicitation::UomQuantityKind::Force },
            UomQuantityKind::Energy           => quote::quote! { elicitation::UomQuantityKind::Energy },
            UomQuantityKind::Power            => quote::quote! { elicitation::UomQuantityKind::Power },
            UomQuantityKind::Pressure         => quote::quote! { elicitation::UomQuantityKind::Pressure },
            UomQuantityKind::Frequency        => quote::quote! { elicitation::UomQuantityKind::Frequency },
            UomQuantityKind::Area             => quote::quote! { elicitation::UomQuantityKind::Area },
            UomQuantityKind::Volume           => quote::quote! { elicitation::UomQuantityKind::Volume },
            UomQuantityKind::Density          => quote::quote! { elicitation::UomQuantityKind::Density },
            UomQuantityKind::Angle            => quote::quote! { elicitation::UomQuantityKind::Angle },
        };
        let _ = variant; // suppress unused warning — match above is exhaustive
        ident
    }
}

// --- UomUnitSystem -----------------------------------------------------------

impl Prompt for UomUnitSystem {
    fn prompt() -> Option<&'static str> {
        Some("Choose the unit system:")
    }
}

impl Select for UomUnitSystem {
    fn options() -> Vec<Self> {
        vec![UomUnitSystem::Si, UomUnitSystem::Imperial, UomUnitSystem::Natural]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(UomUnitSystem => UomUnitSystemStyle);

impl Elicitation for UomUnitSystem {
    type Style = UomUnitSystemStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UomUnitSystem");
        let params = crate::mcp::select_params(
            Self::prompt().unwrap_or("Choose unit system:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(crate::mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid UomUnitSystem: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "elicitation::UomUnitSystem",
            "si",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::UomUnitSystem")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum("elicitation::UomUnitSystem")
    }
}

impl ElicitIntrospect for UomUnitSystem {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::UomUnitSystem",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for UomUnitSystem {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "UomUnitSystem".to_string(),
            type_name: "UomUnitSystem".to_string(),
            options: Self::labels(),
            branches: vec![None, None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for UomUnitSystem {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            UomUnitSystem::Si       => quote::quote! { elicitation::UomUnitSystem::Si },
            UomUnitSystem::Imperial => quote::quote! { elicitation::UomUnitSystem::Imperial },
            UomUnitSystem::Natural  => quote::quote! { elicitation::UomUnitSystem::Natural },
        }
    }
}
