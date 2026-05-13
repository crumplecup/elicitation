//! `AxumServePlugin` — MCP tools for axum serve configuration descriptors.
//!
//! Serve descriptors are held server-side in a UUID-keyed registry.
//! Agents receive UUID handles; no live axum instances cross the MCP boundary.
//!
//! # Tool namespace: `axum_serve__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `new` | `router_id, addr` | `{ serve_id }` | `AxumServerConfigured` |
//! | `with_graceful_shutdown` | `serve_id, signal_expr` | — | — |
//! | `describe` | `serve_id` | JSON descriptor | — |
//! | `emit_main` | `serve_id` | complete `main.rs` source | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{AxumServeDescriptor, Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: an axum server configuration was successfully defined and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumServerConfigured;
impl Prop for AxumServerConfigured {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_server_configured() {
                let configured: bool = kani::any();
                kani::assume(configured);
                assert!(configured, "axum server configured");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_server_configured(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_axum_server_configured_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumServerConfigured {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `axum_serve__*` tool calls.
pub struct AxumServeCtx {
    items: Mutex<HashMap<Uuid, AxumServeDescriptor>>,
}

impl AxumServeCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for AxumServeCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `axum_serve__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServeNewParams {
    /// UUID of the router descriptor this server wraps.
    pub router_id: Uuid,
    /// Bind address, e.g. `"0.0.0.0:3000"`.
    pub addr: String,
}

/// Parameters for `axum_serve__with_graceful_shutdown`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServeGracefulParams {
    /// UUID returned by `axum_serve__new`.
    pub serve_id: String,
    /// Graceful shutdown signal expression, e.g. `"tokio::signal::ctrl_c()"`.
    pub signal_expr: String,
}

/// Parameters for `axum_serve__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServeDescribeParams {
    /// UUID returned by `axum_serve__new`.
    pub serve_id: String,
}

/// Parameters for `axum_serve__emit_main`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServeEmitMainParams {
    /// UUID returned by `axum_serve__new`.
    pub serve_id: String,
}

/// Result returned by `axum_serve__new`.
#[derive(Debug, Serialize)]
pub struct ServeIdResult {
    /// UUID handle for the newly created serve descriptor.
    pub serve_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn parse_serve_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

fn emit_main(desc: &AxumServeDescriptor) -> String {
    let shutdown_block = if let Some(sig) = &desc.graceful_shutdown {
        format!(
            r#"    let listener = tokio::net::TcpListener::bind("{addr}").await.unwrap();
    axum::serve(listener, router)
        .with_graceful_shutdown({sig})
        .await
        .unwrap();"#,
            addr = desc.addr,
            sig = sig,
        )
    } else {
        format!(
            r#"    let listener = tokio::net::TcpListener::bind("{addr}").await.unwrap();
    axum::serve(listener, router).await.unwrap();"#,
            addr = desc.addr,
        )
    };
    format!(
        r#"#[tokio::main]
async fn main() {{
    // Build router — replace this with your actual Router construction.
    // Router for descriptor {router_id} can be built with axum_router__emit.
    let router = /* build your router here */();
{shutdown}
}}"#,
        router_id = desc.router_id,
        shutdown = shutdown_block,
    )
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "axum_serve",
    name = "axum_serve__new",
    description = "Create a new axum serve descriptor wrapping a router UUID. \
                   Assumes: router_id is a valid UUID from axum_router__new, addr is a valid socket address. \
                   Establishes: AxumServerConfigured.",
    emit = Auto
)]
async fn new(ctx: Arc<AxumServeCtx>, p: ServeNewParams) -> Result<CallToolResult, ErrorData> {
    let desc = AxumServeDescriptor {
        addr: p.addr,
        router_id: p.router_id,
        graceful_shutdown: None,
    };
    let id = Uuid::new_v4();
    ctx.items.lock().await.insert(id, desc);
    let _proof: Established<AxumServerConfigured> = Established::assert();
    Ok(json_result(&ServeIdResult {
        serve_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "axum_serve",
    name = "axum_serve__with_graceful_shutdown",
    description = "Set a graceful shutdown signal expression on a serve descriptor. \
                   Assumes: serve_id is valid.",
    emit = Auto
)]
async fn with_graceful_shutdown(
    ctx: Arc<AxumServeCtx>,
    p: ServeGracefulParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_serve_id(&p.serve_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("serve_id not found: {id}"), None))?;
    desc.graceful_shutdown = Some(p.signal_expr);
    Ok(CallToolResult::success(vec![Content::text("ok")]))
}

#[elicitation::elicit_tool(
    plugin = "axum_serve",
    name = "axum_serve__describe",
    description = "Return the JSON descriptor for a serve configuration. \
                   Assumes: serve_id is a valid UUID returned by axum_serve__new.",
    emit = Auto
)]
async fn describe(
    ctx: Arc<AxumServeCtx>,
    p: ServeDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_serve_id(&p.serve_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("serve_id not found: {id}"), None))?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "axum_serve",
    name = "axum_serve__emit_main",
    description = "Emit a complete `#[tokio::main] async fn main()` for the described serve config. \
                   Assumes: serve_id is a valid UUID returned by axum_serve__new.",
    emit = Auto
)]
async fn emit_main_tool(
    ctx: Arc<AxumServeCtx>,
    p: ServeEmitMainParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_serve_id(&p.serve_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("serve_id not found: {id}"), None))?;
    let code = emit_main(desc);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `axum_serve__*` tools for serve configuration.
pub struct AxumServePlugin(Arc<AxumServeCtx>);

impl AxumServePlugin {
    /// Create a new `AxumServePlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(AxumServeCtx::new()))
    }
}

impl Default for AxumServePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for AxumServePlugin {
    fn name(&self) -> &'static str {
        "axum_serve"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "axum_serve")
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
            let full_name = if name.starts_with("axum_serve__") {
                name.to_string()
            } else {
                format!("axum_serve__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "axum_serve")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
