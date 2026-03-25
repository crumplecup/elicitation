//! `RowData` — serializable SQL row representation.
//!
//! A row of column name/value pairs that can cross the MCP boundary.
//! Constructed from [`sqlx::any::AnyRow`] in the `elicit_sqlx` crate.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ColumnValue, ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, TypeMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single named column value within a SQL row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ColumnEntry {
    /// Column name as returned by the database.
    pub name: String,
    /// The value for this column in this row.
    pub value: ColumnValue,
}

impl ColumnEntry {
    /// Construct a column entry.
    pub fn new(name: impl Into<String>, value: ColumnValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

/// Serializable SQL row data.
///
/// Bridges [`sqlx::any::AnyRow`] to the MCP boundary by materializing
/// all column values into owned, serializable types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RowData {
    /// All columns and their values for this row.
    pub columns: Vec<ColumnEntry>,
}

impl RowData {
    /// Construct a row with the given column entries.
    pub fn new(columns: Vec<ColumnEntry>) -> Self {
        Self { columns }
    }

    /// Number of columns in this row.
    pub fn len(&self) -> usize {
        self.columns.len()
    }

    /// Returns true if this row has no columns.
    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    /// Look up a column value by name.
    pub fn get(&self, name: &str) -> Option<&ColumnValue> {
        self.columns
            .iter()
            .find(|c| c.name == name)
            .map(|c| &c.value)
    }

    /// Iterate over (name, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &ColumnValue)> {
        self.columns.iter().map(|c| (c.name.as_str(), &c.value))
    }
}

crate::default_style!(RowData => RowDataStyle);
crate::default_style!(ColumnEntry => ColumnEntryStyle);

impl Prompt for ColumnEntry {
    fn prompt() -> Option<&'static str> {
        Some("Enter a column name and value:")
    }
}

impl Elicitation for ColumnEntry {
    type Style = ColumnEntryStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ColumnEntry");

        let name_params = mcp::text_params("Column name:");
        let name_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(name_params),
            )
            .await?;
        let name_value = mcp::extract_value(name_result)?;
        let name = mcp::parse_string(name_value)?;

        let value = ColumnValue::elicit(communicator).await?;

        tracing::debug!(name = %name, "Elicited ColumnEntry");
        Ok(ColumnEntry {
            name: name.trim().to_string(),
            value,
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

impl ElicitIntrospect for ColumnEntry {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::ColumnEntry",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "name",
                        type_name: "String",
                        prompt: Some("Column name"),
                    },
                    FieldInfo {
                        name: "value",
                        type_name: "ColumnValue",
                        prompt: Some("SQL value for this column"),
                    },
                ],
            },
        }
    }
}

impl Prompt for RowData {
    fn prompt() -> Option<&'static str> {
        Some("Enter SQL row data (columns and values):")
    }
}

impl Elicitation for RowData {
    type Style = RowDataStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RowData");

        let count_params = mcp::text_params("How many columns does this row have?");
        let count_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(count_params),
            )
            .await?;
        let count_value = mcp::extract_value(count_result)?;
        let count_str = mcp::parse_string(count_value)?;
        let count: usize = count_str.trim().parse().unwrap_or(1);

        let mut columns = Vec::with_capacity(count);
        for i in 0..count {
            tracing::debug!(column = i, "Eliciting column entry");
            let entry = ColumnEntry::elicit(communicator).await?;
            columns.push(entry);
        }

        tracing::debug!(column_count = count, "Elicited RowData");
        Ok(RowData { columns })
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

impl ElicitIntrospect for RowData {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::RowData",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "columns",
                    type_name: "Vec<ColumnEntry>",
                    prompt: Some("All column name/value pairs in this row"),
                }],
            },
        }
    }
}
