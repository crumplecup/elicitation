//! Trenchcoat wrapper for [`surrealdb_types::RecordId`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// A SurrealDB record identifier.
///
/// Wraps an upstream `surrealdb_types::RecordId` to add [`JsonSchema`] for
/// MCP boundary crossing. A record id is a `(table, key)` pair where the key
/// can be any JSON-serializable value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RecordId {
    /// The table the record belongs to.
    pub table: String,
    /// The key uniquely identifying the record within the table.
    ///
    /// Can be a string, integer, UUID, or a JSON object.
    pub key: JsonValue,
}

impl RecordId {
    /// Create a new record id.
    pub fn new(table: impl Into<String>, key: impl Into<JsonValue>) -> Self {
        Self {
            table: table.into(),
            key: key.into(),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::RecordId> for RecordId {
    fn from(rid: surrealdb_types::RecordId) -> Self {
        let table = rid.table.into_inner();
        let key = record_id_key_to_json(rid.key);
        Self { table, key }
    }
}

/// Convert a [`surrealdb_types::RecordIdKey`] to a [`serde_json::Value`].
#[cfg(feature = "surreal-types")]
fn record_id_key_to_json(key: surrealdb_types::RecordIdKey) -> JsonValue {
    match key {
        surrealdb_types::RecordIdKey::Number(n) => serde_json::json!(n),
        surrealdb_types::RecordIdKey::String(s) => serde_json::json!(s),
        surrealdb_types::RecordIdKey::Uuid(u) => serde_json::json!(u.to_string()),
        surrealdb_types::RecordIdKey::Array(a) => {
            serde_json::to_value(a).unwrap_or(JsonValue::Null)
        }
        surrealdb_types::RecordIdKey::Object(o) => {
            serde_json::to_value(o).unwrap_or(JsonValue::Null)
        }
        surrealdb_types::RecordIdKey::Range(r) => {
            // Serialize range as its SurrealQL string representation.
            use surrealdb_types::ToSql;
            serde_json::json!(r.to_sql())
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<RecordId> for surrealdb_types::RecordId {
    fn from(rid: RecordId) -> Self {
        let table = surrealdb_types::Table::new(rid.table);
        let key = json_to_record_id_key(rid.key);
        surrealdb_types::RecordId::new(table, key)
    }
}

/// Convert a [`serde_json::Value`] to a [`surrealdb_types::RecordIdKey`].
#[cfg(feature = "surreal-types")]
fn json_to_record_id_key(v: JsonValue) -> surrealdb_types::RecordIdKey {
    match v {
        JsonValue::Number(n) if n.is_i64() => {
            surrealdb_types::RecordIdKey::Number(n.as_i64().unwrap_or(0))
        }
        JsonValue::String(s) => surrealdb_types::RecordIdKey::String(s),
        JsonValue::Array(_) | JsonValue::Object(_) => {
            serde_json::from_value::<surrealdb_types::RecordIdKey>(v)
                .unwrap_or_else(|_| surrealdb_types::RecordIdKey::String("unknown".to_string()))
        }
        other => surrealdb_types::RecordIdKey::String(other.to_string()),
    }
}

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Survey, TypeMetadata, mcp,
};

crate::default_style!(RecordId => RecordIdStyle);

impl Prompt for RecordId {
    fn prompt() -> Option<&'static str> {
        Some("Provide a SurrealDB record identifier (table + key):")
    }
}

impl Survey for RecordId {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "table",
                prompt: Some("Enter the table name:"),
                type_name: "String",
            },
            FieldInfo {
                name: "key",
                prompt: Some("Enter the record key as JSON:"),
                type_name: "JSON",
            },
        ]
    }
}

impl Elicitation for RecordId {
    type Style = RecordIdStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RecordId");

        let table_params = mcp::text_params("Enter the table name (e.g. \"user\"):");
        let table_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(table_params),
            )
            .await?;
        let table = mcp::parse_string(mcp::extract_value(table_result)?)?;
        let table = table.trim().to_string();
        tracing::debug!(table = %table, "Elicited table");

        let key_params =
            mcp::text_params("Enter the record key as JSON (e.g. 1, \"abc\", {\"x\": 1}):");
        let key_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(key_params),
            )
            .await?;
        let key_str = mcp::parse_string(mcp::extract_value(key_result)?)?;
        let key: JsonValue = serde_json::from_str(key_str.trim()).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid JSON key \"{}\": {}",
                key_str.trim(),
                e
            )))
        })?;
        tracing::debug!("Elicited RecordId key");

        Ok(RecordId::new(table, key))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("record_id")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("record_id")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("record_id")
    }
}

impl ElicitIntrospect for RecordId {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealRecordId",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: Self::fields(),
            },
        }
    }
}
