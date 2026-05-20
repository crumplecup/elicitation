//! `SqlTypeKind` — locally-owned SQL type category enum.
//!
//! Mirrors [`sqlx::any::AnyTypeInfoKind`] but is fully owned, serializable,
//! and derives [`JsonSchema`], allowing it to appear in MCP tool schemas
//! and cross the JSON boundary.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Serializable SQL column type category.
///
/// Locally-owned equivalent of [`sqlx::any::AnyTypeInfoKind`] that derives
/// `Serialize`, `Deserialize`, and `JsonSchema` for use in MCP tool schemas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum SqlTypeKind {
    /// SQL NULL type.
    Null,
    /// Boolean (BOOLEAN).
    Bool,
    /// 16-bit integer (SMALLINT).
    SmallInt,
    /// 32-bit integer (INTEGER).
    Integer,
    /// 64-bit integer (BIGINT).
    BigInt,
    /// 32-bit float (REAL).
    Real,
    /// 64-bit float (DOUBLE PRECISION).
    Double,
    /// Text string (TEXT / VARCHAR).
    Text,
    /// Binary data (BLOB / BYTEA).
    Blob,
}

impl SqlTypeKind {
    /// SQL type name as used in DDL statements.
    pub fn sql_name(&self) -> &'static str {
        match self {
            SqlTypeKind::Null => "NULL",
            SqlTypeKind::Bool => "BOOLEAN",
            SqlTypeKind::SmallInt => "SMALLINT",
            SqlTypeKind::Integer => "INTEGER",
            SqlTypeKind::BigInt => "BIGINT",
            SqlTypeKind::Real => "REAL",
            SqlTypeKind::Double => "DOUBLE",
            SqlTypeKind::Text => "TEXT",
            SqlTypeKind::Blob => "BLOB",
        }
    }

    /// Returns true if this type is an integer variant.
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            SqlTypeKind::SmallInt | SqlTypeKind::Integer | SqlTypeKind::BigInt
        )
    }

    /// Returns true if this type is a floating-point variant.
    pub fn is_float(&self) -> bool {
        matches!(self, SqlTypeKind::Real | SqlTypeKind::Double)
    }
}

impl std::fmt::Display for SqlTypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.sql_name())
    }
}

/// Convert from sqlx's [`AnyTypeInfoKind`] to our serializable [`SqlTypeKind`].
impl From<sqlx::any::AnyTypeInfoKind> for SqlTypeKind {
    fn from(kind: sqlx::any::AnyTypeInfoKind) -> Self {
        use sqlx::any::AnyTypeInfoKind as K;
        match kind {
            K::Null => SqlTypeKind::Null,
            K::Bool => SqlTypeKind::Bool,
            K::SmallInt => SqlTypeKind::SmallInt,
            K::Integer => SqlTypeKind::Integer,
            K::BigInt => SqlTypeKind::BigInt,
            K::Real => SqlTypeKind::Real,
            K::Double => SqlTypeKind::Double,
            K::Text => SqlTypeKind::Text,
            K::Blob => SqlTypeKind::Blob,
        }
    }
}

/// Convert back from [`SqlTypeKind`] to sqlx's [`AnyTypeInfoKind`].
impl From<SqlTypeKind> for sqlx::any::AnyTypeInfoKind {
    fn from(kind: SqlTypeKind) -> Self {
        use sqlx::any::AnyTypeInfoKind as K;
        match kind {
            SqlTypeKind::Null => K::Null,
            SqlTypeKind::Bool => K::Bool,
            SqlTypeKind::SmallInt => K::SmallInt,
            SqlTypeKind::Integer => K::Integer,
            SqlTypeKind::BigInt => K::BigInt,
            SqlTypeKind::Real => K::Real,
            SqlTypeKind::Double => K::Double,
            SqlTypeKind::Text => K::Text,
            SqlTypeKind::Blob => K::Blob,
        }
    }
}

impl Prompt for SqlTypeKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SQL column type:")
    }
}

impl Select for SqlTypeKind {
    fn options() -> Vec<Self> {
        vec![
            SqlTypeKind::Null,
            SqlTypeKind::Bool,
            SqlTypeKind::SmallInt,
            SqlTypeKind::Integer,
            SqlTypeKind::BigInt,
            SqlTypeKind::Real,
            SqlTypeKind::Double,
            SqlTypeKind::Text,
            SqlTypeKind::Blob,
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
            "Null" => Some(SqlTypeKind::Null),
            "Bool" => Some(SqlTypeKind::Bool),
            "SmallInt" => Some(SqlTypeKind::SmallInt),
            "Integer" => Some(SqlTypeKind::Integer),
            "BigInt" => Some(SqlTypeKind::BigInt),
            "Real" => Some(SqlTypeKind::Real),
            "Double" => Some(SqlTypeKind::Double),
            "Text" => Some(SqlTypeKind::Text),
            "Blob" => Some(SqlTypeKind::Blob),
            _ => None,
        }
    }
}

crate::default_style!(SqlTypeKind => SqlTypeKindStyle);

impl Elicitation for SqlTypeKind {
    type Style = SqlTypeKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting SqlTypeKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose SQL column type:"),
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
                "Invalid SqlTypeKind: {}",
                label
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("SqlTypeKind", "Null")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("SqlTypeKind", "Null")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("SqlTypeKind", "Null")
    }
}

impl ElicitIntrospect for SqlTypeKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::SqlTypeKind",
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

impl crate::ElicitPromptTree for SqlTypeKind {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose the SQL column type:").to_string(),
            type_name: "SqlTypeKind".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

impl crate::emit_code::ToCodeLiteral for SqlTypeKind {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            SqlTypeKind::Null     => quote::quote! { elicitation::SqlTypeKind::Null },
            SqlTypeKind::Bool     => quote::quote! { elicitation::SqlTypeKind::Bool },
            SqlTypeKind::SmallInt => quote::quote! { elicitation::SqlTypeKind::SmallInt },
            SqlTypeKind::Integer  => quote::quote! { elicitation::SqlTypeKind::Integer },
            SqlTypeKind::BigInt   => quote::quote! { elicitation::SqlTypeKind::BigInt },
            SqlTypeKind::Real     => quote::quote! { elicitation::SqlTypeKind::Real },
            SqlTypeKind::Double   => quote::quote! { elicitation::SqlTypeKind::Double },
            SqlTypeKind::Text     => quote::quote! { elicitation::SqlTypeKind::Text },
            SqlTypeKind::Blob     => quote::quote! { elicitation::SqlTypeKind::Blob },
        }
    }
}
