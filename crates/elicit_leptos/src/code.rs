//! `LeptosCodePlugin` — MCP tools for Leptos 0.8 code generation.
//!
//! # Tool namespace: `leptos_code__*`

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use elicitation::{LeptosComponentDescriptor, LeptosPropDescriptor};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_text(s: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s.into())]))
}

fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    serde_json::to_string(v)
        .map(|s| CallToolResult::success(vec![Content::text(s)]))
        .map_err(|e| tool_err(format!("serialise: {e}")))
}

fn parse_params<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let raw = params
        .arguments
        .as_ref()
        .map(|a| serde_json::Value::Object(a.clone()))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    serde_json::from_value(raw)
        .map_err(|e| ErrorData::invalid_params(format!("param parse: {e}"), None))
}

fn build_tool(
    name: impl Into<std::borrow::Cow<'static, str>>,
    description: impl Into<std::borrow::Cow<'static, str>>,
    schema: serde_json::Value,
) -> Tool {
    let schema_obj: Arc<rmcp::model::JsonObject> = match schema {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

fn schema_of<T: schemars::JsonSchema>() -> serde_json::Value {
    serde_json::to_value(schemars::schema_for!(T)).unwrap_or_default()
}

// ── Stored entries ────────────────────────────────────────────────────────────

/// A stored server function entry.
#[derive(Debug, Clone)]
pub struct ServerFnEntry {
    /// Function name.
    pub name: String,
    /// Arguments as (name, type) pairs.
    pub args: Vec<(String, String)>,
    /// Return type string.
    pub return_type: String,
    /// Function body.
    pub body: String,
}

/// A stored route entry.
#[derive(Debug, Clone)]
pub struct RouteEntry {
    /// URL path.
    pub path: String,
    /// View component name.
    pub view: String,
    /// Nested route UUIDs.
    pub nested: Vec<Uuid>,
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for `leptos_code__*` tools.
pub struct LeptosCodeContext {
    components: Mutex<HashMap<Uuid, LeptosComponentDescriptor>>,
    server_fns: Mutex<HashMap<Uuid, ServerFnEntry>>,
    routes: Mutex<HashMap<Uuid, RouteEntry>>,
    apps: Mutex<HashMap<Uuid, AppEntry>>,
}

/// A stored application entry.
#[derive(Debug, Clone)]
pub struct AppEntry {
    /// Package name (snake_case).
    pub package_name: String,
    /// Rendering mode string.
    pub mode: String,
    /// Component UUIDs.
    pub component_ids: Vec<Uuid>,
    /// Route UUIDs.
    pub route_ids: Vec<Uuid>,
}

impl LeptosCodeContext {
    fn new() -> Self {
        Self {
            components: Mutex::new(HashMap::new()),
            server_fns: Mutex::new(HashMap::new()),
            routes: Mutex::new(HashMap::new()),
            apps: Mutex::new(HashMap::new()),
        }
    }
}

impl elicitation::PluginContext for LeptosCodeContext {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `leptos_code__component_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentNewParams {
    /// Component name (PascalCase).
    pub name: String,
}

/// Parameters for `leptos_code__component_add_prop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentAddPropParams {
    /// UUID of the component.
    pub id: String,
    /// Prop name (Rust identifier).
    pub name: String,
    /// Prop type as Rust type string.
    pub ty: String,
    /// Whether the prop is optional.
    #[serde(default)]
    pub optional: bool,
    /// Default value expression.
    #[serde(default)]
    pub default_value: Option<String>,
    /// Whether to apply `#[prop(into)]`.
    #[serde(default)]
    pub into: bool,
}

/// Parameters for `leptos_code__component_set_body`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentSetBodyParams {
    /// UUID of the component.
    pub id: String,
    /// Body code (view! macro body or raw Rust).
    pub body: String,
}

/// Parameters for `leptos_code__component_set_island`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentSetIslandParams {
    /// UUID of the component.
    pub id: String,
    /// Whether this is an island component.
    pub island: bool,
}

/// Parameters for `leptos_code__component_get` / delete / emit.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IdParams {
    /// UUID of the item.
    pub id: String,
}

/// Parameters for `leptos_code__view_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ViewEmitParams {
    /// Inner content of the view! macro.
    pub content: String,
}

/// Parameters for `leptos_code__element_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ElementEmitParams {
    /// HTML tag name.
    pub tag: String,
    /// Static attributes as (name, value) pairs.
    #[serde(default)]
    pub attrs: Vec<(String, String)>,
    /// Event handlers as (event, handler_body) pairs.
    #[serde(default)]
    pub on_events: Vec<(String, String)>,
    /// Inner children string.
    #[serde(default)]
    pub children: String,
    /// Whether this is a self-closing tag.
    #[serde(default)]
    pub self_closing: bool,
}

/// Parameters for `leptos_code__show_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShowEmitParams {
    /// Condition expression.
    pub when_expr: String,
    /// Children rendered when true.
    pub children: String,
    /// Fallback rendered when false.
    #[serde(default)]
    pub fallback: Option<String>,
}

/// Parameters for `leptos_code__for_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ForEmitParams {
    /// Expression yielding an iterator.
    pub each_expr: String,
    /// Key expression.
    pub key_expr: String,
    /// Variable name bound in loop.
    pub let_var: String,
    /// Loop body.
    pub children: String,
}

/// Parameters for `leptos_code__suspense_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SuspenseEmitParams {
    /// Fallback content while loading.
    pub fallback: String,
    /// Inner content.
    pub children: String,
}

/// Parameters for `leptos_code__transition_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TransitionEmitParams {
    /// Fallback content during first load.
    #[serde(default)]
    pub fallback: Option<String>,
    /// Inner content.
    pub children: String,
}

/// Parameters for `leptos_code__error_boundary_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ErrorBoundaryEmitParams {
    /// Fallback render function body.
    pub fallback: String,
    /// Inner content.
    pub children: String,
}

/// Parameters for `leptos_code__reactive_binding_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReactiveBindingEmitParams {
    /// Signal variable name.
    pub signal: String,
    /// Kind: "get", "set", or "rw".
    pub kind: String,
}

/// Parameters for `leptos_code__event_handler_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventHandlerEmitParams {
    /// Event name (e.g. "click", "input").
    pub event: String,
    /// Handler body.
    pub handler: String,
}

/// Parameters for `leptos_code__class_binding_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClassBindingEmitParams {
    /// CSS class name.
    pub class: String,
    /// Condition expression.
    pub condition: String,
}

/// Parameters for `leptos_code__attr_binding_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AttrBindingEmitParams {
    /// Attribute name.
    pub attr: String,
    /// Value expression.
    pub value_expr: String,
}

/// Parameters for `leptos_code__router_link_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterLinkEmitParams {
    /// Href path.
    pub href: String,
    /// Link text or inner content.
    pub children: String,
}

/// Parameters for `leptos_code__server_fn_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServerFnNewParams {
    /// Function name.
    pub name: String,
    /// Return type (default "()").
    #[serde(default)]
    pub return_type: Option<String>,
    /// Function body.
    #[serde(default)]
    pub body: Option<String>,
}

/// Parameters for `leptos_code__server_fn_add_arg`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServerFnAddArgParams {
    /// UUID of the server function.
    pub id: String,
    /// Argument name.
    pub name: String,
    /// Argument type.
    pub ty: String,
}

/// Parameters for `leptos_code__server_fn_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServerFnEmitParams {
    /// UUID of the server function.
    pub id: String,
}

/// Parameters for `leptos_code__resource_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResourceEmitParams {
    /// Signal source expression.
    pub source: String,
    /// Async fetcher expression.
    pub fetcher: String,
    /// Variable name for the resource.
    #[serde(default)]
    pub var_name: Option<String>,
}

/// Parameters for `leptos_code__action_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActionEmitParams {
    /// Action input type.
    pub input_type: String,
    /// Async handler expression (closure body).
    pub handler: String,
    /// Variable name for the action.
    #[serde(default)]
    pub var_name: Option<String>,
}

/// Parameters for `leptos_code__server_action_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ServerActionEmitParams {
    /// Server function name to wrap.
    pub server_fn_name: String,
    /// Variable name.
    #[serde(default)]
    pub var_name: Option<String>,
}

/// Parameters for `leptos_code__action_form_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActionFormEmitParams {
    /// Action variable name.
    pub action: String,
    /// Inner form content.
    pub children: String,
}

/// Parameters for `leptos_code__route_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouteNewParams {
    /// URL path pattern.
    pub path: String,
    /// View component name.
    pub view: String,
}

/// Parameters for `leptos_code__route_add_nested`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouteAddNestedParams {
    /// Parent route UUID.
    pub parent_id: String,
    /// Child route UUID.
    pub child_id: String,
}

/// Parameters for `leptos_code__router_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterEmitParams {
    /// Route UUIDs to include.
    pub route_ids: Vec<String>,
}

/// Parameters for `leptos_code__route_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouteEmitParams {
    /// Route UUID.
    pub id: String,
}

/// Parameters for `leptos_code__use_params_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UseParamsEmitParams {
    /// Param name.
    pub param: String,
    /// Variable name.
    #[serde(default)]
    pub var_name: Option<String>,
}

/// Parameters for `leptos_code__use_navigate_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UseNavigateEmitParams {
    /// Target path.
    pub path: String,
    /// Variable name for the navigate function.
    #[serde(default)]
    pub var_name: Option<String>,
}

/// Parameters for `leptos_code__redirect_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RedirectEmitParams {
    /// Target path.
    pub path: String,
}

/// Parameters for `leptos_code__meta_title_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetaTitleEmitParams {
    /// Page title text.
    pub title: String,
}

/// Parameters for `leptos_code__meta_tag_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetaTagEmitParams {
    /// Meta name attribute.
    pub name: String,
    /// Meta content attribute.
    pub content: String,
}

/// Parameters for `leptos_code__meta_link_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetaLinkEmitParams {
    /// Link rel attribute.
    pub rel: String,
    /// Link href attribute.
    pub href: String,
}

/// Parameters for `leptos_code__meta_stylesheet_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MetaStylesheetEmitParams {
    /// Stylesheet href.
    pub href: String,
}

/// Parameters for `leptos_code__app_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppNewParams {
    /// Package name (snake_case).
    pub package_name: String,
    /// Rendering mode: "csr", "ssr", "hydrate", "islands".
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_mode() -> String {
    "csr".to_string()
}

/// Parameters for `leptos_code__app_add_component`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppAddComponentParams {
    /// App UUID.
    pub app_id: String,
    /// Component UUID.
    pub component_id: String,
}

/// Parameters for `leptos_code__app_add_route`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppAddRouteParams {
    /// App UUID.
    pub app_id: String,
    /// Route UUID.
    pub route_id: String,
}

/// Parameters for `leptos_code__app_emit_component`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppEmitComponentParams {
    /// App UUID.
    pub app_id: String,
    /// Component UUID.
    pub component_id: String,
}

/// Parameters for `leptos_code__app_emit_main_rs`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppEmitMainRsParams {
    /// App UUID.
    pub app_id: String,
}

/// Parameters for `leptos_code__app_emit_lib_rs`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppEmitLibRsParams {
    /// App UUID.
    pub app_id: String,
}

/// Parameters for `leptos_code__app_emit_cargo_toml`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppEmitCargoTomlParams {
    /// Rendering mode: "csr", "ssr", "hydrate", "islands".
    pub mode: String,
    /// Package name.
    pub package_name: String,
}

/// Parameters for `leptos_code__app_emit_all`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AppEmitAllParams {
    /// App UUID.
    pub app_id: String,
}

/// Parameters for `leptos_code__catalog_template`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CatalogTemplateParams {
    /// Template name: "counter", "todo", "blog".
    pub name: String,
}

/// Parameters for `leptos_code__component_call_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentCallEmitParams {
    /// Component name to call.
    pub name: String,
    /// Props as (name, value_expr) pairs.
    #[serde(default)]
    pub props: Vec<(String, String)>,
    /// Whether to pass children content.
    #[serde(default)]
    pub children: Option<String>,
}

/// Parameters for `leptos_code__component_file_emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ComponentFileEmitParams {
    /// Component UUID.
    pub id: String,
    /// Module name for file header comment.
    #[serde(default)]
    pub module: Option<String>,
}

// ── Code generation helpers ───────────────────────────────────────────────────

fn emit_component(comp: &LeptosComponentDescriptor) -> String {
    let attr = if comp.island {
        "#[island]"
    } else {
        "#[component]"
    };
    let mut prop_lines = Vec::new();
    for p in &comp.props {
        let mut annotations = Vec::new();
        if p.optional {
            annotations.push("optional".to_string());
        }
        if p.into {
            annotations.push("into".to_string());
        }
        if let Some(ref dv) = p.default_value {
            annotations.push(format!("default = {dv}"));
        }
        let ann = if annotations.is_empty() {
            String::new()
        } else {
            format!("    #[prop({})]\n", annotations.join(", "))
        };
        prop_lines.push(format!("    {ann}    {}: {},", p.name, p.ty));
    }
    if comp.has_children {
        prop_lines.push("    children: Children,".to_string());
    }
    let props_str = if prop_lines.is_empty() {
        String::new()
    } else {
        format!("\n{}\n", prop_lines.join("\n"))
    };
    format!(
        "{attr}\npub fn {}({props_str}) -> impl IntoView {{\n    {}\n}}",
        comp.name, comp.body
    )
}

fn emit_route(route: &RouteEntry, routes: &HashMap<Uuid, RouteEntry>) -> String {
    if route.nested.is_empty() {
        format!("    <Route path=\"{}\" view={} />", route.path, route.view)
    } else {
        let nested: Vec<String> = route
            .nested
            .iter()
            .filter_map(|id| routes.get(id))
            .map(|r| emit_route(r, routes))
            .collect();
        format!(
            "    <Route path=\"{}\" view={}>\n{}\n    </Route>",
            route.path,
            route.view,
            nested.join("\n")
        )
    }
}

// ── Tool implementations ──────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn component_new(
    ctx: Arc<LeptosCodeContext>,
    p: ComponentNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let comp = LeptosComponentDescriptor::new(p.name.clone());
    ctx.components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, comp);
    ok_json(&serde_json::json!({ "id": id.to_string(), "name": p.name }))
}

#[instrument(skip(ctx, p))]
async fn component_add_prop(
    ctx: Arc<LeptosCodeContext>,
    p: ComponentAddPropParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    comp.props.push(LeptosPropDescriptor {
        name: p.name.clone(),
        ty: p.ty,
        optional: p.optional,
        default_value: p.default_value,
        into: p.into,
    });
    ok_json(&serde_json::json!({ "id": p.id, "prop_added": p.name }))
}

#[instrument(skip(ctx, p))]
async fn component_set_body(
    ctx: Arc<LeptosCodeContext>,
    p: ComponentSetBodyParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    comp.body = p.body;
    ok_json(&serde_json::json!({ "id": p.id, "body_set": true }))
}

#[instrument(skip(ctx, p))]
async fn component_set_island(
    ctx: Arc<LeptosCodeContext>,
    p: ComponentSetIslandParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    comp.island = p.island;
    ok_json(&serde_json::json!({ "id": p.id, "island": p.island }))
}

#[instrument(skip(ctx, p))]
async fn component_get(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    ok_json(comp)
}

#[instrument(skip(ctx, p))]
async fn component_delete(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx))]
async fn component_list(ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<serde_json::Value> = comps
        .iter()
        .map(|(id, c)| {
            serde_json::json!({
                "id": id.to_string(),
                "name": c.name,
                "island": c.island,
                "prop_count": c.props.len()
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn component_emit(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    ok_text(emit_component(comp))
}

#[instrument(skip(_ctx, p))]
async fn component_call_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ComponentCallEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let props_str: Vec<String> = p
        .props
        .iter()
        .map(|(name, val)| format!("{}={{{val}}}", name))
        .collect();
    let props_joined = if props_str.is_empty() {
        String::new()
    } else {
        format!(" {}", props_str.join(" "))
    };
    let code = if let Some(children) = p.children {
        format!("<{name}{props_joined}>{children}</{name}>", name = p.name)
    } else {
        format!("<{name}{props_joined} />", name = p.name)
    };
    ok_text(code)
}

#[instrument(skip(ctx, p))]
async fn component_file_emit(
    ctx: Arc<LeptosCodeContext>,
    p: ComponentFileEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get(&id)
        .ok_or_else(|| tool_err(format!("component not found: {id}")))?;
    let module_doc = if let Some(m) = &p.module {
        format!("//! {m} component.\n\n")
    } else {
        String::new()
    };
    let code = format!(
        "{module_doc}use leptos::prelude::*;\n\n{}",
        emit_component(comp)
    );
    ok_text(code)
}

// ── View tools ────────────────────────────────────────────────────────────────

#[instrument(skip(_ctx, p))]
async fn view_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ViewEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("view! {{\n    {}\n}}", p.content))
}

#[instrument(skip(_ctx, p))]
async fn element_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ElementEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let mut parts = vec![format!("<{}", p.tag)];
    for (name, val) in &p.attrs {
        parts.push(format!(" {name}=\"{val}\""));
    }
    for (event, handler) in &p.on_events {
        parts.push(format!(" on:{event}={{move |_| {{ {handler} }}}}"));
    }
    if p.self_closing {
        parts.push(" />".to_string());
        ok_text(parts.join(""))
    } else {
        parts.push(">".to_string());
        if !p.children.is_empty() {
            parts.push(p.children.clone());
        }
        parts.push(format!("</{}>", p.tag));
        ok_text(parts.join(""))
    }
}

#[instrument(skip(_ctx, p))]
async fn show_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ShowEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let fallback = p.fallback.as_deref().unwrap_or("").to_string();
    let code = format!(
        "<Show\n    when=move || {{{}}}\n    fallback=|| view! {{ {fallback} }}\n>\n    {}\n</Show>",
        p.when_expr, p.children
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn for_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ForEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "<For\n    each=move || {{{}}}\n    key=|{}| {}\n    let:{}\n>\n    {}\n</For>",
        p.each_expr, p.let_var, p.key_expr, p.let_var, p.children
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn suspense_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: SuspenseEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "<Suspense fallback=|| view! {{ {} }}>\n    {}\n</Suspense>",
        p.fallback, p.children
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn transition_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: TransitionEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = if let Some(fb) = p.fallback {
        format!(
            "<Transition fallback=|| view! {{ {fb} }}>\n    {}\n</Transition>",
            p.children
        )
    } else {
        format!("<Transition>\n    {}\n</Transition>", p.children)
    };
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn error_boundary_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ErrorBoundaryEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "<ErrorBoundary fallback=|errors| view! {{\n    {}\n}}>\n    {}\n</ErrorBoundary>",
        p.fallback, p.children
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn reactive_binding_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ReactiveBindingEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = match p.kind.as_str() {
        "get" => format!("{{move || {}.get()}}", p.signal),
        "set" => format!("|v| {}.set(v)", p.signal),
        "rw" => format!(
            "// read: {{move || {signal}.get()}}\n// write: move |v| {signal}.set(v)",
            signal = p.signal
        ),
        other => return Err(tool_err(format!("unknown binding kind: {other}"))),
    };
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn event_handler_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: EventHandlerEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("on:{}={{move |_ev| {{ {} }}}}", p.event, p.handler))
}

#[instrument(skip(_ctx, p))]
async fn class_binding_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ClassBindingEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("class:{}={{move || {}}}", p.class, p.condition))
}

#[instrument(skip(_ctx, p))]
async fn attr_binding_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: AttrBindingEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("{}={{move || {}}}", p.attr, p.value_expr))
}

#[instrument(skip(_ctx, p))]
async fn router_link_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: RouterLinkEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("<A href=\"{}\">{}</A>", p.href, p.children))
}

// ── Server function tools ─────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn server_fn_new(
    ctx: Arc<LeptosCodeContext>,
    p: ServerFnNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let entry = ServerFnEntry {
        name: p.name.clone(),
        args: vec![],
        return_type: p
            .return_type
            .unwrap_or_else(|| "Result<(), ServerFnError>".to_string()),
        body: p.body.unwrap_or_default(),
    };
    ctx.server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({ "id": id.to_string(), "name": p.name }))
}

#[instrument(skip(ctx, p))]
async fn server_fn_add_arg(
    ctx: Arc<LeptosCodeContext>,
    p: ServerFnAddArgParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let mut fns = ctx
        .server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = fns
        .get_mut(&id)
        .ok_or_else(|| tool_err(format!("server_fn not found: {id}")))?;
    entry.args.push((p.name.clone(), p.ty));
    ok_json(&serde_json::json!({ "id": p.id, "arg_added": p.name }))
}

#[instrument(skip(ctx, p))]
async fn server_fn_get(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let fns = ctx
        .server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = fns
        .get(&id)
        .ok_or_else(|| tool_err(format!("server_fn not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "name": entry.name,
        "args": entry.args,
        "return_type": entry.return_type,
        "body": entry.body
    }))
}

#[instrument(skip(ctx, p))]
async fn server_fn_delete(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx))]
async fn server_fn_list(ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let fns = ctx
        .server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<serde_json::Value> = fns
        .iter()
        .map(|(id, f)| {
            serde_json::json!({
                "id": id.to_string(),
                "name": f.name,
                "arg_count": f.args.len()
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn server_fn_emit(
    ctx: Arc<LeptosCodeContext>,
    p: ServerFnEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let fns = ctx
        .server_fns
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = fns
        .get(&id)
        .ok_or_else(|| tool_err(format!("server_fn not found: {id}")))?;
    let args_str: Vec<String> = entry
        .args
        .iter()
        .map(|(n, t)| format!("{n}: {t}"))
        .collect();
    let code = format!(
        "#[server]\npub async fn {}({}) -> {} {{\n    {}\n}}",
        entry.name,
        args_str.join(", "),
        entry.return_type,
        entry.body
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn resource_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ResourceEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.unwrap_or_else(|| "resource".to_string());
    let code = format!(
        "let {var} = Resource::new(\n    move || {{{}}},\n    move |_| async move {{ {} }},\n);",
        p.source, p.fetcher
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn action_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ActionEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.unwrap_or_else(|| "action".to_string());
    let code = format!(
        "let {var} = Action::<{}, _>::new(move |input: &{}| {{\n    {}\n}});",
        p.input_type, p.input_type, p.handler
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn server_action_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ServerActionEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.unwrap_or_else(|| "action".to_string());
    let code = format!("let {var} = ServerAction::<{}>::new();", p.server_fn_name);
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn action_form_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: ActionFormEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "<ActionForm action={{{}}}>\n    {}\n</ActionForm>",
        p.action, p.children
    );
    ok_text(code)
}

// ── Routing tools ─────────────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn route_new(
    ctx: Arc<LeptosCodeContext>,
    p: RouteNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let entry = RouteEntry {
        path: p.path.clone(),
        view: p.view.clone(),
        nested: vec![],
    };
    ctx.routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({
        "id": id.to_string(),
        "path": p.path,
        "view": p.view
    }))
}

#[instrument(skip(ctx, p))]
async fn route_add_nested(
    ctx: Arc<LeptosCodeContext>,
    p: RouteAddNestedParams,
) -> Result<CallToolResult, ErrorData> {
    let parent_id: Uuid = p
        .parent_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.parent_id)))?;
    let child_id: Uuid = p
        .child_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.child_id)))?;
    let mut routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    if !routes.contains_key(&child_id) {
        return Err(tool_err(format!("child route not found: {child_id}")));
    }
    let parent = routes
        .get_mut(&parent_id)
        .ok_or_else(|| tool_err(format!("parent route not found: {parent_id}")))?;
    parent.nested.push(child_id);
    ok_json(
        &serde_json::json!({ "parent_id": p.parent_id, "child_id": p.child_id, "nested": true }),
    )
}

#[instrument(skip(ctx, p))]
async fn route_get(ctx: Arc<LeptosCodeContext>, p: IdParams) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = routes
        .get(&id)
        .ok_or_else(|| tool_err(format!("route not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "path": entry.path,
        "view": entry.view,
        "nested_count": entry.nested.len()
    }))
}

#[instrument(skip(ctx, p))]
async fn route_delete(
    ctx: Arc<LeptosCodeContext>,
    p: IdParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx))]
async fn route_list(ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<serde_json::Value> = routes
        .iter()
        .map(|(id, r)| {
            serde_json::json!({
                "id": id.to_string(),
                "path": r.path,
                "view": r.view
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
async fn router_emit(
    ctx: Arc<LeptosCodeContext>,
    p: RouterEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let mut route_lines = Vec::new();
    for id_str in &p.route_ids {
        let id: Uuid = id_str
            .parse()
            .map_err(|_| tool_err(format!("invalid UUID: {id_str}")))?;
        if let Some(route) = routes.get(&id) {
            route_lines.push(emit_route(route, &routes));
        }
    }
    let code = format!(
        "<Router>\n<Routes fallback=|| view! {{ <p>\"Not Found\"</p> }}>\n{}\n</Routes>\n</Router>",
        route_lines.join("\n")
    );
    ok_text(code)
}

#[instrument(skip(ctx, p))]
async fn route_emit(
    ctx: Arc<LeptosCodeContext>,
    p: RouteEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let entry = routes
        .get(&id)
        .ok_or_else(|| tool_err(format!("route not found: {id}")))?;
    ok_text(emit_route(entry, &routes))
}

#[instrument(skip(_ctx, p))]
async fn use_params_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: UseParamsEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.unwrap_or_else(|| p.param.clone());
    let code = format!(
        "let params = use_params_map();\nlet {var} = move || params.with(|p| p.get(\"{param}\").cloned().unwrap_or_default());",
        param = p.param
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn use_navigate_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: UseNavigateEmitParams,
) -> Result<CallToolResult, ErrorData> {
    let var = p.var_name.unwrap_or_else(|| "navigate".to_string());
    let code = format!(
        "let {var} = use_navigate();\n{var}(\"{}\", Default::default());",
        p.path
    );
    ok_text(code)
}

#[instrument(skip(_ctx, p))]
async fn redirect_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: RedirectEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("<Redirect path=\"{}\" />", p.path))
}

#[instrument(skip(_ctx))]
async fn outlet_emit(_ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    ok_text("<Outlet />")
}

// ── Meta tools ────────────────────────────────────────────────────────────────

#[instrument(skip(_ctx, p))]
async fn meta_title_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: MetaTitleEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("<Title text=\"{}\"/>", p.title))
}

#[instrument(skip(_ctx, p))]
async fn meta_tag_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: MetaTagEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!(
        "<Meta name=\"{}\" content=\"{}\"/>",
        p.name, p.content
    ))
}

#[instrument(skip(_ctx, p))]
async fn meta_link_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: MetaLinkEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("<Link rel=\"{}\" href=\"{}\"/>", p.rel, p.href))
}

#[instrument(skip(_ctx, p))]
async fn meta_stylesheet_emit(
    _ctx: Arc<LeptosCodeContext>,
    p: MetaStylesheetEmitParams,
) -> Result<CallToolResult, ErrorData> {
    ok_text(format!("<Stylesheet href=\"{}\"/>", p.href))
}

// ── App scaffolding tools ─────────────────────────────────────────────────────

#[instrument(skip(ctx, p))]
async fn app_new(
    ctx: Arc<LeptosCodeContext>,
    p: AppNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let entry = AppEntry {
        package_name: p.package_name.clone(),
        mode: p.mode,
        component_ids: vec![],
        route_ids: vec![],
    };
    ctx.apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .insert(id, entry);
    ok_json(&serde_json::json!({ "id": id.to_string(), "package_name": p.package_name }))
}

#[instrument(skip(ctx, p))]
async fn app_add_component(
    ctx: Arc<LeptosCodeContext>,
    p: AppAddComponentParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;
    let comp_id: Uuid = p
        .component_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.component_id)))?;
    let mut apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let app = apps
        .get_mut(&app_id)
        .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
    app.component_ids.push(comp_id);
    ok_json(&serde_json::json!({ "app_id": p.app_id, "component_id": p.component_id }))
}

#[instrument(skip(ctx, p))]
async fn app_add_route(
    ctx: Arc<LeptosCodeContext>,
    p: AppAddRouteParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;
    let route_id: Uuid = p
        .route_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.route_id)))?;
    let mut apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let app = apps
        .get_mut(&app_id)
        .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
    app.route_ids.push(route_id);
    ok_json(&serde_json::json!({ "app_id": p.app_id, "route_id": p.route_id }))
}

#[instrument(skip(ctx, p))]
async fn app_get(ctx: Arc<LeptosCodeContext>, p: IdParams) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let app = apps
        .get(&id)
        .ok_or_else(|| tool_err(format!("app not found: {id}")))?;
    ok_json(&serde_json::json!({
        "id": p.id,
        "package_name": app.package_name,
        "mode": app.mode,
        "component_count": app.component_ids.len(),
        "route_count": app.route_ids.len()
    }))
}

#[instrument(skip(ctx, p))]
async fn app_delete(ctx: Arc<LeptosCodeContext>, p: IdParams) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| tool_err(format!("invalid UUID: {}", p.id)))?;
    let removed = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?
        .remove(&id)
        .is_some();
    ok_json(&serde_json::json!({ "deleted": removed, "id": p.id }))
}

#[instrument(skip(ctx))]
async fn app_list(ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let items: Vec<serde_json::Value> = apps
        .iter()
        .map(|(id, a)| {
            serde_json::json!({
                "id": id.to_string(),
                "package_name": a.package_name,
                "mode": a.mode
            })
        })
        .collect();
    ok_json(&items)
}

#[instrument(skip(ctx, p))]
#[instrument(skip(ctx, p))]
async fn app_emit_component(
    ctx: Arc<LeptosCodeContext>,
    p: AppEmitComponentParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;
    let comp_id: Uuid = p
        .component_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.component_id)))?;
    {
        let apps = ctx
            .apps
            .lock()
            .map_err(|e| tool_err(format!("lock: {e}")))?;
        let app = apps
            .get(&app_id)
            .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
        if !app.component_ids.contains(&comp_id) {
            return Err(tool_err(format!("component {comp_id} not in app {app_id}")));
        }
    }
    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let comp = comps
        .get(&comp_id)
        .ok_or_else(|| tool_err(format!("component not found: {comp_id}")))?;
    ok_text(emit_component(comp))
}

#[instrument(skip(ctx, p))]
async fn app_emit_main_rs(
    ctx: Arc<LeptosCodeContext>,
    p: AppEmitMainRsParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;
    let apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let app = apps
        .get(&app_id)
        .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
    let pkg = &app.package_name;
    let code = match app.mode.as_str() {
        "ssr" => format!(
            "use leptos::prelude::*;\nuse {pkg}::*;\n\n#[tokio::main]\nasync fn main() {{\n    leptos_axum::generate_route_list(App);\n}}"
        ),
        _ => format!(
            "use leptos::prelude::*;\nuse {pkg}::*;\n\nfn main() {{\n    mount_to_body(App);\n}}"
        ),
    };
    ok_text(code)
}

#[instrument(skip(ctx, p))]
async fn app_emit_lib_rs(
    ctx: Arc<LeptosCodeContext>,
    p: AppEmitLibRsParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;
    let apps = ctx
        .apps
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let app = apps
        .get(&app_id)
        .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
    let comp_ids = app.component_ids.clone();
    let route_ids = app.route_ids.clone();
    drop(apps);

    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;

    let mut parts = vec!["use leptos::prelude::*;\nuse leptos_router::prelude::*;\n".to_string()];

    for id in &comp_ids {
        if let Some(comp) = comps.get(id) {
            parts.push(emit_component(comp));
        }
    }

    let route_strs: Vec<String> = route_ids
        .iter()
        .filter_map(|id| routes.get(id))
        .map(|r| emit_route(r, &routes))
        .collect();

    let router_block = if route_strs.is_empty() {
        String::new()
    } else {
        format!(
            "\n<Router>\n<Routes fallback=|| view! {{ <p>\"Not Found\"</p> }}>\n{}\n</Routes>\n</Router>",
            route_strs.join("\n")
        )
    };

    parts.push(format!(
        "#[component]\npub fn App() -> impl IntoView {{\n    view! {{\n        {router_block}\n    }}\n}}"
    ));

    ok_text(parts.join("\n\n"))
}

#[instrument(skip(_ctx, p))]
async fn app_emit_cargo_toml(
    _ctx: Arc<LeptosCodeContext>,
    p: AppEmitCargoTomlParams,
) -> Result<CallToolResult, ErrorData> {
    let pkg = &p.package_name;
    let mode = &p.mode;
    let leptos_feature = match mode.as_str() {
        "ssr" => "ssr",
        "hydrate" => "hydrate",
        "islands" => "islands",
        _ => "csr",
    };
    let extra_deps = if mode == "ssr" {
        "\nleptos-axum = { version = \"0.8\" }\ntokio = { version = \"1\", features = [\"full\"] }\naxum = \"0.8\"".to_string()
    } else {
        String::new()
    };
    let code = format!(
        "[package]\nname = \"{pkg}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[dependencies]\nleptos = {{ version = \"0.8\", features = [\"{leptos_feature}\"] }}\nleptos_router = {{ version = \"0.8\" }}\nconsole_error_panic_hook = \"0.1\"{extra_deps}\n"
    );
    ok_text(code)
}

#[instrument(skip(ctx, p))]
async fn app_emit_all(
    ctx: Arc<LeptosCodeContext>,
    p: AppEmitAllParams,
) -> Result<CallToolResult, ErrorData> {
    let app_id: Uuid = p
        .app_id
        .parse()
        .map_err(|_| tool_err(format!("invalid UUID: {}", p.app_id)))?;

    let (pkg, mode, comp_ids, route_ids) = {
        let apps = ctx
            .apps
            .lock()
            .map_err(|e| tool_err(format!("lock: {e}")))?;
        let app = apps
            .get(&app_id)
            .ok_or_else(|| tool_err(format!("app not found: {app_id}")))?;
        (
            app.package_name.clone(),
            app.mode.clone(),
            app.component_ids.clone(),
            app.route_ids.clone(),
        )
    };

    let comps = ctx
        .components
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;
    let routes = ctx
        .routes
        .lock()
        .map_err(|e| tool_err(format!("lock: {e}")))?;

    let comp_code: Vec<String> = comp_ids
        .iter()
        .filter_map(|id| comps.get(id))
        .map(emit_component)
        .collect();

    let route_strs: Vec<String> = route_ids
        .iter()
        .filter_map(|id| routes.get(id))
        .map(|r| emit_route(r, &routes))
        .collect();

    let router_block = if route_strs.is_empty() {
        String::new()
    } else {
        format!(
            "<Router>\n<Routes fallback=|| view! {{ <p>\"Not Found\"</p> }}>\n{}\n</Routes>\n</Router>",
            route_strs.join("\n")
        )
    };

    let lib_rs = format!(
        "use leptos::prelude::*;\nuse leptos_router::prelude::*;\n\n{}\n\n#[component]\npub fn App() -> impl IntoView {{\n    view! {{ {router_block} }}\n}}",
        comp_code.join("\n\n")
    );

    let leptos_feature = match mode.as_str() {
        "ssr" => "ssr",
        "hydrate" => "hydrate",
        "islands" => "islands",
        _ => "csr",
    };
    let extra_deps = if mode == "ssr" {
        "\nleptos-axum = { version = \"0.8\" }\ntokio = { version = \"1\", features = [\"full\"] }\naxum = \"0.8\"".to_string()
    } else {
        String::new()
    };
    let cargo_toml = format!(
        "[package]\nname = \"{pkg}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[dependencies]\nleptos = {{ version = \"0.8\", features = [\"{leptos_feature}\"] }}\nleptos_router = {{ version = \"0.8\" }}\nconsole_error_panic_hook = \"0.1\"{extra_deps}\n"
    );

    let main_rs = match mode.as_str() {
        "ssr" => format!(
            "use leptos::prelude::*;\nuse {pkg}::*;\n\n#[tokio::main]\nasync fn main() {{\n    leptos_axum::generate_route_list(App);\n}}"
        ),
        _ => format!(
            "use leptos::prelude::*;\nuse {pkg}::*;\n\nfn main() {{\n    mount_to_body(App);\n}}"
        ),
    };

    ok_json(&serde_json::json!({
        "src/lib.rs": lib_rs,
        "src/main.rs": main_rs,
        "Cargo.toml": cargo_toml
    }))
}

// ── Catalog tools ─────────────────────────────────────────────────────────────

#[instrument(skip(_ctx))]
async fn catalog_html_tags(_ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let tags = vec![
        "div", "span", "p", "a", "button", "input", "form", "h1", "h2", "h3", "ul", "ol", "li",
        "img", "nav", "main", "section", "article", "header", "footer", "aside", "table", "tr",
        "td", "th", "select", "option", "textarea", "label", "strong", "em", "code", "pre",
    ];
    ok_json(&tags)
}

#[instrument(skip(_ctx))]
async fn catalog_leptos_components(
    _ctx: Arc<LeptosCodeContext>,
) -> Result<CallToolResult, ErrorData> {
    let components = vec![
        "Show",
        "For",
        "Suspense",
        "Transition",
        "ErrorBoundary",
        "Router",
        "Routes",
        "Route",
        "Outlet",
        "A",
        "Form",
        "ActionForm",
        "Title",
        "Meta",
        "Link",
        "Stylesheet",
        "Redirect",
    ];
    ok_json(&components)
}

#[instrument(skip(_ctx))]
async fn catalog_events(_ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let events = vec![
        "click",
        "dblclick",
        "mousedown",
        "mouseup",
        "mouseover",
        "mouseout",
        "mousemove",
        "keydown",
        "keyup",
        "keypress",
        "input",
        "change",
        "submit",
        "focus",
        "blur",
        "scroll",
        "resize",
        "load",
        "touchstart",
        "touchend",
        "touchmove",
    ];
    ok_json(&events)
}

#[instrument(skip(_ctx))]
async fn catalog_prop_attrs(_ctx: Arc<LeptosCodeContext>) -> Result<CallToolResult, ErrorData> {
    let props = vec![
        serde_json::json!({ "attr": "class", "desc": "CSS class names" }),
        serde_json::json!({ "attr": "id", "desc": "Element ID" }),
        serde_json::json!({ "attr": "style", "desc": "Inline styles" }),
        serde_json::json!({ "attr": "href", "desc": "Hyperlink target" }),
        serde_json::json!({ "attr": "src", "desc": "Resource URL" }),
        serde_json::json!({ "attr": "type", "desc": "Input/button type" }),
        serde_json::json!({ "attr": "value", "desc": "Input value" }),
        serde_json::json!({ "attr": "disabled", "desc": "Disabled state" }),
        serde_json::json!({ "attr": "placeholder", "desc": "Input placeholder" }),
        serde_json::json!({ "attr": "checked", "desc": "Checkbox state" }),
    ];
    ok_json(&props)
}

#[instrument(skip(_ctx))]
async fn catalog_leptos_features(
    _ctx: Arc<LeptosCodeContext>,
) -> Result<CallToolResult, ErrorData> {
    let features = vec![
        serde_json::json!({ "feature": "csr", "desc": "Client-side rendering only" }),
        serde_json::json!({ "feature": "ssr", "desc": "Server-side rendering with hydration" }),
        serde_json::json!({ "feature": "hydrate", "desc": "Hydration mode (client side of SSR)" }),
        serde_json::json!({ "feature": "islands", "desc": "Islands architecture" }),
        serde_json::json!({ "feature": "nightly", "desc": "Nightly Rust features for ergonomic signals" }),
    ];
    ok_json(&features)
}

#[instrument(skip(_ctx, p))]
async fn catalog_template(
    _ctx: Arc<LeptosCodeContext>,
    p: CatalogTemplateParams,
) -> Result<CallToolResult, ErrorData> {
    let code = match p.name.as_str() {
        "counter" => {
            r#"use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let count = RwSignal::new(0i32);

    view! {
        <div class="counter">
            <button on:click=move |_| count.update(|n| *n -= 1)>"-"</button>
            <span>{move || count.get()}</span>
            <button on:click=move |_| count.update(|n| *n += 1)>"+"</button>
        </div>
    }
}"#
        }
        "todo" => {
            r#"use leptos::prelude::*;

#[derive(Clone, Debug)]
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

#[component]
pub fn TodoApp() -> impl IntoView {
    let todos = RwSignal::new(Vec::<Todo>::new());
    let input = RwSignal::new(String::new());
    let next_id = RwSignal::new(0u32);

    let add_todo = move |_| {
        let text = input.get();
        if !text.is_empty() {
            todos.update(|list| list.push(Todo { id: next_id.get(), text, done: false }));
            next_id.update(|n| *n += 1);
            input.set(String::new());
        }
    };

    view! {
        <div>
            <input
                type="text"
                prop:value=move || input.get()
                on:input=move |ev| input.set(event_target_value(&ev))
            />
            <button on:click=add_todo>"Add"</button>
            <ul>
                <For
                    each=move || todos.get()
                    key=|todo| todo.id
                    let:todo
                >
                    <li>{todo.text}</li>
                </For>
            </ul>
        </div>
    }
}"#
        }
        "blog" => {
            r#"use leptos::prelude::*;
use leptos_router::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/about">"About"</A>
            </nav>
            <main>
                <Routes fallback=|| view! { <p>"Not Found"</p> }>
                    <Route path="/" view=Home />
                    <Route path="/about" view=About />
                    <Route path="/post/:id" view=Post />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <h1>"Welcome to the Blog"</h1> }
}

#[component]
fn About() -> impl IntoView {
    view! { <h1>"About"</h1> }
}

#[component]
fn Post() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());
    view! { <h1>"Post: " {id}</h1> }
}"#
        }
        other => {
            return Err(tool_err(format!(
                "unknown template: {other}. Use: counter, todo, blog"
            )));
        }
    };
    ok_text(code)
}

// ── Dispatch ──────────────────────────────────────────────────────────────────

async fn dispatch_code(
    ctx: Arc<LeptosCodeContext>,
    name: &str,
    params: &CallToolRequestParams,
) -> Result<CallToolResult, ErrorData> {
    match name {
        "leptos_code__component_new" => component_new(ctx, parse_params(params)?).await,
        "leptos_code__component_add_prop" => component_add_prop(ctx, parse_params(params)?).await,
        "leptos_code__component_set_body" => component_set_body(ctx, parse_params(params)?).await,
        "leptos_code__component_set_island" => {
            component_set_island(ctx, parse_params(params)?).await
        }
        "leptos_code__component_get" => component_get(ctx, parse_params(params)?).await,
        "leptos_code__component_delete" => component_delete(ctx, parse_params(params)?).await,
        "leptos_code__component_list" => component_list(ctx).await,
        "leptos_code__component_emit" => component_emit(ctx, parse_params(params)?).await,
        "leptos_code__component_call_emit" => component_call_emit(ctx, parse_params(params)?).await,
        "leptos_code__component_file_emit" => component_file_emit(ctx, parse_params(params)?).await,
        "leptos_code__view_emit" => view_emit(ctx, parse_params(params)?).await,
        "leptos_code__element_emit" => element_emit(ctx, parse_params(params)?).await,
        "leptos_code__show_emit" => show_emit(ctx, parse_params(params)?).await,
        "leptos_code__for_emit" => for_emit(ctx, parse_params(params)?).await,
        "leptos_code__suspense_emit" => suspense_emit(ctx, parse_params(params)?).await,
        "leptos_code__transition_emit" => transition_emit(ctx, parse_params(params)?).await,
        "leptos_code__error_boundary_emit" => error_boundary_emit(ctx, parse_params(params)?).await,
        "leptos_code__reactive_binding_emit" => {
            reactive_binding_emit(ctx, parse_params(params)?).await
        }
        "leptos_code__event_handler_emit" => event_handler_emit(ctx, parse_params(params)?).await,
        "leptos_code__class_binding_emit" => class_binding_emit(ctx, parse_params(params)?).await,
        "leptos_code__attr_binding_emit" => attr_binding_emit(ctx, parse_params(params)?).await,
        "leptos_code__router_link_emit" => router_link_emit(ctx, parse_params(params)?).await,
        "leptos_code__server_fn_new" => server_fn_new(ctx, parse_params(params)?).await,
        "leptos_code__server_fn_add_arg" => server_fn_add_arg(ctx, parse_params(params)?).await,
        "leptos_code__server_fn_get" => server_fn_get(ctx, parse_params(params)?).await,
        "leptos_code__server_fn_delete" => server_fn_delete(ctx, parse_params(params)?).await,
        "leptos_code__server_fn_list" => server_fn_list(ctx).await,
        "leptos_code__server_fn_emit" => server_fn_emit(ctx, parse_params(params)?).await,
        "leptos_code__resource_emit" => resource_emit(ctx, parse_params(params)?).await,
        "leptos_code__action_emit" => action_emit(ctx, parse_params(params)?).await,
        "leptos_code__server_action_emit" => server_action_emit(ctx, parse_params(params)?).await,
        "leptos_code__action_form_emit" => action_form_emit(ctx, parse_params(params)?).await,
        "leptos_code__route_new" => route_new(ctx, parse_params(params)?).await,
        "leptos_code__route_add_nested" => route_add_nested(ctx, parse_params(params)?).await,
        "leptos_code__route_get" => route_get(ctx, parse_params(params)?).await,
        "leptos_code__route_delete" => route_delete(ctx, parse_params(params)?).await,
        "leptos_code__route_list" => route_list(ctx).await,
        "leptos_code__router_emit" => router_emit(ctx, parse_params(params)?).await,
        "leptos_code__route_emit" => route_emit(ctx, parse_params(params)?).await,
        "leptos_code__use_params_emit" => use_params_emit(ctx, parse_params(params)?).await,
        "leptos_code__use_navigate_emit" => use_navigate_emit(ctx, parse_params(params)?).await,
        "leptos_code__redirect_emit" => redirect_emit(ctx, parse_params(params)?).await,
        "leptos_code__outlet_emit" => outlet_emit(ctx).await,
        "leptos_code__meta_title_emit" => meta_title_emit(ctx, parse_params(params)?).await,
        "leptos_code__meta_tag_emit" => meta_tag_emit(ctx, parse_params(params)?).await,
        "leptos_code__meta_link_emit" => meta_link_emit(ctx, parse_params(params)?).await,
        "leptos_code__meta_stylesheet_emit" => {
            meta_stylesheet_emit(ctx, parse_params(params)?).await
        }
        "leptos_code__app_new" => app_new(ctx, parse_params(params)?).await,
        "leptos_code__app_add_component" => app_add_component(ctx, parse_params(params)?).await,
        "leptos_code__app_add_route" => app_add_route(ctx, parse_params(params)?).await,
        "leptos_code__app_get" => app_get(ctx, parse_params(params)?).await,
        "leptos_code__app_delete" => app_delete(ctx, parse_params(params)?).await,
        "leptos_code__app_list" => app_list(ctx).await,
        "leptos_code__app_emit_component" => app_emit_component(ctx, parse_params(params)?).await,
        "leptos_code__app_emit_main_rs" => app_emit_main_rs(ctx, parse_params(params)?).await,
        "leptos_code__app_emit_lib_rs" => app_emit_lib_rs(ctx, parse_params(params)?).await,
        "leptos_code__app_emit_cargo_toml" => app_emit_cargo_toml(ctx, parse_params(params)?).await,
        "leptos_code__app_emit_all" => app_emit_all(ctx, parse_params(params)?).await,
        "leptos_code__catalog_html_tags" => catalog_html_tags(ctx).await,
        "leptos_code__catalog_leptos_components" => catalog_leptos_components(ctx).await,
        "leptos_code__catalog_events" => catalog_events(ctx).await,
        "leptos_code__catalog_prop_attrs" => catalog_prop_attrs(ctx).await,
        "leptos_code__catalog_leptos_features" => catalog_leptos_features(ctx).await,
        "leptos_code__catalog_template" => catalog_template(ctx, parse_params(params)?).await,
        _ => Err(ErrorData::invalid_params(
            format!("unknown tool: {name}"),
            None,
        )),
    }
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin providing `leptos_code__*` tools.
pub struct LeptosCodePlugin(pub Arc<LeptosCodeContext>);

impl LeptosCodePlugin {
    /// Create a new plugin with empty code state.
    pub fn new() -> Self {
        Self(Arc::new(LeptosCodeContext::new()))
    }

    /// Invoke a tool by name with a JSON arguments object.
    ///
    /// Convenience for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned: String = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        dispatch_code(self.0.clone(), name, &params).await
    }
}

impl Default for LeptosCodePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for LeptosCodePlugin {
    fn name(&self) -> &'static str {
        "leptos_code"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            // Component tools
            build_tool(
                "leptos_code__component_new",
                "Create a new Leptos component descriptor.",
                schema_of::<ComponentNewParams>(),
            ),
            build_tool(
                "leptos_code__component_add_prop",
                "Add a prop to a stored component.",
                schema_of::<ComponentAddPropParams>(),
            ),
            build_tool(
                "leptos_code__component_set_body",
                "Set the body of a stored component.",
                schema_of::<ComponentSetBodyParams>(),
            ),
            build_tool(
                "leptos_code__component_set_island",
                "Mark a component as an island (#[island]).",
                schema_of::<ComponentSetIslandParams>(),
            ),
            build_tool(
                "leptos_code__component_get",
                "Get a stored component descriptor by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__component_delete",
                "Delete a stored component by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__component_list",
                "List all stored components.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__component_emit",
                "Emit Rust #[component] source for a stored component.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__component_call_emit",
                "Emit a component call expression (JSX-like).",
                schema_of::<ComponentCallEmitParams>(),
            ),
            build_tool(
                "leptos_code__component_file_emit",
                "Emit a complete component source file with use leptos::prelude::*.",
                schema_of::<ComponentFileEmitParams>(),
            ),
            // View tools
            build_tool(
                "leptos_code__view_emit",
                "Wrap content in a view! macro.",
                schema_of::<ViewEmitParams>(),
            ),
            build_tool(
                "leptos_code__element_emit",
                "Emit an HTML element with attributes and event handlers.",
                schema_of::<ElementEmitParams>(),
            ),
            build_tool(
                "leptos_code__show_emit",
                "Emit a <Show> conditional component.",
                schema_of::<ShowEmitParams>(),
            ),
            build_tool(
                "leptos_code__for_emit",
                "Emit a <For> list component.",
                schema_of::<ForEmitParams>(),
            ),
            build_tool(
                "leptos_code__suspense_emit",
                "Emit a <Suspense> component.",
                schema_of::<SuspenseEmitParams>(),
            ),
            build_tool(
                "leptos_code__transition_emit",
                "Emit a <Transition> component.",
                schema_of::<TransitionEmitParams>(),
            ),
            build_tool(
                "leptos_code__error_boundary_emit",
                "Emit an <ErrorBoundary> component.",
                schema_of::<ErrorBoundaryEmitParams>(),
            ),
            build_tool(
                "leptos_code__reactive_binding_emit",
                "Emit a reactive signal binding expression.",
                schema_of::<ReactiveBindingEmitParams>(),
            ),
            build_tool(
                "leptos_code__event_handler_emit",
                "Emit an event handler attribute.",
                schema_of::<EventHandlerEmitParams>(),
            ),
            build_tool(
                "leptos_code__class_binding_emit",
                "Emit a reactive class: binding.",
                schema_of::<ClassBindingEmitParams>(),
            ),
            build_tool(
                "leptos_code__attr_binding_emit",
                "Emit a reactive attribute binding.",
                schema_of::<AttrBindingEmitParams>(),
            ),
            build_tool(
                "leptos_code__router_link_emit",
                "Emit an <A> router link.",
                schema_of::<RouterLinkEmitParams>(),
            ),
            // Server function tools
            build_tool(
                "leptos_code__server_fn_new",
                "Create a new server function descriptor.",
                schema_of::<ServerFnNewParams>(),
            ),
            build_tool(
                "leptos_code__server_fn_add_arg",
                "Add an argument to a server function.",
                schema_of::<ServerFnAddArgParams>(),
            ),
            build_tool(
                "leptos_code__server_fn_get",
                "Get a server function descriptor by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__server_fn_delete",
                "Delete a server function by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__server_fn_list",
                "List all server functions.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__server_fn_emit",
                "Emit #[server] function source.",
                schema_of::<ServerFnEmitParams>(),
            ),
            build_tool(
                "leptos_code__resource_emit",
                "Emit a Resource::new(...) expression.",
                schema_of::<ResourceEmitParams>(),
            ),
            build_tool(
                "leptos_code__action_emit",
                "Emit an Action::new(...) expression.",
                schema_of::<ActionEmitParams>(),
            ),
            build_tool(
                "leptos_code__server_action_emit",
                "Emit a ServerAction::<FnName>::new() expression.",
                schema_of::<ServerActionEmitParams>(),
            ),
            build_tool(
                "leptos_code__action_form_emit",
                "Emit an <ActionForm> component.",
                schema_of::<ActionFormEmitParams>(),
            ),
            // Routing tools
            build_tool(
                "leptos_code__route_new",
                "Create a new route descriptor.",
                schema_of::<RouteNewParams>(),
            ),
            build_tool(
                "leptos_code__route_add_nested",
                "Add a nested route to a parent route.",
                schema_of::<RouteAddNestedParams>(),
            ),
            build_tool(
                "leptos_code__route_get",
                "Get a route descriptor by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__route_delete",
                "Delete a route by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__route_list",
                "List all stored routes.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__router_emit",
                "Emit a <Router> with stored routes.",
                schema_of::<RouterEmitParams>(),
            ),
            build_tool(
                "leptos_code__route_emit",
                "Emit a single <Route> element.",
                schema_of::<RouteEmitParams>(),
            ),
            build_tool(
                "leptos_code__use_params_emit",
                "Emit use_params_map() access code.",
                schema_of::<UseParamsEmitParams>(),
            ),
            build_tool(
                "leptos_code__use_navigate_emit",
                "Emit use_navigate() call code.",
                schema_of::<UseNavigateEmitParams>(),
            ),
            build_tool(
                "leptos_code__redirect_emit",
                "Emit a <Redirect> element.",
                schema_of::<RedirectEmitParams>(),
            ),
            build_tool(
                "leptos_code__outlet_emit",
                "Emit an <Outlet /> for nested routes.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            // Meta tools
            build_tool(
                "leptos_code__meta_title_emit",
                "Emit a <Title> meta tag.",
                schema_of::<MetaTitleEmitParams>(),
            ),
            build_tool(
                "leptos_code__meta_tag_emit",
                "Emit a <Meta name content> tag.",
                schema_of::<MetaTagEmitParams>(),
            ),
            build_tool(
                "leptos_code__meta_link_emit",
                "Emit a <Link rel href> tag.",
                schema_of::<MetaLinkEmitParams>(),
            ),
            build_tool(
                "leptos_code__meta_stylesheet_emit",
                "Emit a <Stylesheet href> tag.",
                schema_of::<MetaStylesheetEmitParams>(),
            ),
            // App scaffolding
            build_tool(
                "leptos_code__app_new",
                "Create a new app descriptor.",
                schema_of::<AppNewParams>(),
            ),
            build_tool(
                "leptos_code__app_add_component",
                "Add a component to an app.",
                schema_of::<AppAddComponentParams>(),
            ),
            build_tool(
                "leptos_code__app_add_route",
                "Add a route to an app.",
                schema_of::<AppAddRouteParams>(),
            ),
            build_tool(
                "leptos_code__app_get",
                "Get an app descriptor by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__app_delete",
                "Delete an app by UUID.",
                schema_of::<IdParams>(),
            ),
            build_tool(
                "leptos_code__app_list",
                "List all apps.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__app_emit_component",
                "Emit a component from an app.",
                schema_of::<AppEmitComponentParams>(),
            ),
            build_tool(
                "leptos_code__app_emit_main_rs",
                "Emit src/main.rs for an app.",
                schema_of::<AppEmitMainRsParams>(),
            ),
            build_tool(
                "leptos_code__app_emit_lib_rs",
                "Emit src/lib.rs with all components and App for an app.",
                schema_of::<AppEmitLibRsParams>(),
            ),
            build_tool(
                "leptos_code__app_emit_cargo_toml",
                "Emit a Cargo.toml for a Leptos app.",
                schema_of::<AppEmitCargoTomlParams>(),
            ),
            build_tool(
                "leptos_code__app_emit_all",
                "Emit all scaffold files (lib.rs, main.rs, Cargo.toml) as a JSON map.",
                schema_of::<AppEmitAllParams>(),
            ),
            // Catalog
            build_tool(
                "leptos_code__catalog_html_tags",
                "List all supported HTML5 element tags.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__catalog_leptos_components",
                "List built-in Leptos components (Show, For, Suspense, etc.).",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__catalog_events",
                "List common DOM event names for on:event handlers.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__catalog_prop_attrs",
                "List common HTML attributes with descriptions.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__catalog_leptos_features",
                "List Leptos Cargo feature flags with descriptions.",
                serde_json::json!({"type":"object","properties":{}}),
            ),
            build_tool(
                "leptos_code__catalog_template",
                "Get a starter template: counter, todo, or blog.",
                schema_of::<CatalogTemplateParams>(),
            ),
        ]
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            dispatch_code(ctx, name, &params).await
        })
    }
}
