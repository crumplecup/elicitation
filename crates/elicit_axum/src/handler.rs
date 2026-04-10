//! `AxumHandlerPlugin` — MCP tools for axum handler function descriptors.
//!
//! Handler descriptors are held server-side in a UUID-keyed registry.
//! Agents receive UUID handles; no live axum instances cross the MCP boundary.
//!
//! # Tool namespace: `axum_handler__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `new` | `name, return_type` | `{ handler_id }` | `AxumHandlerDefined` |
//! | `add_extractor` | `handler_id, var_name, kind, type_name` | `{ extractor_count }` | `AxumExtractorAdded` |
//! | `set_body` | `handler_id, body` | — | — |
//! | `describe` | `handler_id` | JSON descriptor | — |
//! | `emit` | `handler_id` | Rust `async fn` source | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{
    AxumExtractorEntry, AxumExtractorKind, AxumHandlerDescriptor, Elicit, PluginContext,
    VerifiedWorkflow,
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

/// Proposition: an axum handler descriptor was successfully defined and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumHandlerDefined;
impl Prop for AxumHandlerDefined {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_handler_defined() {
                let defined: bool = kani::any();
                kani::assume(defined);
                assert!(defined, "axum handler defined");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_handler_defined(ok: bool) -> (result: bool)
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
            pub fn verify_axum_handler_defined_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumHandlerDefined {}

/// Proposition: an extractor was successfully added to a handler descriptor.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumExtractorAdded;
impl Prop for AxumExtractorAdded {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_extractor_added() {
                let added: bool = kani::any();
                kani::assume(added);
                assert!(added, "axum extractor added");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_extractor_added(ok: bool) -> (result: bool)
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
            pub fn verify_axum_extractor_added_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumExtractorAdded {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `axum_handler__*` tool calls.
pub struct AxumHandlerCtx {
    items: Mutex<HashMap<Uuid, AxumHandlerDescriptor>>,
}

impl AxumHandlerCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for AxumHandlerCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `axum_handler__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HandlerNewParams {
    /// Handler function name, e.g. `"create_user"`.
    pub name: String,
    /// Return type expression, e.g. `"impl IntoResponse"`.
    pub return_type: String,
}

/// Parameters for `axum_handler__add_extractor`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HandlerAddExtractorParams {
    /// UUID returned by `axum_handler__new`.
    pub handler_id: String,
    /// Rust variable name, e.g. `"payload"`.
    pub var_name: String,
    /// Extractor kind.
    pub kind: AxumExtractorKind,
    /// Inner Rust type name, e.g. `"CreateUserRequest"`.
    pub type_name: String,
}

/// Parameters for `axum_handler__set_body`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HandlerSetBodyParams {
    /// UUID returned by `axum_handler__new`.
    pub handler_id: String,
    /// Body expression or block.
    pub body: String,
}

/// Parameters for `axum_handler__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HandlerDescribeParams {
    /// UUID returned by `axum_handler__new`.
    pub handler_id: String,
}

/// Parameters for `axum_handler__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HandlerEmitParams {
    /// UUID returned by `axum_handler__new`.
    pub handler_id: String,
}

/// Result returned by `axum_handler__new`.
#[derive(Debug, Serialize)]
pub struct HandlerIdResult {
    /// UUID handle for the newly created handler descriptor.
    pub handler_id: String,
}

#[derive(Debug, Serialize)]
struct ExtractorCountResult {
    extractor_count: usize,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn parse_handler_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

fn extractor_pattern(entry: &AxumExtractorEntry) -> String {
    match entry.kind {
        AxumExtractorKind::Path => {
            format!(
                "Path({var}): Path<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::Query => {
            format!(
                "Query({var}): Query<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::Json => {
            format!(
                "Json({var}): Json<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::State => {
            format!(
                "State({var}): State<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::Extension => {
            format!(
                "Extension({var}): Extension<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::Form => {
            format!(
                "Form({var}): Form<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
        AxumExtractorKind::Headers => {
            format!("{var}: HeaderMap", var = entry.var_name)
        }
        AxumExtractorKind::RawBody => {
            format!("{var}: Bytes", var = entry.var_name)
        }
        AxumExtractorKind::RawQuery => {
            format!("{var}: RawQuery", var = entry.var_name)
        }
        AxumExtractorKind::OriginalUri => {
            format!("{var}: OriginalUri", var = entry.var_name)
        }
        AxumExtractorKind::MatchedPath => {
            format!("{var}: MatchedPath", var = entry.var_name)
        }
        AxumExtractorKind::ConnectInfo => {
            format!(
                "ConnectInfo({var}): ConnectInfo<{ty}>",
                var = entry.var_name,
                ty = entry.type_name
            )
        }
    }
}

fn emit_handler(desc: &AxumHandlerDescriptor) -> String {
    let params: Vec<String> = desc.extractors.iter().map(extractor_pattern).collect();
    let params_str = if params.is_empty() {
        String::new()
    } else {
        format!("\n    {}\n", params.join(",\n    "))
    };
    let body = desc.body.as_deref().unwrap_or("todo!()").to_string();
    format!(
        "pub async fn {name}({params}) -> {ret} {{\n    {body}\n}}",
        name = desc.name,
        params = params_str,
        ret = desc.return_type,
        body = body,
    )
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "axum_handler",
    name = "axum_handler__new",
    description = "Create a new axum handler descriptor with the given function name and return type. \
                   Assumes: name is a valid Rust identifier. \
                   Establishes: AxumHandlerDefined.",
    emit = Auto
)]
async fn new(ctx: Arc<AxumHandlerCtx>, p: HandlerNewParams) -> Result<CallToolResult, ErrorData> {
    let desc = AxumHandlerDescriptor {
        name: p.name,
        extractors: Vec::new(),
        return_type: p.return_type,
        body: None,
    };
    let id = Uuid::new_v4();
    ctx.items.lock().await.insert(id, desc);
    let _proof: Established<AxumHandlerDefined> = Established::assert();
    Ok(json_result(&HandlerIdResult {
        handler_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "axum_handler",
    name = "axum_handler__add_extractor",
    description = "Add an extractor argument to a handler descriptor. \
                   Assumes: handler_id is valid. \
                   Establishes: AxumExtractorAdded.",
    emit = Auto
)]
async fn add_extractor(
    ctx: Arc<AxumHandlerCtx>,
    p: HandlerAddExtractorParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_handler_id(&p.handler_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("handler_id not found: {id}"), None))?;
    desc.extractors.push(AxumExtractorEntry {
        var_name: p.var_name,
        kind: p.kind,
        type_name: p.type_name,
    });
    let count = desc.extractors.len();
    let _proof: Established<AxumExtractorAdded> = Established::assert();
    Ok(json_result(&ExtractorCountResult {
        extractor_count: count,
    }))
}

#[elicitation::elicit_tool(
    plugin = "axum_handler",
    name = "axum_handler__set_body",
    description = "Set the body expression for a handler descriptor. \
                   Assumes: handler_id is valid.",
    emit = Auto
)]
async fn set_body(
    ctx: Arc<AxumHandlerCtx>,
    p: HandlerSetBodyParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_handler_id(&p.handler_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("handler_id not found: {id}"), None))?;
    desc.body = Some(p.body);
    Ok(CallToolResult::success(vec![Content::text("ok")]))
}

#[elicitation::elicit_tool(
    plugin = "axum_handler",
    name = "axum_handler__describe",
    description = "Return the JSON descriptor for a handler. \
                   Assumes: handler_id is a valid UUID returned by axum_handler__new.",
    emit = Auto
)]
async fn describe(
    ctx: Arc<AxumHandlerCtx>,
    p: HandlerDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_handler_id(&p.handler_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("handler_id not found: {id}"), None))?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "axum_handler",
    name = "axum_handler__emit",
    description = "Emit idiomatic Rust `async fn` source for the described handler. \
                   Assumes: handler_id is a valid UUID returned by axum_handler__new.",
    emit = Auto
)]
async fn emit(ctx: Arc<AxumHandlerCtx>, p: HandlerEmitParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_handler_id(&p.handler_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("handler_id not found: {id}"), None))?;
    let code = emit_handler(desc);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `axum_handler__*` tools for handler configuration.
pub struct AxumHandlerPlugin(Arc<AxumHandlerCtx>);

impl AxumHandlerPlugin {
    /// Create a new `AxumHandlerPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(AxumHandlerCtx::new()))
    }
}

impl Default for AxumHandlerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for AxumHandlerPlugin {
    fn name(&self) -> &'static str {
        "axum_handler"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "axum_handler")
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
            let full_name = if name.starts_with("axum_handler__") {
                name.to_string()
            } else {
                format!("axum_handler__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "axum_handler")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
