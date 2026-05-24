//! Elicitation for [`accesskit::CustomAction`].

use accesskit::CustomAction;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};

impl Prompt for CustomAction {
    fn prompt() -> Option<&'static str> {
        Some("Specify a custom accessibility action (ID and description):")
    }
}

crate::default_style!(CustomAction => CustomActionStyle);

impl Elicitation for CustomAction {
    type Style = CustomActionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting accesskit::CustomAction");
        let id = i32::elicit(communicator).await?;
        let description = String::elicit(communicator).await?.into_boxed_str();
        Ok(Self { id, description })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <i32 as Elicitation>::kani_proof();
        ts.extend(<String as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <i32 as Elicitation>::verus_proof();
        ts.extend(<String as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <i32 as Elicitation>::creusot_proof();
        ts.extend(<String as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for CustomAction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "accesskit::CustomAction",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "id",
                        type_name: "i32",
                        prompt: Some(
                            "Action ID (matches CustomAction::id in the accessibility tree):",
                        ),
                    },
                    FieldInfo {
                        name: "description",
                        type_name: "Box<str>",
                        prompt: Some("Human-readable description of the action:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for CustomAction {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "accesskit::CustomAction".to_string(),
            fields: vec![
                ("id".to_string(), Box::new(i32::prompt_tree())),
                ("description".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for CustomAction {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let id = self.id;
        let desc = self.description.as_ref();
        quote::quote! {
            accesskit::CustomAction {
                id: #id,
                description: #desc.to_string().into_boxed_str(),
            }
        }
    }
}
