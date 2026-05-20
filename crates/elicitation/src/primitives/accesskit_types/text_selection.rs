//! Elicitation for [`accesskit::TextSelection`].

use accesskit::TextSelection;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

use accesskit::TextPosition;

impl Prompt for TextSelection {
    fn prompt() -> Option<&'static str> {
        Some("Specify a text selection (anchor and focus positions):")
    }
}

crate::default_style!(TextSelection => TextSelectionStyle);

impl Elicitation for TextSelection {
    type Style = TextSelectionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TextSelection");
        Ok(Self {
            anchor: TextPosition::elicit(communicator).await?,
            focus: TextPosition::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextPosition as Elicitation>::kani_proof();
        ts.extend(<TextPosition as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextPosition as Elicitation>::verus_proof();
        ts.extend(<TextPosition as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextPosition as Elicitation>::creusot_proof();
        ts.extend(<TextPosition as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TextSelection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TextSelection",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "anchor",
                        type_name: "accesskit::TextPosition",
                        prompt: Some("Anchor position (selection start):"),
                    },
                    FieldInfo {
                        name: "focus",
                        type_name: "accesskit::TextPosition",
                        prompt: Some("Focus position (active end / caret):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TextSelection {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::TextSelection".to_string(),
            fields: vec![
                ("anchor".to_string(), Box::new(TextPosition::prompt_tree())),
                ("focus".to_string(), Box::new(TextPosition::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for TextSelection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let anchor_lit = self.anchor.to_code_literal();
        let focus_lit = self.focus.to_code_literal();
        quote::quote! {
            accesskit::TextSelection { anchor: #anchor_lit, focus: #focus_lit }
        }
    }
}
