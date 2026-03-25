//! `ColumnValue` — serializable SQL column value enum.
//!
//! Mirrors [`sqlx::any::AnyValueKind`] but is fully owned and serde-serializable,
//! allowing SQL row data to cross the MCP boundary.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Serializable SQL column value.
///
/// Mirrors the variants of `sqlx::any::AnyValueKind` with owned, serializable types.
/// Blob data is base64-encoded for JSON transport.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum ColumnValue {
    /// SQL NULL value.
    Null,
    /// Boolean value.
    Bool(bool),
    /// 16-bit integer (SMALLINT).
    SmallInt(i16),
    /// 32-bit integer (INTEGER / INT).
    Integer(i32),
    /// 64-bit integer (BIGINT).
    BigInt(i64),
    /// 32-bit float (REAL / FLOAT4).
    Real(f32),
    /// 64-bit float (DOUBLE PRECISION / FLOAT8).
    Double(f64),
    /// Text string.
    Text(String),
    /// Binary blob (base64-encoded in JSON).
    Blob(Vec<u8>),
}

impl ColumnValue {
    /// Returns true if this value is `Null`.
    pub fn is_null(&self) -> bool {
        matches!(self, ColumnValue::Null)
    }

    /// Returns the type kind label for this value.
    pub fn type_label(&self) -> &'static str {
        match self {
            ColumnValue::Null => "Null",
            ColumnValue::Bool(_) => "Bool",
            ColumnValue::SmallInt(_) => "SmallInt",
            ColumnValue::Integer(_) => "Integer",
            ColumnValue::BigInt(_) => "BigInt",
            ColumnValue::Real(_) => "Real",
            ColumnValue::Double(_) => "Double",
            ColumnValue::Text(_) => "Text",
            ColumnValue::Blob(_) => "Blob",
        }
    }
}

impl std::fmt::Display for ColumnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnValue::Null => write!(f, "NULL"),
            ColumnValue::Bool(b) => write!(f, "{b}"),
            ColumnValue::SmallInt(n) => write!(f, "{n}"),
            ColumnValue::Integer(n) => write!(f, "{n}"),
            ColumnValue::BigInt(n) => write!(f, "{n}"),
            ColumnValue::Real(n) => write!(f, "{n}"),
            ColumnValue::Double(n) => write!(f, "{n}"),
            ColumnValue::Text(s) => write!(f, "{s}"),
            ColumnValue::Blob(b) => write!(f, "<blob {} bytes>", b.len()),
        }
    }
}

impl Prompt for ColumnValue {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SQL value type:")
    }
}

impl Select for ColumnValue {
    fn options() -> Vec<Self> {
        vec![
            ColumnValue::Null,
            ColumnValue::Bool(false),
            ColumnValue::SmallInt(0),
            ColumnValue::Integer(0),
            ColumnValue::BigInt(0),
            ColumnValue::Real(0.0),
            ColumnValue::Double(0.0),
            ColumnValue::Text(String::new()),
            ColumnValue::Blob(vec![]),
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Null".to_string(),
            "Bool".to_string(),
            "SmallInt".to_string(),
            "Integer".to_string(),
            "BigInt".to_string(),
            "Real".to_string(),
            "Double".to_string(),
            "Text".to_string(),
            "Blob".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Null" => Some(ColumnValue::Null),
            "Bool" => Some(ColumnValue::Bool(false)),
            "SmallInt" => Some(ColumnValue::SmallInt(0)),
            "Integer" => Some(ColumnValue::Integer(0)),
            "BigInt" => Some(ColumnValue::BigInt(0)),
            "Real" => Some(ColumnValue::Real(0.0)),
            "Double" => Some(ColumnValue::Double(0.0)),
            "Text" => Some(ColumnValue::Text(String::new())),
            "Blob" => Some(ColumnValue::Blob(vec![])),
            _ => None,
        }
    }
}

crate::default_style!(ColumnValue => ColumnValueStyle);

impl Elicitation for ColumnValue {
    type Style = ColumnValueStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ColumnValue type selection");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose SQL value type:"),
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
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ColumnValue type: {}",
                label
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::new()
    }
}

impl ElicitIntrospect for ColumnValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ColumnValue",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}
