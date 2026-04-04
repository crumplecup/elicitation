//! `DriverKind` — SQL database driver category enum.
//!
//! Serializable, formally verified enum for selecting which sqlx backend
//! to target. Enables agents to reason about driver selection as a
//! type-safe MCP vocabulary entry rather than an opaque string.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// SQL database driver category.
///
/// Selects which sqlx backend a pool or connection targets. All three drivers
/// are supported at runtime — the connection URL determines which backend
/// is actually used:
///
/// - `postgres://user:pass@host/db` → [`DriverKind::Postgres`]
/// - `sqlite://path/to/file.db` → [`DriverKind::Sqlite`]
/// - `mysql://user:pass@host/db` → [`DriverKind::MySql`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum DriverKind {
    /// PostgreSQL / CockroachDB backend.
    Postgres,
    /// SQLite embedded database backend.
    Sqlite,
    /// MySQL / MariaDB backend.
    MySql,
}

impl DriverKind {
    /// Returns the URL scheme prefix for this driver.
    pub fn url_scheme(&self) -> &'static str {
        match self {
            DriverKind::Postgres => "postgres",
            DriverKind::Sqlite => "sqlite",
            DriverKind::MySql => "mysql",
        }
    }

    /// Returns the canonical crate feature name for this driver.
    pub fn feature_name(&self) -> &'static str {
        match self {
            DriverKind::Postgres => "postgres",
            DriverKind::Sqlite => "sqlite",
            DriverKind::MySql => "mysql",
        }
    }
}

impl std::fmt::Display for DriverKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverKind::Postgres => f.write_str("Postgres"),
            DriverKind::Sqlite => f.write_str("Sqlite"),
            DriverKind::MySql => f.write_str("MySql"),
        }
    }
}

impl Prompt for DriverKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SQL database driver:")
    }
}

impl Select for DriverKind {
    fn options() -> Vec<Self> {
        vec![DriverKind::Postgres, DriverKind::Sqlite, DriverKind::MySql]
    }

    fn labels() -> Vec<String> {
        vec![
            "Postgres".to_string(),
            "Sqlite".to_string(),
            "MySql".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Postgres" => Some(DriverKind::Postgres),
            "Sqlite" => Some(DriverKind::Sqlite),
            "MySql" => Some(DriverKind::MySql),
            _ => None,
        }
    }
}

crate::default_style!(DriverKind => DriverKindStyle);

impl Elicitation for DriverKind {
    type Style = DriverKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DriverKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the SQL database driver:"),
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
                "Invalid DriverKind: {}",
                label
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("DriverKind", "Postgres")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("DriverKind", "Postgres")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("DriverKind", "Postgres")
    }
}

impl ElicitIntrospect for DriverKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::DriverKind",
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
