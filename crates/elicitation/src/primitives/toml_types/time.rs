//! Trenchcoat wrapper for [`toml_datetime::Time`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A TOML local time value: hour, minute, second, and nanosecond.
///
/// Wraps `toml_datetime::Time` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TomlTime {
    /// Hour of the day (0–23).
    pub hour: u8,
    /// Minute of the hour (0–59).
    pub minute: u8,
    /// Second of the minute (0–60, allowing leap seconds).
    pub second: u8,
    /// Sub-second nanoseconds (0–999_999_999).
    pub nanosecond: u32,
}

#[cfg(feature = "toml-types")]
impl From<toml_datetime::Time> for TomlTime {
    fn from(t: toml_datetime::Time) -> Self {
        TomlTime {
            hour: t.hour,
            minute: t.minute,
            second: t.second.unwrap_or(0),
            nanosecond: t.nanosecond.unwrap_or(0),
        }
    }
}

#[cfg(feature = "toml-types")]
impl From<TomlTime> for toml_datetime::Time {
    fn from(t: TomlTime) -> Self {
        toml_datetime::Time {
            hour: t.hour,
            minute: t.minute,
            second: Some(t.second),
            nanosecond: Some(t.nanosecond),
        }
    }
}

impl Prompt for TomlTime {
    fn prompt() -> Option<&'static str> {
        Some("Enter a TOML local time (hour, minute, second, nanosecond):")
    }
}

crate::default_style!(TomlTime => TomlTimeStyle);

impl Elicitation for TomlTime {
    type Style = TomlTimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TomlTime");
        Ok(Self {
            hour: u8::elicit(communicator).await?,
            minute: u8::elicit(communicator).await?,
            second: u8::elicit(communicator).await?,
            nanosecond: u32::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u8 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u8 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u8 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TomlTime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml_datetime::Time",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "hour",
                        type_name: "u8",
                        prompt: Some("Hour (0–23):"),
                    },
                    FieldInfo {
                        name: "minute",
                        type_name: "u8",
                        prompt: Some("Minute (0–59):"),
                    },
                    FieldInfo {
                        name: "second",
                        type_name: "u8",
                        prompt: Some("Second (0–60):"),
                    },
                    FieldInfo {
                        name: "nanosecond",
                        type_name: "u32",
                        prompt: Some("Nanosecond (0–999_999_999):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlTime {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TomlTime".to_string(),
            fields: vec![
                (
                    "hour".to_string(),
                    Box::new(u8::prompt_tree().with_prompt(Some("Hour (0–23):".to_string()))),
                ),
                (
                    "minute".to_string(),
                    Box::new(u8::prompt_tree().with_prompt(Some("Minute (0–59):".to_string()))),
                ),
                (
                    "second".to_string(),
                    Box::new(u8::prompt_tree().with_prompt(Some("Second (0–60):".to_string()))),
                ),
                (
                    "nanosecond".to_string(),
                    Box::new(
                        u32::prompt_tree()
                            .with_prompt(Some("Nanosecond (0–999_999_999):".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlTime {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let hour = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.hour);
        let minute = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.minute);
        let second = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.second);
        let nanosecond =
            <u32 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.nanosecond);
        quote::quote! {
            ::elicitation::TomlTime {
                hour: #hour,
                minute: #minute,
                second: #second,
                nanosecond: #nanosecond,
            }
        }
    }
}
