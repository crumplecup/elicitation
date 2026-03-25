//! `ColumnDescriptor` — serializable SQL column metadata.
//!
//! Represents a column's name, ordinal position, and SQL type kind in a form
//! that can cross the MCP boundary. Constructed from [`sqlx::any::AnyColumn`]
//! in the `elicit_sqlx` crate.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, SqlTypeKind, TypeMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Serializable SQL column metadata.
///
/// Bridges [`sqlx::any::AnyColumn`] to the MCP boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ColumnDescriptor {
    /// Zero-based position of this column in the result set.
    pub ordinal: usize,
    /// Column name as returned by the database.
    pub name: String,
    /// SQL type category for this column.
    pub type_kind: SqlTypeKind,
}

impl ColumnDescriptor {
    /// Construct a column descriptor.
    pub fn new(ordinal: usize, name: impl Into<String>, type_kind: SqlTypeKind) -> Self {
        Self {
            ordinal,
            name: name.into(),
            type_kind,
        }
    }
}

impl std::fmt::Display for ColumnDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({:?})", self.ordinal, self.name, self.type_kind)
    }
}

crate::default_style!(ColumnDescriptor => ColumnDescriptorStyle);

impl Prompt for ColumnDescriptor {
    fn prompt() -> Option<&'static str> {
        Some("Describe a SQL column (ordinal, name, and type):")
    }
}

impl Elicitation for ColumnDescriptor {
    type Style = ColumnDescriptorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ColumnDescriptor");

        let ord_params = mcp::text_params("Column ordinal (0-based position):");
        let ord_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(ord_params),
            )
            .await?;
        let ord_value = mcp::extract_value(ord_result)?;
        let ord_str = mcp::parse_string(ord_value)?;
        let ordinal: usize = ord_str.trim().parse().unwrap_or(0);

        let name_params = mcp::text_params("Column name:");
        let name_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(name_params),
            )
            .await?;
        let name_value = mcp::extract_value(name_result)?;
        let name = mcp::parse_string(name_value)?;

        let type_kind = SqlTypeKind::elicit(communicator).await?;

        tracing::debug!(ordinal, name = %name, "Elicited ColumnDescriptor");
        Ok(ColumnDescriptor {
            ordinal,
            name: name.trim().to_string(),
            type_kind,
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

impl ElicitIntrospect for ColumnDescriptor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ColumnDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "ordinal",
                        type_name: "usize",
                        prompt: Some("Zero-based column position"),
                    },
                    FieldInfo {
                        name: "name",
                        type_name: "String",
                        prompt: Some("Column name as returned by the database"),
                    },
                    FieldInfo {
                        name: "type_kind",
                        type_name: "SqlTypeKind",
                        prompt: Some("SQL type category"),
                    },
                ],
            },
        }
    }
}
