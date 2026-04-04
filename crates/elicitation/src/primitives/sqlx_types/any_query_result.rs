//! [`sqlx::any::AnyQueryResult`] elicitation.
//!
//! Available with the `sqlx-types` feature.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use sqlx::any::AnyQueryResult;

crate::default_style!(AnyQueryResult => AnyQueryResultStyle);

impl Prompt for AnyQueryResult {
    fn prompt() -> Option<&'static str> {
        Some("Enter query result statistics:")
    }
}

impl Elicitation for AnyQueryResult {
    type Style = AnyQueryResultStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting AnyQueryResult");

        let rows_params = mcp::text_params("Number of rows affected:");
        let rows_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(rows_params),
            )
            .await?;
        let rows_value = mcp::extract_value(rows_result)?;
        let rows_str = mcp::parse_string(rows_value)?;
        let rows_affected: u64 = rows_str.trim().parse().unwrap_or(0);

        let id_params = mcp::text_params("Last inserted row ID (leave empty for none):");
        let id_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(id_params),
            )
            .await?;
        let id_value = mcp::extract_value(id_result)?;
        let id_str = mcp::parse_string(id_value)?;
        let last_insert_id: Option<i64> = {
            let trimmed = id_str.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse().ok()
            }
        };

        tracing::debug!(rows_affected, ?last_insert_id, "Elicited AnyQueryResult");
        Ok(AnyQueryResult {
            rows_affected,
            last_insert_id,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("any_query_result")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("any_query_result")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("any_query_result")
    }
}

impl ElicitIntrospect for AnyQueryResult {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "sqlx::any::AnyQueryResult",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "rows_affected",
                        type_name: "u64",
                        prompt: Some("Number of rows modified by the query"),
                    },
                    FieldInfo {
                        name: "last_insert_id",
                        type_name: "Option<i64>",
                        prompt: Some("ID of the last inserted row, if applicable"),
                    },
                ],
            },
        }
    }
}
