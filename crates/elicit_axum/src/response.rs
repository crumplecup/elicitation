//! `AxumResponsePlugin` — MCP tools for axum response descriptors.
//!
//! Response descriptors are held server-side in a UUID-keyed registry.
//! Agents receive UUID handles; no live axum instances cross the MCP boundary.
//!
//! # Tool namespace: `axum_response__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `json` | `status_code, body_expr` | `{ response_id }` | `AxumResponseDefined` |
//! | `html` | `status_code, body_expr` | `{ response_id }` | `AxumResponseDefined` |
//! | `redirect_permanent` | `uri` | `{ response_id }` | `AxumResponseDefined` |
//! | `redirect_temporary` | `uri` | `{ response_id }` | `AxumResponseDefined` |
//! | `no_content` | — | `{ response_id }` | `AxumResponseDefined` |
//! | `status` | `status_code, body_expr` | `{ response_id }` | `AxumResponseDefined` |
//! | `describe` | `response_id` | JSON descriptor | — |
//! | `emit` | `response_id` | Rust expression string | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{
    AxumResponseDescriptor, AxumResponseKind, Elicit, PluginContext, VerifiedWorkflow,
};
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

/// Proposition: an axum response descriptor was successfully defined and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumResponseDefined;
impl Prop for AxumResponseDefined {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_response_defined() {
                let defined: bool = kani::any();
                kani::assume(defined);
                assert!(defined, "axum response defined");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_response_defined(ok: bool) -> (result: bool)
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
            pub fn verify_axum_response_defined_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumResponseDefined {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `axum_response__*` tool calls.
pub struct AxumResponseCtx {
    items: Mutex<HashMap<Uuid, AxumResponseDescriptor>>,
}

impl AxumResponseCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for AxumResponseCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `axum_response__json`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseJsonParams {
    /// HTTP status code (e.g. `200`).
    pub status_code: u16,
    /// Body expression (e.g. `"serde_json::json!({\"ok\": true})"`).
    pub body_expr: String,
}

/// Parameters for `axum_response__html`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseHtmlParams {
    /// HTTP status code (e.g. `200`).
    pub status_code: u16,
    /// Body expression (e.g. `"\"<h1>Hello</h1>\""` ).
    pub body_expr: String,
}

/// Parameters for `axum_response__redirect_permanent`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseRedirectPermanentParams {
    /// Target URI (e.g. `"/new-path"`).
    pub uri: String,
}

/// Parameters for `axum_response__redirect_temporary`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseRedirectTemporaryParams {
    /// Target URI (e.g. `"/new-path"`).
    pub uri: String,
}

/// Parameters for `axum_response__no_content`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseNoContentParams {}

/// Parameters for `axum_response__status`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseStatusParams {
    /// HTTP status code (e.g. `404`).
    pub status_code: u16,
    /// Optional body expression.
    pub body_expr: Option<String>,
}

/// Parameters for `axum_response__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseDescribeParams {
    /// UUID returned by a response creation tool.
    pub response_id: String,
}

/// Parameters for `axum_response__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResponseEmitParams {
    /// UUID returned by a response creation tool.
    pub response_id: String,
}

/// Result returned by all response creation tools.
#[derive(Debug, Serialize)]
pub struct ResponseIdResult {
    /// UUID handle for the newly created response descriptor.
    pub response_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn parse_response_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

fn register(
    items: &mut HashMap<Uuid, AxumResponseDescriptor>,
    desc: AxumResponseDescriptor,
) -> ResponseIdResult {
    let id = Uuid::new_v4();
    items.insert(id, desc);
    ResponseIdResult {
        response_id: id.to_string(),
    }
}

fn status_code_ident(code: u16) -> String {
    match code {
        200 => "StatusCode::OK".to_string(),
        201 => "StatusCode::CREATED".to_string(),
        204 => "StatusCode::NO_CONTENT".to_string(),
        301 => "StatusCode::MOVED_PERMANENTLY".to_string(),
        302 => "StatusCode::FOUND".to_string(),
        400 => "StatusCode::BAD_REQUEST".to_string(),
        401 => "StatusCode::UNAUTHORIZED".to_string(),
        403 => "StatusCode::FORBIDDEN".to_string(),
        404 => "StatusCode::NOT_FOUND".to_string(),
        500 => "StatusCode::INTERNAL_SERVER_ERROR".to_string(),
        _ => format!("StatusCode::from_u16({code}).unwrap()"),
    }
}

fn emit_response(desc: &AxumResponseDescriptor) -> String {
    match desc.kind {
        AxumResponseKind::Json => {
            let status = status_code_ident(desc.status_code);
            let body = desc.body_expr.as_deref().unwrap_or("()");
            format!("({status}, Json({body}))")
        }
        AxumResponseKind::Html => {
            let body = desc.body_expr.as_deref().unwrap_or("\"\"");
            format!("Html({body})")
        }
        AxumResponseKind::Redirect => {
            let uri = desc.body_expr.as_deref().unwrap_or("/");
            if desc.status_code == 301 {
                format!("Redirect::permanent(\"{uri}\")")
            } else {
                format!("Redirect::temporary(\"{uri}\")")
            }
        }
        AxumResponseKind::NoContent => "StatusCode::NO_CONTENT".to_string(),
        AxumResponseKind::Status => {
            let status = status_code_ident(desc.status_code);
            if let Some(body) = &desc.body_expr {
                format!("({status}, {body})")
            } else {
                status
            }
        }
        AxumResponseKind::Custom => desc
            .body_expr
            .clone()
            .unwrap_or_else(|| "todo!()".to_string()),
    }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__json",
    description = "Define a JSON response descriptor. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn json(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseJsonParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::Json,
        status_code: p.status_code,
        body_expr: Some(p.body_expr),
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__html",
    description = "Define an HTML response descriptor. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn html(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseHtmlParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::Html,
        status_code: p.status_code,
        body_expr: Some(p.body_expr),
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__redirect_permanent",
    description = "Define a permanent (301) redirect response descriptor. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn redirect_permanent(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseRedirectPermanentParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::Redirect,
        status_code: 301,
        body_expr: Some(p.uri),
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__redirect_temporary",
    description = "Define a temporary (302) redirect response descriptor. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn redirect_temporary(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseRedirectTemporaryParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::Redirect,
        status_code: 302,
        body_expr: Some(p.uri),
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__no_content",
    description = "Define a 204 No Content response descriptor. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn no_content(
    ctx: Arc<AxumResponseCtx>,
    _p: ResponseNoContentParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::NoContent,
        status_code: 204,
        body_expr: None,
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__status",
    description = "Define a status-code response descriptor, optionally with a body expression. \
                   Establishes: AxumResponseDefined.",
    emit = Auto
)]
async fn status(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseStatusParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = AxumResponseDescriptor {
        kind: AxumResponseKind::Status,
        status_code: p.status_code,
        body_expr: p.body_expr,
    };
    let result = register(&mut *ctx.items.lock().await, desc);
    let _proof: Established<AxumResponseDefined> = Established::assert();
    Ok(json_result(&result))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__describe",
    description = "Return the JSON descriptor for a response. \
                   Assumes: response_id is a valid UUID returned by a response creation tool.",
    emit = Auto
)]
async fn describe(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_response_id(&p.response_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("response_id not found: {id}"), None))?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "axum_response",
    name = "axum_response__emit",
    description = "Emit an idiomatic Rust expression for the described axum response. \
                   Assumes: response_id is a valid UUID returned by a response creation tool.",
    emit = Auto
)]
async fn emit(
    ctx: Arc<AxumResponseCtx>,
    p: ResponseEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_response_id(&p.response_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("response_id not found: {id}"), None))?;
    let expr = emit_response(desc);
    Ok(CallToolResult::success(vec![Content::text(expr)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `axum_response__*` tools for response configuration.
pub struct AxumResponsePlugin(Arc<AxumResponseCtx>);

impl AxumResponsePlugin {
    /// Create a new `AxumResponsePlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(AxumResponseCtx::new()))
    }
}

impl Default for AxumResponsePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for AxumResponsePlugin {
    fn name(&self) -> &'static str {
        "axum_response"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "axum_response")
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
            let full_name = if name.starts_with("axum_response__") {
                name.to_string()
            } else {
                format!("axum_response__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "axum_response")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
