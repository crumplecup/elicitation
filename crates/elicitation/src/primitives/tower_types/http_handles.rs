//! UUID handle trenchcoat for tower-http runtime service wrappers.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// UUID handle for a live tower-http layered service stored in the plugin registry.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerHttpServiceHandle {
    /// Registry key (UUID) for this service instance.
    pub id: String,
}

crate::default_style!(TowerHttpServiceHandle => TowerHttpServiceHandleStyle);

impl Prompt for TowerHttpServiceHandle {
    fn prompt() -> Option<&'static str> {
        Some("Enter the tower-http service registry UUID:")
    }
}

impl Elicitation for TowerHttpServiceHandle {
    type Style = TowerHttpServiceHandleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerHttpServiceHandle");
        let id = String::elicit(communicator).await?;
        Ok(Self { id })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TowerHttpServiceHandle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower_http service (handle)",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "id",
                    type_name: "String",
                    prompt: Some("Service UUID:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerHttpServiceHandle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerHttpServiceHandle".to_string(),
            fields: vec![("id".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}
