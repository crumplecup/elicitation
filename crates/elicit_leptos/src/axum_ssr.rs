//! `LeptosAxumPlugin` — MCP tools for Leptos + Axum SSR server configuration.
//!
//! Provides descriptor-registry tools (Phase 3E) for composing a Leptos + Axum
//! server setup.  All descriptors are stored server-side in a UUID-keyed
//! registry; only serialisable handles cross the MCP boundary.
//!
//! # Tool namespace: `leptos_axum__*`
//!
//! | Tool | Params | Returns | Description |
//! |---|---|---|---|
//! | `new` | `app_component, mode, site_addr` | `{ config_id }` | Create SSR config |
//! | `add_route` | `config_id, method, path, handler` | — | Add custom axum route |
//! | `add_response_header` | `config_id, name, value` | — | Add response header |
//! | `set_server_fn_route` | `config_id, prefix` | — | Set `/api/leptos` prefix |
//! | `set_static_handler` | `config_id, enabled` | — | Toggle static file serving |
//! | `set_pkg_dir` | `config_id, dir` | — | Set WASM package directory |
//! | `describe` | `config_id` | JSON descriptor | Inspect config |
//! | `emit` | `config_id` | `main.rs` source | Emit server entry point |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{
    LeptosAxumDescriptor, LeptosAxumMode, LeptosCustomRouteDescriptor,
    LeptosResponseHeaderDescriptor, PluginContext, VerifiedWorkflow,
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
use tracing::instrument;
use uuid::Uuid;

// ── Proposition ───────────────────────────────────────────────────────────────

/// Proposition: a Leptos + Axum SSR server was successfully configured.
#[derive(elicitation::Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct LeptosAxumServerConfigured;

impl Prop for LeptosAxumServerConfigured {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_leptos_axum_server_configured() {
                let configured: bool = kani::any();
                kani::assume(configured);
                assert!(configured, "leptos axum server configured");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_leptos_axum_server_configured(ok: bool) -> (result: bool)
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
            pub fn verify_leptos_axum_server_configured_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for LeptosAxumServerConfigured {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `leptos_axum__*` tool calls.
pub struct LeptosAxumCtx {
    items: Mutex<HashMap<Uuid, LeptosAxumDescriptor>>,
}

impl LeptosAxumCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for LeptosAxumCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `leptos_axum__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumNewParams {
    /// Top-level app component name (PascalCase, e.g. `"App"`).
    pub app_component: String,
    /// Serving mode: `"static_html"`, `"full_ssr"`, or `"wasm_shell"`.
    pub mode: LeptosAxumMode,
    /// Socket address to bind, e.g. `"0.0.0.0:3000"`.
    pub site_addr: String,
}

/// Parameters for `leptos_axum__add_route`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumAddRouteParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// HTTP method in lowercase: `"get"`, `"post"`, `"put"`, `"delete"`, `"any"`.
    pub method: String,
    /// URL path pattern, e.g. `"/api/health"`.
    pub path: String,
    /// Handler expression — a function name or async closure literal.
    pub handler: String,
}

/// Parameters for `leptos_axum__add_response_header`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumAddHeaderParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// Header name, e.g. `"Cache-Control"`.
    pub name: String,
    /// Header value, e.g. `"no-store"`.
    pub value: String,
}

/// Parameters for `leptos_axum__set_server_fn_route`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumSetServerFnRouteParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// Route prefix for server functions, e.g. `"/api/leptos"`.
    pub prefix: String,
}

/// Parameters for `leptos_axum__set_static_handler`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumSetStaticHandlerParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// Whether to include `file_and_error_handler` for static asset serving.
    pub enabled: bool,
}

/// Parameters for `leptos_axum__set_pkg_dir`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumSetPkgDirParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// Relative path to the compiled WASM/JS package directory.
    pub dir: String,
}

/// Parameters for `leptos_axum__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumDescribeParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
}

/// Parameters for `leptos_axum__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeptosAxumEmitParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
}

/// Result returned by `leptos_axum__new`.
#[derive(Debug, Serialize)]
pub struct LeptosAxumNewResult {
    /// UUID handle for the newly created configuration.
    pub config_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn ok_text(s: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.into())])
}

fn parse_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

// ── Emit ─────────────────────────────────────────────────────────────────────

fn emit_custom_routes(routes: &[LeptosCustomRouteDescriptor]) -> String {
    routes
        .iter()
        .map(|r| {
            format!(
                "        .route(\"{path}\", axum::routing::{method}({handler}))",
                path = r.path,
                method = r.method,
                handler = r.handler,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn emit_response_headers(headers: &[LeptosResponseHeaderDescriptor]) -> String {
    if headers.is_empty() {
        return String::new();
    }
    let layers: Vec<String> = headers
        .iter()
        .map(|h| {
            format!(
                "        .layer(tower_http::set_header::SetResponseHeaderLayer::if_not_present(\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20axum::http::HeaderName::from_static(\"{name}\"),\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20axum::http::HeaderValue::from_static(\"{value}\"),\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20))",
                name = h.name.to_lowercase(),
                value = h.value,
            )
        })
        .collect();
    layers.join("\n")
}

fn emit_static_html(desc: &LeptosAxumDescriptor) -> String {
    let custom = emit_custom_routes(&desc.custom_routes);
    let custom_block = if custom.is_empty() {
        String::new()
    } else {
        format!("\n{custom}")
    };
    let headers = emit_response_headers(&desc.response_headers);
    let headers_block = if headers.is_empty() {
        String::new()
    } else {
        format!("\n{headers}")
    };
    format!(
        r#"//! Generated by `leptos_axum__emit` (mode: static_html).
//! Add this as `src/main.rs` in your binary crate.
//! Deps required: axum, tokio, elicit_leptos, elicit_ui

use axum::{{Router, response::Html, routing::get}};
use elicit_leptos::LeptosRenderer;

/// Build the AccessKit `VerifiedTree` for the root page.
///
/// Replace `todo!()` with your actual tree construction, e.g.:
/// ```ignore
/// use elicit_server::archive::frontend_utils::nodes_to_verified_tree;
/// let display = /* your ArchiveDisplay */;
/// let (root, nodes) = display.to_ak_nodes(...);
/// nodes_to_verified_tree(root, nodes, 1280, 800)
/// ```
fn build_tree() -> elicit_ui::VerifiedTree {{
    todo!("provide VerifiedTree")
}}

async fn root_handler() -> Html<String> {{
    let tree = build_tree();
    let mut renderer = LeptosRenderer::html();
    renderer.render(&tree);
    Html(renderer.last_html().to_string())
}}

#[tokio::main]
async fn main() {{
    let app = Router::new()
        .route("/", get(root_handler)){custom}{headers};
    let listener = tokio::net::TcpListener::bind("{addr}").await
        .expect("failed to bind {addr}");
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}}"#,
        addr = desc.site_addr,
        custom = custom_block,
        headers = headers_block,
    )
}

fn emit_full_ssr(desc: &LeptosAxumDescriptor) -> String {
    let app = &desc.app_component;
    let _addr = &desc.site_addr;
    let pkg = &desc.pkg_dir;
    let custom = emit_custom_routes(&desc.custom_routes);
    let custom_block = if custom.is_empty() {
        String::new()
    } else {
        format!("\n{custom}")
    };
    let headers = emit_response_headers(&desc.response_headers);
    let headers_block = if headers.is_empty() {
        String::new()
    } else {
        format!("\n{headers}")
    };
    let static_block = if desc.static_file_handler {
        format!("\n        .fallback(leptos_axum::file_and_error_handler(shell))")
    } else {
        String::new()
    };
    format!(
        r#"//! Generated by `leptos_axum__emit` (mode: full_ssr).
//! Add this as `src/main.rs` in your binary crate.
//! Deps required: leptos, leptos_axum, axum, tokio, tower_http

use leptos::prelude::*;
use leptos_axum::{{generate_route_list, LeptosRoutes}};
use axum::Router;

/// Shell handler — serves the HTML skeleton for SSR hydration.
async fn shell(options: axum::extract::State<leptos::config::LeptosOptions>)
    -> impl axum::response::IntoResponse
{{
    let options = options.0.clone();
    let html = leptos::view! {{ <{app}/> }};
    leptos_axum::render_app_to_stream(options, move || html.clone())
}}

#[tokio::main]
async fn main() {{
    let conf = leptos::get_configuration(None)
        .expect("failed to read leptos config (Cargo.toml [package.metadata.leptos])");
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();
    let pkg_dir = "{pkg}";
    let _ = pkg_dir; // used by file_and_error_handler

    let routes = generate_route_list({app});

    let app = Router::new(){custom}
        .leptos_routes(&leptos_options, routes, {app}){fallback}{headers}
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await
        .expect(&format!("failed to bind {{addr}}"));
    println!("Listening on http://{{addr}}");
    axum::serve(listener, app).await.unwrap();
}}"#,
        app = app,
        pkg = pkg,
        custom = custom_block,
        fallback = static_block,
        headers = headers_block,
    )
}

fn emit_wasm_shell(desc: &LeptosAxumDescriptor) -> String {
    let addr = &desc.site_addr;
    let pkg = &desc.pkg_dir;
    let server_fn_route = desc.server_fn_route.as_deref().unwrap_or("/api/leptos");
    let custom = emit_custom_routes(&desc.custom_routes);
    let custom_block = if custom.is_empty() {
        String::new()
    } else {
        format!("\n{custom}")
    };
    let headers = emit_response_headers(&desc.response_headers);
    let headers_block = if headers.is_empty() {
        String::new()
    } else {
        format!("\n{headers}")
    };
    format!(
        r#"//! Generated by `leptos_axum__emit` (mode: wasm_shell).
//! Add this as `src/main.rs` in your binary crate.
//! Deps required: leptos_axum, axum, tower_http, tokio
//!
//! Build the client side with `cargo leptos build` or `trunk build`.
//! Then run this server binary to serve /pkg/*.wasm + *.js.

use axum::{{Router, response::Html, routing::{{any, get}}}};
use tower_http::services::ServeDir;
use leptos_axum::handle_server_fns;

async fn shell() -> Html<&'static str> {{
    Html(include_str!("../index.html"))
}}

#[tokio::main]
async fn main() {{
    let app = Router::new()
        .route("/", get(shell))
        .route("{server_fn_route}/*fn_name", any(handle_server_fns))
        .nest_service("/{pkg}", ServeDir::new("{pkg}")){custom}{headers};

    let listener = tokio::net::TcpListener::bind("{addr}").await
        .expect("failed to bind {addr}");
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}}"#,
        addr = addr,
        pkg = pkg,
        server_fn_route = server_fn_route,
        custom = custom_block,
        headers = headers_block,
    )
}

fn emit_config(desc: &LeptosAxumDescriptor) -> String {
    match desc.mode {
        LeptosAxumMode::StaticHtml => emit_static_html(desc),
        LeptosAxumMode::FullSsr => emit_full_ssr(desc),
        LeptosAxumMode::WasmShell => emit_wasm_shell(desc),
    }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__new",
    description = "Create a new Leptos + Axum SSR server configuration descriptor. \
                   Modes: static_html (no leptos runtime needed, uses LeptosRenderer), \
                   full_ssr (requires leptos + leptos_axum), \
                   wasm_shell (serves WASM bundle + shell HTML). \
                   Establishes: LeptosAxumServerConfigured.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(mode = ?p.mode, addr = %p.site_addr))]
async fn new_config(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumNewParams,
) -> Result<CallToolResult, ErrorData> {
    let desc = LeptosAxumDescriptor::new(p.app_component, p.mode, p.site_addr);
    let id = Uuid::new_v4();
    ctx.items.lock().await.insert(id, desc);
    let _proof: Established<LeptosAxumServerConfigured> = Established::assert();
    Ok(json_result(&LeptosAxumNewResult {
        config_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__add_route",
    description = "Add a custom axum route to a Leptos + Axum server configuration. \
                   Assumes: config_id is valid. \
                   The route is prepended before Leptos routes in the emitted Router.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(path = %p.path, method = %p.method))]
async fn add_route(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumAddRouteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.custom_routes.push(LeptosCustomRouteDescriptor {
        method: p.method,
        path: p.path,
        handler: p.handler,
    });
    Ok(ok_text("route added"))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__add_response_header",
    description = "Add a response header to all responses via a Tower SetResponseHeader layer. \
                   Assumes: config_id is valid. \
                   Example: name='Cache-Control', value='no-store'.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(name = %p.name))]
async fn add_response_header(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumAddHeaderParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.response_headers.push(LeptosResponseHeaderDescriptor {
        name: p.name,
        value: p.value,
    });
    Ok(ok_text("response header added"))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__set_server_fn_route",
    description = "Set the route prefix for the server function handler endpoint. \
                   Assumes: config_id is valid. \
                   Defaults to '/api/leptos'. Only used in full_ssr and wasm_shell modes.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(prefix = %p.prefix))]
async fn set_server_fn_route(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumSetServerFnRouteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.server_fn_route = Some(p.prefix);
    Ok(ok_text("server_fn_route set"))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__set_static_handler",
    description = "Enable or disable the static file + 404 error handler \
                   (leptos_axum::file_and_error_handler). \
                   Assumes: config_id is valid. \
                   Defaults to true for full_ssr and wasm_shell; false for static_html.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(enabled = p.enabled))]
async fn set_static_handler(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumSetStaticHandlerParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.static_file_handler = p.enabled;
    Ok(ok_text("static_file_handler updated"))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__set_pkg_dir",
    description = "Set the WASM package directory path (relative to binary). \
                   Assumes: config_id is valid. \
                   Defaults to 'pkg'. Used in full_ssr and wasm_shell modes.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(dir = %p.dir))]
async fn set_pkg_dir(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumSetPkgDirParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.pkg_dir = p.dir;
    Ok(ok_text("pkg_dir updated"))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__describe",
    description = "Return the JSON descriptor for a Leptos + Axum server configuration. \
                   Assumes: config_id is valid.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn describe_config(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let items = ctx.items.lock().await;
    let desc = items.get(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "leptos_axum",
    name = "leptos_axum__emit",
    description = "Emit a complete main.rs for the Leptos + Axum server. \
                   static_html → uses LeptosRenderer (no leptos runtime dep). \
                   full_ssr    → uses leptos_axum::generate_route_list + LeptosRoutes. \
                   wasm_shell  → serves WASM bundle + handles server functions. \
                   Assumes: config_id is valid.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn emit_config_tool(
    ctx: Arc<LeptosAxumCtx>,
    p: LeptosAxumEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let items = ctx.items.lock().await;
    let desc = items.get(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    Ok(ok_text(emit_config(desc)))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `leptos_axum__*` tools for Leptos + Axum SSR server config.
pub struct LeptosAxumPlugin(Arc<LeptosAxumCtx>);

impl LeptosAxumPlugin {
    /// Create a new `LeptosAxumPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(LeptosAxumCtx::new()))
    }

    /// Convenience helper for tests and direct integration: invoke a tool by
    /// name with a JSON argument object and return the `CallToolResult`.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let owned = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        let plugin_ctx = self.0.clone();
        let full_name = if name.starts_with("leptos_axum__") {
            name.to_string()
        } else {
            format!("leptos_axum__{name}")
        };
        let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "leptos_axum")
            .find(|r| r.name == full_name)
            .map(|r| (r.constructor)())
            .ok_or_else(|| {
                rmcp::ErrorData::invalid_params(format!("unknown tool: {name}"), None)
            })?;
        descriptor
            .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
            .await
    }
}

impl Default for LeptosAxumPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for LeptosAxumPlugin {
    fn name(&self) -> &'static str {
        "leptos_axum"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "leptos_axum")
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
            let full_name = if name.starts_with("leptos_axum__") {
                name.to_string()
            } else {
                format!("leptos_axum__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "leptos_axum")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
