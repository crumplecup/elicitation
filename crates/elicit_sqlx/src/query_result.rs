//! `QueryResultData` — serializable snapshot of [`sqlx::any::AnyQueryResult`].
//!
//! `AnyQueryResult` doesn't derive `Serialize`, so we carry its two `pub`
//! fields into a local owned type that crosses the MCP boundary cleanly.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

elicit_newtype!(sqlx::any::AnyQueryResult, as AnyQueryResult);

/// Serializable snapshot of a query execution result.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryResultData {
    /// Number of rows modified by the statement.
    pub rows_affected: u64,
    /// Last auto-inserted row ID, if the backend provides one.
    pub last_insert_id: Option<i64>,
}

#[reflect_methods]
impl AnyQueryResult {
    /// Returns the number of rows affected by the statement.
    #[instrument(skip(self))]
    pub fn rows_affected(&self) -> u64 {
        self.0.rows_affected
    }

    /// Returns the last auto-inserted row ID, if any.
    #[instrument(skip(self))]
    pub fn last_insert_id(&self) -> Option<i64> {
        self.0.last_insert_id
    }

    /// Materializes this result as a serializable [`QueryResultData`].
    #[instrument(skip(self))]
    pub fn to_result_data(&self) -> QueryResultData {
        QueryResultData {
            rows_affected: self.0.rows_affected,
            last_insert_id: self.0.last_insert_id,
        }
    }
}
