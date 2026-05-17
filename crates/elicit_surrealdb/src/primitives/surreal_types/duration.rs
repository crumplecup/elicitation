//! Trenchcoat wrapper for [`surrealdb_types::Duration`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use elicitation::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};

/// A SurrealDB duration value.
///
/// Wraps an upstream `surrealdb_types::Duration` to add [`JsonSchema`] for
/// MCP boundary crossing.  Duration strings use SurrealDB notation, e.g.
/// `"1y2w3d4h5m6s"` or `"500ms"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Duration {
    /// SurrealDB duration string, e.g. `"1y2w3d4h5m6s"`.
    ///
    /// Supported units: `y` (years), `w` (weeks), `d` (days), `h` (hours),
    /// `m` (minutes), `s` (seconds), `ms` (milliseconds), `us`/`µs`
    /// (microseconds), `ns` (nanoseconds).
    pub value: String,
}

impl Duration {
    /// Create a new duration from a SurrealDB duration string.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<surrealdb_types::Duration> for Duration {
    fn from(d: surrealdb_types::Duration) -> Self {
        // Display formats the duration in SurrealDB string notation (e.g. "1y2w3d").
        Self {
            value: d.to_string(),
        }
    }
}

impl From<Duration> for surrealdb_types::Duration {
    fn from(d: Duration) -> Self {
        d.value
            .parse::<surrealdb_types::Duration>()
            .unwrap_or_default()
    }
}

elicitation::default_style!(Duration => DurationStyle);

impl Prompt for Duration {
    fn prompt() -> Option<&'static str> {
        Some("Enter a SurrealDB duration string (e.g. \"1y2w3d\", \"500ms\", \"1h30m\"):")
    }
}

impl Elicitation for Duration {
    type Style = DurationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Duration");
        let params = mcp::text_params(Self::prompt().unwrap());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?;
        tracing::debug!(duration = %s, "Elicited Duration");
        Ok(Self::new(s.trim().to_string()))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_trusted_opaque("duration")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_trusted_opaque("duration")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_trusted_opaque("duration")
    }
}

impl ElicitIntrospect for Duration {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealDuration",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl elicitation::ElicitPromptTree for Duration {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("Enter a SurrealDB duration string (e.g. \"1h30m\" or \"500ms\"):")
                .to_string(),
            type_name: "SurrealDuration".to_string(),
        }
    }
}

impl elicitation::emit_code::ToCodeLiteral for Duration {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = &self.value;
        quote::quote! { elicit_surrealdb::SurrealDuration { value: #v.to_string() } }
    }
}
