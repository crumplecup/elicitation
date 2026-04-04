//! [`sqlx::error::ErrorKind`] elicitation.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use sqlx::error::ErrorKind;

impl Prompt for ErrorKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the sqlx database error kind:")
    }
}

impl Select for ErrorKind {
    fn options() -> Vec<Self> {
        vec![
            ErrorKind::UniqueViolation,
            ErrorKind::ForeignKeyViolation,
            ErrorKind::NotNullViolation,
            ErrorKind::CheckViolation,
            ErrorKind::Other,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "UniqueViolation".to_string(),
            "ForeignKeyViolation".to_string(),
            "NotNullViolation".to_string(),
            "CheckViolation".to_string(),
            "Other".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "UniqueViolation" => Some(ErrorKind::UniqueViolation),
            "ForeignKeyViolation" => Some(ErrorKind::ForeignKeyViolation),
            "NotNullViolation" => Some(ErrorKind::NotNullViolation),
            "CheckViolation" => Some(ErrorKind::CheckViolation),
            "Other" => Some(ErrorKind::Other),
            _ => None,
        }
    }
}

crate::default_style!(ErrorKind => SqlxErrorKindStyle);

impl Elicitation for ErrorKind {
    type Style = SqlxErrorKindStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting sqlx ErrorKind");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose database error kind:"),
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
                "Invalid sqlx ErrorKind: {}",
                label
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("SqlxErrorKind", "UniqueViolation")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("SqlxErrorKind", "UniqueViolation")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "SqlxErrorKind",
            "UniqueViolation",
        )
    }
}

impl ElicitIntrospect for ErrorKind {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "sqlx::error::ErrorKind",
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
