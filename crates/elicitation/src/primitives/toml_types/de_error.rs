//! Trenchcoat wrapper for [`toml::de::Error`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A TOML deserialization error captured as a message string.
///
/// Wraps `toml::de::Error` to add [`JsonSchema`] for MCP boundary crossing.
/// Since `toml::de::Error` does not expose its internals directly, the
/// message is extracted via [`Display`](std::fmt::Display).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TomlDeError {
    /// Human-readable error message from `toml::de::Error`.
    pub message: String,
}

#[cfg(feature = "toml-types")]
impl From<toml::de::Error> for TomlDeError {
    fn from(e: toml::de::Error) -> Self {
        TomlDeError {
            message: e.to_string(),
        }
    }
}

impl Prompt for TomlDeError {
    fn prompt() -> Option<&'static str> {
        Some("Describe the TOML deserialization error:")
    }
}

crate::default_style!(TomlDeError => TomlDeErrorStyle);

impl Elicitation for TomlDeError {
    type Style = TomlDeErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TomlDeError");
        Ok(Self {
            message: String::elicit(communicator).await?,
        })
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

impl ElicitIntrospect for TomlDeError {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml::de::Error",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "message",
                    type_name: "String",
                    prompt: Some("Error message:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlDeError {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TomlDeError".to_string(),
            fields: vec![(
                "message".to_string(),
                Box::new(String::prompt_tree().with_prompt(Some("Error message:".to_string()))),
            )],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlDeError {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let message = <String as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.message);
        quote::quote! {
            ::elicitation::TomlDeError { message: #message }
        }
    }
}
