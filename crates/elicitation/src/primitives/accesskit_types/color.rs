//! Elicitation for [`accesskit::Color`].
//!
//! `Color` is an RGBA struct used as a field of [`accesskit::TextDecoration`].
//! It is not directly in the ReadyNow list but is required to implement
//! `TextDecoration` elicitation.

use accesskit::Color;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for Color {
    fn prompt() -> Option<&'static str> {
        Some("Specify an RGBA color (red, green, blue, alpha — each 0–255):")
    }
}

crate::default_style!(Color => ColorStyle);

impl Elicitation for Color {
    type Style = ColorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::Color");
        Ok(Self {
            red: u8::elicit(communicator).await?,
            green: u8::elicit(communicator).await?,
            blue: u8::elicit(communicator).await?,
            alpha: u8::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u8 as Elicitation>::kani_proof();
        ts.extend(<u8 as Elicitation>::kani_proof());
        ts.extend(<u8 as Elicitation>::kani_proof());
        ts.extend(<u8 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u8 as Elicitation>::verus_proof();
        ts.extend(<u8 as Elicitation>::verus_proof());
        ts.extend(<u8 as Elicitation>::verus_proof());
        ts.extend(<u8 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u8 as Elicitation>::creusot_proof();
        ts.extend(<u8 as Elicitation>::creusot_proof());
        ts.extend(<u8 as Elicitation>::creusot_proof());
        ts.extend(<u8 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for Color {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::Color",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "red", type_name: "u8", prompt: Some("Red channel (0–255):") },
                    FieldInfo { name: "green", type_name: "u8", prompt: Some("Green channel (0–255):") },
                    FieldInfo { name: "blue", type_name: "u8", prompt: Some("Blue channel (0–255):") },
                    FieldInfo { name: "alpha", type_name: "u8", prompt: Some("Alpha channel (0=transparent, 255=opaque):") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for Color {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::Color".to_string(),
            fields: vec![
                ("red".to_string(), Box::new(u8::prompt_tree())),
                ("green".to_string(), Box::new(u8::prompt_tree())),
                ("blue".to_string(), Box::new(u8::prompt_tree())),
                ("alpha".to_string(), Box::new(u8::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for Color {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = self.red;
        let g = self.green;
        let b = self.blue;
        let a = self.alpha;
        quote::quote! {
            accesskit::Color { red: #r, green: #g, blue: #b, alpha: #a }
        }
    }
}
