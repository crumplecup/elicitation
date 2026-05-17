//! Trenchcoat wrapper for [`surrealdb_types::Table`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use elicitation::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};

/// A SurrealDB table name.
///
/// Wraps an upstream `surrealdb_types::Table` to add [`JsonSchema`] for MCP
/// boundary crossing. Table names must be alphanumeric or underscore.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Table {
    /// The table name string.
    pub name: String,
}

impl Table {
    /// Create a new table name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl From<surrealdb_types::Table> for Table {
    fn from(t: surrealdb_types::Table) -> Self {
        Self {
            name: t.into_inner(),
        }
    }
}

impl From<Table> for surrealdb_types::Table {
    fn from(t: Table) -> Self {
        surrealdb_types::Table::new(t.name)
    }
}

elicitation::default_style!(Table => TableStyle);

impl Prompt for Table {
    fn prompt() -> Option<&'static str> {
        Some("Enter the SurrealDB table name (e.g. \"user\", \"post\"):")
    }
}

impl Elicitation for Table {
    type Style = TableStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting Table");
        let params = mcp::text_params(Self::prompt().unwrap());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?;
        tracing::debug!(table = %s, "Elicited Table");
        Ok(Self::new(s.trim().to_string()))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::kani_trusted_opaque("table")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::verus_trusted_opaque("table")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        elicitation::verification::proof_helpers::creusot_trusted_opaque("table")
    }
}

impl ElicitIntrospect for Table {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "SurrealTable",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl elicitation::ElicitPromptTree for Table {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Leaf {
            prompt: Self::prompt()
                .unwrap_or("Enter the SurrealDB table name:")
                .to_string(),
            type_name: "SurrealTable".to_string(),
        }
    }
}

impl elicitation::emit_code::ToCodeLiteral for Table {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let n = &self.name;
        quote::quote! { elicit_surrealdb::SurrealTable { name: #n.to_string() } }
    }
}
