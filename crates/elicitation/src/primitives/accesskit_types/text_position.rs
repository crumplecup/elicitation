//! Elicitation for [`accesskit::TextPosition`].

use accesskit::{NodeId, TextPosition};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for TextPosition {
    fn prompt() -> Option<&'static str> {
        Some("Specify a text position (node ID and character index):")
    }
}

crate::default_style!(TextPosition => TextPositionStyle);

impl Elicitation for TextPosition {
    type Style = TextPositionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TextPosition");
        Ok(Self {
            node: NodeId::elicit(communicator).await?,
            character_index: usize::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::kani_proof();
        ts.extend(<usize as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::verus_proof();
        ts.extend(<usize as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <NodeId as Elicitation>::creusot_proof();
        ts.extend(<usize as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TextPosition {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TextPosition",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "node",
                        type_name: "accesskit::NodeId",
                        prompt: Some("TextRun node ID:"),
                    },
                    FieldInfo {
                        name: "character_index",
                        type_name: "usize",
                        prompt: Some("Character index within the text run:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TextPosition {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::TextPosition".to_string(),
            fields: vec![
                ("node".to_string(), Box::new(NodeId::prompt_tree())),
                ("character_index".to_string(), Box::new(usize::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for TextPosition {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let node_lit = self.node.to_code_literal();
        let idx = self.character_index;
        quote::quote! {
            accesskit::TextPosition { node: #node_lit, character_index: #idx }
        }
    }
}
