//! `TokioSignalPlugin` — MCP tools for OS signal handling.
//!
//! `ctrl_c` is a stateless one-shot await. Unix signal streams are stored
//! server-side in a UUID-keyed registry so agents can wait on specific signals
//! multiple times.
//!
//! # Tool namespace: `tokio_signal__*`
//!
//! | Tool | Params | Returns | Platform |
//! |---|---|---|---|
//! | `ctrl_c` | — | `{ ok }` | all |
//! | `unix_signal_create` | `kind` | `{ signal_id }` | unix only |
//! | `unix_signal_recv` | `signal_id` | `{ ok }` | unix only |
//! | `unix_signal_close` | `signal_id` | `{ ok }` | unix only |
//!
//! ## Unix signal kinds
//!
//! `hangup`, `interrupt`, `quit`, `terminate`, `user_defined1`, `user_defined2`,
//! `pipe`, `alarm`, `child`, `io`, `window_change`

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
#[cfg(unix)]
use std::collections::HashMap;
#[cfg(unix)]
use tokio::sync::Mutex;
#[cfg(unix)]
use uuid::Uuid;

// ── Propositions ─────────────────────────────────────────────────────────────

/// Proposition: `tokio::signal::ctrl_c()` returned — a Ctrl+C signal was received.
#[derive(Elicit)]
pub struct CtrlCReceived {}
impl Prop for CtrlCReceived {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_ctrl_c_received_axiom() {
                let ctrl_c_ok: bool = kani::any();
                kani::assume(ctrl_c_ok);
                assert!(ctrl_c_ok, "tokio::signal::ctrl_c axiom: Ok => SIGINT received");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_ctrl_c_received(ctrl_c_returned_ok: bool) -> (result: bool)
                ensures result == ctrl_c_returned_ok,
            {
                ctrl_c_returned_ok
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
            pub fn verify_ctrl_c_received_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a Unix signal handler was registered successfully.
#[cfg(unix)]
#[derive(Elicit)]
pub struct SignalHandlerRegistered {}
#[cfg(unix)]
impl Prop for SignalHandlerRegistered {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_signal_handler_registered_axiom() {
                let register_ok: bool = kani::any();
                kani::assume(register_ok);
                assert!(register_ok, "tokio::signal::unix::signal axiom: Ok => OS handler registered");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_signal_handler_registered(handler_installed: bool) -> (result: bool)
                ensures result == handler_installed,
            {
                handler_installed
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
            pub fn verify_signal_handler_registered_contract() -> bool {
                true
            }
        }
    }
}

/// Proposition: a registered Unix signal stream received a signal.
#[cfg(unix)]
#[derive(Elicit)]
pub struct SignalReceived {}
#[cfg(unix)]
impl Prop for SignalReceived {
    #[cfg(feature = "proofs")]
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_signal_received_axiom() {
                let sig_some: bool = kani::any();
                kani::assume(sig_some);
                assert!(sig_some, "Signal::recv axiom: Some(()) => registered signal was delivered");
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_signal_received(signal_delivered: bool) -> (result: bool)
                ensures result == signal_delivered,
            {
                signal_delivered
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
            pub fn verify_signal_received_contract() -> bool {
                true
            }
        }
    }
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_signal__*` tool calls.
pub struct SignalCtx {
    #[cfg(unix)]
    unix_signals: Mutex<HashMap<Uuid, Mutex<tokio::signal::unix::Signal>>>,
    #[cfg(not(unix))]
    _unused: (),
}

impl SignalCtx {
    fn new() -> Self {
        Self {
            #[cfg(unix)]
            unix_signals: Mutex::new(HashMap::new()),
            #[cfg(not(unix))]
            _unused: (),
        }
    }
}

impl PluginContext for SignalCtx {}

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

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `tokio_signal__ctrl_c` (none required).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CtrlCParams {}

/// Unix signal kind for `tokio_signal__unix_signal_create`.
#[cfg(unix)]
#[derive(Debug, Deserialize, JsonSchema, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum UnixSignalKind {
    /// SIGHUP — terminal disconnected or daemon reload
    Hangup,
    /// SIGINT — Ctrl+C
    Interrupt,
    /// SIGQUIT — quit with core dump
    Quit,
    /// SIGTERM — graceful termination request
    Terminate,
    /// SIGUSR1 — user-defined signal 1
    UserDefined1,
    /// SIGUSR2 — user-defined signal 2
    UserDefined2,
    /// SIGPIPE — broken pipe
    Pipe,
    /// SIGALRM — timer alarm
    Alarm,
    /// SIGCHLD — child process status changed
    Child,
    /// SIGIO — I/O ready on file descriptor
    Io,
    /// SIGWINCH — terminal window size changed
    WindowChange,
}

/// Parameters for `tokio_signal__unix_signal_create`.
#[cfg(unix)]
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixSignalCreateParams {
    /// The Unix signal to listen for.
    pub kind: UnixSignalKind,
}

#[cfg(unix)]
#[derive(Serialize)]
struct UnixSignalCreateResult {
    signal_id: Uuid,
}

/// Parameters for `tokio_signal__unix_signal_recv`.
#[cfg(unix)]
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixSignalRecvParams {
    /// Signal UUID returned by `unix_signal_create`.
    pub signal_id: Uuid,
}

/// Parameters for `tokio_signal__unix_signal_close`.
#[cfg(unix)]
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnixSignalCloseParams {
    /// Signal UUID to remove.
    pub signal_id: Uuid,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_signal",
    name = "tokio_signal__ctrl_c",
    description = "Wait for a Ctrl+C (SIGINT) signal. Blocks until the signal is received. \
                   Available on all platforms.",
    emit = Auto
)]
async fn signal_ctrl_c(_ctx: Arc<SignalCtx>, _p: CtrlCParams) -> Result<CallToolResult, ErrorData> {
    tokio::signal::ctrl_c()
        .await
        .map_err(|e| ErrorData::invalid_params(format!("ctrl_c failed: {e}"), None))?;
    let _proof: Established<CtrlCReceived> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[cfg(unix)]
#[elicitation::elicit_tool(
    plugin = "tokio_signal",
    name = "tokio_signal__unix_signal_create",
    description = "Register a handler for a Unix signal. Returns a signal_id UUID that can be \
                   passed to `unix_signal_recv` to block until the signal arrives. \
                   Unix only.",
    emit = None
)]
async fn signal_unix_create(
    ctx: Arc<SignalCtx>,
    p: UnixSignalCreateParams,
) -> Result<CallToolResult, ErrorData> {
    use tokio::signal::unix::{SignalKind, signal};
    let kind = match p.kind {
        UnixSignalKind::Hangup => SignalKind::hangup(),
        UnixSignalKind::Interrupt => SignalKind::interrupt(),
        UnixSignalKind::Quit => SignalKind::quit(),
        UnixSignalKind::Terminate => SignalKind::terminate(),
        UnixSignalKind::UserDefined1 => SignalKind::user_defined1(),
        UnixSignalKind::UserDefined2 => SignalKind::user_defined2(),
        UnixSignalKind::Pipe => SignalKind::pipe(),
        UnixSignalKind::Alarm => SignalKind::alarm(),
        UnixSignalKind::Child => SignalKind::child(),
        UnixSignalKind::Io => SignalKind::io(),
        UnixSignalKind::WindowChange => SignalKind::window_change(),
    };
    let sig = signal(kind)
        .map_err(|e| ErrorData::invalid_params(format!("signal registration failed: {e}"), None))?;
    let signal_id = Uuid::new_v4();
    ctx.unix_signals
        .lock()
        .await
        .insert(signal_id, Mutex::new(sig));
    let _proof: Established<SignalHandlerRegistered> = Established::assert();
    Ok(json_result(&UnixSignalCreateResult { signal_id }))
}

#[cfg(unix)]
#[elicitation::elicit_tool(
    plugin = "tokio_signal",
    name = "tokio_signal__unix_signal_recv",
    description = "Wait for the next occurrence of a registered Unix signal. Blocks until the \
                   signal is received. Call again to wait for subsequent occurrences. \
                   Assumes: signal_id was returned by `unix_signal_create`. Unix only.",
    emit = Auto
)]
async fn signal_unix_recv(
    ctx: Arc<SignalCtx>,
    p: UnixSignalRecvParams,
) -> Result<CallToolResult, ErrorData> {
    let signals = ctx.unix_signals.lock().await;
    let sig_mutex = signals.get(&p.signal_id).ok_or_else(|| {
        ErrorData::invalid_params(format!("signal_id not found: {}", p.signal_id), None)
    })?;
    let received = sig_mutex.lock().await.recv().await;
    if received.is_some() {
        let _proof: Established<SignalReceived> = Established::assert();
        Ok(json_result(&OkResult { ok: true }))
    } else {
        Err(ErrorData::invalid_params("signal stream closed", None))
    }
}

#[cfg(unix)]
#[elicitation::elicit_tool(
    plugin = "tokio_signal",
    name = "tokio_signal__unix_signal_close",
    description = "Remove a Unix signal handler from the registry. \
                   Assumes: signal_id was returned by `unix_signal_create`. Unix only.",
    emit = Auto
)]
async fn signal_unix_close(
    ctx: Arc<SignalCtx>,
    p: UnixSignalCloseParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.unix_signals
        .lock()
        .await
        .remove(&p.signal_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("signal_id not found: {}", p.signal_id), None)
        })?;
    Ok(json_result(&OkResult { ok: true }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_signal__*` tools for OS signal handling.
///
/// `ctrl_c` is available on all platforms. Unix signal tools (`unix_signal_*`)
/// are compiled only on Unix targets.
pub struct TokioSignalPlugin(Arc<SignalCtx>);

impl TokioSignalPlugin {
    /// Create a new `TokioSignalPlugin`.
    pub fn new() -> Self {
        Self(Arc::new(SignalCtx::new()))
    }
}

impl Default for TokioSignalPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioSignalPlugin {
    fn name(&self) -> &'static str {
        "tokio_signal"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_signal")
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
            let full_name = if name.starts_with("tokio_signal__") {
                name.to_string()
            } else {
                format!("tokio_signal__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_signal")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
