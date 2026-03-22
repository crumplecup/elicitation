//! `TokioTaskPlugin` — MCP tools for tokio task utilities.
//!
//! # Blocker note
//!
//! The following tokio task APIs cannot be exposed as MCP tools because they
//! require closures or futures as arguments, which cannot cross the JSON
//! boundary:
//!
//! - `tokio::task::spawn(future)` — future not serializable
//! - `tokio::task::spawn_blocking(closure)` — closure not serializable
//! - `tokio::task::block_in_place(closure)` — closure not serializable
//! - `tokio::task::spawn_local(future)` — future not serializable
//! - `tokio::task::JoinSet` — requires spawning futures
//! - `tokio::task::LocalSet` — requires spawning futures
//!
//! `tokio::task::id()` exposes the current task's ID but requires the
//! `tokio_unstable` cfg flag; omitted for stable builds.
//!
//! # Tool namespace: `tokio_task__*`
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `yield_now` | — | `{ ok }` |

use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Param / result types ──────────────────────────────────────────────────────

/// Parameters for `tokio_task__yield_now` (none required).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct YieldNowParams {}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    use rmcp::model::Content;
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_task",
    name = "tokio_task__yield_now",
    description = "Yield execution back to the tokio scheduler, allowing other tasks to run \
                   before this one continues. Useful in tight loops to avoid starving the \
                   runtime.",
    emit = Auto
)]
async fn task_yield_now(_p: YieldNowParams) -> Result<CallToolResult, ErrorData> {
    tokio::task::yield_now().await;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct ─────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_task__*` tools.
///
/// # Blocker
///
/// `spawn`, `spawn_blocking`, `block_in_place`, `spawn_local` are not
/// implementable as MCP tools — they require closures or futures as arguments
/// which cannot be serialized over JSON.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tokio_task")]
pub struct TokioTaskPlugin;
