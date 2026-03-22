//! `TokioUnixPlugin` — MCP tools for Unix domain sockets.
//!
//! **Unix only.** This entire module is `#[cfg(unix)]`.
//!
//! Listener, stream, and datagram socket handles are stored server-side in
//! UUID-keyed registries. Path arguments accept any valid Unix socket path
//! (e.g. `"/tmp/my.sock"`). Abstract namespace sockets are not supported
//! as paths — use a filesystem path.
//!
//! # Tool namespace: `tokio_unix__*`
//!
//! ## UnixListener
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `unix_listener_bind` | `path` | `{ listener_id, local_path }` |
//! | `unix_listener_accept` | `listener_id` | `{ stream_id, peer_path? }` |
//! | `unix_listener_local_addr` | `listener_id` | `{ path? }` |
//! | `unix_listener_close` | `listener_id` | `{ ok }` |
//!
//! ## UnixStream
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `unix_stream_connect` | `path` | `{ stream_id, local_path?, peer_path? }` |
//! | `unix_stream_read` | `stream_id, max_bytes?` | `{ data, bytes_read, eof }` |
//! | `unix_stream_write` | `stream_id, data` | `{ bytes_written }` |
//! | `unix_stream_local_addr` | `stream_id` | `{ path? }` |
//! | `unix_stream_peer_addr` | `stream_id` | `{ path? }` |
//! | `unix_stream_close` | `stream_id` | `{ ok }` |
//!
//! ## UnixDatagram
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `unix_datagram_bind` | `path` | `{ socket_id, local_path? }` |
//! | `unix_datagram_send_to` | `socket_id, data, path` | `{ bytes_sent }` |
//! | `unix_datagram_recv_from` | `socket_id, max_bytes?` | `{ data, bytes_received, from_path? }` |
//! | `unix_datagram_local_addr` | `socket_id` | `{ path? }` |
//! | `unix_datagram_close` | `socket_id` | `{ ok }` |

#![cfg(unix)]

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::PluginContext;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixDatagram, UnixListener, UnixStream};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_unix__*` tool calls.
pub struct UnixCtx {
    listeners: Mutex<HashMap<Uuid, Arc<UnixListener>>>,
    streams: Mutex<HashMap<Uuid, Arc<Mutex<UnixStream>>>>,
    datagrams: Mutex<HashMap<Uuid, Arc<UnixDatagram>>>,
}

impl UnixCtx {
    fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
            streams: Mutex::new(HashMap::new()),
            datagrams: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for UnixCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

fn err_not_found(label: &str, id: Uuid) -> ErrorData {
    ErrorData::invalid_params(format!("{label} not found: {id}"), None)
}

fn unix_addr_path(addr: &tokio::net::unix::SocketAddr) -> Option<String> {
    addr.as_pathname()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string())
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

#[derive(Serialize)]
struct AddrResult {
    path: Option<String>,
}

// ── Param / result types ──────────────────────────────────────────────────────

/// Parameters for `tokio_unix__unix_listener_bind`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixListenerBindParams {
    /// Filesystem path for the Unix socket, e.g. `"/tmp/my.sock"`.
    pub path: String,
}

#[derive(Serialize)]
struct UnixListenerBindResult {
    listener_id: Uuid,
    local_path: Option<String>,
}

/// Parameters for `tokio_unix__unix_listener_accept`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixListenerAcceptParams {
    /// Listener UUID returned by `unix_listener_bind`.
    pub listener_id: Uuid,
}

#[derive(Serialize)]
struct UnixListenerAcceptResult {
    stream_id: Uuid,
    peer_path: Option<String>,
}

/// Parameters for `tokio_unix__unix_listener_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixListenerLocalAddrParams {
    /// Listener UUID returned by `unix_listener_bind`.
    pub listener_id: Uuid,
}

/// Parameters for `tokio_unix__unix_listener_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixListenerCloseParams {
    /// Listener UUID to remove from the registry.
    pub listener_id: Uuid,
}

/// Parameters for `tokio_unix__unix_stream_connect`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamConnectParams {
    /// Path of the Unix socket to connect to.
    pub path: String,
}

#[derive(Serialize)]
struct UnixStreamConnectResult {
    stream_id: Uuid,
    local_path: Option<String>,
    peer_path: Option<String>,
}

/// Parameters for `tokio_unix__unix_stream_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamReadParams {
    /// Stream UUID.
    pub stream_id: Uuid,
    /// Maximum bytes to read (default 4096).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct UnixStreamReadResult {
    data: Vec<u8>,
    bytes_read: usize,
    eof: bool,
}

/// Parameters for `tokio_unix__unix_stream_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamWriteParams {
    /// Stream UUID.
    pub stream_id: Uuid,
    /// Raw bytes to write (JSON array of u8).
    pub data: Vec<u8>,
}

#[derive(Serialize)]
struct UnixStreamWriteResult {
    bytes_written: usize,
}

/// Parameters for `tokio_unix__unix_stream_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamLocalAddrParams {
    /// Stream UUID.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_unix__unix_stream_peer_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamPeerAddrParams {
    /// Stream UUID.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_unix__unix_stream_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixStreamCloseParams {
    /// Stream UUID to close and remove.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_unix__unix_datagram_bind`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixDatagramBindParams {
    /// Filesystem path for the socket. Use a unique path per socket.
    pub path: String,
}

#[derive(Serialize)]
struct UnixDatagramBindResult {
    socket_id: Uuid,
    local_path: Option<String>,
}

/// Parameters for `tokio_unix__unix_datagram_send_to`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixDatagramSendToParams {
    /// Socket UUID returned by `unix_datagram_bind`.
    pub socket_id: Uuid,
    /// Raw bytes to send (JSON array of u8).
    pub data: Vec<u8>,
    /// Destination socket path.
    pub path: String,
}

#[derive(Serialize)]
struct UnixDatagramSendToResult {
    bytes_sent: usize,
}

/// Parameters for `tokio_unix__unix_datagram_recv_from`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixDatagramRecvFromParams {
    /// Socket UUID returned by `unix_datagram_bind`.
    pub socket_id: Uuid,
    /// Maximum datagram size (default 65507).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct UnixDatagramRecvFromResult {
    data: Vec<u8>,
    bytes_received: usize,
    from_path: Option<String>,
}

/// Parameters for `tokio_unix__unix_datagram_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixDatagramLocalAddrParams {
    /// Socket UUID.
    pub socket_id: Uuid,
}

/// Parameters for `tokio_unix__unix_datagram_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixDatagramCloseParams {
    /// Socket UUID to close and remove.
    pub socket_id: Uuid,
}

// ── Unix listener tools ───────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_listener_bind",
    description = "Bind a Unix domain socket listener at the given filesystem path. \
                   Returns a listener_id UUID. The socket file is created on disk; \
                   remove it with `remove_file` after closing if it should not persist.",
    emit = Auto
)]
async fn unix_listener_bind(
    ctx: Arc<UnixCtx>,
    p: UnixListenerBindParams,
) -> Result<CallToolResult, ErrorData> {
    let listener = UnixListener::bind(&p.path)
        .map_err(|e| ErrorData::invalid_params(format!("bind failed for {}: {e}", p.path), None))?;
    let local_path = listener.local_addr().ok().as_ref().and_then(unix_addr_path);
    let listener_id = Uuid::new_v4();
    ctx.listeners
        .lock()
        .await
        .insert(listener_id, Arc::new(listener));
    Ok(json_result(&UnixListenerBindResult {
        listener_id,
        local_path,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_listener_accept",
    description = "Accept the next incoming Unix domain stream connection. Blocks until a \
                   client connects. Returns a stream_id UUID and the peer path (if named). \
                   Assumes: listener_id was returned by `unix_listener_bind`.",
    emit = Auto
)]
async fn unix_listener_accept(
    ctx: Arc<UnixCtx>,
    p: UnixListenerAcceptParams,
) -> Result<CallToolResult, ErrorData> {
    let listener = ctx
        .listeners
        .lock()
        .await
        .get(&p.listener_id)
        .cloned()
        .ok_or_else(|| err_not_found("listener_id", p.listener_id))?;
    let (stream, addr) = listener
        .accept()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("accept failed: {e}"), None))?;
    let stream_id = Uuid::new_v4();
    ctx.streams
        .lock()
        .await
        .insert(stream_id, Arc::new(Mutex::new(stream)));
    Ok(json_result(&UnixListenerAcceptResult {
        stream_id,
        peer_path: unix_addr_path(&addr),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_listener_local_addr",
    description = "Return the filesystem path the Unix listener is bound to. \
                   Assumes: listener_id was returned by `unix_listener_bind`.",
    emit = Auto
)]
async fn unix_listener_local_addr(
    ctx: Arc<UnixCtx>,
    p: UnixListenerLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let path = ctx
        .listeners
        .lock()
        .await
        .get(&p.listener_id)
        .ok_or_else(|| err_not_found("listener_id", p.listener_id))?
        .local_addr()
        .ok()
        .as_ref()
        .and_then(unix_addr_path);
    Ok(json_result(&AddrResult { path }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_listener_close",
    description = "Remove a Unix listener from the registry, closing it. The socket file on \
                   disk is NOT automatically deleted — use `tokio_fs__remove_file` if needed. \
                   Assumes: listener_id was returned by `unix_listener_bind`.",
    emit = Auto
)]
async fn unix_listener_close(
    ctx: Arc<UnixCtx>,
    p: UnixListenerCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.listeners
        .lock()
        .await
        .remove(&p.listener_id)
        .ok_or_else(|| err_not_found("listener_id", p.listener_id))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Unix stream tools ─────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_connect",
    description = "Connect a Unix domain stream socket to a server at the given path. \
                   Returns a stream_id UUID for subsequent read/write/close calls.",
    emit = Auto
)]
async fn unix_stream_connect(
    ctx: Arc<UnixCtx>,
    p: UnixStreamConnectParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = UnixStream::connect(&p.path).await.map_err(|e| {
        ErrorData::invalid_params(format!("connect failed for {}: {e}", p.path), None)
    })?;
    let local_path = stream.local_addr().ok().as_ref().and_then(unix_addr_path);
    let peer_path = stream.peer_addr().ok().as_ref().and_then(unix_addr_path);
    let stream_id = Uuid::new_v4();
    ctx.streams
        .lock()
        .await
        .insert(stream_id, Arc::new(Mutex::new(stream)));
    Ok(json_result(&UnixStreamConnectResult {
        stream_id,
        local_path,
        peer_path,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_read",
    description = "Read up to `max_bytes` (default 4096) from a Unix stream. Returns raw \
                   bytes as a JSON array of u8. `eof = true` when the peer has closed. \
                   Assumes: stream_id was returned by `unix_stream_connect` or \
                   `unix_listener_accept`.",
    emit = Auto
)]
async fn unix_stream_read(
    ctx: Arc<UnixCtx>,
    p: UnixStreamReadParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .cloned()
        .ok_or_else(|| err_not_found("stream_id", p.stream_id))?;
    let max = p.max_bytes.unwrap_or(4096).min(1 << 20);
    let mut buf = vec![0u8; max];
    let bytes_read = stream
        .lock()
        .await
        .read(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("read failed: {e}"), None))?;
    buf.truncate(bytes_read);
    Ok(json_result(&UnixStreamReadResult {
        data: buf,
        bytes_read,
        eof: bytes_read == 0,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_write",
    description = "Write all bytes in `data` to a Unix stream. Returns total bytes written. \
                   Assumes: stream_id was returned by `unix_stream_connect` or \
                   `unix_listener_accept`.",
    emit = Auto
)]
async fn unix_stream_write(
    ctx: Arc<UnixCtx>,
    p: UnixStreamWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .cloned()
        .ok_or_else(|| err_not_found("stream_id", p.stream_id))?;
    let bytes_written = p.data.len();
    stream
        .lock()
        .await
        .write_all(&p.data)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("write failed: {e}"), None))?;
    Ok(json_result(&UnixStreamWriteResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_local_addr",
    description = "Return the local address of a Unix stream (path, or None if unnamed). \
                   Assumes: stream_id was returned by `unix_stream_connect` or \
                   `unix_listener_accept`.",
    emit = Auto
)]
async fn unix_stream_local_addr(
    ctx: Arc<UnixCtx>,
    p: UnixStreamLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let path = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .ok_or_else(|| err_not_found("stream_id", p.stream_id))?
        .blocking_lock()
        .local_addr()
        .ok()
        .as_ref()
        .and_then(unix_addr_path);
    Ok(json_result(&AddrResult { path }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_peer_addr",
    description = "Return the peer address of a Unix stream (path, or None if unnamed). \
                   Assumes: stream_id was returned by `unix_stream_connect` or \
                   `unix_listener_accept`.",
    emit = Auto
)]
async fn unix_stream_peer_addr(
    ctx: Arc<UnixCtx>,
    p: UnixStreamPeerAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let path = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .ok_or_else(|| err_not_found("stream_id", p.stream_id))?
        .blocking_lock()
        .peer_addr()
        .ok()
        .as_ref()
        .and_then(unix_addr_path);
    Ok(json_result(&AddrResult { path }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_stream_close",
    description = "Close and remove a Unix stream from the registry. \
                   Assumes: stream_id was returned by `unix_stream_connect` or \
                   `unix_listener_accept`.",
    emit = Auto
)]
async fn unix_stream_close(
    ctx: Arc<UnixCtx>,
    p: UnixStreamCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.streams
        .lock()
        .await
        .remove(&p.stream_id)
        .ok_or_else(|| err_not_found("stream_id", p.stream_id))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Unix datagram tools ───────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_datagram_bind",
    description = "Bind a Unix datagram socket to a filesystem path. Returns a socket_id \
                   UUID. The socket file persists on disk until removed.",
    emit = Auto
)]
async fn unix_datagram_bind(
    ctx: Arc<UnixCtx>,
    p: UnixDatagramBindParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = UnixDatagram::bind(&p.path)
        .map_err(|e| ErrorData::invalid_params(format!("bind failed for {}: {e}", p.path), None))?;
    let local_path = socket.local_addr().ok().as_ref().and_then(unix_addr_path);
    let socket_id = Uuid::new_v4();
    ctx.datagrams
        .lock()
        .await
        .insert(socket_id, Arc::new(socket));
    Ok(json_result(&UnixDatagramBindResult {
        socket_id,
        local_path,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_datagram_send_to",
    description = "Send a datagram to a named Unix socket path. Returns bytes sent. \
                   Assumes: socket_id was returned by `unix_datagram_bind`.",
    emit = Auto
)]
async fn unix_datagram_send_to(
    ctx: Arc<UnixCtx>,
    p: UnixDatagramSendToParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = ctx
        .datagrams
        .lock()
        .await
        .get(&p.socket_id)
        .cloned()
        .ok_or_else(|| err_not_found("socket_id", p.socket_id))?;
    let bytes_sent = socket
        .send_to(&p.data, &p.path)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("send_to failed: {e}"), None))?;
    Ok(json_result(&UnixDatagramSendToResult { bytes_sent }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_datagram_recv_from",
    description = "Receive a datagram on a Unix socket. Blocks until a datagram arrives. \
                   Returns the data, byte count, and sender path (if named). \
                   `max_bytes` defaults to 65507. \
                   Assumes: socket_id was returned by `unix_datagram_bind`.",
    emit = Auto
)]
async fn unix_datagram_recv_from(
    ctx: Arc<UnixCtx>,
    p: UnixDatagramRecvFromParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = ctx
        .datagrams
        .lock()
        .await
        .get(&p.socket_id)
        .cloned()
        .ok_or_else(|| err_not_found("socket_id", p.socket_id))?;
    let max = p.max_bytes.unwrap_or(65507).min(65507);
    let mut buf = vec![0u8; max];
    let (bytes_received, addr) = socket
        .recv_from(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("recv_from failed: {e}"), None))?;
    buf.truncate(bytes_received);
    Ok(json_result(&UnixDatagramRecvFromResult {
        data: buf,
        bytes_received,
        from_path: unix_addr_path(&addr),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_datagram_local_addr",
    description = "Return the local path a Unix datagram socket is bound to. \
                   Assumes: socket_id was returned by `unix_datagram_bind`.",
    emit = Auto
)]
async fn unix_datagram_local_addr(
    ctx: Arc<UnixCtx>,
    p: UnixDatagramLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let path = ctx
        .datagrams
        .lock()
        .await
        .get(&p.socket_id)
        .ok_or_else(|| err_not_found("socket_id", p.socket_id))?
        .local_addr()
        .ok()
        .as_ref()
        .and_then(unix_addr_path);
    Ok(json_result(&AddrResult { path }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_unix",
    name = "tokio_unix__unix_datagram_close",
    description = "Close and remove a Unix datagram socket from the registry. The socket \
                   file on disk is NOT automatically deleted. \
                   Assumes: socket_id was returned by `unix_datagram_bind`.",
    emit = Auto
)]
async fn unix_datagram_close(
    ctx: Arc<UnixCtx>,
    p: UnixDatagramCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.datagrams
        .lock()
        .await
        .remove(&p.socket_id)
        .ok_or_else(|| err_not_found("socket_id", p.socket_id))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_unix__*` tools for Unix domain sockets.
///
/// Holds UUID-keyed registries for `UnixListener`, `UnixStream`, and
/// `UnixDatagram`. **Unix only.**
pub struct TokioUnixPlugin(Arc<UnixCtx>);

impl TokioUnixPlugin {
    /// Create a new `TokioUnixPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(UnixCtx::new()))
    }
}

impl Default for TokioUnixPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioUnixPlugin {
    fn name(&self) -> &'static str {
        "tokio_unix"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_unix")
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
            let full_name = if name.starts_with("tokio_unix__") {
                name.to_string()
            } else {
                format!("tokio_unix__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_unix")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
