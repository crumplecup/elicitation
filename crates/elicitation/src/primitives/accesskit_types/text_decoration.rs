//! Elicitation for [`accesskit::TextDecoration`].

use accesskit::{Color, TextDecoration, TextDecorationStyle};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for TextDecoration {
    fn prompt() -> Option<&'static str> {
        Some("Specify a text decoration (style and color):")
    }
}

crate::default_style!(TextDecoration => TextDecorationStyle2);

impl Elicitation for TextDecoration {
    type Style = TextDecorationStyle2;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::TextDecoration");
        Ok(Self {
            style: TextDecorationStyle::elicit(communicator).await?,
            color: Color::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextDecorationStyle as Elicitation>::kani_proof();
        ts.extend(<Color as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextDecorationStyle as Elicitation>::verus_proof();
        ts.extend(<Color as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <TextDecorationStyle as Elicitation>::creusot_proof();
        ts.extend(<Color as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for TextDecoration {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::TextDecoration",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "style",
                        type_name: "accesskit::TextDecorationStyle",
                        prompt: Some("Decoration line style:"),
                    },
                    FieldInfo {
                        name: "color",
                        type_name: "accesskit::Color",
                        prompt: Some("Decoration color (RGBA):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TextDecoration {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::TextDecoration".to_string(),
            fields: vec![
                ("style".to_string(), Box::new(TextDecorationStyle::prompt_tree())),
                ("color".to_string(), Box::new(Color::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for TextDecoration {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let style_lit = self.style.to_code_literal();
        let color_lit = self.color.to_code_literal();
        quote::quote! {
            accesskit::TextDecoration { style: #style_lit, color: #color_lit }
        }
    }
}
