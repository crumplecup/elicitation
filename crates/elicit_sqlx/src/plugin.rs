//! MCP plugin exposing live sqlx database tools.
//!
//! Registers five tools under the `"sqlx"` namespace:
//! `connect_check`, `execute`, `fetch_all`, `fetch_one`, `fetch_optional`.
//!
//! Each tool accepts a `database_url` and executes SQL against it using a
//! short-lived [`AnyPool`][sqlx::any::AnyPool].

use std::sync::Arc;

use elicitation::ElicitPlugin;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use sqlx::AnyPool;
use tracing::instrument;

use crate::query_result::QueryResultData;
use crate::row::AnyRow;

/// MCP plugin for sqlx database operations.
///
/// Registers five tools: `connect_check`, `execute`, `fetch_all`,
/// `fetch_one`, and `fetch_optional`. Each creates a short-lived pool per
/// call — no persistent connection state.
pub struct SqlxPlugin;

impl Default for SqlxPlugin {
    fn default() -> Self {
        Self
    }
}

/// Parameters accepted by all sqlx runtime tools.
#[derive(Debug, Deserialize, JsonSchema)]
struct SqlxParams {
    /// Database connection URL (e.g. `postgres://user:pass@host/db` or
    /// `sqlite://path/to/file.db`).
    database_url: String,

    /// SQL statement to execute (for `execute`, `fetch_*` tools).
    sql: Option<String>,
}

fn typed_tool<T: JsonSchema + 'static>(name: &'static str, description: &'static str) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}

fn parse_args<T: for<'de> Deserialize<'de>>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

fn require_sql(p: &SqlxParams) -> Result<&str, ErrorData> {
    p.sql
        .as_deref()
        .ok_or_else(|| ErrorData::invalid_params("missing required field: sql", None))
}

#[instrument(skip_all, fields(database_url = %database_url))]
async fn open_pool(database_url: &str) -> Result<AnyPool, ErrorData> {
    sqlx::any::install_default_drivers();
    AnyPool::connect(database_url)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("connection failed: {e}"), None))
}

impl ElicitPlugin for SqlxPlugin {
    fn name(&self) -> &'static str {
        "sqlx"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<SqlxParams>(
                "connect_check",
                "Verify a database connection URL is reachable. Returns `{ ok: true }` on \
                 success or an error message.",
            ),
            typed_tool::<SqlxParams>(
                "execute",
                "Execute a non-returning SQL statement (INSERT, UPDATE, DELETE, DDL). \
                 Returns `{ rows_affected, last_insert_id }`.",
            ),
            typed_tool::<SqlxParams>(
                "fetch_all",
                "Execute a SELECT and return all matching rows as a JSON array of \
                 `{ columns: [{ name, value }] }` objects.",
            ),
            typed_tool::<SqlxParams>(
                "fetch_one",
                "Execute a SELECT and return the first row. Returns an error if no row \
                 is found.",
            ),
            typed_tool::<SqlxParams>(
                "fetch_optional",
                "Execute a SELECT and return the first row or `null` if none is found.",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let p: SqlxParams = parse_args(&params)?;
            let pool = open_pool(&p.database_url).await?;

            match params.name.as_ref() {
                "connect_check" => {
                    let json = serde_json::json!({ "ok": true });
                    Ok(CallToolResult::success(vec![Content::text(
                        json.to_string(),
                    )]))
                }

                "execute" => {
                    let sql = require_sql(&p)?.to_owned();
                    match sqlx::query(sqlx::AssertSqlSafe(sql)).execute(&pool).await {
                        Ok(result) => {
                            let data = QueryResultData {
                                rows_affected: result.rows_affected,
                                last_insert_id: result.last_insert_id,
                            };
                            let json = serde_json::to_string(&data)
                                .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
                            Ok(CallToolResult::success(vec![Content::text(json)]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }

                "fetch_all" => {
                    let sql = require_sql(&p)?.to_owned();
                    match sqlx::query(sqlx::AssertSqlSafe(sql)).fetch_all(&pool).await {
                        Ok(rows) => {
                            let data: Vec<_> = rows
                                .into_iter()
                                .map(|r| AnyRow::from(r).to_row_data())
                                .collect();
                            let json = serde_json::to_string(&data)
                                .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
                            Ok(CallToolResult::success(vec![Content::text(json)]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }

                "fetch_one" => {
                    let sql = require_sql(&p)?.to_owned();
                    match sqlx::query(sqlx::AssertSqlSafe(sql)).fetch_one(&pool).await {
                        Ok(row) => {
                            let data = AnyRow::from(row).to_row_data();
                            let json = serde_json::to_string(&data)
                                .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
                            Ok(CallToolResult::success(vec![Content::text(json)]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }

                "fetch_optional" => {
                    let sql = require_sql(&p)?.to_owned();
                    match sqlx::query(sqlx::AssertSqlSafe(sql))
                        .fetch_optional(&pool)
                        .await
                    {
                        Ok(maybe_row) => {
                            let data = maybe_row.map(|r| AnyRow::from(r).to_row_data());
                            let json = serde_json::to_string(&data)
                                .unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"));
                            Ok(CallToolResult::success(vec![Content::text(json)]))
                        }
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }

                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}
