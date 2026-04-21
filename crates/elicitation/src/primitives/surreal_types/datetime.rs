//! Trenchcoat wrapper for [`surrealdb_types::Datetime`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};

/// A SurrealDB datetime value.
///
/// Wraps an upstream `surrealdb_types::Datetime` to add [`JsonSchema`] for
/// MCP boundary crossing. Stored as an ISO 8601 string, e.g.
/// `"2024-01-15T10:30:00Z"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Datetime {
    /// ISO 8601 datetime string in UTC, e.g. `"2024-01-15T10:30:00Z"`.
    pub value: String,
}

impl Datetime {
    /// Create a new datetime from an ISO 8601 string.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<surrealdb_types::Datetime> for Datetime {
    fn from(dt: surrealdb_types::Datetime) -> Self {
        use chrono::SecondsFormat;
        Self {
            value: dt.into_inner().to_rfc3339_opts(SecondsFormat::AutoSi, true),
        }
    }
}

#[cfg(feature = "surreal-types")]
impl From<Datetime> for surrealdb_types::Datetime {
    fn from(dt: Datetime) -> Self {
        dt.value
            .parse::<surrealdb_types::Datetime>()
            .unwrap_or_else(|_| surrealdb_types::Datetime::now())
    }
}

crate::default_style!(Datetime => DatetimeStyle);

impl Prompt for Datetime {
    fn prompt() -> Option<&'static str> {
        Some("Enter an ISO 8601 datetime string (e.g. \"2024-01-15T10:30:00Z\"):")
    }
}

impl Elicitation for Datetime {
    type Style = DatetimeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Datetime");
        let params = mcp::text_params(Self::prompt().unwrap());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?;
        tracing::debug!(datetime = %s, "Elicited Datetime");
        Ok(Self::new(s.trim().to_string()))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("datetime")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("datetime")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("datetime")
    }
}

impl ElicitIntrospect for Datetime {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealDatetime",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
