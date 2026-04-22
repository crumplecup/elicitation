//! Trenchcoat wrapper for [`csv::Terminator`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Record terminator used when reading or writing CSV data.
///
/// Wraps `csv::Terminator` to add [`JsonSchema`] for MCP boundary crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CsvTerminator {
    /// Use `\r\n` as the record terminator (RFC 4180 compliant).
    Crlf,
    /// Accept any of `\r`, `\n`, or `\r\n` as record terminators.
    Any,
    /// Use a specific byte as the record terminator.
    AnyByte(u8),
}

#[cfg(feature = "csv-types")]
impl From<csv::Terminator> for CsvTerminator {
    fn from(t: csv::Terminator) -> Self {
        match t {
            csv::Terminator::CRLF => CsvTerminator::Crlf,
            csv::Terminator::Any(b) => CsvTerminator::AnyByte(b),
            _ => CsvTerminator::Any,
        }
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvTerminator> for csv::Terminator {
    fn from(t: CsvTerminator) -> Self {
        match t {
            CsvTerminator::Crlf => csv::Terminator::CRLF,
            CsvTerminator::Any => csv::Terminator::Any(b'\n'),
            CsvTerminator::AnyByte(b) => csv::Terminator::Any(b),
        }
    }
}

impl Prompt for CsvTerminator {
    fn prompt() -> Option<&'static str> {
        Some("Choose the CSV record terminator:")
    }
}

impl Select for CsvTerminator {
    fn options() -> Vec<Self> {
        vec![
            CsvTerminator::Crlf,
            CsvTerminator::Any,
            CsvTerminator::AnyByte(0),
        ]
    }

    fn labels() -> Vec<String> {
        vec!["Crlf".to_string(), "Any".to_string(), "AnyByte".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Crlf" => Some(CsvTerminator::Crlf),
            "Any" => Some(CsvTerminator::Any),
            "AnyByte" => Some(CsvTerminator::AnyByte(0)),
            _ => None,
        }
    }
}

crate::default_style!(CsvTerminator => CsvTerminatorStyle);

impl Elicitation for CsvTerminator {
    type Style = CsvTerminatorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvTerminator");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose terminator:"),
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
        tracing::debug!(variant = %label, "CsvTerminator variant selected");
        match label.as_str() {
            "Crlf" => Ok(CsvTerminator::Crlf),
            "Any" => Ok(CsvTerminator::Any),
            "AnyByte" => {
                tracing::debug!("Eliciting byte value for AnyByte terminator");
                Ok(CsvTerminator::AnyByte(u8::elicit(communicator).await?))
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid CsvTerminator: {label}"
            )))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("CsvTerminator", "Crlf")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("CsvTerminator", "Crlf")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("CsvTerminator", "Crlf")
    }
}

impl ElicitIntrospect for CsvTerminator {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::Terminator",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Crlf".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Any".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "AnyByte".to_string(),
                        fields: vec![FieldInfo {
                            name: "byte",
                            type_name: "u8",
                            prompt: Some("The byte value to use as the record terminator:"),
                        }],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvTerminator {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose terminator:").to_string(),
            type_name: "CsvTerminator".to_string(),
            options: Self::labels(),
            branches: vec![
                None,
                None,
                Some(Box::new(u8::prompt_tree().with_prompt(Some(
                    "Byte value for the terminator:".to_string(),
                )))),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvTerminator {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            CsvTerminator::Crlf => quote::quote! { ::elicitation::CsvTerminator::Crlf },
            CsvTerminator::Any => quote::quote! { ::elicitation::CsvTerminator::Any },
            CsvTerminator::AnyByte(b) => {
                let b = <u8 as crate::emit_code::ToCodeLiteral>::to_code_literal(b);
                quote::quote! { ::elicitation::CsvTerminator::AnyByte(#b) }
            }
        }
    }
}
