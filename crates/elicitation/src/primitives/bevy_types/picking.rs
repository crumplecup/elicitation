//! Bevy picking type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── PickingInteraction ────────────────────────────────────────────────────────

impl Prompt for bevy::picking::hover::PickingInteraction {
    fn prompt() -> Option<&'static str> {
        Some("Picking interaction state:")
    }
}

impl Select for bevy::picking::hover::PickingInteraction {
    fn options() -> Vec<Self> {
        vec![Self::None, Self::Hovered, Self::Pressed]
    }

    fn labels() -> Vec<String> {
        vec!["None".into(), "Hovered".into(), "Pressed".into()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "None" => Some(Self::None),
            "Hovered" => Some(Self::Hovered),
            "Pressed" => Some(Self::Pressed),
            _ => None,
        }
    }
}

crate::default_style!(bevy::picking::hover::PickingInteraction => PickingInteractionStyle);

impl Elicitation for bevy::picking::hover::PickingInteraction {
    type Style = PickingInteractionStyle;

    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::picking::hover::PickingInteraction")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose interaction:"),
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
                "Invalid PickingInteraction: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "bevy::picking::hover::PickingInteraction",
            "None",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "bevy::picking::hover::PickingInteraction",
            "None",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "bevy::picking::hover::PickingInteraction",
            "None",
        )
    }
}

impl ElicitIntrospect for bevy::picking::hover::PickingInteraction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::picking::hover::PickingInteraction",
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

crate::select_trenchcoat!(bevy::picking::hover::PickingInteraction, as BevyPickingInteraction);
crate::select_trenchcoat_traits!(
    BevyPickingInteraction,
    bevy::picking::hover::PickingInteraction,
    [copy, eq]
);

// ── BevyPickable ─────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::picking::Pickable`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyPickable {
    /// Block entities below this one from being picked.
    pub should_block_lower: bool,
    /// Whether this entity can be hovered.
    pub is_hoverable: bool,
}

crate::default_style!(BevyPickable => BevyPickableStyle);

impl From<bevy::picking::Pickable> for BevyPickable {
    fn from(p: bevy::picking::Pickable) -> Self {
        Self {
            should_block_lower: p.should_block_lower,
            is_hoverable: p.is_hoverable,
        }
    }
}

impl From<BevyPickable> for bevy::picking::Pickable {
    fn from(p: BevyPickable) -> Self {
        bevy::picking::Pickable {
            should_block_lower: p.should_block_lower,
            is_hoverable: p.is_hoverable,
        }
    }
}

impl Prompt for BevyPickable {
    fn prompt() -> Option<&'static str> {
        Some("Picking behaviour (block lower, hoverable):")
    }
}

impl Elicitation for BevyPickable {
    type Style = BevyPickableStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyPickable"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            should_block_lower: bool::elicit(communicator).await?,
            is_hoverable: bool::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <bool as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyPickable {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyPickable",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "should_block_lower",
                        type_name: "bool",
                        prompt: Some("Block lower entities from being picked?"),
                    },
                    FieldInfo {
                        name: "is_hoverable",
                        type_name: "bool",
                        prompt: Some("Can this entity be hovered?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyPickable {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyPickable".to_string(),
            fields: vec![
                (
                    "should_block_lower".to_string(),
                    Box::new(bool::prompt_tree()),
                ),
                ("is_hoverable".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyPickable {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let block = self.should_block_lower;
        let hover = self.is_hoverable;
        quote::quote! {
            bevy::picking::Pickable { should_block_lower: #block, is_hoverable: #hover }
        }
    }
}
