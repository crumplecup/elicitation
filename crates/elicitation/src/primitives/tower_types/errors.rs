//! Tower error type trenchcoats.
//!
//! All tower error types have opaque or private fields; we model them as
//! unit/string-carrying structs that capture the error message at the
//! time of conversion.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

macro_rules! tower_unit_error {
    (
        $name:ident,
        type_name = $type_name:literal,
        prompt = $prompt:literal $(,)?
    ) => {
        #[doc = concat!("Serializable mirror for `", $type_name, "` (unit error).")]
        #[derive(
            Debug,
            Clone,
            PartialEq,
            serde::Serialize,
            serde::Deserialize,
            schemars::JsonSchema,
            elicitation_derive::ToCodeLiteral,
        )]
        pub struct $name;

        paste::paste! {
            crate::default_style!($name => [<$name Style>]);
        }

        impl Prompt for $name {
            fn prompt() -> Option<&'static str> {
                Some($prompt)
            }
        }

        impl Elicitation for $name {
            type Style = paste::paste! { [<$name Style>] };

            #[tracing::instrument(skip(_communicator))]
            async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($name), " (unit error)"));
                Ok(Self)
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_unit_struct(stringify!($name))
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_unit_struct(stringify!($name))
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_unit_struct(stringify!($name))
            }
        }

        impl ElicitIntrospect for $name {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Survey
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: $type_name,
                    description: Self::prompt(),
                    details: PatternDetails::Survey { fields: vec![] },
                }
            }
        }

        impl crate::ElicitPromptTree for $name {
            fn prompt_tree() -> crate::PromptTree {
                crate::PromptTree::Survey {
                    prompt: Self::prompt().map(str::to_string),
                    type_name: stringify!($name).to_string(),
                    fields: vec![],
                }
            }
        }
    };
}

tower_unit_error!(
    TowerElapsed,
    type_name = "tower::timeout::error::Elapsed",
    prompt = "Timeout elapsed error:"
);

tower_unit_error!(
    TowerOverloaded,
    type_name = "tower::load_shed::error::Overloaded",
    prompt = "Service overloaded error:"
);

tower_unit_error!(
    TowerClosed,
    type_name = "tower::buffer::error::Closed",
    prompt = "Buffer worker closed error:"
);

/// Serializable mirror for [`tower::buffer::error::ServiceError`].
///
/// The inner error is captured as a string message since `BoxError` is not serializable.
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct TowerServiceError {
    /// Error message from the underlying service failure.
    pub message: String,
}

crate::default_style!(TowerServiceError => TowerServiceErrorStyle);

impl Prompt for TowerServiceError {
    fn prompt() -> Option<&'static str> {
        Some("Enter buffered service error message:")
    }
}

impl Elicitation for TowerServiceError {
    type Style = TowerServiceErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TowerServiceError");
        let message = String::elicit(communicator).await?;
        Ok(Self { message })
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

impl ElicitIntrospect for TowerServiceError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tower::buffer::error::ServiceError",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![crate::FieldInfo {
                    name: "message",
                    type_name: "String",
                    prompt: Some("Error message:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TowerServiceError {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TowerServiceError".to_string(),
            fields: vec![("message".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}
