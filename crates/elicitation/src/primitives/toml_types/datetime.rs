//! Trenchcoat wrapper for [`toml_datetime::Datetime`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{TomlDate, TomlOffset, TomlTime};

/// A TOML datetime value covering all four TOML datetime variants.
///
/// Wraps `toml_datetime::Datetime` to add [`JsonSchema`] for MCP boundary crossing.
/// The four variants match the TOML specification exactly.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TomlDatetime {
    /// Offset datetime: date, time, and UTC offset.
    OffsetDateTime {
        /// The local date part.
        date: TomlDate,
        /// The local time part.
        time: TomlTime,
        /// The UTC offset.
        offset: TomlOffset,
    },
    /// Local datetime: date and time, no offset.
    LocalDateTime {
        /// The local date part.
        date: TomlDate,
        /// The local time part.
        time: TomlTime,
    },
    /// Local date: date only.
    LocalDate {
        /// The local date.
        date: TomlDate,
    },
    /// Local time: time only.
    LocalTime {
        /// The local time.
        time: TomlTime,
    },
}

#[cfg(feature = "toml-types")]
impl From<toml_datetime::Datetime> for TomlDatetime {
    fn from(dt: toml_datetime::Datetime) -> Self {
        match (dt.date, dt.time, dt.offset) {
            (Some(d), Some(t), Some(o)) => TomlDatetime::OffsetDateTime {
                date: TomlDate::from(d),
                time: TomlTime::from(t),
                offset: TomlOffset::from(o),
            },
            (Some(d), Some(t), None) => TomlDatetime::LocalDateTime {
                date: TomlDate::from(d),
                time: TomlTime::from(t),
            },
            (Some(d), None, _) => TomlDatetime::LocalDate {
                date: TomlDate::from(d),
            },
            (None, Some(t), _) => TomlDatetime::LocalTime {
                time: TomlTime::from(t),
            },
            _ => TomlDatetime::LocalDate {
                date: TomlDate {
                    year: 2000,
                    month: 1,
                    day: 1,
                },
            },
        }
    }
}

#[cfg(feature = "toml-types")]
impl From<TomlDatetime> for toml_datetime::Datetime {
    fn from(dt: TomlDatetime) -> Self {
        match dt {
            TomlDatetime::OffsetDateTime { date, time, offset } => toml_datetime::Datetime {
                date: Some(toml_datetime::Date {
                    year: date.year,
                    month: date.month,
                    day: date.day,
                }),
                time: Some(toml_datetime::Time {
                    hour: time.hour,
                    minute: time.minute,
                    second: Some(time.second),
                    nanosecond: Some(time.nanosecond),
                }),
                offset: Some(match offset {
                    TomlOffset::Z => toml_datetime::Offset::Z,
                    TomlOffset::Custom { hours, minutes } => {
                        let abs_minutes = (hours.unsigned_abs() as i16) * 60 + minutes as i16;
                        let total = if hours < 0 { -abs_minutes } else { abs_minutes };
                        toml_datetime::Offset::Custom { minutes: total }
                    }
                }),
            },
            TomlDatetime::LocalDateTime { date, time } => toml_datetime::Datetime {
                date: Some(toml_datetime::Date {
                    year: date.year,
                    month: date.month,
                    day: date.day,
                }),
                time: Some(toml_datetime::Time {
                    hour: time.hour,
                    minute: time.minute,
                    second: Some(time.second),
                    nanosecond: Some(time.nanosecond),
                }),
                offset: None,
            },
            TomlDatetime::LocalDate { date } => toml_datetime::Datetime {
                date: Some(toml_datetime::Date {
                    year: date.year,
                    month: date.month,
                    day: date.day,
                }),
                time: None,
                offset: None,
            },
            TomlDatetime::LocalTime { time } => toml_datetime::Datetime {
                date: None,
                time: Some(toml_datetime::Time {
                    hour: time.hour,
                    minute: time.minute,
                    second: Some(time.second),
                    nanosecond: Some(time.nanosecond),
                }),
                offset: None,
            },
        }
    }
}

impl Prompt for TomlDatetime {
    fn prompt() -> Option<&'static str> {
        Some("Choose a TOML datetime variant:")
    }
}

impl Select for TomlDatetime {
    fn options() -> Vec<Self> {
        let dummy_date = TomlDate {
            year: 2000,
            month: 1,
            day: 1,
        };
        let dummy_time = TomlTime {
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        };
        vec![
            TomlDatetime::OffsetDateTime {
                date: dummy_date,
                time: dummy_time,
                offset: TomlOffset::Z,
            },
            TomlDatetime::LocalDateTime {
                date: dummy_date,
                time: dummy_time,
            },
            TomlDatetime::LocalDate { date: dummy_date },
            TomlDatetime::LocalTime { time: dummy_time },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "OffsetDateTime".to_string(),
            "LocalDateTime".to_string(),
            "LocalDate".to_string(),
            "LocalTime".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        let dummy_date = TomlDate {
            year: 2000,
            month: 1,
            day: 1,
        };
        let dummy_time = TomlTime {
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
        };
        match label {
            "OffsetDateTime" => Some(TomlDatetime::OffsetDateTime {
                date: dummy_date,
                time: dummy_time,
                offset: TomlOffset::Z,
            }),
            "LocalDateTime" => Some(TomlDatetime::LocalDateTime {
                date: dummy_date,
                time: dummy_time,
            }),
            "LocalDate" => Some(TomlDatetime::LocalDate { date: dummy_date }),
            "LocalTime" => Some(TomlDatetime::LocalTime { time: dummy_time }),
            _ => None,
        }
    }
}

crate::default_style!(TomlDatetime => TomlDatetimeStyle);

impl Elicitation for TomlDatetime {
    type Style = TomlDatetimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting TomlDatetime");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose datetime variant:"),
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
        tracing::debug!(variant = %label, "TomlDatetime variant selected");
        match label.as_str() {
            "OffsetDateTime" => {
                tracing::debug!("Eliciting OffsetDateTime fields");
                Ok(TomlDatetime::OffsetDateTime {
                    date: TomlDate::elicit(communicator).await?,
                    time: TomlTime::elicit(communicator).await?,
                    offset: TomlOffset::elicit(communicator).await?,
                })
            }
            "LocalDateTime" => {
                tracing::debug!("Eliciting LocalDateTime fields");
                Ok(TomlDatetime::LocalDateTime {
                    date: TomlDate::elicit(communicator).await?,
                    time: TomlTime::elicit(communicator).await?,
                })
            }
            "LocalDate" => {
                tracing::debug!("Eliciting LocalDate field");
                Ok(TomlDatetime::LocalDate {
                    date: TomlDate::elicit(communicator).await?,
                })
            }
            "LocalTime" => {
                tracing::debug!("Eliciting LocalTime field");
                Ok(TomlDatetime::LocalTime {
                    time: TomlTime::elicit(communicator).await?,
                })
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TomlDatetime: {label}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("TomlDatetime", "LocalDate")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("TomlDatetime", "LocalDate")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("TomlDatetime", "LocalDate")
    }
}

impl ElicitIntrospect for TomlDatetime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "toml_datetime::Datetime",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "OffsetDateTime".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "date",
                                type_name: "TomlDate",
                                prompt: Some("Date:"),
                            },
                            FieldInfo {
                                name: "time",
                                type_name: "TomlTime",
                                prompt: Some("Time:"),
                            },
                            FieldInfo {
                                name: "offset",
                                type_name: "TomlOffset",
                                prompt: Some("UTC offset:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "LocalDateTime".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "date",
                                type_name: "TomlDate",
                                prompt: Some("Date:"),
                            },
                            FieldInfo {
                                name: "time",
                                type_name: "TomlTime",
                                prompt: Some("Time:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "LocalDate".to_string(),
                        fields: vec![FieldInfo {
                            name: "date",
                            type_name: "TomlDate",
                            prompt: Some("Date:"),
                        }],
                    },
                    VariantMetadata {
                        label: "LocalTime".to_string(),
                        fields: vec![FieldInfo {
                            name: "time",
                            type_name: "TomlTime",
                            prompt: Some("Time:"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for TomlDatetime {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose datetime variant:")
                .to_string(),
            type_name: "TomlDatetime".to_string(),
            options: Self::labels(),
            branches: vec![
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Offset datetime:".to_string()),
                    type_name: "OffsetDateTime".to_string(),
                    fields: vec![
                        (
                            "date".to_string(),
                            Box::new(
                                TomlDate::prompt_tree().with_prompt(Some("Date:".to_string())),
                            ),
                        ),
                        (
                            "time".to_string(),
                            Box::new(
                                TomlTime::prompt_tree().with_prompt(Some("Time:".to_string())),
                            ),
                        ),
                        (
                            "offset".to_string(),
                            Box::new(
                                TomlOffset::prompt_tree()
                                    .with_prompt(Some("UTC offset:".to_string())),
                            ),
                        ),
                    ],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Local datetime:".to_string()),
                    type_name: "LocalDateTime".to_string(),
                    fields: vec![
                        (
                            "date".to_string(),
                            Box::new(
                                TomlDate::prompt_tree().with_prompt(Some("Date:".to_string())),
                            ),
                        ),
                        (
                            "time".to_string(),
                            Box::new(
                                TomlTime::prompt_tree().with_prompt(Some("Time:".to_string())),
                            ),
                        ),
                    ],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Local date:".to_string()),
                    type_name: "LocalDate".to_string(),
                    fields: vec![(
                        "date".to_string(),
                        Box::new(TomlDate::prompt_tree().with_prompt(Some("Date:".to_string()))),
                    )],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Local time:".to_string()),
                    type_name: "LocalTime".to_string(),
                    fields: vec![(
                        "time".to_string(),
                        Box::new(TomlTime::prompt_tree().with_prompt(Some("Time:".to_string()))),
                    )],
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for TomlDatetime {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            TomlDatetime::OffsetDateTime { date, time, offset } => {
                let d = <TomlDate as crate::emit_code::ToCodeLiteral>::to_code_literal(date);
                let t = <TomlTime as crate::emit_code::ToCodeLiteral>::to_code_literal(time);
                let o = <TomlOffset as crate::emit_code::ToCodeLiteral>::to_code_literal(offset);
                quote::quote! {
                    ::elicitation::TomlDatetime::OffsetDateTime {
                        date: #d,
                        time: #t,
                        offset: #o,
                    }
                }
            }
            TomlDatetime::LocalDateTime { date, time } => {
                let d = <TomlDate as crate::emit_code::ToCodeLiteral>::to_code_literal(date);
                let t = <TomlTime as crate::emit_code::ToCodeLiteral>::to_code_literal(time);
                quote::quote! {
                    ::elicitation::TomlDatetime::LocalDateTime { date: #d, time: #t }
                }
            }
            TomlDatetime::LocalDate { date } => {
                let d = <TomlDate as crate::emit_code::ToCodeLiteral>::to_code_literal(date);
                quote::quote! {
                    ::elicitation::TomlDatetime::LocalDate { date: #d }
                }
            }
            TomlDatetime::LocalTime { time } => {
                let t = <TomlTime as crate::emit_code::ToCodeLiteral>::to_code_literal(time);
                quote::quote! {
                    ::elicitation::TomlDatetime::LocalTime { time: #t }
                }
            }
        }
    }
}
