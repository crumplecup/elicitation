//! `TokioIoPlugin` — MCP tools for tokio in-memory I/O pipes.
//!
//! # Cross-plugin I/O blocker
//!
//! `tokio::io::copy(reader, writer)` between TCP streams or other handles
//! registered in `TokioNetPlugin` or `TokioProcessPlugin` requires access to
//! those plugins' registries. Cross-plugin registry coupling is an architectural
//! blocker — implementing it would require a shared global registry or tight
//! crate coupling, neither of which is acceptable here.
//!
//! Instead, this plugin provides in-memory duplex and simplex pipes whose
//! endpoints live in its own registry. These are useful for in-process
//! agent-to-agent byte relay without touching the network.
//!
//! # Tool namespace: `tokio_io__*`
//!
//! ## Duplex (bidirectional in-memory byte stream)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `duplex_create` | `max_buf_size?` | `{ a_id, b_id }` |
//! | `duplex_read` | `id, max_bytes?` | `{ data, bytes_read, eof }` |
//! | `duplex_write` | `id, data` | `{ bytes_written }` |
//! | `duplex_close` | `id` | `{ ok }` |
//!
//! A duplex has two ends (`a` and `b`). Writing to `a` makes bytes readable on
//! `b`, and vice-versa. Both ends are [`tokio::io::DuplexStream`].

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::PluginContext;
use elicitation::contracts::{Established, Prop};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: a `tokio::io::duplex()` pair was created and both ends are registered.
pub struct DuplexCreated {}
impl Prop for DuplexCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_io__*` tool calls.
pub struct IoCtx {
    /// Wrapped in `Arc` so the registry can be shared with `TokioIoCopyPlugin`.
    duplex_streams: Arc<Mutex<HashMap<Uuid, Arc<Mutex<DuplexStream>>>>>,
}

impl IoCtx {
    fn new() -> Self {
        Self {
            duplex_streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl PluginContext for IoCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

// ── Param / result types ──────────────────────────────────────────────────────

/// Parameters for `tokio_io__duplex_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplexCreateParams {
    /// Maximum number of bytes buffered in each direction before writes block
    /// (default 8192).
    pub max_buf_size: Option<usize>,
}

#[derive(Serialize)]
struct DuplexCreateResult {
    /// UUID for end A of the duplex stream.
    a_id: Uuid,
    /// UUID for end B of the duplex stream.
    b_id: Uuid,
}

/// Parameters for `tokio_io__duplex_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplexReadParams {
    /// Duplex end UUID (either `a_id` or `b_id` from `duplex_create`).
    pub id: Uuid,
    /// Maximum bytes to read (default 4096).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct DuplexReadResult {
    data: Vec<u8>,
    bytes_read: usize,
    /// `true` when the other end of the duplex has been closed.
    eof: bool,
}

/// Parameters for `tokio_io__duplex_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplexWriteParams {
    /// Duplex end UUID.
    pub id: Uuid,
    /// Raw bytes to write (JSON array of u8).
    pub data: Vec<u8>,
}

#[derive(Serialize)]
struct DuplexWriteResult {
    bytes_written: usize,
}

/// Parameters for `tokio_io__duplex_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DuplexCloseParams {
    /// Duplex end UUID to remove.
    pub id: Uuid,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_io",
    name = "tokio_io__duplex_create",
    description = "Create a pair of connected in-memory duplex streams. Writing to `a_id` \
                   makes bytes available for reading on `b_id`, and vice-versa. \
                   `max_buf_size` controls how many bytes can be buffered in each direction \
                   before writes block (default 8192).",
    emit = Auto
)]
async fn io_duplex_create(
    ctx: Arc<IoCtx>,
    p: DuplexCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let buf = p.max_buf_size.unwrap_or(8192).max(1);
    let (a, b) = tokio::io::duplex(buf);
    let a_id = Uuid::new_v4();
    let b_id = Uuid::new_v4();
    let mut map = ctx.duplex_streams.lock().await;
    map.insert(a_id, Arc::new(Mutex::new(a)));
    map.insert(b_id, Arc::new(Mutex::new(b)));
    let _proof: Established<DuplexCreated> = Established::assert();
    Ok(json_result(&DuplexCreateResult { a_id, b_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_io",
    name = "tokio_io__duplex_read",
    description = "Read up to `max_bytes` (default 4096) from a duplex stream end. Blocks if \
                   no data is available. Returns `eof = true` when the other end has been \
                   closed. \
                   Assumes: id was returned as `a_id` or `b_id` by `duplex_create`.",
    emit = Auto
)]
async fn io_duplex_read(ctx: Arc<IoCtx>, p: DuplexReadParams) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .duplex_streams
        .lock()
        .await
        .get(&p.id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("duplex id not found: {}", p.id), None))?;
    let max = p.max_bytes.unwrap_or(4096).min(1 << 20);
    let mut buf = vec![0u8; max];
    let bytes_read = stream
        .lock()
        .await
        .read(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("read failed: {e}"), None))?;
    buf.truncate(bytes_read);
    Ok(json_result(&DuplexReadResult {
        data: buf,
        bytes_read,
        eof: bytes_read == 0,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_io",
    name = "tokio_io__duplex_write",
    description = "Write bytes to a duplex stream end. The bytes become readable from the \
                   other end. Blocks if the buffer is full. \
                   Assumes: id was returned as `a_id` or `b_id` by `duplex_create`.",
    emit = Auto
)]
async fn io_duplex_write(
    ctx: Arc<IoCtx>,
    p: DuplexWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .duplex_streams
        .lock()
        .await
        .get(&p.id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("duplex id not found: {}", p.id), None))?;
    let bytes_written = p.data.len();
    stream
        .lock()
        .await
        .write_all(&p.data)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("write failed: {e}"), None))?;
    Ok(json_result(&DuplexWriteResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_io",
    name = "tokio_io__duplex_close",
    description = "Close one end of a duplex stream. The other end will see EOF on its next \
                   read. \
                   Assumes: id was returned as `a_id` or `b_id` by `duplex_create`.",
    emit = Auto
)]
async fn io_duplex_close(
    ctx: Arc<IoCtx>,
    p: DuplexCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.duplex_streams
        .lock()
        .await
        .remove(&p.id)
        .ok_or_else(|| ErrorData::invalid_params(format!("duplex id not found: {}", p.id), None))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_io__*` tools for in-memory I/O pipes.
///
/// Provides duplex (bidirectional) in-memory byte streams whose endpoints
/// are stored in a UUID-keyed registry.
///
/// Use [`TokioIoCopyPlugin`](crate::TokioIoCopyPlugin) with
/// [`duplex_stream_registry`](TokioIoPlugin::duplex_stream_registry) to
/// enable `io::copy` between duplex pipes and other handle types.
pub struct TokioIoPlugin(Arc<IoCtx>);

impl TokioIoPlugin {
    /// Create a new `TokioIoPlugin` with an empty duplex registry.
    pub fn new() -> Self {
        Self(Arc::new(IoCtx::new()))
    }

    /// Shared registry of live duplex stream halves, keyed by UUID.
    ///
    /// Pass a clone to [`TokioIoCopyPlugin`](crate::TokioIoCopyPlugin) to
    /// enable `io::copy` between duplex pipes and other handle types.
    pub fn duplex_stream_registry(&self) -> Arc<Mutex<HashMap<Uuid, Arc<Mutex<DuplexStream>>>>> {
        Arc::clone(&self.0.duplex_streams)
    }
}

impl Default for TokioIoPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioIoPlugin {
    fn name(&self) -> &'static str {
        "tokio_io"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_io")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("tokio_io__") {
                name.to_string()
            } else {
                format!("tokio_io__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_io")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
