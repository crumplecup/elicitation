//! Elicitation for [`accesskit::NodeId`].

use accesskit::NodeId;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for NodeId {
    fn prompt() -> Option<&'static str> {
        Some("Enter the accessibility node ID (non-zero integer):")
    }
}

crate::default_style!(NodeId => NodeIdStyle);

impl Elicitation for NodeId {
    type Style = NodeIdStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::NodeId");
        let id = u64::elicit(communicator).await?;
        Ok(NodeId(id))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for NodeId {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::NodeId",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "u64",
                    prompt: Some("Node ID value (non-zero):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for NodeId {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::NodeId".to_string(),
            fields: vec![("0".to_string(), Box::new(u64::prompt_tree()))],
        }
    }
}

impl ToCodeLiteral for NodeId {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let id = self.0;
        quote::quote! { accesskit::NodeId(#id) }
    }
}
