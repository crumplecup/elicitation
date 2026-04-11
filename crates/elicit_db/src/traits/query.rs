//! [`DbQueryExecutor`] — query execution and planning.
//!
//! Source: ISO/IEC 9075-2 §14; PostgreSQL docs §14 — Performance Tips.

use futures::future::BoxFuture;

use crate::{
    DbExecuteResult, DbExplain, DbQueryRowsResult, DbResult, DbTransactionalExecuteResult, DbValue,
    IsolationLevel,
};

/// Executes SQL queries and statements with contract return types.
///
/// Source: ISO/IEC 9075-2 §14 — Data manipulation
pub trait DbQueryExecutor: Send + Sync {
    /// Execute a DML/DDL statement that does not return rows.
    ///
    /// Returns the number of affected rows.
    ///
    /// Source: ISO/IEC 9075-2 §14.8 — `<insert statement>`
    fn execute(&self, sql: &str, params: &[DbValue]) -> BoxFuture<'_, DbExecuteResult>;

    /// Execute a query and return the result set.
    ///
    /// Source: ISO/IEC 9075-2 §14.1 — `<query expression>`
    fn query_rows(&self, sql: &str, params: &[DbValue]) -> BoxFuture<'_, DbQueryRowsResult>;

    /// Run `EXPLAIN [ANALYZE]` on a query and return the plan.
    ///
    /// Source: PostgreSQL docs §14.1 — Using EXPLAIN
    fn explain(&self, sql: &str, analyze: bool) -> BoxFuture<'_, DbResult<DbExplain>>;

    /// Execute a statement within an auto-managed transaction.
    ///
    /// Source: ISO/IEC 9075-2 §17 — Transaction management
    fn execute_in_transaction(
        &self,
        sql: &str,
        params: &[DbValue],
        isolation: IsolationLevel,
    ) -> BoxFuture<'_, DbTransactionalExecuteResult>;
}
