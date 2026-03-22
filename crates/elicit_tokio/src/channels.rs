//! `TokioChannelPlugin` — MCP tools for tokio sync channels and shared state.
//!
//! All channel endpoints and shared-state objects live server-side in UUID-keyed
//! registries. Values are `serde_json::Value` so agents can pass arbitrary JSON
//! through channels.
//!
//! # Tool namespace: `tokio_channel__*`
//!
//! ## mpsc (multi-producer, single-consumer)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `mpsc_create` | `capacity` | `{ sender_id, receiver_id }` |
//! | `mpsc_send` | `sender_id, value` | `{ ok }` |
//! | `mpsc_try_send` | `sender_id, value` | `{ ok, full, closed }` |
//! | `mpsc_recv` | `receiver_id` | `{ value?, closed }` |
//! | `mpsc_try_recv` | `receiver_id` | `{ value?, empty, closed }` |
//! | `mpsc_sender_close` | `sender_id` | `{ ok }` |
//! | `mpsc_receiver_close` | `receiver_id` | `{ ok }` |
//!
//! ## oneshot (single send, single receive)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `oneshot_create` | — | `{ sender_id, receiver_id }` |
//! | `oneshot_send` | `sender_id, value` | `{ ok }` |
//! | `oneshot_recv` | `receiver_id` | `{ value?, closed }` |
//! | `oneshot_try_recv` | `receiver_id` | `{ value?, empty, closed }` |
//!
//! ## watch (single-value broadcast)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `watch_create` | `initial_value` | `{ sender_id, receiver_id }` |
//! | `watch_send` | `sender_id, value` | `{ ok }` |
//! | `watch_borrow` | `receiver_id` | `{ value }` |
//! | `watch_changed` | `receiver_id` | `{ ok }` |
//! | `watch_subscribe` | `sender_id` | `{ receiver_id }` |
//! | `watch_sender_close` | `sender_id` | `{ ok }` |
//! | `watch_receiver_close` | `receiver_id` | `{ ok }` |
//!
//! ## broadcast (multi-producer, multi-consumer)
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `broadcast_create` | `capacity` | `{ sender_id, receiver_id }` |
//! | `broadcast_send` | `sender_id, value` | `{ receivers_count }` |
//! | `broadcast_recv` | `receiver_id` | `{ value?, closed }` |
//! | `broadcast_try_recv` | `receiver_id` | `{ value?, empty, lagged, closed }` |
//! | `broadcast_subscribe` | `sender_id` | `{ receiver_id }` |
//! | `broadcast_sender_close` | `sender_id` | `{ ok }` |
//! | `broadcast_receiver_close` | `receiver_id` | `{ ok }` |
//!
//! ## Mutex\<Value\>
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `mutex_create` | `value?` | `{ mutex_id }` |
//! | `mutex_lock` | `mutex_id` | `{ value }` |
//! | `mutex_update` | `mutex_id, value` | `{ old_value }` |
//! | `mutex_try_lock` | `mutex_id` | `{ value?, acquired }` |
//! | `mutex_close` | `mutex_id` | `{ ok }` |
//!
//! ## RwLock\<Value\>
//!
//! | Tool | Params | Returns |
//! |---|---|---|
//! | `rwlock_create` | `value?` | `{ rwlock_id }` |
//! | `rwlock_read` | `rwlock_id` | `{ value }` |
//! | `rwlock_write` | `rwlock_id, value` | `{ old_value }` |
//! | `rwlock_try_read` | `rwlock_id` | `{ value?, acquired }` |
//! | `rwlock_try_write` | `rwlock_id, value` | `{ old_value?, acquired }` |
//! | `rwlock_close` | `rwlock_id` | `{ ok }` |

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
use tokio::sync::{Mutex, RwLock, broadcast, mpsc, oneshot, watch};
use uuid::Uuid;

type Value = serde_json::Value;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: a message was sent on a channel successfully.
pub struct MessageSent {}
impl Prop for MessageSent {}

/// Proposition: a message was received from a channel.
pub struct MessageReceived {}
impl Prop for MessageReceived {}

/// Proposition: a channel or end was closed.
pub struct ChannelClosed {}
impl Prop for ChannelClosed {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_channel__*` tool calls.
pub struct ChannelCtx {
    // mpsc
    mpsc_senders: Mutex<HashMap<Uuid, mpsc::Sender<Value>>>,
    mpsc_receivers: Mutex<HashMap<Uuid, Mutex<mpsc::Receiver<Value>>>>,
    // oneshot (sender consumed on send → Option)
    oneshot_senders: Mutex<HashMap<Uuid, Option<oneshot::Sender<Value>>>>,
    oneshot_receivers: Mutex<HashMap<Uuid, Mutex<Option<oneshot::Receiver<Value>>>>>,
    // watch
    watch_senders: Mutex<HashMap<Uuid, watch::Sender<Value>>>,
    watch_receivers: Mutex<HashMap<Uuid, Mutex<watch::Receiver<Value>>>>,
    // broadcast
    broadcast_senders: Mutex<HashMap<Uuid, broadcast::Sender<Value>>>,
    broadcast_receivers: Mutex<HashMap<Uuid, Mutex<broadcast::Receiver<Value>>>>,
    // Mutex<Value>
    mutexes: Mutex<HashMap<Uuid, Arc<Mutex<Value>>>>,
    // RwLock<Value>
    rwlocks: Mutex<HashMap<Uuid, Arc<RwLock<Value>>>>,
}

impl ChannelCtx {
    fn new() -> Self {
        Self {
            mpsc_senders: Mutex::new(HashMap::new()),
            mpsc_receivers: Mutex::new(HashMap::new()),
            oneshot_senders: Mutex::new(HashMap::new()),
            oneshot_receivers: Mutex::new(HashMap::new()),
            watch_senders: Mutex::new(HashMap::new()),
            watch_receivers: Mutex::new(HashMap::new()),
            broadcast_senders: Mutex::new(HashMap::new()),
            broadcast_receivers: Mutex::new(HashMap::new()),
            mutexes: Mutex::new(HashMap::new()),
            rwlocks: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for ChannelCtx {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(v: &T) -> CallToolResult {
    CallToolResult::success(vec![Content::text(
        serde_json::to_string(v).unwrap_or_default(),
    )])
}

fn err_not_found(label: &str, id: Uuid) -> ErrorData {
    ErrorData::invalid_params(format!("{label} not found: {id}"), None)
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

// ── mpsc param/result types ───────────────────────────────────────────────────

/// Parameters for `tokio_channel__mpsc_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscCreateParams {
    /// Channel buffer capacity (number of messages that can queue without blocking).
    pub capacity: usize,
}

#[derive(Serialize)]
struct MpscCreateResult {
    sender_id: Uuid,
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__mpsc_send`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscSendParams {
    /// Sender UUID returned by `mpsc_create`.
    pub sender_id: Uuid,
    /// JSON value to send.
    pub value: Value,
}

/// Parameters for `tokio_channel__mpsc_try_send`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscTrySendParams {
    /// Sender UUID returned by `mpsc_create`.
    pub sender_id: Uuid,
    /// JSON value to send.
    pub value: Value,
}

#[derive(Serialize)]
struct MpscTrySendResult {
    ok: bool,
    full: bool,
    closed: bool,
}

/// Parameters for `tokio_channel__mpsc_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscRecvParams {
    /// Receiver UUID returned by `mpsc_create`.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct MpscRecvResult {
    value: Option<Value>,
    /// `true` if all senders have been dropped and the channel is empty.
    closed: bool,
}

/// Parameters for `tokio_channel__mpsc_try_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscTryRecvParams {
    /// Receiver UUID returned by `mpsc_create`.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct MpscTryRecvResult {
    value: Option<Value>,
    empty: bool,
    closed: bool,
}

/// Parameters for `tokio_channel__mpsc_sender_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscSenderCloseParams {
    /// Sender UUID to remove.
    pub sender_id: Uuid,
}

/// Parameters for `tokio_channel__mpsc_receiver_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MpscReceiverCloseParams {
    /// Receiver UUID to remove.
    pub receiver_id: Uuid,
}

// ── oneshot param/result types ────────────────────────────────────────────────

/// Parameters for `tokio_channel__oneshot_create` (none required).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct OneshotCreateParams {}

#[derive(Serialize)]
struct OneshotCreateResult {
    sender_id: Uuid,
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__oneshot_send`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct OneshotSendParams {
    /// Sender UUID returned by `oneshot_create`. Consumed after use.
    pub sender_id: Uuid,
    /// JSON value to send.
    pub value: Value,
}

#[derive(Serialize)]
struct OneshotSendResult {
    ok: bool,
}

/// Parameters for `tokio_channel__oneshot_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct OneshotRecvParams {
    /// Receiver UUID returned by `oneshot_create`.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct OneshotRecvResult {
    value: Option<Value>,
    /// `true` if the sender was dropped without sending.
    closed: bool,
}

/// Parameters for `tokio_channel__oneshot_try_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct OneshotTryRecvParams {
    /// Receiver UUID returned by `oneshot_create`.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct OneshotTryRecvResult {
    value: Option<Value>,
    empty: bool,
    closed: bool,
}

// ── watch param/result types ──────────────────────────────────────────────────

/// Parameters for `tokio_channel__watch_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchCreateParams {
    /// Initial value for the watch channel.
    pub initial_value: Value,
}

#[derive(Serialize)]
struct WatchCreateResult {
    sender_id: Uuid,
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__watch_send`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchSendParams {
    /// Sender UUID returned by `watch_create`.
    pub sender_id: Uuid,
    /// New value to broadcast to all receivers.
    pub value: Value,
}

/// Parameters for `tokio_channel__watch_borrow`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchBorrowParams {
    /// Receiver UUID.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct WatchValueResult {
    value: Value,
}

/// Parameters for `tokio_channel__watch_changed`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchChangedParams {
    /// Receiver UUID. Blocks until the watched value changes.
    pub receiver_id: Uuid,
}

/// Parameters for `tokio_channel__watch_subscribe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchSubscribeParams {
    /// Sender UUID to subscribe a new receiver from.
    pub sender_id: Uuid,
}

#[derive(Serialize)]
struct WatchSubscribeResult {
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__watch_sender_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchSenderCloseParams {
    /// Sender UUID to remove.
    pub sender_id: Uuid,
}

/// Parameters for `tokio_channel__watch_receiver_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WatchReceiverCloseParams {
    /// Receiver UUID to remove.
    pub receiver_id: Uuid,
}

// ── broadcast param/result types ──────────────────────────────────────────────

/// Parameters for `tokio_channel__broadcast_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastCreateParams {
    /// Channel buffer capacity (per-receiver lag window).
    pub capacity: usize,
}

#[derive(Serialize)]
struct BroadcastCreateResult {
    sender_id: Uuid,
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__broadcast_send`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastSendParams {
    /// Sender UUID returned by `broadcast_create`.
    pub sender_id: Uuid,
    /// JSON value to broadcast.
    pub value: Value,
}

#[derive(Serialize)]
struct BroadcastSendResult {
    /// Number of active receivers that will receive this message.
    receivers_count: usize,
}

/// Parameters for `tokio_channel__broadcast_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastRecvParams {
    /// Receiver UUID.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct BroadcastRecvResult {
    value: Option<Value>,
    closed: bool,
}

/// Parameters for `tokio_channel__broadcast_try_recv`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastTryRecvParams {
    /// Receiver UUID.
    pub receiver_id: Uuid,
}

#[derive(Serialize)]
struct BroadcastTryRecvResult {
    value: Option<Value>,
    empty: bool,
    /// `true` if messages were dropped because the receiver fell behind.
    lagged: bool,
    closed: bool,
}

/// Parameters for `tokio_channel__broadcast_subscribe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastSubscribeParams {
    /// Sender UUID to create a new receiver from.
    pub sender_id: Uuid,
}

#[derive(Serialize)]
struct BroadcastSubscribeResult {
    receiver_id: Uuid,
}

/// Parameters for `tokio_channel__broadcast_sender_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastSenderCloseParams {
    /// Sender UUID to remove.
    pub sender_id: Uuid,
}

/// Parameters for `tokio_channel__broadcast_receiver_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastReceiverCloseParams {
    /// Receiver UUID to remove.
    pub receiver_id: Uuid,
}

// ── Mutex<Value> param/result types ──────────────────────────────────────────

/// Parameters for `tokio_channel__mutex_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MutexCreateParams {
    /// Initial JSON value (default: `null`).
    pub value: Option<Value>,
}

#[derive(Serialize)]
struct MutexCreateResult {
    mutex_id: Uuid,
}

/// Parameters for `tokio_channel__mutex_lock`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MutexLockParams {
    /// Mutex UUID returned by `mutex_create`.
    pub mutex_id: Uuid,
}

#[derive(Serialize)]
struct MutexValueResult {
    value: Value,
}

/// Parameters for `tokio_channel__mutex_update`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MutexUpdateParams {
    /// Mutex UUID returned by `mutex_create`.
    pub mutex_id: Uuid,
    /// New value to store.
    pub value: Value,
}

#[derive(Serialize)]
struct MutexUpdateResult {
    old_value: Value,
}

/// Parameters for `tokio_channel__mutex_try_lock`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MutexTryLockParams {
    /// Mutex UUID returned by `mutex_create`.
    pub mutex_id: Uuid,
}

#[derive(Serialize)]
struct MutexTryLockResult {
    value: Option<Value>,
    acquired: bool,
}

/// Parameters for `tokio_channel__mutex_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MutexCloseParams {
    /// Mutex UUID to remove.
    pub mutex_id: Uuid,
}

// ── RwLock<Value> param/result types ─────────────────────────────────────────

/// Parameters for `tokio_channel__rwlock_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockCreateParams {
    /// Initial JSON value (default: `null`).
    pub value: Option<Value>,
}

#[derive(Serialize)]
struct RwLockCreateResult {
    rwlock_id: Uuid,
}

/// Parameters for `tokio_channel__rwlock_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockReadParams {
    /// RwLock UUID returned by `rwlock_create`.
    pub rwlock_id: Uuid,
}

/// Parameters for `tokio_channel__rwlock_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockWriteParams {
    /// RwLock UUID returned by `rwlock_create`.
    pub rwlock_id: Uuid,
    /// New value to store.
    pub value: Value,
}

#[derive(Serialize)]
struct RwLockWriteResult {
    old_value: Value,
}

/// Parameters for `tokio_channel__rwlock_try_read`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockTryReadParams {
    /// RwLock UUID returned by `rwlock_create`.
    pub rwlock_id: Uuid,
}

#[derive(Serialize)]
struct RwLockTryReadResult {
    value: Option<Value>,
    acquired: bool,
}

/// Parameters for `tokio_channel__rwlock_try_write`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockTryWriteParams {
    /// RwLock UUID returned by `rwlock_create`.
    pub rwlock_id: Uuid,
    /// New value to store if the lock is acquired.
    pub value: Value,
}

#[derive(Serialize)]
struct RwLockTryWriteResult {
    old_value: Option<Value>,
    acquired: bool,
}

/// Parameters for `tokio_channel__rwlock_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RwLockCloseParams {
    /// RwLock UUID to remove.
    pub rwlock_id: Uuid,
}

// ── mpsc tools ────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_create",
    description = "Create a bounded mpsc (multi-producer, single-consumer) channel. Returns \
                   sender_id and receiver_id UUIDs. Multiple senders can be created by \
                   calling `mpsc_create` or by cloning — here each create gives a fresh pair.",
    emit = Auto
)]
async fn channel_mpsc_create(
    ctx: Arc<ChannelCtx>,
    p: MpscCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = mpsc::channel(p.capacity.max(1));
    let sender_id = Uuid::new_v4();
    let receiver_id = Uuid::new_v4();
    ctx.mpsc_senders.lock().await.insert(sender_id, tx);
    ctx.mpsc_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(rx));
    Ok(json_result(&MpscCreateResult {
        sender_id,
        receiver_id,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_send",
    description = "Send a JSON value to an mpsc channel. Blocks if the channel is full until \
                   space is available. Errors if the receiver has been closed. \
                   Assumes: sender_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_send(
    ctx: Arc<ChannelCtx>,
    p: MpscSendParams,
) -> Result<CallToolResult, ErrorData> {
    let sender = ctx
        .mpsc_senders
        .lock()
        .await
        .get(&p.sender_id)
        .cloned()
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    sender
        .send(p.value)
        .await
        .map_err(|_| ErrorData::invalid_params("send failed: receiver closed", None))?;
    let _proof: Established<MessageSent> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_try_send",
    description = "Attempt to send a JSON value without blocking. Returns `ok = false` with \
                   `full = true` if the buffer is at capacity, or `closed = true` if the \
                   receiver is gone. \
                   Assumes: sender_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_try_send(
    ctx: Arc<ChannelCtx>,
    p: MpscTrySendParams,
) -> Result<CallToolResult, ErrorData> {
    let sender = ctx
        .mpsc_senders
        .lock()
        .await
        .get(&p.sender_id)
        .cloned()
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    match sender.try_send(p.value) {
        Ok(_) => {
            let _proof: Established<MessageSent> = Established::assert();
            Ok(json_result(&MpscTrySendResult {
                ok: true,
                full: false,
                closed: false,
            }))
        }
        Err(mpsc::error::TrySendError::Full(_)) => Ok(json_result(&MpscTrySendResult {
            ok: false,
            full: true,
            closed: false,
        })),
        Err(mpsc::error::TrySendError::Closed(_)) => Ok(json_result(&MpscTrySendResult {
            ok: false,
            full: false,
            closed: true,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_recv",
    description = "Receive the next value from an mpsc channel. Blocks until a value arrives. \
                   Returns `{ value: null, closed: true }` when all senders are dropped and \
                   the channel is empty. \
                   Assumes: receiver_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_recv(
    ctx: Arc<ChannelCtx>,
    p: MpscRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.mpsc_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let value = rx_mutex.lock().await.recv().await;
    if value.is_some() {
        let _proof: Established<MessageReceived> = Established::assert();
    }
    Ok(json_result(&MpscRecvResult {
        closed: value.is_none(),
        value,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_try_recv",
    description = "Non-blocking receive from an mpsc channel. Returns `empty = true` if no \
                   message is available, `closed = true` if all senders are dropped. \
                   Assumes: receiver_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_try_recv(
    ctx: Arc<ChannelCtx>,
    p: MpscTryRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.mpsc_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    match rx_mutex.lock().await.try_recv() {
        Ok(v) => {
            let _proof: Established<MessageReceived> = Established::assert();
            Ok(json_result(&MpscTryRecvResult {
                value: Some(v),
                empty: false,
                closed: false,
            }))
        }
        Err(mpsc::error::TryRecvError::Empty) => Ok(json_result(&MpscTryRecvResult {
            value: None,
            empty: true,
            closed: false,
        })),
        Err(mpsc::error::TryRecvError::Disconnected) => Ok(json_result(&MpscTryRecvResult {
            value: None,
            empty: false,
            closed: true,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_sender_close",
    description = "Remove a sender from the registry, dropping it. When all senders for a \
                   channel are dropped the receiver will see `closed = true`. \
                   Assumes: sender_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_sender_close(
    ctx: Arc<ChannelCtx>,
    p: MpscSenderCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.mpsc_senders
        .lock()
        .await
        .remove(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mpsc_receiver_close",
    description = "Remove a receiver from the registry, closing the channel. Subsequent sends \
                   will fail. \
                   Assumes: receiver_id was returned by `mpsc_create`.",
    emit = Auto
)]
async fn channel_mpsc_receiver_close(
    ctx: Arc<ChannelCtx>,
    p: MpscReceiverCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.mpsc_receivers
        .lock()
        .await
        .remove(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

// ── oneshot tools ─────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__oneshot_create",
    description = "Create a oneshot channel for a single send/receive pair. Returns sender_id \
                   and receiver_id UUIDs.",
    emit = Auto
)]
async fn channel_oneshot_create(
    ctx: Arc<ChannelCtx>,
    _p: OneshotCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel::<Value>();
    let sender_id = Uuid::new_v4();
    let receiver_id = Uuid::new_v4();
    ctx.oneshot_senders.lock().await.insert(sender_id, Some(tx));
    ctx.oneshot_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(Some(rx)));
    Ok(json_result(&OneshotCreateResult {
        sender_id,
        receiver_id,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__oneshot_send",
    description = "Send a value through a oneshot channel. The sender is consumed — calling \
                   `oneshot_send` again on the same sender_id will fail. \
                   Assumes: sender_id was returned by `oneshot_create`.",
    emit = Auto
)]
async fn channel_oneshot_send(
    ctx: Arc<ChannelCtx>,
    p: OneshotSendParams,
) -> Result<CallToolResult, ErrorData> {
    let mut senders = ctx.oneshot_senders.lock().await;
    let slot = senders
        .get_mut(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    let tx = slot
        .take()
        .ok_or_else(|| ErrorData::invalid_params("oneshot sender already used", None))?;
    tx.send(p.value)
        .map_err(|_| ErrorData::invalid_params("send failed: receiver dropped", None))?;
    let _proof: Established<MessageSent> = Established::assert();
    Ok(json_result(&OneshotSendResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__oneshot_recv",
    description = "Await a value from a oneshot channel. Blocks until the sender sends or is \
                   dropped. Returns `{ value: null, closed: true }` if the sender was dropped \
                   without sending. \
                   Assumes: receiver_id was returned by `oneshot_create`.",
    emit = Auto
)]
async fn channel_oneshot_recv(
    ctx: Arc<ChannelCtx>,
    p: OneshotRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.oneshot_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let mut guard = rx_mutex.lock().await;
    let rx = guard
        .take()
        .ok_or_else(|| ErrorData::invalid_params("oneshot receiver already consumed", None))?;
    match rx.await {
        Ok(v) => {
            let _proof: Established<MessageReceived> = Established::assert();
            Ok(json_result(&OneshotRecvResult {
                value: Some(v),
                closed: false,
            }))
        }
        Err(_) => Ok(json_result(&OneshotRecvResult {
            value: None,
            closed: true,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__oneshot_try_recv",
    description = "Non-blocking check for a oneshot value. Returns `empty = true` if the \
                   sender hasn't sent yet, `closed = true` if it was dropped. \
                   Assumes: receiver_id was returned by `oneshot_create`.",
    emit = Auto
)]
async fn channel_oneshot_try_recv(
    ctx: Arc<ChannelCtx>,
    p: OneshotTryRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.oneshot_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let mut guard = rx_mutex.lock().await;
    let rx = guard
        .as_mut()
        .ok_or_else(|| ErrorData::invalid_params("oneshot receiver already consumed", None))?;
    match rx.try_recv() {
        Ok(v) => {
            *guard = None; // consumed
            let _proof: Established<MessageReceived> = Established::assert();
            Ok(json_result(&OneshotTryRecvResult {
                value: Some(v),
                empty: false,
                closed: false,
            }))
        }
        Err(oneshot::error::TryRecvError::Empty) => Ok(json_result(&OneshotTryRecvResult {
            value: None,
            empty: true,
            closed: false,
        })),
        Err(oneshot::error::TryRecvError::Closed) => Ok(json_result(&OneshotTryRecvResult {
            value: None,
            empty: false,
            closed: true,
        })),
    }
}

// ── watch tools ───────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_create",
    description = "Create a watch channel with an initial value. All receivers always see the \
                   latest value sent. Returns sender_id and receiver_id UUIDs.",
    emit = Auto
)]
async fn channel_watch_create(
    ctx: Arc<ChannelCtx>,
    p: WatchCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = watch::channel(p.initial_value);
    let sender_id = Uuid::new_v4();
    let receiver_id = Uuid::new_v4();
    ctx.watch_senders.lock().await.insert(sender_id, tx);
    ctx.watch_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(rx));
    Ok(json_result(&WatchCreateResult {
        sender_id,
        receiver_id,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_send",
    description = "Broadcast a new value to all watch receivers. \
                   Assumes: sender_id was returned by `watch_create`.",
    emit = Auto
)]
async fn channel_watch_send(
    ctx: Arc<ChannelCtx>,
    p: WatchSendParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.watch_senders
        .lock()
        .await
        .get(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?
        .send(p.value)
        .map_err(|_| ErrorData::invalid_params("send failed: all receivers closed", None))?;
    let _proof: Established<MessageSent> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_borrow",
    description = "Read the current value from a watch receiver without waiting. Returns the \
                   latest value sent (or the initial value if nothing has been sent yet). \
                   Assumes: receiver_id was returned by `watch_create` or `watch_subscribe`.",
    emit = Auto
)]
async fn channel_watch_borrow(
    ctx: Arc<ChannelCtx>,
    p: WatchBorrowParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.watch_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let value = rx_mutex.lock().await.borrow().clone();
    let _proof: Established<MessageReceived> = Established::assert();
    Ok(json_result(&WatchValueResult { value }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_changed",
    description = "Wait until the watched value changes. Blocks until `watch_send` is called \
                   with a new value. After returning, the receiver is marked as seen. \
                   Assumes: receiver_id was returned by `watch_create` or `watch_subscribe`.",
    emit = Auto
)]
async fn channel_watch_changed(
    ctx: Arc<ChannelCtx>,
    p: WatchChangedParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.watch_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    rx_mutex
        .lock()
        .await
        .changed()
        .await
        .map_err(|_| ErrorData::invalid_params("watch sender closed", None))?;
    let _proof: Established<MessageReceived> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_subscribe",
    description = "Create a new receiver from a watch sender, initially marked as changed \
                   (first `watch_borrow` will return the current value). \
                   Assumes: sender_id was returned by `watch_create`.",
    emit = Auto
)]
async fn channel_watch_subscribe(
    ctx: Arc<ChannelCtx>,
    p: WatchSubscribeParams,
) -> Result<CallToolResult, ErrorData> {
    let rx = ctx
        .watch_senders
        .lock()
        .await
        .get(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?
        .subscribe();
    let receiver_id = Uuid::new_v4();
    ctx.watch_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(rx));
    Ok(json_result(&WatchSubscribeResult { receiver_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_sender_close",
    description = "Remove a watch sender from the registry. \
                   Assumes: sender_id was returned by `watch_create`.",
    emit = Auto
)]
async fn channel_watch_sender_close(
    ctx: Arc<ChannelCtx>,
    p: WatchSenderCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.watch_senders
        .lock()
        .await
        .remove(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__watch_receiver_close",
    description = "Remove a watch receiver from the registry. \
                   Assumes: receiver_id was returned by `watch_create` or `watch_subscribe`.",
    emit = Auto
)]
async fn channel_watch_receiver_close(
    ctx: Arc<ChannelCtx>,
    p: WatchReceiverCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.watch_receivers
        .lock()
        .await
        .remove(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

// ── broadcast tools ───────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_create",
    description = "Create a broadcast channel. Every active receiver will receive every sent \
                   message. Receivers that fall behind by more than `capacity` messages will \
                   receive a `lagged` error on next recv. Returns sender_id and receiver_id.",
    emit = Auto
)]
async fn channel_broadcast_create(
    ctx: Arc<ChannelCtx>,
    p: BroadcastCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = broadcast::channel(p.capacity.max(1));
    let sender_id = Uuid::new_v4();
    let receiver_id = Uuid::new_v4();
    ctx.broadcast_senders.lock().await.insert(sender_id, tx);
    ctx.broadcast_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(rx));
    Ok(json_result(&BroadcastCreateResult {
        sender_id,
        receiver_id,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_send",
    description = "Broadcast a JSON value to all active receivers. Returns the number of \
                   receivers that will receive this message. \
                   Assumes: sender_id was returned by `broadcast_create`.",
    emit = Auto
)]
async fn channel_broadcast_send(
    ctx: Arc<ChannelCtx>,
    p: BroadcastSendParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers_count = ctx
        .broadcast_senders
        .lock()
        .await
        .get(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?
        .send(p.value)
        .map_err(|_| ErrorData::invalid_params("send failed: no active receivers", None))?;
    let _proof: Established<MessageSent> = Established::assert();
    Ok(json_result(&BroadcastSendResult { receivers_count }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_recv",
    description = "Receive the next broadcast message. Blocks until a message is available. \
                   Returns `{ value: null, closed: true }` when the sender is dropped. \
                   Assumes: receiver_id was returned by `broadcast_create` or \
                   `broadcast_subscribe`.",
    emit = Auto
)]
async fn channel_broadcast_recv(
    ctx: Arc<ChannelCtx>,
    p: BroadcastRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.broadcast_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    match rx_mutex.lock().await.recv().await {
        Ok(v) => {
            let _proof: Established<MessageReceived> = Established::assert();
            Ok(json_result(&BroadcastRecvResult {
                value: Some(v),
                closed: false,
            }))
        }
        Err(broadcast::error::RecvError::Closed) => Ok(json_result(&BroadcastRecvResult {
            value: None,
            closed: true,
        })),
        Err(broadcast::error::RecvError::Lagged(n)) => Err(ErrorData::invalid_params(
            format!("broadcast receiver lagged by {n} messages"),
            None,
        )),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_try_recv",
    description = "Non-blocking broadcast receive. Returns `empty = true` if no message is \
                   queued, `lagged = true` if messages were dropped due to buffer overflow, \
                   `closed = true` if the sender is gone. \
                   Assumes: receiver_id was returned by `broadcast_create` or \
                   `broadcast_subscribe`.",
    emit = Auto
)]
async fn channel_broadcast_try_recv(
    ctx: Arc<ChannelCtx>,
    p: BroadcastTryRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let receivers = ctx.broadcast_receivers.lock().await;
    let rx_mutex = receivers
        .get(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    match rx_mutex.lock().await.try_recv() {
        Ok(v) => {
            let _proof: Established<MessageReceived> = Established::assert();
            Ok(json_result(&BroadcastTryRecvResult {
                value: Some(v),
                empty: false,
                lagged: false,
                closed: false,
            }))
        }
        Err(broadcast::error::TryRecvError::Empty) => Ok(json_result(&BroadcastTryRecvResult {
            value: None,
            empty: true,
            lagged: false,
            closed: false,
        })),
        Err(broadcast::error::TryRecvError::Closed) => Ok(json_result(&BroadcastTryRecvResult {
            value: None,
            empty: false,
            lagged: false,
            closed: true,
        })),
        Err(broadcast::error::TryRecvError::Lagged(_)) => {
            Ok(json_result(&BroadcastTryRecvResult {
                value: None,
                empty: false,
                lagged: true,
                closed: false,
            }))
        }
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_subscribe",
    description = "Create a new receiver from a broadcast sender. The new receiver will only \
                   see messages sent after subscription. \
                   Assumes: sender_id was returned by `broadcast_create`.",
    emit = Auto
)]
async fn channel_broadcast_subscribe(
    ctx: Arc<ChannelCtx>,
    p: BroadcastSubscribeParams,
) -> Result<CallToolResult, ErrorData> {
    let rx = ctx
        .broadcast_senders
        .lock()
        .await
        .get(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?
        .subscribe();
    let receiver_id = Uuid::new_v4();
    ctx.broadcast_receivers
        .lock()
        .await
        .insert(receiver_id, Mutex::new(rx));
    Ok(json_result(&BroadcastSubscribeResult { receiver_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_sender_close",
    description = "Remove a broadcast sender from the registry. \
                   Assumes: sender_id was returned by `broadcast_create`.",
    emit = Auto
)]
async fn channel_broadcast_sender_close(
    ctx: Arc<ChannelCtx>,
    p: BroadcastSenderCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.broadcast_senders
        .lock()
        .await
        .remove(&p.sender_id)
        .ok_or_else(|| err_not_found("sender_id", p.sender_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__broadcast_receiver_close",
    description = "Remove a broadcast receiver from the registry. \
                   Assumes: receiver_id was returned by `broadcast_create` or \
                   `broadcast_subscribe`.",
    emit = Auto
)]
async fn channel_broadcast_receiver_close(
    ctx: Arc<ChannelCtx>,
    p: BroadcastReceiverCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.broadcast_receivers
        .lock()
        .await
        .remove(&p.receiver_id)
        .ok_or_else(|| err_not_found("receiver_id", p.receiver_id))?;
    let _proof: Established<ChannelClosed> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

// ── Mutex<Value> tools ────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mutex_create",
    description = "Create a tokio async Mutex holding a JSON value (default: null). Returns a \
                   mutex_id UUID for subsequent lock/update/close calls.",
    emit = Auto
)]
async fn channel_mutex_create(
    ctx: Arc<ChannelCtx>,
    p: MutexCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let mutex_id = Uuid::new_v4();
    ctx.mutexes.lock().await.insert(
        mutex_id,
        Arc::new(Mutex::new(p.value.unwrap_or(Value::Null))),
    );
    Ok(json_result(&MutexCreateResult { mutex_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mutex_lock",
    description = "Acquire the mutex and read its current value. Blocks if another task holds \
                   the lock. \
                   Assumes: mutex_id was returned by `mutex_create`.",
    emit = Auto
)]
async fn channel_mutex_lock(
    ctx: Arc<ChannelCtx>,
    p: MutexLockParams,
) -> Result<CallToolResult, ErrorData> {
    let m = ctx
        .mutexes
        .lock()
        .await
        .get(&p.mutex_id)
        .cloned()
        .ok_or_else(|| err_not_found("mutex_id", p.mutex_id))?;
    let value = m.lock().await.clone();
    Ok(json_result(&MutexValueResult { value }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mutex_update",
    description = "Acquire the mutex, replace its value, and release. Returns the old value. \
                   Assumes: mutex_id was returned by `mutex_create`.",
    emit = Auto
)]
async fn channel_mutex_update(
    ctx: Arc<ChannelCtx>,
    p: MutexUpdateParams,
) -> Result<CallToolResult, ErrorData> {
    let m = ctx
        .mutexes
        .lock()
        .await
        .get(&p.mutex_id)
        .cloned()
        .ok_or_else(|| err_not_found("mutex_id", p.mutex_id))?;
    let mut guard = m.lock().await;
    let old_value = std::mem::replace(&mut *guard, p.value);
    Ok(json_result(&MutexUpdateResult { old_value }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mutex_try_lock",
    description = "Attempt to acquire the mutex without blocking. Returns `{ acquired: false }` \
                   if the lock is currently held. \
                   Assumes: mutex_id was returned by `mutex_create`.",
    emit = Auto
)]
async fn channel_mutex_try_lock(
    ctx: Arc<ChannelCtx>,
    p: MutexTryLockParams,
) -> Result<CallToolResult, ErrorData> {
    let m = ctx
        .mutexes
        .lock()
        .await
        .get(&p.mutex_id)
        .cloned()
        .ok_or_else(|| err_not_found("mutex_id", p.mutex_id))?;
    match m.try_lock() {
        Ok(guard) => Ok(json_result(&MutexTryLockResult {
            value: Some(guard.clone()),
            acquired: true,
        })),
        Err(_) => Ok(json_result(&MutexTryLockResult {
            value: None,
            acquired: false,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__mutex_close",
    description = "Remove a mutex from the registry, dropping it. \
                   Assumes: mutex_id was returned by `mutex_create`.",
    emit = Auto
)]
async fn channel_mutex_close(
    ctx: Arc<ChannelCtx>,
    p: MutexCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.mutexes
        .lock()
        .await
        .remove(&p.mutex_id)
        .ok_or_else(|| err_not_found("mutex_id", p.mutex_id))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── RwLock<Value> tools ───────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_create",
    description = "Create a tokio async RwLock holding a JSON value (default: null). Multiple \
                   concurrent readers are allowed; writers get exclusive access. Returns a \
                   rwlock_id UUID.",
    emit = Auto
)]
async fn channel_rwlock_create(
    ctx: Arc<ChannelCtx>,
    p: RwLockCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let rwlock_id = Uuid::new_v4();
    ctx.rwlocks.lock().await.insert(
        rwlock_id,
        Arc::new(RwLock::new(p.value.unwrap_or(Value::Null))),
    );
    Ok(json_result(&RwLockCreateResult { rwlock_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_read",
    description = "Acquire a shared read lock and return the current value. Multiple \
                   concurrent reads are allowed. Blocks if a writer holds the lock. \
                   Assumes: rwlock_id was returned by `rwlock_create`.",
    emit = Auto
)]
async fn channel_rwlock_read(
    ctx: Arc<ChannelCtx>,
    p: RwLockReadParams,
) -> Result<CallToolResult, ErrorData> {
    let rw = ctx
        .rwlocks
        .lock()
        .await
        .get(&p.rwlock_id)
        .cloned()
        .ok_or_else(|| err_not_found("rwlock_id", p.rwlock_id))?;
    let value = rw.read().await.clone();
    Ok(json_result(&MutexValueResult { value }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_write",
    description = "Acquire an exclusive write lock, replace the value, and release. Returns \
                   the old value. \
                   Assumes: rwlock_id was returned by `rwlock_create`.",
    emit = Auto
)]
async fn channel_rwlock_write(
    ctx: Arc<ChannelCtx>,
    p: RwLockWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let rw = ctx
        .rwlocks
        .lock()
        .await
        .get(&p.rwlock_id)
        .cloned()
        .ok_or_else(|| err_not_found("rwlock_id", p.rwlock_id))?;
    let mut guard = rw.write().await;
    let old_value = std::mem::replace(&mut *guard, p.value);
    Ok(json_result(&RwLockWriteResult { old_value }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_try_read",
    description = "Attempt a non-blocking shared read. Returns `{ acquired: false }` if a \
                   writer currently holds the lock. \
                   Assumes: rwlock_id was returned by `rwlock_create`.",
    emit = Auto
)]
async fn channel_rwlock_try_read(
    ctx: Arc<ChannelCtx>,
    p: RwLockTryReadParams,
) -> Result<CallToolResult, ErrorData> {
    let rw = ctx
        .rwlocks
        .lock()
        .await
        .get(&p.rwlock_id)
        .cloned()
        .ok_or_else(|| err_not_found("rwlock_id", p.rwlock_id))?;
    match rw.try_read() {
        Ok(guard) => Ok(json_result(&RwLockTryReadResult {
            value: Some(guard.clone()),
            acquired: true,
        })),
        Err(_) => Ok(json_result(&RwLockTryReadResult {
            value: None,
            acquired: false,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_try_write",
    description = "Attempt a non-blocking exclusive write. Returns `{ acquired: false }` if \
                   any reader or writer currently holds the lock. \
                   Assumes: rwlock_id was returned by `rwlock_create`.",
    emit = Auto
)]
async fn channel_rwlock_try_write(
    ctx: Arc<ChannelCtx>,
    p: RwLockTryWriteParams,
) -> Result<CallToolResult, ErrorData> {
    let rw = ctx
        .rwlocks
        .lock()
        .await
        .get(&p.rwlock_id)
        .cloned()
        .ok_or_else(|| err_not_found("rwlock_id", p.rwlock_id))?;
    match rw.try_write() {
        Ok(mut guard) => {
            let old_value = std::mem::replace(&mut *guard, p.value);
            Ok(json_result(&RwLockTryWriteResult {
                old_value: Some(old_value),
                acquired: true,
            }))
        }
        Err(_) => Ok(json_result(&RwLockTryWriteResult {
            old_value: None,
            acquired: false,
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_channel",
    name = "tokio_channel__rwlock_close",
    description = "Remove a RwLock from the registry, dropping it. \
                   Assumes: rwlock_id was returned by `rwlock_create`.",
    emit = Auto
)]
async fn channel_rwlock_close(
    ctx: Arc<ChannelCtx>,
    p: RwLockCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.rwlocks
        .lock()
        .await
        .remove(&p.rwlock_id)
        .ok_or_else(|| err_not_found("rwlock_id", p.rwlock_id))?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_channel__*` tools for async channels and shared state.
///
/// Holds UUID-keyed registries for:
/// - mpsc senders and receivers
/// - oneshot senders and receivers
/// - watch senders and receivers
/// - broadcast senders and receivers
/// - `Mutex<serde_json::Value>` objects
/// - `RwLock<serde_json::Value>` objects
///
/// All values crossing the MCP boundary are `serde_json::Value`.
pub struct TokioChannelPlugin(Arc<ChannelCtx>);

impl TokioChannelPlugin {
    /// Create a new `TokioChannelPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(ChannelCtx::new()))
    }
}

impl Default for TokioChannelPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioChannelPlugin {
    fn name(&self) -> &'static str {
        "tokio_channel"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_channel")
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
            let full_name = if name.starts_with("tokio_channel__") {
                name.to_string()
            } else {
                format!("tokio_channel__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_channel")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
