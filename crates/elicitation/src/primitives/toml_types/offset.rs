//! Trenchcoat wrapper for [`toml_datetime::Offset`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A TOML UTC offset: either `Z` (UTC) or a custom hours/minutes offset.
///
/// Wraps `toml_datetime::Offset` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TomlOffset {
    /// UTC (zero offset).
    Z,
    /// Custom offset in hours and minutes from UTC.
    Custom {
        /// Offset hours (−23..=23).
        hours: i8,
        /// Offset minutes (0–59).
        minutes: u8,
    },
}

#[cfg(feature = "toml-types")]
impl From<toml_datetime::Offset> for TomlOffset {
    fn from(o: toml_datetime::Offset) -> Self {
        match o {
            toml_datetime::Offset::Z => TomlOffset::Z,
            toml_datetime::Offset::Custom { minutes: m } => {
                let abs = m.abs();
                let sign: i8 = if m < 0 { -1 } else { 1 };
                TomlOffset::Custom {
                    hours: sign * (abs / 60) as i8,
                    minutes: (abs % 60) as u8,
                }
            }
        }
    }
}

#[cfg(feature = "toml-types")]
impl From<TomlOffset> for toml_datetime::Offset {
    fn from(o: TomlOffset) -> Self {
        match o {
            TomlOffset::Z => toml_datetime::Offset::Z,
            TomlOffset::Custom { hours, minutes } => {
                let abs_minutes = (hours.unsigned_abs() as i16) * 60 + minutes as i16;
                let total = if hours < 0 { -abs_minutes } else { abs_minutes };
                toml_datetime::Offset::Custom { minutes: total }
            }
        }
    }
}

impl Prompt for TomlOffset {
    fn prompt() -> Option<&'static str> {
        Some("Choose a TOML UTC offset:")
    }
}

impl Select for TomlOffset {
    fn options() -> Vec<Self> {
        vec![
            TomlOffset::Z,
            TomlOffset::Custom {
                hours: 0,
                minutes: 0,
            },
        ]
    }

    fn labels() -> Vec<String> {
        vec!["Z".to_string(), "Custom".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Z" => Some(TomlOffset::Z),
            "Custom" => Some(TomlOffset::Custom {
                hours: 0,
                minutes: 0,
            }),
            _ => None,
        }
    }
}

crate::default_style!(TomlOffset => TomlOffsetStyle);

impl Elicitation for TomlOffset {
    type Style = TomlOffsetStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TomlOffset");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose UTC offset:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        tracing::debug!(variant = %label, "TomlOffset variant selected");
        match label.as_str() {
            "Z" => Ok(TomlOffset::Z),
            "Custom" => {
                tracing::debug!("Eliciting hours and minutes for Custom offset");
                Ok(TomlOffset::Custom {
                    hours: i8::elicit(communicator).await?,
                    minutes: u8::elicit(communicator).await?,
                })
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TomlOffset: {label}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("TomlOffset", "Z")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("TomlOffset", "Z")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("TomlOffset", "Z")
    }
}

impl ElicitIntrospect for TomlOffset {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml_datetime::Offset",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Z".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Custom".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "hours",
                                type_name: "i8",
                                prompt: Some("Offset hours (−23..=23):"),
                            },
                            FieldInfo {
                                name: "minutes",
                                type_name: "u8",
                                prompt: Some("Offset minutes (0–59):"),
                            },
                        ],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlOffset {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose UTC offset:").to_string(),
            type_name: "TomlOffset".to_string(),
            options: Self::labels(),
            branches: vec![
                None,
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Custom UTC offset:".to_string()),
                    type_name: "Custom".to_string(),
                    fields: vec![
                        (
                            "hours".to_string(),
                            Box::new(
                                i8::prompt_tree()
                                    .with_prompt(Some("Offset hours (−23..=23):".to_string())),
                            ),
                        ),
                        (
                            "minutes".to_string(),
                            Box::new(
                                u8::prompt_tree()
                                    .with_prompt(Some("Offset minutes (0–59):".to_string())),
                            ),
                        ),
                    ],
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlOffset {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            TomlOffset::Z => quote::quote! { ::elicitation::TomlOffset::Z },
            TomlOffset::Custom { hours, minutes } => {
                let hours = <i8 as crate::emit_code::ToCodeLiteral>::to_code_literal(hours);
                let minutes = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(minutes);
                quote::quote! {
                    ::elicitation::TomlOffset::Custom { hours: #hours, minutes: #minutes }
                }
            }
        }
    }
}
