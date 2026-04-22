//! Trenchcoat wrapper for [`csv::Position`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Position within a CSV stream: byte offset, line number, and record index.
///
/// Wraps `csv::Position` to add [`JsonSchema`] for MCP boundary crossing.
/// All indices are zero-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct CsvPosition {
    /// Byte offset from the start of the CSV data.
    pub byte: u64,
    /// Line number (zero-based).
    pub line: u64,
    /// Record index (zero-based, excludes the header row when `has_headers` is true).
    pub record: u64,
}

#[cfg(feature = "csv-types")]
impl From<&csv::Position> for CsvPosition {
    fn from(p: &csv::Position) -> Self {
        CsvPosition {
            byte: p.byte(),
            line: p.line(),
            record: p.record(),
        }
    }
}

#[cfg(feature = "csv-types")]
impl From<CsvPosition> for csv::Position {
    fn from(p: CsvPosition) -> Self {
        let mut pos = csv::Position::new();
        pos.set_byte(p.byte);
        pos.set_line(p.line);
        pos.set_record(p.record);
        pos
    }
}

impl Prompt for CsvPosition {
    fn prompt() -> Option<&'static str> {
        Some("Specify a position within a CSV stream:")
    }
}

crate::default_style!(CsvPosition => CsvPositionStyle);

impl Elicitation for CsvPosition {
    type Style = CsvPositionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting CsvPosition");
        Ok(Self {
            byte: u64::elicit(communicator).await?,
            line: u64::elicit(communicator).await?,
            record: u64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u64 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for CsvPosition {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "csv::Position",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "byte",
                        type_name: "u64",
                        prompt: Some("Byte offset from the start of the CSV data:"),
                    },
                    FieldInfo {
                        name: "line",
                        type_name: "u64",
                        prompt: Some("Line number (zero-based):"),
                    },
                    FieldInfo {
                        name: "record",
                        type_name: "u64",
                        prompt: Some("Record index (zero-based):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for CsvPosition {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "CsvPosition".to_string(),
            fields: vec![
                (
                    "byte".to_string(),
                    Box::new(
                        u64::prompt_tree()
                            .with_prompt(Some("Byte offset from the start:".to_string())),
                    ),
                ),
                (
                    "line".to_string(),
                    Box::new(
                        u64::prompt_tree()
                            .with_prompt(Some("Line number (zero-based):".to_string())),
                    ),
                ),
                (
                    "record".to_string(),
                    Box::new(
                        u64::prompt_tree()
                            .with_prompt(Some("Record index (zero-based):".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for CsvPosition {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let byte = <u64 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.byte);
        let line = <u64 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.line);
        let record = <u64 as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.record);
        quote::quote! {
            ::elicitation::CsvPosition { byte: #byte, line: #line, record: #record }
        }
    }
}
