//! Bridge from a [`LeptosAxumPlugin`] descriptor to an [`AxumRouterPlugin`] descriptor.
//!
//! [`LeptosAxumBridgePlugin`] holds shared references to both a
//! [`LeptosAxumCtx`] and an [`elicit_axum::router::AxumRouterCtx`].
//! Its single tool — `leptos_axum_bridge__from_config` — reads a
//! [`LeptosAxumDescriptor`] by UUID and injects an equivalent
//! [`elicit_axum::AxumRouterDescriptor`] into the axum registry.
//!
//! The returned UUID is immediately composable with all `axum_router__*` and
//! `axum_serve__*` tools, so the entire pipeline remains verified tool calls:
//!
//! ```text
//! leptos_axum__new(...)                         → leptos_config_id
//! leptos_axum_bridge__from_config(
//!     leptos_config_id, display_mode: "standard" → axum_router_id
//! )
//! axum_router__add_layer(axum_router_id, "TraceLayer::new_for_http()")
//! axum_serve__new(axum_router_id, "0.0.0.0:8080") → server_id
//! axum_serve__emit(server_id)                   → main.rs
//! ```
//!
//! # Display modes
//!
//! [`LeptosDisplayMode`] selects the shell component that wraps the app:
//!
//! | Variant | Shell | Description |
//! |---------|-------|-------------|
//! | `bare` | `<{App}/>` | No wrapper — raw component |
//! | `standard` | `<StandardShell>` | Responsive shell with nav header |
//! | `dashboard` | `<DashboardShell>` | Sidebar + main pane layout |

use std::sync::Arc;

use elicit_axum::{AxumRouterPlugin, router::AxumRouterCtx};
use elicitation::{LeptosAxumMode, LeptosDisplayMode};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData, RoleServer,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::axum_ssr::{LeptosAxumCtx, LeptosAxumPlugin};

// ── Shell emit helpers ────────────────────────────────────────────────────────

fn shell_import(display: &LeptosDisplayMode) -> Option<String> {
    match display {
        LeptosDisplayMode::Bare => None,
        LeptosDisplayMode::Standard => {
            Some("use elicit_leptos::shells::StandardShell;".to_string())
        }
        LeptosDisplayMode::Dashboard => {
            Some("use elicit_leptos::shells::DashboardShell;".to_string())
        }
    }
}

fn wrap_app(app: &str, display: &LeptosDisplayMode) -> String {
    match display {
        LeptosDisplayMode::Bare => app.to_owned(),
        LeptosDisplayMode::Standard => {
            format!("move || view! {{ <StandardShell><{app}/></StandardShell> }}")
        }
        LeptosDisplayMode::Dashboard => {
            format!("move || view! {{ <DashboardShell><{app}/></DashboardShell> }}")
        }
    }
}

// ── Descriptor translation ────────────────────────────────────────────────────

/// Translate a [`LeptosAxumDescriptor`] into the raw method-call strings and
/// route entries that an [`elicit_axum::AxumRouterDescriptor`] needs.
///
/// When `db_slot` is provided and `provide_leptos_context` is `true`, the
/// `leptos_routes` call is upgraded to `leptos_routes_with_context` so that
/// every Leptos server function can call `use_context::<{pool_type}>()`.
async fn build_axum_descriptor(
    leptos_ctx: &LeptosAxumCtx,
    config_id: Uuid,
    display: &LeptosDisplayMode,
    db_slot: Option<elicitation::AxumDbSlot>,
) -> Result<elicitation::AxumRouterDescriptor, ErrorData> {
    let desc = leptos_ctx.get(config_id).await.ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {config_id}"), None)
    })?;

    let app = &desc.app_component;
    let mut routes: Vec<elicitation::AxumRouteEntry> = Vec::new();
    let mut raw_calls: Vec<String> = Vec::new();
    let mut layers: Vec<String> = Vec::new();
    let fallback: Option<String>;
    let state_type: String;

    // Translate custom routes
    for r in &desc.custom_routes {
        routes.push(elicitation::AxumRouteEntry {
            method: elicitation::AxumHttpMethod::Any,
            path: r.path.clone(),
            handler: r.handler.clone(),
        });
    }

    match desc.mode {
        LeptosAxumMode::StaticHtml => {
            state_type = db_slot
                .as_ref()
                .map(|s| s.pool_type.clone())
                .unwrap_or_else(|| "()".to_string());
            // Static HTML uses axum::response::Html directly — no leptos_axum state.
            fallback = None;
        }

        LeptosAxumMode::FullSsr | LeptosAxumMode::WasmShell => {
            state_type = db_slot
                .as_ref()
                .map(|s| s.pool_type.clone())
                .unwrap_or_else(|| "LeptosOptions".to_string());

            // Server-fn handler route
            let sfn_path = desc.server_fn_route.as_deref().unwrap_or("/api/leptos");
            routes.push(elicitation::AxumRouteEntry {
                method: elicitation::AxumHttpMethod::Post,
                path: sfn_path.to_string(),
                handler: "leptos_axum::handle_server_fns".to_string(),
            });

            // WASM shell: serve /pkg directory
            if matches!(desc.mode, LeptosAxumMode::WasmShell) {
                let pkg = &desc.pkg_dir;
                layers.push(format!("tower_http::services::ServeDir::new(\"{pkg}\")"));
            }

            // .leptos_routes() — upgraded to _with_context when a db slot is present.
            let app_expr = wrap_app(app, display);
            if let Some(slot) = &db_slot {
                if slot.provide_leptos_context {
                    let var = &slot.var_name;
                    raw_calls.push(format!(
                        "leptos_routes_with_context(\
                         &leptos_options, routes, \
                         move || {{ provide_context({var}.clone()); }}, \
                         {app_expr})"
                    ));
                } else {
                    raw_calls.push(format!(
                        "leptos_routes(&leptos_options, routes, {app_expr})"
                    ));
                }
            } else {
                raw_calls.push(format!(
                    "leptos_routes(&leptos_options, routes, {app_expr})"
                ));
            }

            fallback = if desc.static_file_handler {
                Some("leptos_axum::file_and_error_handler(shell)".to_string())
            } else {
                None
            };
        }
    }

    // Response headers as tower layers
    for h in &desc.response_headers {
        layers.push(format!(
            "tower_http::set_header::SetResponseHeaderLayer::overriding(\
             axum::http::header::HeaderName::from_static(\"{name}\"), \
             axum::http::HeaderValue::from_static(\"{value}\"))",
            name = h.name,
            value = h.value,
        ));
    }

    // Prepend shell import comment if a non-bare mode is used
    if let Some(import) = shell_import(display) {
        layers.insert(0, format!("/* {import} */"));
    }

    Ok(elicitation::AxumRouterDescriptor {
        state_type,
        routes,
        raw_method_calls: raw_calls,
        layers,
        fallback,
        db_slot,
        custom_state_expr: None,
    })
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for [`LeptosAxumBridgePlugin`].
pub struct LeptosAxumBridgeCtx {
    leptos: Arc<LeptosAxumCtx>,
    router: Arc<AxumRouterCtx>,
}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `leptos_axum_bridge__from_config`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BridgeFromConfigParams {
    /// UUID returned by `leptos_axum__new`.
    pub config_id: String,
    /// Shell wrapper / theme applied around the app component.
    /// One of `"bare"`, `"standard"` (default), or `"dashboard"`.
    pub display_mode: Option<LeptosDisplayMode>,
    /// Rust type of the db pool or state struct to wire as Axum state
    /// (e.g. `"sqlx::AnyPool"`, `"Arc<AppState>"`).
    ///
    /// When set together with `db_var_name`, the bridge emits
    /// `.with_state({db_var_name})` and (if `provide_leptos_context` is true)
    /// upgrades `leptos_routes` to `leptos_routes_with_context`.
    pub db_pool_type: Option<String>,
    /// Variable name used in `.with_state({db_var_name})` (e.g. `"pool"`).
    pub db_var_name: Option<String>,
    /// When `true` (and `db_pool_type` is set), injects `provide_context` so
    /// Leptos server functions can call `use_context::<{db_pool_type}>()`.
    pub provide_leptos_context: Option<bool>,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Bridge plugin that converts Leptos descriptors to Axum router descriptors.
///
/// Construct with shared context from both plugins:
/// ```ignore
/// let leptos_plugin = LeptosAxumPlugin::new();
/// let router_plugin = AxumRouterPlugin::new();
/// let bridge = LeptosAxumBridgePlugin::new(&leptos_plugin, &router_plugin);
/// ```
pub struct LeptosAxumBridgePlugin(Arc<LeptosAxumBridgeCtx>);

impl LeptosAxumBridgePlugin {
    /// Create a bridge plugin sharing registries with the provided plugins.
    pub fn new(leptos: &LeptosAxumPlugin, router: &AxumRouterPlugin) -> Self {
        Self(Arc::new(LeptosAxumBridgeCtx {
            leptos: leptos.ctx(),
            router: router.ctx(),
        }))
    }

    /// Invoke a bridge tool by name with a JSON argument object.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let ctx = Arc::clone(&self.0);
        let args_val: serde_json::Value = if args.is_object() {
            args
        } else {
            serde_json::Value::Object(Default::default())
        };
        match name.trim_start_matches("leptos_axum_bridge__") {
            "from_config" => {
                let p: BridgeFromConfigParams = serde_json::from_value(args_val)
                    .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                from_config_impl(ctx, p).await
            }
            other => Err(ErrorData::invalid_params(
                format!("unknown bridge tool: {other}"),
                None,
            )),
        }
    }
}

// ── Tool implementation ───────────────────────────────────────────────────────

#[instrument(skip(ctx), fields(config_id = %p.config_id))]
async fn from_config_impl(
    ctx: Arc<LeptosAxumBridgeCtx>,
    p: BridgeFromConfigParams,
) -> Result<CallToolResult, ErrorData> {
    let config_id: Uuid = p
        .config_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.config_id), None))?;
    let display = p.display_mode.unwrap_or_default();

    // Build optional db slot from convenience params.
    let db_slot = match (p.db_pool_type, p.db_var_name) {
        (Some(pool_type), Some(var_name)) => Some(elicitation::AxumDbSlot {
            pool_type,
            var_name,
            provide_leptos_context: p.provide_leptos_context.unwrap_or(false),
        }),
        (Some(_), None) | (None, Some(_)) => {
            return Err(ErrorData::invalid_params(
                "db_pool_type and db_var_name must both be set (or both omitted)",
                None,
            ));
        }
        (None, None) => None,
    };

    let desc = build_axum_descriptor(&ctx.leptos, config_id, &display, db_slot).await?;
    let router_id = ctx.router.insert(desc).await;

    Ok(CallToolResult::success(vec![Content::text(
        serde_json::json!({ "router_id": router_id.to_string() }).to_string(),
    )]))
}

// ── ElicitPlugin impl ─────────────────────────────────────────────────────────

impl elicitation::ElicitPlugin for LeptosAxumBridgePlugin {
    fn name(&self) -> &'static str {
        "leptos_axum_bridge"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![bridge_tool_definition()]
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let ctx = Arc::clone(&self.0);
        Box::pin(async move {
            let name = params.name.as_ref();
            let args: serde_json::Value = params
                .arguments
                .map(serde_json::Value::Object)
                .unwrap_or(serde_json::Value::Object(Default::default()));
            match name {
                "leptos_axum_bridge__from_config" => {
                    let p: BridgeFromConfigParams = serde_json::from_value(args)
                        .map_err(|e| ErrorData::invalid_params(e.to_string(), None))?;
                    from_config_impl(ctx, p).await
                }
                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}

fn bridge_tool_definition() -> Tool {
    use std::sync::Arc;
    let schema = schemars::schema_for!(BridgeFromConfigParams);
    let schema_value = serde_json::to_value(&schema).unwrap_or_default();
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => unreachable!(),
    };
    Tool::new(
        "leptos_axum_bridge__from_config",
        "Convert a LeptosAxumPlugin descriptor (by UUID) into an AxumRouterPlugin descriptor \
         that is immediately composable with axum_router__* and axum_serve__* tools. \
         Assumes: config_id is a valid UUID returned by leptos_axum__new. \
         Establishes: the returned router_id is usable with axum_router__add_layer, \
         axum_router__add_route, axum_serve__new, etc.",
        schema_obj,
    )
}
