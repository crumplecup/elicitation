//! `TokioNetPlugin` — MCP tools for tokio network I/O.
//!
//! TCP listeners, TCP streams, and UDP sockets held server-side in UUID-keyed
//! registries. Agents interact via UUID handles — no sockets cross the MCP
//! boundary.
//!
//! # Tool namespace: `tokio_net__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `tcp_listener_bind` | `addr` | `{ listener_id, local_addr }` | `ListenerBound` |
//! | `tcp_listener_accept` | `listener_id` | `{ stream_id, peer_addr }` | `ConnectionAccepted` |
//! | `tcp_listener_local_addr` | `listener_id` | `{ addr }` | — |
//! | `tcp_listener_close` | `listener_id` | `{ ok }` | — |
//! | `tcp_stream_connect` | `addr` | `{ stream_id, local_addr, peer_addr }` | `StreamConnected` |
//! | `tcp_stream_read` | `stream_id, max_bytes` | `{ data, bytes_read, eof }` | `DataReceived` |
//! | `tcp_stream_write` | `stream_id, data` | `{ bytes_written }` | — |
//! | `tcp_stream_local_addr` | `stream_id` | `{ addr }` | — |
//! | `tcp_stream_peer_addr` | `stream_id` | `{ addr }` | — |
//! | `tcp_stream_close` | `stream_id` | `{ ok }` | — |
//! | `udp_socket_bind` | `addr` | `{ socket_id, local_addr }` | — |
//! | `udp_socket_send_to` | `socket_id, data, addr` | `{ bytes_sent }` | — |
//! | `udp_socket_recv_from` | `socket_id, max_bytes` | `{ data, bytes_received, from_addr }` | `DataReceived` |
//! | `udp_socket_local_addr` | `socket_id` | `{ addr }` | — |
//! | `udp_socket_close` | `socket_id` | `{ ok }` | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a TCP listener was successfully bound to a local address.
#[derive(Elicit)]
pub struct ListenerBound;
impl Prop for ListenerBound {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_listener_bound_axiom() {
                let bind_ok: bool = kani::any();
                kani::assume(bind_ok);
                assert!(bind_ok, "tokio::net::TcpListener::bind axiom: Ok => socket bound");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_listener_bound(bind_returned_ok: bool) -> (result: bool)
                ensures result == bind_returned_ok,
            {
                bind_returned_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_listener_bound_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: an incoming TCP connection was accepted by a listener.
#[derive(Elicit)]
pub struct ConnectionAccepted;
impl Prop for ConnectionAccepted {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_connection_accepted_axiom() {
                let accept_ok: bool = kani::any();
                kani::assume(accept_ok);
                assert!(accept_ok, "tokio::net::TcpListener::accept axiom: Ok => connection stream ready");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_connection_accepted(accept_returned_ok: bool) -> (result: bool)
                ensures result == accept_returned_ok,
            {
                accept_returned_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_connection_accepted_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a TCP stream was successfully connected to a remote address.
#[derive(Elicit)]
pub struct StreamConnected;
impl Prop for StreamConnected {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_stream_connected_axiom() {
                let connect_ok: bool = kani::any();
                kani::assume(connect_ok);
                assert!(connect_ok, "tokio::net::TcpStream::connect axiom: Ok => TCP handshake complete");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_stream_connected(connect_returned_ok: bool) -> (result: bool)
                ensures result == connect_returned_ok,
            {
                connect_returned_ok
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_stream_connected_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: at least one byte was received from a stream or socket.
#[derive(Elicit)]
pub struct DataReceived;
impl Prop for DataReceived {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_data_received_axiom() {
                let bytes: usize = kani::any();
                kani::assume(bytes > 0);
                assert!(bytes > 0, "AsyncReadExt::read axiom: Ok(n > 0) => bytes available");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_data_received(bytes_available: bool) -> (result: bool)
                ensures result == bytes_available,
            {
                bytes_available
            }
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_data_received_contract() -> bool {
                true
            }
        }
    }
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_net__*` tool calls.
pub struct NetCtx {
    /// `TcpListener::accept` takes `&self`, so `Arc<TcpListener>` suffices.
    listeners: Mutex<HashMap<Uuid, Arc<TcpListener>>>,
    /// `AsyncReadExt`/`AsyncWriteExt` require `&mut self`; inner `Mutex` lets
    /// us release the map lock before awaiting on the stream.
    /// Wrapped in `Arc` so the registry can be shared with `TokioIoCopyPlugin`.
    streams: Arc<Mutex<HashMap<Uuid, Arc<Mutex<TcpStream>>>>>,
    /// `UdpSocket::send_to`/`recv_from` take `&self`; `Arc` alone is enough.
    udp_sockets: Mutex<HashMap<Uuid, Arc<UdpSocket>>>,
}

impl NetCtx {
    fn new() -> Self {
        Self {
            listeners: Mutex::new(HashMap::new()),
            streams: Arc::new(Mutex::new(HashMap::new())),
            udp_sockets: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for NetCtx {}

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

#[derive(Serialize)]
struct AddrResult {
    addr: String,
}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `tokio_net__tcp_listener_bind`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpListenerBindParams {
    /// Local address to bind, e.g. `"0.0.0.0:8080"` or `"127.0.0.1:0"`.
    pub addr: String,
}

#[derive(Serialize)]
struct TcpListenerBindResult {
    listener_id: Uuid,
    local_addr: String,
}

/// Parameters for `tokio_net__tcp_listener_accept`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpListenerAcceptParams {
    /// Listener UUID returned by `tcp_listener_bind`.
    pub listener_id: Uuid,
}

#[derive(Serialize)]
struct TcpListenerAcceptResult {
    stream_id: Uuid,
    peer_addr: String,
}

/// Parameters for `tokio_net__tcp_listener_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpListenerLocalAddrParams {
    /// Listener UUID returned by `tcp_listener_bind`.
    pub listener_id: Uuid,
}

/// Parameters for `tokio_net__tcp_listener_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpListenerCloseParams {
    /// Listener UUID returned by `tcp_listener_bind`.
    pub listener_id: Uuid,
}

/// Parameters for `tokio_net__tcp_stream_connect`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamConnectParams {
    /// Remote address to connect to, e.g. `"127.0.0.1:8080"`.
    pub addr: String,
}

#[derive(Serialize)]
struct TcpStreamConnectResult {
    stream_id: Uuid,
    local_addr: String,
    peer_addr: String,
}

/// Parameters for `tokio_net__tcp_stream_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamReadParams {
    /// Stream UUID returned by `tcp_stream_connect` or `tcp_listener_accept`.
    pub stream_id: Uuid,
    /// Maximum number of bytes to read (default 4096).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct TcpStreamReadResult {
    /// Raw bytes as a JSON array of `u8` values.
    data: Vec<u8>,
    bytes_read: usize,
    /// `true` if the remote peer has closed the write half.
    eof: bool,
}

/// Parameters for `tokio_net__tcp_stream_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamWriteParams {
    /// Stream UUID returned by `tcp_stream_connect` or `tcp_listener_accept`.
    pub stream_id: Uuid,
    /// Raw bytes to write, as a JSON array of `u8` values.
    pub data: Vec<u8>,
}

#[derive(Serialize)]
struct TcpStreamWriteResult {
    bytes_written: usize,
}

/// Parameters for `tokio_net__tcp_stream_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamLocalAddrParams {
    /// Stream UUID.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_net__tcp_stream_peer_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamPeerAddrParams {
    /// Stream UUID.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_net__tcp_stream_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TcpStreamCloseParams {
    /// Stream UUID to close and remove from the registry.
    pub stream_id: Uuid,
}

/// Parameters for `tokio_net__udp_socket_bind`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UdpSocketBindParams {
    /// Local address to bind, e.g. `"0.0.0.0:0"` or `"127.0.0.1:9000"`.
    pub addr: String,
}

#[derive(Serialize)]
struct UdpSocketBindResult {
    socket_id: Uuid,
    local_addr: String,
}

/// Parameters for `tokio_net__udp_socket_send_to`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UdpSocketSendToParams {
    /// Socket UUID returned by `udp_socket_bind`.
    pub socket_id: Uuid,
    /// Raw bytes to send, as a JSON array of `u8` values.
    pub data: Vec<u8>,
    /// Destination address, e.g. `"127.0.0.1:9001"`.
    pub addr: String,
}

#[derive(Serialize)]
struct UdpSocketSendToResult {
    bytes_sent: usize,
}

/// Parameters for `tokio_net__udp_socket_recv_from`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UdpSocketRecvFromParams {
    /// Socket UUID returned by `udp_socket_bind`.
    pub socket_id: Uuid,
    /// Maximum datagram size to receive (default 65507).
    pub max_bytes: Option<usize>,
}

#[derive(Serialize)]
struct UdpSocketRecvFromResult {
    data: Vec<u8>,
    bytes_received: usize,
    from_addr: String,
}

/// Parameters for `tokio_net__udp_socket_local_addr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UdpSocketLocalAddrParams {
    /// Socket UUID returned by `udp_socket_bind`.
    pub socket_id: Uuid,
}

/// Parameters for `tokio_net__udp_socket_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UdpSocketCloseParams {
    /// Socket UUID to close and remove from the registry.
    pub socket_id: Uuid,
}

// ── TCP listener tools ────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_listener_bind",
    description = "Bind a TCP listener to a local address. Returns a listener_id UUID for \
                   subsequent accept/close calls. Use `\"127.0.0.1:0\"` to let the OS pick \
                   an ephemeral port.",
    emit = Auto
)]
async fn net_tcp_listener_bind(
    ctx: Arc<NetCtx>,
    p: TcpListenerBindParams,
) -> Result<CallToolResult, ErrorData> {
    let listener = TcpListener::bind(&p.addr)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("bind failed for {}: {e}", p.addr), None))?;
    let local_addr = listener
        .local_addr()
        .map(|a| a.to_string())
        .unwrap_or_default();
    let listener_id = Uuid::new_v4();
    ctx.listeners
        .lock()
        .await
        .insert(listener_id, Arc::new(listener));
    let _proof: Established<ListenerBound> = Established::assert();
    Ok(json_result(&TcpListenerBindResult {
        listener_id,
        local_addr,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_listener_accept",
    description = "Accept the next incoming TCP connection on a listener. Blocks until a \
                   client connects. Returns a stream_id UUID and the peer address. \
                   Assumes: listener_id was returned by `tcp_listener_bind`.",
    emit = Auto
)]
async fn net_tcp_listener_accept(
    ctx: Arc<NetCtx>,
    p: TcpListenerAcceptParams,
) -> Result<CallToolResult, ErrorData> {
    let listener = ctx
        .listeners
        .lock()
        .await
        .get(&p.listener_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("listener_id not found: {}", p.listener_id), None)
        })?;
    let (stream, peer_addr) = listener
        .accept()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("accept failed: {e}"), None))?;
    let stream_id = Uuid::new_v4();
    ctx.streams
        .lock()
        .await
        .insert(stream_id, Arc::new(Mutex::new(stream)));
    let _proof: Established<ConnectionAccepted> = Established::assert();
    Ok(json_result(&TcpListenerAcceptResult {
        stream_id,
        peer_addr: peer_addr.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_listener_local_addr",
    description = "Return the local address the listener is bound to. \
                   Assumes: listener_id was returned by `tcp_listener_bind`.",
    emit = Auto
)]
async fn net_tcp_listener_local_addr(
    ctx: Arc<NetCtx>,
    p: TcpListenerLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let addr = ctx
        .listeners
        .lock()
        .await
        .get(&p.listener_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("listener_id not found: {}", p.listener_id), None)
        })?
        .local_addr()
        .map(|a| a.to_string())
        .map_err(|e| ErrorData::invalid_params(format!("local_addr failed: {e}"), None))?;
    Ok(json_result(&AddrResult { addr }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_listener_close",
    description = "Remove a TCP listener from the registry, closing it. \
                   Assumes: listener_id was returned by `tcp_listener_bind`.",
    emit = Auto
)]
async fn net_tcp_listener_close(
    ctx: Arc<NetCtx>,
    p: TcpListenerCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.listeners
        .lock()
        .await
        .remove(&p.listener_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("listener_id not found: {}", p.listener_id), None)
        })?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── TCP stream tools ──────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_connect",
    description = "Open a TCP connection to a remote address. Returns a stream_id UUID for \
                   subsequent read/write/close calls.",
    emit = Auto
)]
async fn net_tcp_stream_connect(
    ctx: Arc<NetCtx>,
    p: TcpStreamConnectParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = TcpStream::connect(&p.addr).await.map_err(|e| {
        ErrorData::invalid_params(format!("connect failed for {}: {e}", p.addr), None)
    })?;
    let local_addr = stream
        .local_addr()
        .map(|a| a.to_string())
        .unwrap_or_default();
    let peer_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_default();
    let stream_id = Uuid::new_v4();
    ctx.streams
        .lock()
        .await
        .insert(stream_id, Arc::new(Mutex::new(stream)));
    let _proof: Established<StreamConnected> = Established::assert();
    Ok(json_result(&TcpStreamConnectResult {
        stream_id,
        local_addr,
        peer_addr,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_read",
    description = "Read up to `max_bytes` (default 4096) bytes from a TCP stream. Returns \
                   the raw bytes as a JSON array of u8 values, the actual count read, and \
                   whether EOF was reached. \
                   Assumes: stream_id was returned by `tcp_stream_connect` or \
                   `tcp_listener_accept`.",
    emit = Auto
)]
async fn net_tcp_stream_read(
    ctx: Arc<NetCtx>,
    p: TcpStreamReadParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("stream_id not found: {}", p.stream_id), None)
        })?;
    let max = p.max_bytes.unwrap_or(4096).min(1 << 20); // cap at 1 MiB
    let mut buf = vec![0u8; max];
    let bytes_read = stream
        .lock()
        .await
        .read(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("read failed: {e}"), None))?;
    buf.truncate(bytes_read);
    let eof = bytes_read == 0;
    if bytes_read > 0 {
        let _proof: Established<DataReceived> = Established::assert();
    }
    Ok(json_result(&TcpStreamReadResult {
        data: buf,
        bytes_read,
        eof,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_write",
    description = "Write all bytes in `data` to a TCP stream. Returns the total bytes written. \
                   Assumes: stream_id was returned by `tcp_stream_connect` or \
                   `tcp_listener_accept`.",
    emit = Auto
)]
async fn net_tcp_stream_write(
    ctx: Arc<NetCtx>,
    p: TcpStreamWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let stream = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("stream_id not found: {}", p.stream_id), None)
        })?;
    let bytes_written = p.data.len();
    stream
        .lock()
        .await
        .write_all(&p.data)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("write failed: {e}"), None))?;
    Ok(json_result(&TcpStreamWriteResult { bytes_written }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_local_addr",
    description = "Return the local socket address of a TCP stream. \
                   Assumes: stream_id was returned by `tcp_stream_connect` or \
                   `tcp_listener_accept`.",
    emit = Auto
)]
async fn net_tcp_stream_local_addr(
    ctx: Arc<NetCtx>,
    p: TcpStreamLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let addr = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("stream_id not found: {}", p.stream_id), None)
        })?
        .blocking_lock()
        .local_addr()
        .map(|a| a.to_string())
        .map_err(|e| ErrorData::invalid_params(format!("local_addr failed: {e}"), None))?;
    Ok(json_result(&AddrResult { addr }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_peer_addr",
    description = "Return the remote socket address of a TCP stream. \
                   Assumes: stream_id was returned by `tcp_stream_connect` or \
                   `tcp_listener_accept`.",
    emit = Auto
)]
async fn net_tcp_stream_peer_addr(
    ctx: Arc<NetCtx>,
    p: TcpStreamPeerAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let addr = ctx
        .streams
        .lock()
        .await
        .get(&p.stream_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("stream_id not found: {}", p.stream_id), None)
        })?
        .blocking_lock()
        .peer_addr()
        .map(|a| a.to_string())
        .map_err(|e| ErrorData::invalid_params(format!("peer_addr failed: {e}"), None))?;
    Ok(json_result(&AddrResult { addr }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__tcp_stream_close",
    description = "Close and remove a TCP stream from the registry. \
                   Assumes: stream_id was returned by `tcp_stream_connect` or \
                   `tcp_listener_accept`.",
    emit = Auto
)]
async fn net_tcp_stream_close(
    ctx: Arc<NetCtx>,
    p: TcpStreamCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.streams
        .lock()
        .await
        .remove(&p.stream_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("stream_id not found: {}", p.stream_id), None)
        })?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── UDP socket tools ──────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__udp_socket_bind",
    description = "Bind a UDP socket to a local address. Returns a socket_id UUID for \
                   subsequent send/recv/close calls. Use `\"0.0.0.0:0\"` for an ephemeral port.",
    emit = Auto
)]
async fn net_udp_socket_bind(
    ctx: Arc<NetCtx>,
    p: UdpSocketBindParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = UdpSocket::bind(&p.addr)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("bind failed for {}: {e}", p.addr), None))?;
    let local_addr = socket
        .local_addr()
        .map(|a| a.to_string())
        .unwrap_or_default();
    let socket_id = Uuid::new_v4();
    ctx.udp_sockets
        .lock()
        .await
        .insert(socket_id, Arc::new(socket));
    Ok(json_result(&UdpSocketBindResult {
        socket_id,
        local_addr,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__udp_socket_send_to",
    description = "Send a UDP datagram to a specific address. Returns the number of bytes sent. \
                   Assumes: socket_id was returned by `udp_socket_bind`.",
    emit = Auto
)]
async fn net_udp_socket_send_to(
    ctx: Arc<NetCtx>,
    p: UdpSocketSendToParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = ctx
        .udp_sockets
        .lock()
        .await
        .get(&p.socket_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("socket_id not found: {}", p.socket_id), None)
        })?;
    let bytes_sent = socket
        .send_to(&p.data, &p.addr)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("send_to failed: {e}"), None))?;
    Ok(json_result(&UdpSocketSendToResult { bytes_sent }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__udp_socket_recv_from",
    description = "Receive a UDP datagram. Blocks until a datagram arrives. Returns the data as \
                   a JSON array of u8 values, the byte count, and the sender address. \
                   `max_bytes` defaults to 65507 (max UDP payload). \
                   Assumes: socket_id was returned by `udp_socket_bind`.",
    emit = Auto
)]
async fn net_udp_socket_recv_from(
    ctx: Arc<NetCtx>,
    p: UdpSocketRecvFromParams,
) -> Result<CallToolResult, ErrorData> {
    let socket = ctx
        .udp_sockets
        .lock()
        .await
        .get(&p.socket_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("socket_id not found: {}", p.socket_id), None)
        })?;
    let max = p.max_bytes.unwrap_or(65507).min(65507);
    let mut buf = vec![0u8; max];
    let (bytes_received, from) = socket
        .recv_from(&mut buf)
        .await
        .map_err(|e| ErrorData::invalid_params(format!("recv_from failed: {e}"), None))?;
    buf.truncate(bytes_received);
    if bytes_received > 0 {
        let _proof: Established<DataReceived> = Established::assert();
    }
    Ok(json_result(&UdpSocketRecvFromResult {
        data: buf,
        bytes_received,
        from_addr: from.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__udp_socket_local_addr",
    description = "Return the local address a UDP socket is bound to. \
                   Assumes: socket_id was returned by `udp_socket_bind`.",
    emit = Auto
)]
async fn net_udp_socket_local_addr(
    ctx: Arc<NetCtx>,
    p: UdpSocketLocalAddrParams,
) -> Result<CallToolResult, ErrorData> {
    let addr = ctx
        .udp_sockets
        .lock()
        .await
        .get(&p.socket_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("socket_id not found: {}", p.socket_id), None)
        })?
        .local_addr()
        .map(|a| a.to_string())
        .map_err(|e| ErrorData::invalid_params(format!("local_addr failed: {e}"), None))?;
    Ok(json_result(&AddrResult { addr }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_net",
    name = "tokio_net__udp_socket_close",
    description = "Close and remove a UDP socket from the registry. \
                   Assumes: socket_id was returned by `udp_socket_bind`.",
    emit = Auto
)]
async fn net_udp_socket_close(
    ctx: Arc<NetCtx>,
    p: UdpSocketCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.udp_sockets
        .lock()
        .await
        .remove(&p.socket_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("socket_id not found: {}", p.socket_id), None)
        })?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_net__*` tools for network I/O.
///
/// Holds UUID-keyed registries for TCP listeners, TCP streams, and UDP sockets.
/// All socket objects live server-side; agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tokio_net"` namespace and named
/// `tokio_net__<verb>`.
pub struct TokioNetPlugin(Arc<NetCtx>);

impl TokioNetPlugin {
    /// Create a new `TokioNetPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(NetCtx::new()))
    }

    /// Shared registry of live TCP streams, keyed by UUID.
    ///
    /// Pass a clone to [`TokioIoCopyPlugin`](crate::TokioIoCopyPlugin) to
    /// enable `io::copy` between TCP streams and other handle types.
    pub fn tcp_stream_registry(&self) -> Arc<Mutex<HashMap<Uuid, Arc<Mutex<TcpStream>>>>> {
        Arc::clone(&self.0.streams)
    }
}

impl Default for TokioNetPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioNetPlugin {
    fn name(&self) -> &'static str {
        "tokio_net"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_net")
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
            let full_name = if name.starts_with("tokio_net__") {
                name.to_string()
            } else {
                format!("tokio_net__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_net")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
