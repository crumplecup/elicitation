//! Trenchcoat wrapper for [`toml_datetime::Date`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A TOML local date value: year, month, and day.
///
/// Wraps `toml_datetime::Date` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TomlDate {
    /// Gregorian year.
    pub year: u16,
    /// Month of the year (1–12).
    pub month: u8,
    /// Day of the month (1–31).
    pub day: u8,
}

#[cfg(feature = "toml-types")]
impl From<toml_datetime::Date> for TomlDate {
    fn from(d: toml_datetime::Date) -> Self {
        TomlDate {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }
}

#[cfg(feature = "toml-types")]
impl From<TomlDate> for toml_datetime::Date {
    fn from(d: TomlDate) -> Self {
        toml_datetime::Date {
            year: d.year,
            month: d.month,
            day: d.day,
        }
    }
}

impl Prompt for TomlDate {
    fn prompt() -> Option<&'static str> {
        Some("Enter a TOML local date (year, month, day):")
    }
}

crate::default_style!(TomlDate => TomlDateStyle);

impl Elicitation for TomlDate {
    type Style = TomlDateStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TomlDate");
        Ok(Self {
            year: u16::elicit(communicator).await?,
            month: u8::elicit(communicator).await?,
            day: u8::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for TomlDate {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml_datetime::Date",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "year",
                        type_name: "u16",
                        prompt: Some("Year:"),
                    },
                    FieldInfo {
                        name: "month",
                        type_name: "u8",
                        prompt: Some("Month (1–12):"),
                    },
                    FieldInfo {
                        name: "day",
                        type_name: "u8",
                        prompt: Some("Day (1–31):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlDate {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "TomlDate".to_string(),
            fields: vec![
                (
                    "year".to_string(),
                    Box::new(u16::prompt_tree().with_prompt(Some("Year:".to_string()))),
                ),
                (
                    "month".to_string(),
                    Box::new(u8::prompt_tree().with_prompt(Some("Month (1–12):".to_string()))),
                ),
                (
                    "day".to_string(),
                    Box::new(u8::prompt_tree().with_prompt(Some("Day (1–31):".to_string()))),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlDate {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let year = <u16 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.year);
        let month = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.month);
        let day = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.day);
        quote::quote! {
            ::elicitation::TomlDate { year: #year, month: #month, day: #day }
        }
    }
}
