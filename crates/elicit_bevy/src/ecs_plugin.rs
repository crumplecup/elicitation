//! `BevyEcsPlugin` — fragment tools that emit Bevy ECS/app wiring snippets.
//!
//! These tools cover the next Phase 3 surface for `elicit_bevy`: the common
//! `App`, `Commands`, and system-configuration fragments that agents need to
//! assemble Bevy programs without hand-authoring the repetitive boilerplate.

use elicitation::emit_code::{CrateDep, EmitCode};
use elicitation::{ElicitPlugin, elicit_tool};
use proc_macro2::TokenStream;
use quote::quote;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// A query data item, e.g. `&Transform` or `&mut Velocity`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryItemSpec {
    /// The queried Rust type.
    pub ty: String,
    /// Whether the item is mutable (`&mut T`) rather than shared (`&T`).
    #[serde(default)]
    pub mutable: bool,
}

/// Parameters for `bevy_ecs__add_systems`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddSystemsParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Schedule label expression, e.g. `Update`.
    pub schedule: String,
    /// System expressions or function items.
    pub systems: Vec<String>,
    /// Whether to wrap the systems tuple in `.chain()`.
    #[serde(default)]
    pub chain: bool,
    /// Optional `.run_if(...)` condition.
    #[serde(default)]
    pub run_if: Option<String>,
    /// Optional `.in_set(...)` set expression.
    #[serde(default)]
    pub in_set: Option<String>,
    /// Optional `.before(...)` constraints.
    #[serde(default)]
    pub before: Vec<String>,
    /// Optional `.after(...)` constraints.
    #[serde(default)]
    pub after: Vec<String>,
}

/// Parameters for `bevy_ecs__add_plugins`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPluginsParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Plugin expressions to pass to `add_plugins`.
    pub plugins: Vec<String>,
}

/// Parameters for `bevy_ecs__insert_resource`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InsertResourceParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Resource construction expression.
    pub resource_expr: String,
}

/// Parameters for `bevy_ecs__init_resource`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InitResourceParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Resource type used in `init_resource::<T>()`.
    pub resource_type: String,
}

/// Parameters for `bevy_ecs__add_event`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddEventParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Event type used in `add_event::<T>()`.
    pub event_type: String,
}

/// Parameters for `bevy_ecs__register_type`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RegisterTypeParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Reflected type used in `register_type::<T>()`.
    pub reflected_type: String,
}

/// Parameters for `bevy_ecs__spawn_entity`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnEntityParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Component expressions inserted into the spawned tuple.
    pub components: Vec<String>,
}

/// Parameters for `bevy_ecs__spawn_bundle`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpawnBundleParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Bundle expression to spawn.
    pub bundle_expr: String,
}

/// Parameters for `bevy_ecs__with_children`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WithChildrenParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Bundle or component tuple for the parent entity.
    pub parent_expr: String,
    /// Child bundle/component expressions. Each becomes `parent.spawn(expr);`.
    #[serde(default)]
    pub children: Vec<String>,
}

/// Parameters for `bevy_ecs__insert_component`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InsertComponentParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Entity expression passed to `commands.entity(...)`.
    pub entity_expr: String,
    /// Component expression to insert.
    pub component_expr: String,
}

/// Parameters for `bevy_ecs__remove_component`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveComponentParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Entity expression passed to `commands.entity(...)`.
    pub entity_expr: String,
    /// Component type used in `remove::<T>()`.
    pub component_type: String,
}

/// Parameters for `bevy_ecs__despawn`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DespawnParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Entity expression passed to `commands.entity(...)`.
    pub entity_expr: String,
    /// Whether to despawn children before despawning the parent.
    #[serde(default)]
    pub recursive: bool,
}

/// Parameters for `bevy_ecs__query_for`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct QueryForParams {
    /// Query item references, e.g. `&Transform`, `&mut Velocity`.
    pub items: Vec<QueryItemSpec>,
    /// Filter expressions, e.g. `With<Player>`, `Without<Dead>`, `Changed<Health>`.
    #[serde(default)]
    pub filters: Vec<String>,
}

/// Parameters for `bevy_ecs__run_criteria`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RunCriteriaParams {
    /// System expression to wrap.
    pub system_expr: String,
    /// Condition expression passed to `.run_if(...)`.
    pub condition_expr: String,
}

/// Parameters for `bevy_ecs__in_set`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InSetParams {
    /// System expression to wrap.
    pub system_expr: String,
    /// System-set expression passed to `.in_set(...)`.
    pub set_expr: String,
}

/// Parameters for `bevy_ecs__chain`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ChainParams {
    /// System expressions to join in-order.
    pub systems: Vec<String>,
}

/// Parameters for `bevy_ecs__pipe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PipeParams {
    /// Left-hand system expression.
    pub left: String,
    /// Right-hand system expression.
    pub right: String,
}

/// Parameters for `bevy_ecs__observer`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ObserverParams {
    /// App-like receiver expression, defaults to `app`.
    #[serde(default = "default_app_var")]
    pub app_var: String,
    /// Observer system expression passed to `add_observer`.
    pub observer_expr: String,
}

/// Parameters for `bevy_ecs__trigger`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TriggerParams {
    /// Commands-like receiver expression, defaults to `commands`.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Event expression passed to `trigger`.
    pub event_expr: String,
}

fn default_app_var() -> String {
    "app".to_string()
}

fn default_commands_var() -> String {
    "commands".to_string()
}

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_source(source: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

fn parse_expr(src: &str, context: &str) -> Result<syn::Expr, ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map_err(|error| tool_err(format!("invalid {context} expression `{src}`: {error}")))
}

fn parse_type(src: &str, context: &str) -> Result<syn::Type, ErrorData> {
    syn::parse_str::<syn::Type>(src)
        .map_err(|error| tool_err(format!("invalid {context} type `{src}`: {error}")))
}

fn validate_exprs(values: &[String], context: &str) -> Result<(), ErrorData> {
    for value in values {
        let _ = parse_expr(value, context)?;
    }
    Ok(())
}

fn validate_non_empty<T>(values: &[T], context: &str) -> Result<(), ErrorData> {
    if values.is_empty() {
        Err(tool_err(format!("{context} must not be empty")))
    } else {
        Ok(())
    }
}

fn expr_tokens(src: &str, context: &str) -> syn::Expr {
    parse_expr(src, context).expect("validated expressions must parse")
}

fn type_tokens(src: &str, context: &str) -> syn::Type {
    parse_type(src, context).expect("validated types must parse")
}

fn system_group_tokens(systems: &[String]) -> Vec<syn::Expr> {
    systems
        .iter()
        .map(|system| expr_tokens(system, "system"))
        .collect()
}

fn render_systems_expr(systems: &[String], chain: bool) -> TokenStream {
    let system_tokens = system_group_tokens(systems);
    if system_tokens.len() == 1 {
        let system = &system_tokens[0];
        quote! { #system }
    } else if chain {
        quote! { (#(#system_tokens),*).chain() }
    } else {
        quote! { (#(#system_tokens),*) }
    }
}

fn render_filter_tokens(filters: &[String]) -> TokenStream {
    let filter_tokens: Vec<syn::Type> = filters
        .iter()
        .map(|filter| type_tokens(filter, "query filter"))
        .collect();
    match filter_tokens.len() {
        0 => TokenStream::new(),
        1 => {
            let filter = &filter_tokens[0];
            quote! { , #filter }
        }
        _ => quote! { , (#(#filter_tokens),*) },
    }
}

fn bevy_dep() -> Vec<CrateDep> {
    vec![CrateDep::new("bevy", "0.18")]
}

impl EmitCode for AddSystemsParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let schedule = expr_tokens(&self.schedule, "schedule");
        let mut config = render_systems_expr(&self.systems, self.chain);
        if let Some(run_if) = &self.run_if {
            let condition = expr_tokens(run_if, "run_if");
            config = quote! { (#config).run_if(#condition) };
        }
        if let Some(in_set) = &self.in_set {
            let set = expr_tokens(in_set, "in_set");
            config = quote! { (#config).in_set(#set) };
        }
        for before in &self.before {
            let constraint = expr_tokens(before, "before");
            config = quote! { (#config).before(#constraint) };
        }
        for after in &self.after {
            let constraint = expr_tokens(after, "after");
            config = quote! { (#config).after(#constraint) };
        }
        quote! { #app.add_systems(#schedule, #config) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for AddPluginsParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let plugins: Vec<syn::Expr> = self
            .plugins
            .iter()
            .map(|plugin| expr_tokens(plugin, "plugin"))
            .collect();
        if plugins.len() == 1 {
            let plugin = &plugins[0];
            quote! { #app.add_plugins(#plugin) }
        } else {
            quote! { #app.add_plugins((#(#plugins),*)) }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for InsertResourceParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let resource = expr_tokens(&self.resource_expr, "resource");
        quote! { #app.insert_resource(#resource) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for InitResourceParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let resource = type_tokens(&self.resource_type, "resource");
        quote! { #app.init_resource::<#resource>() }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for AddEventParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let event = type_tokens(&self.event_type, "event");
        quote! { #app.add_event::<#event>() }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for RegisterTypeParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let reflected = type_tokens(&self.reflected_type, "reflected");
        quote! { #app.register_type::<#reflected>() }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for SpawnEntityParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let components: Vec<syn::Expr> = self
            .components
            .iter()
            .map(|component| expr_tokens(component, "component"))
            .collect();
        if components.len() == 1 {
            let component = &components[0];
            quote! { #commands.spawn(#component) }
        } else {
            quote! { #commands.spawn((#(#components),*)) }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for SpawnBundleParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let bundle = expr_tokens(&self.bundle_expr, "bundle");
        quote! { #commands.spawn(#bundle) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for WithChildrenParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let parent_expr = expr_tokens(&self.parent_expr, "parent");
        let children: Vec<syn::Expr> = self
            .children
            .iter()
            .map(|child| expr_tokens(child, "child"))
            .collect();
        quote! {
            #commands.spawn(#parent_expr).with_children(|parent| {
                #(parent.spawn(#children);)*
            })
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for InsertComponentParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let entity = expr_tokens(&self.entity_expr, "entity");
        let component = expr_tokens(&self.component_expr, "component");
        quote! { #commands.entity(#entity).insert(#component) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for RemoveComponentParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let entity = expr_tokens(&self.entity_expr, "entity");
        let component = type_tokens(&self.component_type, "component");
        quote! { #commands.entity(#entity).remove::<#component>() }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for DespawnParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let entity = expr_tokens(&self.entity_expr, "entity");
        if self.recursive {
            quote! { #commands.entity(#entity).despawn_children().despawn() }
        } else {
            quote! { #commands.entity(#entity).despawn() }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for QueryForParams {
    fn emit_code(&self) -> TokenStream {
        let items: Vec<TokenStream> = self
            .items
            .iter()
            .map(|item| {
                let ty = type_tokens(&item.ty, "query item");
                if item.mutable {
                    quote! { &mut #ty }
                } else {
                    quote! { &#ty }
                }
            })
            .collect();
        let data = if items.len() == 1 {
            let item = &items[0];
            quote! { #item }
        } else {
            quote! { (#(#items),*) }
        };
        let filters = render_filter_tokens(&self.filters);
        quote! { Query<#data #filters> }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for RunCriteriaParams {
    fn emit_code(&self) -> TokenStream {
        let system = expr_tokens(&self.system_expr, "system");
        let condition = expr_tokens(&self.condition_expr, "condition");
        quote! { (#system).run_if(#condition) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for InSetParams {
    fn emit_code(&self) -> TokenStream {
        let system = expr_tokens(&self.system_expr, "system");
        let set = expr_tokens(&self.set_expr, "set");
        quote! { (#system).in_set(#set) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for ChainParams {
    fn emit_code(&self) -> TokenStream {
        let systems = system_group_tokens(&self.systems);
        quote! { (#(#systems),*).chain() }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for PipeParams {
    fn emit_code(&self) -> TokenStream {
        let left = expr_tokens(&self.left, "left");
        let right = expr_tokens(&self.right, "right");
        quote! { (#left).pipe(#right) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for ObserverParams {
    fn emit_code(&self) -> TokenStream {
        let app = expr_tokens(&self.app_var, "app receiver");
        let observer = expr_tokens(&self.observer_expr, "observer");
        quote! { #app.add_observer(#observer) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for TriggerParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let event = expr_tokens(&self.event_expr, "event");
        quote! { #commands.trigger(#event) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

elicitation::register_emit!("add_systems", AddSystemsParams);
elicitation::register_emit!("add_plugins", AddPluginsParams);
elicitation::register_emit!("insert_resource", InsertResourceParams);
elicitation::register_emit!("init_resource", InitResourceParams);
elicitation::register_emit!("add_event", AddEventParams);
elicitation::register_emit!("register_type", RegisterTypeParams);
elicitation::register_emit!("spawn_entity", SpawnEntityParams);
elicitation::register_emit!("spawn_bundle", SpawnBundleParams);
elicitation::register_emit!("with_children", WithChildrenParams);
elicitation::register_emit!("insert_component", InsertComponentParams);
elicitation::register_emit!("remove_component", RemoveComponentParams);
elicitation::register_emit!("despawn", DespawnParams);
elicitation::register_emit!("query_for", QueryForParams);
elicitation::register_emit!("run_criteria", RunCriteriaParams);
elicitation::register_emit!("in_set", InSetParams);
elicitation::register_emit!("chain", ChainParams);
elicitation::register_emit!("pipe", PipeParams);
elicitation::register_emit!("observer", ObserverParams);
elicitation::register_emit!("trigger", TriggerParams);

/// MCP plugin exposing Bevy ECS/app-wiring fragment tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_ecs")]
pub struct BevyEcsPlugin;

impl BevyEcsPlugin {
    /// Creates a new Bevy ECS fragment plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyEcsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_add_systems(params: &AddSystemsParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_expr(&params.schedule, "schedule")?;
    validate_non_empty(&params.systems, "systems")?;
    validate_exprs(&params.systems, "system")?;
    if params.chain && params.systems.len() < 2 {
        return Err(tool_err("chain requires at least two systems"));
    }
    if let Some(run_if) = &params.run_if {
        let _ = parse_expr(run_if, "run_if")?;
    }
    if let Some(in_set) = &params.in_set {
        let _ = parse_expr(in_set, "in_set")?;
    }
    validate_exprs(&params.before, "before")?;
    validate_exprs(&params.after, "after")?;
    Ok(())
}

fn validate_add_plugins(params: &AddPluginsParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    validate_non_empty(&params.plugins, "plugins")?;
    validate_exprs(&params.plugins, "plugin")?;
    Ok(())
}

fn validate_insert_resource(params: &InsertResourceParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_expr(&params.resource_expr, "resource")?;
    Ok(())
}

fn validate_init_resource(params: &InitResourceParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_type(&params.resource_type, "resource")?;
    Ok(())
}

fn validate_add_event(params: &AddEventParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_type(&params.event_type, "event")?;
    Ok(())
}

fn validate_register_type(params: &RegisterTypeParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_type(&params.reflected_type, "reflected")?;
    Ok(())
}

fn validate_spawn_entity(params: &SpawnEntityParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_non_empty(&params.components, "components")?;
    validate_exprs(&params.components, "component")?;
    Ok(())
}

fn validate_spawn_bundle(params: &SpawnBundleParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.bundle_expr, "bundle")?;
    Ok(())
}

fn validate_with_children(params: &WithChildrenParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.parent_expr, "parent")?;
    validate_exprs(&params.children, "child")?;
    Ok(())
}

fn validate_insert_component(params: &InsertComponentParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.entity_expr, "entity")?;
    let _ = parse_expr(&params.component_expr, "component")?;
    Ok(())
}

fn validate_remove_component(params: &RemoveComponentParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.entity_expr, "entity")?;
    let _ = parse_type(&params.component_type, "component")?;
    Ok(())
}

fn validate_despawn(params: &DespawnParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.entity_expr, "entity")?;
    Ok(())
}

fn validate_query_for(params: &QueryForParams) -> Result<(), ErrorData> {
    validate_non_empty(&params.items, "query items")?;
    for item in &params.items {
        let _ = parse_type(&item.ty, "query item")?;
    }
    for filter in &params.filters {
        let _ = parse_type(filter, "query filter")?;
    }
    Ok(())
}

fn validate_run_criteria(params: &RunCriteriaParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.system_expr, "system")?;
    let _ = parse_expr(&params.condition_expr, "condition")?;
    Ok(())
}

fn validate_in_set(params: &InSetParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.system_expr, "system")?;
    let _ = parse_expr(&params.set_expr, "set")?;
    Ok(())
}

fn validate_chain(params: &ChainParams) -> Result<(), ErrorData> {
    validate_non_empty(&params.systems, "systems")?;
    if params.systems.len() < 2 {
        return Err(tool_err("chain requires at least two systems"));
    }
    validate_exprs(&params.systems, "system")?;
    Ok(())
}

fn validate_pipe(params: &PipeParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.left, "left")?;
    let _ = parse_expr(&params.right, "right")?;
    Ok(())
}

fn validate_observer(params: &ObserverParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.app_var, "app receiver")?;
    let _ = parse_expr(&params.observer_expr, "observer")?;
    Ok(())
}

fn validate_trigger(params: &TriggerParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    let _ = parse_expr(&params.event_expr, "event")?;
    Ok(())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "add_systems",
    description = "Emit an `app.add_systems(schedule, systems)` fragment with optional `.chain()`, `.run_if(...)`, `.in_set(...)`, `.before(...)`, and `.after(...)`."
)]
#[instrument(skip_all)]
async fn emit_add_systems(p: AddSystemsParams) -> Result<CallToolResult, ErrorData> {
    validate_add_systems(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "add_plugins",
    description = "Emit an `app.add_plugins(...)` fragment for one plugin expression or a tuple of plugins."
)]
#[instrument(skip_all)]
async fn emit_add_plugins(p: AddPluginsParams) -> Result<CallToolResult, ErrorData> {
    validate_add_plugins(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "insert_resource",
    description = "Emit an `app.insert_resource(resource_expr)` fragment."
)]
#[instrument(skip_all)]
async fn emit_insert_resource(p: InsertResourceParams) -> Result<CallToolResult, ErrorData> {
    validate_insert_resource(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "init_resource",
    description = "Emit an `app.init_resource::<T>()` fragment."
)]
#[instrument(skip_all)]
async fn emit_init_resource(p: InitResourceParams) -> Result<CallToolResult, ErrorData> {
    validate_init_resource(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "add_event",
    description = "Emit an `app.add_event::<T>()` fragment."
)]
#[instrument(skip_all)]
async fn emit_add_event(p: AddEventParams) -> Result<CallToolResult, ErrorData> {
    validate_add_event(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "register_type",
    description = "Emit an `app.register_type::<T>()` fragment."
)]
#[instrument(skip_all)]
async fn emit_register_type(p: RegisterTypeParams) -> Result<CallToolResult, ErrorData> {
    validate_register_type(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "spawn_entity",
    description = "Emit a `commands.spawn(...)` fragment from one component expression or a component tuple."
)]
#[instrument(skip_all)]
async fn emit_spawn_entity(p: SpawnEntityParams) -> Result<CallToolResult, ErrorData> {
    validate_spawn_entity(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "spawn_bundle",
    description = "Emit a `commands.spawn(bundle_expr)` fragment."
)]
#[instrument(skip_all)]
async fn emit_spawn_bundle(p: SpawnBundleParams) -> Result<CallToolResult, ErrorData> {
    validate_spawn_bundle(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "with_children",
    description = "Emit a `commands.spawn(parent).with_children(|parent| { ... })` fragment using child spawn expressions."
)]
#[instrument(skip_all)]
async fn emit_with_children(p: WithChildrenParams) -> Result<CallToolResult, ErrorData> {
    validate_with_children(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "insert_component",
    description = "Emit a `commands.entity(entity).insert(component)` fragment."
)]
#[instrument(skip_all)]
async fn emit_insert_component(p: InsertComponentParams) -> Result<CallToolResult, ErrorData> {
    validate_insert_component(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "remove_component",
    description = "Emit a `commands.entity(entity).remove::<T>()` fragment."
)]
#[instrument(skip_all)]
async fn emit_remove_component(p: RemoveComponentParams) -> Result<CallToolResult, ErrorData> {
    validate_remove_component(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "despawn",
    description = "Emit a `commands.entity(entity).despawn()` fragment, or `despawn_children().despawn()` for recursive hierarchy cleanup in Bevy 0.18."
)]
#[instrument(skip_all)]
async fn emit_despawn(p: DespawnParams) -> Result<CallToolResult, ErrorData> {
    validate_despawn(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "query_for",
    description = "Emit a `Query<...>` type fragment from shared/mutable query items plus optional filters."
)]
#[instrument(skip_all)]
async fn emit_query_for(p: QueryForParams) -> Result<CallToolResult, ErrorData> {
    validate_query_for(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "run_criteria",
    description = "Emit a `system.run_if(condition)` fragment."
)]
#[instrument(skip_all)]
async fn emit_run_criteria(p: RunCriteriaParams) -> Result<CallToolResult, ErrorData> {
    validate_run_criteria(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "in_set",
    description = "Emit a `system.in_set(set_expr)` fragment."
)]
#[instrument(skip_all)]
async fn emit_in_set(p: InSetParams) -> Result<CallToolResult, ErrorData> {
    validate_in_set(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "chain",
    description = "Emit a `(system_a, system_b, ...).chain()` fragment."
)]
#[instrument(skip_all)]
async fn emit_chain(p: ChainParams) -> Result<CallToolResult, ErrorData> {
    validate_chain(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "pipe",
    description = "Emit a `system_a.pipe(system_b)` fragment."
)]
#[instrument(skip_all)]
async fn emit_pipe(p: PipeParams) -> Result<CallToolResult, ErrorData> {
    validate_pipe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "observer",
    description = "Emit an `app.add_observer(observer_system)` fragment."
)]
#[instrument(skip_all)]
async fn emit_observer(p: ObserverParams) -> Result<CallToolResult, ErrorData> {
    validate_observer(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_ecs",
    emit = None,
    name = "trigger",
    description = "Emit a `commands.trigger(event_expr)` fragment."
)]
#[instrument(skip_all)]
async fn emit_trigger(p: TriggerParams) -> Result<CallToolResult, ErrorData> {
    validate_trigger(&p)?;
    ok_source(p.emit_code().to_string())
}
