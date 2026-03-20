//! [`sqlx::any::AnyTypeInfoKind`] elicitation.
//!
//! Implements the local `Elicitation`, `Select`, and `Prompt` traits for
//! `AnyTypeInfoKind`. For a serializable/`JsonSchema`-enabled equivalent,
//! see [`crate::SqlTypeKind`].
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use sqlx::any::AnyTypeInfoKind;

impl Prompt for AnyTypeInfoKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the SQL column type:")
    }
}

impl Select for AnyTypeInfoKind {
    fn options() -> Vec<Self> {
        vec![
            AnyTypeInfoKind::Null,
            AnyTypeInfoKind::Bool,
            AnyTypeInfoKind::SmallInt,
            AnyTypeInfoKind::Integer,
            AnyTypeInfoKind::BigInt,
            AnyTypeInfoKind::Real,
            AnyTypeInfoKind::Double,
            AnyTypeInfoKind::Text,
            AnyTypeInfoKind::Blob,
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
            "Null" => Some(AnyTypeInfoKind::Null),
            "Bool" => Some(AnyTypeInfoKind::Bool),
            "SmallInt" => Some(AnyTypeInfoKind::SmallInt),
            "Integer" => Some(AnyTypeInfoKind::Integer),
            "BigInt" => Some(AnyTypeInfoKind::BigInt),
            "Real" => Some(AnyTypeInfoKind::Real),
            "Double" => Some(AnyTypeInfoKind::Double),
            "Text" => Some(AnyTypeInfoKind::Text),
            "Blob" => Some(AnyTypeInfoKind::Blob),
            _ => None,
        }
    }
}

crate::default_style!(AnyTypeInfoKind => AnyTypeInfoKindStyle);

impl Elicitation for AnyTypeInfoKind {
    type Style = AnyTypeInfoKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AnyTypeInfoKind");
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
                "Invalid AnyTypeInfoKind: {}",
                label
            )))
        })
    }
}

impl ElicitIntrospect for AnyTypeInfoKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "sqlx::any::AnyTypeInfoKind",
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
