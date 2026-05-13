//! `BevyAppPlugin` — descriptor-registry tools for Bevy app assembly.
//!
//! This plugin stores app descriptors server-side and emits a complete Bevy
//! `main.rs` scaffold on demand. It also exposes a few related stateless
//! skeleton emitters for plugin, plugin-group, and state-machine boilerplate.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use elicitation::{
    PluginContext, PluginToolRegistration, StatefulPlugin, ToolDescriptor, elicit_tool,
};
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

/// Stored configuration for a generated Bevy app.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyAppDescriptor {
    /// Human-readable app name.
    pub name: String,
    /// Optional DefaultPlugins configuration.
    pub default_plugins: Option<BevyDefaultPluginsDescriptor>,
    /// Additional plugin expressions registered on the app.
    pub plugins: Vec<String>,
    /// Custom schedules attached to the app.
    pub schedules: Vec<BevyScheduleDescriptor>,
    /// Optional runner expression passed to `app.set_runner(...)`.
    pub runner_expr: Option<String>,
}

/// Configuration for `DefaultPlugins`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyDefaultPluginsDescriptor {
    /// Optional `WindowPlugin { .. }` expression passed to `DefaultPlugins.set(...)`.
    pub window_plugin_expr: Option<String>,
}

/// Ordering metadata for a custom Bevy schedule.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyScheduleDescriptor {
    /// Schedule label expression, e.g. `MySchedule`.
    pub label_expr: String,
    /// If true, order this schedule in the startup schedule list.
    #[serde(default)]
    pub startup: bool,
    /// Insert after this existing schedule label.
    pub after: Option<String>,
    /// Insert before this existing schedule label.
    pub before: Option<String>,
}

/// A state hook used by `bevy_app__state_machine`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyStateHook {
    /// State variant name, e.g. `Loading`.
    pub state: String,
    /// System expression added for this transition.
    pub system_expr: String,
}

/// Shared registry context for `bevy_app__*` tools.
#[derive(Debug)]
pub struct BevyAppCtx {
    items: Mutex<HashMap<Uuid, BevyAppDescriptor>>,
}

impl BevyAppCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn lock_items(&self) -> Result<MutexGuard<'_, HashMap<Uuid, BevyAppDescriptor>>, ErrorData> {
        self.items
            .lock()
            .map_err(|_| ErrorData::internal_error("bevy_app registry lock poisoned", None))
    }
}

impl PluginContext for BevyAppCtx {}

/// Parameters for `bevy_app__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppNewParams {
    /// Human-readable app name.
    pub name: String,
}

/// Parameters for `bevy_app__add_default_plugins`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppAddDefaultPluginsParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
    /// Optional `WindowPlugin { .. }` expression passed to `DefaultPlugins.set(...)`.
    #[serde(default)]
    pub window_plugin_expr: Option<String>,
}

/// Parameters for `bevy_app__add_plugin`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppAddPluginParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
    /// Plugin expression, e.g. `MyGameplayPlugin` or `FrameTimeDiagnosticsPlugin`.
    pub plugin_expr: String,
}

/// Parameters for `bevy_app__add_schedule`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppAddScheduleParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
    /// Custom schedule label expression.
    pub label_expr: String,
    /// Whether this schedule belongs in startup ordering.
    #[serde(default)]
    pub startup: bool,
    /// Insert this schedule after an existing one.
    #[serde(default)]
    pub after: Option<String>,
    /// Insert this schedule before an existing one.
    #[serde(default)]
    pub before: Option<String>,
}

/// Parameters for `bevy_app__set_runner`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppSetRunnerParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
    /// Runner expression, e.g. `ScheduleRunnerPlugin::run_loop(...)` wrapper or custom closure.
    pub runner_expr: String,
}

/// Parameters for `bevy_app__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppDescribeParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
}

/// Parameters for `bevy_app__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyAppEmitParams {
    /// UUID returned by `bevy_app__new`.
    pub config_id: String,
}

/// Parameters for `bevy_app__plugin_struct`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyPluginStructParams {
    /// Plugin struct name.
    pub name: String,
    /// Optional body inserted into `fn build(&self, app: &mut App) { ... }`.
    #[serde(default)]
    pub body: Option<String>,
}

/// Parameters for `bevy_app__plugin_group`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyPluginGroupParams {
    /// Plugin-group struct name.
    pub name: String,
    /// Plugin expressions appended to the builder.
    #[serde(default)]
    pub plugins: Vec<String>,
}

/// Parameters for `bevy_app__state_machine`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyStateMachineParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// State enum name.
    pub enum_name: String,
    /// Optional visibility, e.g. `pub`.
    #[serde(default)]
    pub visibility: Option<String>,
    /// State variants in display order.
    pub variants: Vec<String>,
    /// Optional initial/default state variant.
    #[serde(default)]
    pub initial_state: Option<String>,
    /// Systems to register on `OnEnter`.
    #[serde(default)]
    pub on_enter: Vec<BevyStateHook>,
    /// Systems to register on `OnExit`.
    #[serde(default)]
    pub on_exit: Vec<BevyStateHook>,
}

/// Result returned by `bevy_app__new`.
#[derive(Debug, Serialize)]
pub struct BevyAppNewResult {
    /// UUID handle for the stored app descriptor.
    pub config_id: String,
}

fn default_app_var() -> String {
    "app".to_string()
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(serialized) => CallToolResult::success(vec![Content::text(serialized)]),
        Err(error) => {
            CallToolResult::error(vec![Content::text(format!("serialize error: {error}"))])
        }
    }
}

fn ok_text(text: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(text.into())])
}

fn parse_id(id: &str) -> Result<Uuid, ErrorData> {
    id.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {id}"), None))
}

fn parse_ident(src: &str, context: &str) -> Result<syn::Ident, ErrorData> {
    syn::parse_str::<syn::Ident>(src).map_err(|error| {
        ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
    })
}

fn parse_visibility(src: &str) -> Result<syn::Visibility, ErrorData> {
    syn::parse_str::<syn::Visibility>(src).map_err(|error| {
        ErrorData::invalid_params(format!("invalid visibility `{src}`: {error}"), None)
    })
}

fn parse_expr(src: &str, context: &str) -> Result<(), ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map(|_| ())
        .map_err(|error| {
            ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
        })
}

fn parse_tokens(src: &str, context: &str) -> Result<(), ErrorData> {
    src.parse::<proc_macro2::TokenStream>()
        .map(|_| ())
        .map_err(|error| {
            ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
        })
}

fn validate_schedule_params(params: &BevyAppAddScheduleParams) -> Result<(), ErrorData> {
    parse_expr(&params.label_expr, "schedule label")?;
    if let Some(after) = &params.after {
        parse_expr(after, "schedule after")?;
    }
    if let Some(before) = &params.before {
        parse_expr(before, "schedule before")?;
    }
    if params.after.is_some() && params.before.is_some() {
        return Err(ErrorData::invalid_params(
            "schedule ordering can use either `after` or `before`, not both".to_string(),
            None,
        ));
    }
    Ok(())
}

fn validate_state_machine_params(params: &BevyStateMachineParams) -> Result<(), ErrorData> {
    parse_expr(&params.app_var, "app receiver")?;
    parse_ident(&params.enum_name, "enum name")?;
    if let Some(visibility) = &params.visibility {
        let _ = parse_visibility(visibility)?;
    }
    if params.variants.is_empty() {
        return Err(ErrorData::invalid_params(
            "state variants must not be empty".to_string(),
            None,
        ));
    }
    for variant in &params.variants {
        parse_ident(variant, "state variant")?;
    }
    if let Some(initial) = &params.initial_state {
        parse_ident(initial, "initial state")?;
        if !params.variants.iter().any(|variant| variant == initial) {
            return Err(ErrorData::invalid_params(
                format!("initial_state `{initial}` is not in variants"),
                None,
            ));
        }
    }
    for hook in params.on_enter.iter().chain(params.on_exit.iter()) {
        parse_ident(&hook.state, "hook state")?;
        if !params.variants.iter().any(|variant| variant == &hook.state) {
            return Err(ErrorData::invalid_params(
                format!("hook state `{}` is not in variants", hook.state),
                None,
            ));
        }
        parse_expr(&hook.system_expr, "hook system expression")?;
    }
    Ok(())
}

fn emit_default_plugins(desc: &BevyDefaultPluginsDescriptor) -> String {
    match &desc.window_plugin_expr {
        Some(window) => format!("::bevy::DefaultPlugins.set({window})"),
        None => "::bevy::DefaultPlugins".to_string(),
    }
}

fn emit_schedule(desc: &BevyScheduleDescriptor) -> Vec<String> {
    let mut lines = vec![format!(
        "    app.add_schedule(::bevy::ecs::schedule::Schedule::new({}));",
        desc.label_expr
    )];
    match (&desc.after, &desc.before, desc.startup) {
        (Some(after), None, true) => {
            lines.push("    app.init_resource::<::bevy::app::MainScheduleOrder>();".to_string());
            lines.push(format!(
                "    app.world_mut().resource_mut::<::bevy::app::MainScheduleOrder>().insert_startup_after({after}, {});",
                desc.label_expr
            ));
        }
        (None, Some(before), true) => {
            lines.push("    app.init_resource::<::bevy::app::MainScheduleOrder>();".to_string());
            lines.push(format!(
                "    app.world_mut().resource_mut::<::bevy::app::MainScheduleOrder>().insert_startup_before({before}, {});",
                desc.label_expr
            ));
        }
        (Some(after), None, false) => {
            lines.push("    app.init_resource::<::bevy::app::MainScheduleOrder>();".to_string());
            lines.push(format!(
                "    app.world_mut().resource_mut::<::bevy::app::MainScheduleOrder>().insert_after({after}, {});",
                desc.label_expr
            ));
        }
        (None, Some(before), false) => {
            lines.push("    app.init_resource::<::bevy::app::MainScheduleOrder>();".to_string());
            lines.push(format!(
                "    app.world_mut().resource_mut::<::bevy::app::MainScheduleOrder>().insert_before({before}, {});",
                desc.label_expr
            ));
        }
        _ => {}
    }
    lines
}

fn emit_app(desc: &BevyAppDescriptor) -> String {
    let mut lines = vec![format!("// Generated Bevy app descriptor: {}", desc.name)];
    lines.push("fn main() {".to_string());
    lines.push("    let mut app = ::bevy::app::App::new();".to_string());
    if let Some(default_plugins) = &desc.default_plugins {
        lines.push(format!(
            "    app.add_plugins({});",
            emit_default_plugins(default_plugins)
        ));
    }
    for plugin in &desc.plugins {
        lines.push(format!("    app.add_plugins({plugin});"));
    }
    for schedule in &desc.schedules {
        lines.extend(emit_schedule(schedule));
    }
    if let Some(runner) = &desc.runner_expr {
        lines.push(format!("    app.set_runner({runner});"));
    }
    lines.push("    app.run();".to_string());
    lines.push("}".to_string());
    lines.join("\n")
}

fn emit_plugin_struct(name: &str, body: Option<&str>) -> String {
    let body = body.unwrap_or("// configure plugin systems and resources here");
    format!(
        "pub struct {name};\n\n\
         impl ::bevy::app::Plugin for {name} {{\n\
         \x20   fn build(&self, app: &mut ::bevy::app::App) {{\n\
         \x20       let _ = app;\n\
         \x20       {body}\n\
         \x20   }}\n\
         }}\n"
    )
}

fn emit_plugin_group(name: &str, plugins: &[String]) -> String {
    let builder = if plugins.is_empty() {
        "        ::bevy::app::PluginGroupBuilder::start::<Self>()".to_string()
    } else {
        let chain = plugins
            .iter()
            .map(|plugin| format!("            .add({plugin})"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("        ::bevy::app::PluginGroupBuilder::start::<Self>()\n{chain}")
    };
    format!(
        "pub struct {name};\n\n\
         impl ::bevy::app::PluginGroup for {name} {{\n\
         \x20   fn build(self) -> ::bevy::app::PluginGroupBuilder {{\n\
         {builder}\n\
         \x20   }}\n\
         }}\n"
    )
}

fn emit_state_machine(params: &BevyStateMachineParams) -> String {
    let visibility = params.visibility.as_deref().unwrap_or("pub");
    let default_variant = params
        .initial_state
        .clone()
        .unwrap_or_else(|| params.variants[0].clone());
    let enum_body = params
        .variants
        .iter()
        .map(|variant| {
            if variant == &default_variant {
                format!("    #[default]\n    {variant},")
            } else {
                format!("    {variant},")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut wiring = vec![format!(
        "{app}.init_state::<{enum_name}>();",
        app = params.app_var,
        enum_name = params.enum_name
    )];
    for hook in &params.on_enter {
        wiring.push(format!(
            "{app}.add_systems(::bevy::prelude::OnEnter({enum_name}::{state}), {system});",
            app = params.app_var,
            enum_name = params.enum_name,
            state = hook.state,
            system = hook.system_expr
        ));
    }
    for hook in &params.on_exit {
        wiring.push(format!(
            "{app}.add_systems(::bevy::prelude::OnExit({enum_name}::{state}), {system});",
            app = params.app_var,
            enum_name = params.enum_name,
            state = hook.state,
            system = hook.system_expr
        ));
    }
    format!(
        "#[derive(::bevy::prelude::States, Debug, Clone, PartialEq, Eq, Hash, Default)]\n\
         {visibility} enum {enum_name} {{\n\
         {enum_body}\n\
         }}\n\n\
         {wiring}\n",
        visibility = visibility,
        enum_name = params.enum_name,
        enum_body = enum_body,
        wiring = wiring.join("\n")
    )
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "new",
    description = "Create a new Bevy app descriptor registry entry.",
    emit = None
)]
#[instrument(skip(ctx), fields(name = %p.name))]
async fn new_app(ctx: Arc<BevyAppCtx>, p: BevyAppNewParams) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(
        id,
        BevyAppDescriptor {
            name: p.name,
            default_plugins: None,
            plugins: vec![],
            schedules: vec![],
            runner_expr: None,
        },
    );
    Ok(json_result(&BevyAppNewResult {
        config_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "add_default_plugins",
    description = "Attach DefaultPlugins to a stored Bevy app descriptor, optionally with a WindowPlugin expression.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn add_default_plugins(
    ctx: Arc<BevyAppCtx>,
    p: BevyAppAddDefaultPluginsParams,
) -> Result<CallToolResult, ErrorData> {
    if let Some(window) = &p.window_plugin_expr {
        parse_expr(window, "window plugin expression")?;
    }
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.lock_items()?;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.default_plugins = Some(BevyDefaultPluginsDescriptor {
        window_plugin_expr: p.window_plugin_expr,
    });
    Ok(ok_text("default plugins configured"))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "add_plugin",
    description = "Register an additional plugin expression on a stored Bevy app descriptor.",
    emit = None
)]
#[instrument(skip(ctx), fields(plugin = %p.plugin_expr))]
async fn add_plugin(
    ctx: Arc<BevyAppCtx>,
    p: BevyAppAddPluginParams,
) -> Result<CallToolResult, ErrorData> {
    parse_expr(&p.plugin_expr, "plugin expression")?;
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.lock_items()?;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.plugins.push(p.plugin_expr);
    Ok(ok_text("plugin added"))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "add_schedule",
    description = "Attach a custom schedule plus optional MainScheduleOrder placement to a stored Bevy app descriptor.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn add_schedule(
    ctx: Arc<BevyAppCtx>,
    p: BevyAppAddScheduleParams,
) -> Result<CallToolResult, ErrorData> {
    validate_schedule_params(&p)?;
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.lock_items()?;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.schedules.push(BevyScheduleDescriptor {
        label_expr: p.label_expr,
        startup: p.startup,
        after: p.after,
        before: p.before,
    });
    Ok(ok_text("schedule added"))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "set_runner",
    description = "Set the app runner expression on a stored Bevy app descriptor.",
    emit = None
)]
#[instrument(skip(ctx), fields(runner = %p.runner_expr))]
async fn set_runner(
    ctx: Arc<BevyAppCtx>,
    p: BevyAppSetRunnerParams,
) -> Result<CallToolResult, ErrorData> {
    parse_expr(&p.runner_expr, "runner expression")?;
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.lock_items()?;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.runner_expr = Some(p.runner_expr);
    Ok(ok_text("runner set"))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "describe",
    description = "Return the JSON descriptor for a stored Bevy app configuration.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn describe(
    ctx: Arc<BevyAppCtx>,
    p: BevyAppDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let items = ctx.lock_items()?;
    let desc = items.get(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    Ok(json_result(desc))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "emit",
    description = "Emit a complete Bevy `fn main()` scaffold from a stored app descriptor.",
    emit = None
)]
#[instrument(skip(ctx))]
async fn emit(ctx: Arc<BevyAppCtx>, p: BevyAppEmitParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let items = ctx.lock_items()?;
    let desc = items.get(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    Ok(ok_text(emit_app(desc)))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "plugin_struct",
    description = "Emit a `struct MyPlugin; impl Plugin for MyPlugin { ... }` skeleton.",
    emit = None
)]
#[instrument(skip_all)]
async fn plugin_struct(p: BevyPluginStructParams) -> Result<CallToolResult, ErrorData> {
    parse_ident(&p.name, "plugin name")?;
    if let Some(body) = &p.body {
        parse_tokens(body, "plugin body")?;
    }
    Ok(ok_text(emit_plugin_struct(&p.name, p.body.as_deref())))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "plugin_group",
    description = "Emit a `struct MyGroup; impl PluginGroup for MyGroup { ... }` skeleton.",
    emit = None
)]
#[instrument(skip_all)]
async fn plugin_group(p: BevyPluginGroupParams) -> Result<CallToolResult, ErrorData> {
    parse_ident(&p.name, "plugin group name")?;
    for plugin in &p.plugins {
        parse_expr(plugin, "plugin expression")?;
    }
    Ok(ok_text(emit_plugin_group(&p.name, &p.plugins)))
}

#[elicit_tool(
    plugin = "bevy_app",
    name = "state_machine",
    description = "Emit a `States` enum plus `init_state`, `OnEnter`, and `OnExit` app wiring.",
    emit = None
)]
#[instrument(skip_all)]
async fn state_machine(p: BevyStateMachineParams) -> Result<CallToolResult, ErrorData> {
    validate_state_machine_params(&p)?;
    Ok(ok_text(emit_state_machine(&p)))
}

/// MCP plugin providing `bevy_app__*` tools for Bevy app descriptors.
#[derive(Debug)]
pub struct BevyAppPlugin(Arc<BevyAppCtx>);

impl BevyAppPlugin {
    /// Creates a new `BevyAppPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(BevyAppCtx::new()))
    }

    /// Returns a shared reference to the underlying app registry context.
    pub fn ctx(&self) -> Arc<BevyAppCtx> {
        Arc::clone(&self.0)
    }

    /// Convenience helper for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let bare = name.strip_prefix("bevy_app__").unwrap_or(name).to_string();
        let params = if let Some(map) = args.as_object().cloned() {
            CallToolRequestParams::new(bare.clone()).with_arguments(map)
        } else {
            CallToolRequestParams::new(bare.clone())
        };
        let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_app")
            .find(|registration| registration.name == bare)
            .map(|registration| (registration.constructor)())
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
        descriptor
            .dispatch(
                self.0.clone() as Arc<dyn std::any::Any + Send + Sync>,
                params,
            )
            .await
    }
}

impl Default for BevyAppPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for BevyAppPlugin {
    type Context = BevyAppCtx;

    fn name(&self) -> &'static str {
        "bevy_app"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_app")
            .map(|registration| (registration.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_app")
            .map(|registration| (registration.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}
