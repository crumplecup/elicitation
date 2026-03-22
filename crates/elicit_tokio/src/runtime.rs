//! `TokioRuntimePlugin` — MCP tools for inspecting the active tokio runtime.
//!
//! Uses [`tokio::runtime::Handle::current`] to introspect the runtime the MCP
//! server is already running on. No futures or runtime objects cross the MCP
//! boundary — results are serializable values.
//!
//! # Tool namespace: `tokio_runtime__*`
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `inspect_flavor` | — | `{ flavor }` |

use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Runtime flavor mirror ─────────────────────────────────────────────────────

/// Serializable mirror of [`tokio::runtime::RuntimeFlavor`].
///
/// Returned by `tokio_runtime__inspect_flavor` to describe the threading
/// model of the active tokio runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFlavorKind {
    /// Single-threaded scheduler (`Builder::new_current_thread`).
    CurrentThread,
    /// Multi-threaded work-stealing scheduler (`Builder::new_multi_thread`).
    MultiThread,
    /// Unknown or future variant.
    Other,
}

impl From<tokio::runtime::RuntimeFlavor> for RuntimeFlavorKind {
    fn from(f: tokio::runtime::RuntimeFlavor) -> Self {
        match f {
            tokio::runtime::RuntimeFlavor::CurrentThread => RuntimeFlavorKind::CurrentThread,
            tokio::runtime::RuntimeFlavor::MultiThread => RuntimeFlavorKind::MultiThread,
            _ => RuntimeFlavorKind::Other,
        }
    }
}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tokio_runtime__inspect_flavor`.
///
/// No configuration needed — inspects the current runtime context.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct InspectFlavorParams {}

#[derive(Serialize)]
struct InspectFlavorResult {
    flavor: RuntimeFlavorKind,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `tokio_runtime__*` tools for runtime introspection.
///
/// Operates on the tokio runtime the MCP server is already running on via
/// [`tokio::runtime::Handle::current`].
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tokio_runtime")]
pub struct TokioRuntimePlugin;

#[elicitation::elicit_tool(
    plugin = "tokio_runtime",
    name = "tokio_runtime__inspect_flavor",
    description = "Inspect the threading model of the active tokio runtime. \
                   Returns 'current_thread' (single-threaded scheduler) or \
                   'multi_thread' (work-stealing multi-threaded scheduler). \
                   Useful for understanding the execution context of the MCP server.",
    emit = Auto
)]
async fn runtime_inspect_flavor(p: InspectFlavorParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    let flavor = RuntimeFlavorKind::from(tokio::runtime::Handle::current().runtime_flavor());
    let result = serde_json::to_string(&InspectFlavorResult { flavor })
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        result,
    )]))
}
