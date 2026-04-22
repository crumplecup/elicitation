//! Trenchcoat wrapper for [`csv::ErrorKind`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The category of a CSV processing error.
///
/// Wraps `csv::ErrorKind` to add [`JsonSchema`] for MCP error reporting.
/// Only the variants that can reasonably be described without borrowing
/// internal state are included; all others map to [`CsvErrorKind::Other`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CsvErrorKind {
    /// An I/O error occurred while reading or writing.
    Io,
    /// A record had an unexpected number of fields.
    UnequalLengths {
        /// Number of fields expected (from the first record).
        expected_len: u64,
        /// Position of the first record.
        pos: Option<u64>,
        /// Number of fields found in the current record.
        len: u64,
    },
    /// UTF-8 validation failed on a field.
    Utf8 {
        /// Index of the field that failed UTF-8 validation.
        field: usize,
    },
    /// A Serde deserialization error occurred.
    Deserialize {
        /// Human-readable description of the deserialization error.
        message: String,
    },
    /// Any other error kind not listed above.
    Other,
}

#[cfg(feature = "csv-types")]
impl From<&csv::ErrorKind> for CsvErrorKind {
    fn from(k: &csv::ErrorKind) -> Self {
        match k {
            csv::ErrorKind::Io(_) => CsvErrorKind::Io,
            csv::ErrorKind::UnequalLengths {
                expected_len,
                pos,
                len,
            } => CsvErrorKind::UnequalLengths {
                expected_len: *expected_len,
                pos: pos.as_ref().map(|p| p.byte()),
                len: *len,
            },
            csv::ErrorKind::Utf8 { err, .. } => CsvErrorKind::Utf8 { field: err.field() },
            csv::ErrorKind::Deserialize { err, .. } => CsvErrorKind::Deserialize {
                message: err.to_string(),
            },
            _ => CsvErrorKind::Other,
        }
    }
}

#[cfg(feature = "csv-types")]
impl From<&csv::Error> for CsvErrorKind {
    fn from(e: &csv::Error) -> Self {
        CsvErrorKind::from(e.kind())
    }
}

impl Prompt for CsvErrorKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the CSV error kind:")
    }
}

impl Select for CsvErrorKind {
    fn options() -> Vec<Self> {
        vec![
            CsvErrorKind::Io,
            CsvErrorKind::UnequalLengths {
                expected_len: 0,
                pos: None,
                len: 0,
            },
            CsvErrorKind::Utf8 { field: 0 },
            CsvErrorKind::Deserialize {
                message: String::new(),
            },
            CsvErrorKind::Other,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Io".to_string(),
            "UnequalLengths".to_string(),
            "Utf8".to_string(),
            "Deserialize".to_string(),
            "Other".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Io" => Some(CsvErrorKind::Io),
            "UnequalLengths" => Some(CsvErrorKind::UnequalLengths {
                expected_len: 0,
                pos: None,
                len: 0,
            }),
            "Utf8" => Some(CsvErrorKind::Utf8 { field: 0 }),
            "Deserialize" => Some(CsvErrorKind::Deserialize {
                message: String::new(),
            }),
            "Other" => Some(CsvErrorKind::Other),
            _ => None,
        }
    }
}

crate::default_style!(CsvErrorKind => CsvErrorKindStyle);

impl Elicitation for CsvErrorKind {
    type Style = CsvErrorKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvErrorKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose error kind:"),
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
        tracing::debug!(variant = %label, "CsvErrorKind variant selected");
        match label.as_str() {
            "Io" => Ok(CsvErrorKind::Io),
            "UnequalLengths" => {
                tracing::debug!("Eliciting UnequalLengths fields");
                Ok(CsvErrorKind::UnequalLengths {
                    expected_len: u64::elicit(communicator).await?,
                    pos: Option::<u64>::elicit(communicator).await?,
                    len: u64::elicit(communicator).await?,
                })
            }
            "Utf8" => {
                tracing::debug!("Eliciting Utf8 field");
                Ok(CsvErrorKind::Utf8 {
                    field: usize::elicit(communicator).await?,
                })
            }
            "Deserialize" => {
                tracing::debug!("Eliciting Deserialize message");
                Ok(CsvErrorKind::Deserialize {
                    message: String::elicit(communicator).await?,
                })
            }
            "Other" => Ok(CsvErrorKind::Other),
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid CsvErrorKind: {label}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("CsvErrorKind", "Io")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("CsvErrorKind", "Io")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("CsvErrorKind", "Io")
    }
}

impl ElicitIntrospect for CsvErrorKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::ErrorKind",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Io".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "UnequalLengths".to_string(),
                        fields: vec![
                            FieldInfo {
                                name: "expected_len",
                                type_name: "u64",
                                prompt: Some("Expected number of fields:"),
                            },
                            FieldInfo {
                                name: "pos",
                                type_name: "Option<u64>",
                                prompt: Some("Byte position of the first record:"),
                            },
                            FieldInfo {
                                name: "len",
                                type_name: "u64",
                                prompt: Some("Actual number of fields found:"),
                            },
                        ],
                    },
                    VariantMetadata {
                        label: "Utf8".to_string(),
                        fields: vec![FieldInfo {
                            name: "field",
                            type_name: "usize",
                            prompt: Some("Field index that failed UTF-8 validation:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Deserialize".to_string(),
                        fields: vec![FieldInfo {
                            name: "message",
                            type_name: "String",
                            prompt: Some("Deserialization error message:"),
                        }],
                    },
                    VariantMetadata {
                        label: "Other".to_string(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvErrorKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose error kind:").to_string(),
            type_name: "CsvErrorKind".to_string(),
            options: Self::labels(),
            branches: vec![
                None,
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Unequal lengths error details:".to_string()),
                    type_name: "UnequalLengths".to_string(),
                    fields: vec![
                        (
                            "expected_len".to_string(),
                            Box::new(
                                u64::prompt_tree()
                                    .with_prompt(Some("Expected number of fields:".to_string())),
                            ),
                        ),
                        (
                            "pos".to_string(),
                            Box::new(
                                <Option<u64> as crate::ElicitPromptTree>::prompt_tree()
                                    .with_prompt(Some(
                                        "Byte position of the first record (optional):".to_string(),
                                    )),
                            ),
                        ),
                        (
                            "len".to_string(),
                            Box::new(
                                u64::prompt_tree()
                                    .with_prompt(Some("Actual field count:".to_string())),
                            ),
                        ),
                    ],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("UTF-8 error details:".to_string()),
                    type_name: "Utf8".to_string(),
                    fields: vec![(
                        "field".to_string(),
                        Box::new(usize::prompt_tree().with_prompt(Some(
                            "Field index that failed UTF-8 validation:".to_string(),
                        ))),
                    )],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: Some("Deserialization error details:".to_string()),
                    type_name: "Deserialize".to_string(),
                    fields: vec![(
                        "message".to_string(),
                        Box::new(
                            String::prompt_tree().with_prompt(Some("Error message:".to_string())),
                        ),
                    )],
                })),
                None,
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvErrorKind {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            CsvErrorKind::Io => quote::quote! { ::elicitation::CsvErrorKind::Io },
            CsvErrorKind::UnequalLengths {
                expected_len,
                pos,
                len,
            } => {
                let expected_len =
                    <u64 as crate::emit_code::ToCodeLiteral>::to_code_literal(expected_len);
                let pos = <Option<u64> as crate::emit_code::ToCodeLiteral>::to_code_literal(pos);
                let len = <u64 as crate::emit_code::ToCodeLiteral>::to_code_literal(len);
                quote::quote! {
                    ::elicitation::CsvErrorKind::UnequalLengths {
                        expected_len: #expected_len,
                        pos: #pos,
                        len: #len,
                    }
                }
            }
            CsvErrorKind::Utf8 { field } => {
                let field = <usize as crate::emit_code::ToCodeLiteral>::to_code_literal(field);
                quote::quote! { ::elicitation::CsvErrorKind::Utf8 { field: #field } }
            }
            CsvErrorKind::Deserialize { message } => {
                let message = <String as crate::emit_code::ToCodeLiteral>::to_code_literal(message);
                quote::quote! { ::elicitation::CsvErrorKind::Deserialize { message: #message } }
            }
            CsvErrorKind::Other => quote::quote! { ::elicitation::CsvErrorKind::Other },
        }
    }
}
