//! `BevyScenePlugin` — descriptor-registry tools for Bevy scene assembly.
//!
//! The live `DynamicScene` runtime type depends on Bevy reflection state and asset
//! plumbing that is not directly serializable over MCP. This plugin therefore
//! stores a scene descriptor server-side and can emit either:
//!
//! - a RON descriptor manifest that captures the intended scene contents
//! - Rust `Commands` snippets that recreate the same resources and entities

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

/// A typed value included in a scene descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevySceneValueDescriptor {
    /// Fully qualified Rust type path for the component or resource.
    pub type_path: String,
    /// Rust expression used when emitting spawn code.
    pub value_expr: String,
}

/// A single entity stored in a scene descriptor.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevySceneEntityDescriptor {
    /// Optional Bevy `Name` component value added during spawn-code emission.
    pub name: Option<String>,
    /// Components inserted when the entity is spawned.
    pub components: Vec<BevySceneValueDescriptor>,
}

/// Stored configuration for a generated Bevy scene.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevySceneDescriptor {
    /// Human-readable scene name.
    pub name: String,
    /// Resources inserted before entity spawns.
    pub resources: Vec<BevySceneValueDescriptor>,
    /// Entity descriptors emitted into the scene.
    pub entities: Vec<BevySceneEntityDescriptor>,
}

/// Shared registry context for `bevy_scene__*` tools.
#[derive(Debug)]
pub struct BevySceneCtx {
    items: Mutex<HashMap<Uuid, BevySceneDescriptor>>,
}

impl BevySceneCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn lock_items(&self) -> Result<MutexGuard<'_, HashMap<Uuid, BevySceneDescriptor>>, ErrorData> {
        self.items
            .lock()
            .map_err(|_| ErrorData::internal_error("bevy_scene registry lock poisoned", None))
    }
}

impl PluginContext for BevySceneCtx {}

/// Parameters for `bevy_scene__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevySceneNewParams {
    /// Human-readable scene name.
    pub name: String,
}

/// Parameters for `bevy_scene__add_entity`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevySceneAddEntityParams {
    /// UUID returned by `bevy_scene__new`.
    pub scene_id: String,
    /// Optional `Name` component value.
    #[serde(default)]
    pub name: Option<String>,
    /// Components attached to the new entity.
    pub components: Vec<BevySceneValueDescriptor>,
}

/// Parameters for `bevy_scene__add_resource`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevySceneAddResourceParams {
    /// UUID returned by `bevy_scene__new`.
    pub scene_id: String,
    /// Resource inserted into the scene setup snippet.
    pub resource: BevySceneValueDescriptor,
}

/// Parameters for `bevy_scene__emit_ron`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevySceneEmitRonParams {
    /// UUID returned by `bevy_scene__new`.
    pub scene_id: String,
}

/// Parameters for `bevy_scene__emit_spawn_code`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevySceneEmitSpawnCodeParams {
    /// UUID returned by `bevy_scene__new`.
    pub scene_id: String,
    /// Receiver expression used for resource insertion and entity spawning.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
}

/// Result returned by `bevy_scene__new`.
#[derive(Debug, Serialize)]
pub struct BevySceneNewResult {
    /// UUID handle for the stored scene descriptor.
    pub scene_id: String,
}

fn default_commands_var() -> String {
    "commands".to_string()
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

fn parse_path(src: &str, context: &str) -> Result<(), ErrorData> {
    syn::parse_str::<syn::Path>(src)
        .map(|_| ())
        .map_err(|error| {
            ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
        })
}

fn parse_expr(src: &str, context: &str) -> Result<(), ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map(|_| ())
        .map_err(|error| {
            ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
        })
}

fn validate_value(value: &BevySceneValueDescriptor, context: &str) -> Result<(), ErrorData> {
    parse_path(&value.type_path, &format!("{context} type path"))?;
    parse_expr(&value.value_expr, &format!("{context} value expression"))
}

fn validate_add_entity_params(params: &BevySceneAddEntityParams) -> Result<(), ErrorData> {
    if params.name.is_none() && params.components.is_empty() {
        return Err(ErrorData::invalid_params(
            "scene entities must include a name or at least one component",
            None,
        ));
    }
    for component in &params.components {
        validate_value(component, "component")?;
    }
    Ok(())
}

fn validate_add_resource_params(params: &BevySceneAddResourceParams) -> Result<(), ErrorData> {
    validate_value(&params.resource, "resource")
}

fn validate_spawn_code_params(params: &BevySceneEmitSpawnCodeParams) -> Result<(), ErrorData> {
    parse_expr(&params.commands_var, "commands receiver")
}

fn ron_string(value: &str) -> String {
    format!("{value:?}")
}

fn emit_ron_value(value: &BevySceneValueDescriptor, indent: &str) -> String {
    format!(
        "{indent}(\n{indent}    type_path: {},\n{indent}    value_expr: {},\n{indent}),",
        ron_string(&value.type_path),
        ron_string(&value.value_expr),
    )
}

fn emit_scene_ron(scene: &BevySceneDescriptor) -> String {
    let mut out = String::new();
    out.push_str("(\n");
    out.push_str(&format!("    name: {},\n", ron_string(&scene.name)));
    out.push_str("    resources: [\n");
    for resource in &scene.resources {
        out.push_str(&emit_ron_value(resource, "        "));
        out.push('\n');
    }
    out.push_str("    ],\n");
    out.push_str("    entities: [\n");
    for entity in &scene.entities {
        out.push_str("        (\n");
        match &entity.name {
            Some(name) => out.push_str(&format!("            name: Some({}),\n", ron_string(name))),
            None => out.push_str("            name: None,\n"),
        }
        out.push_str("            components: [\n");
        for component in &entity.components {
            out.push_str(&emit_ron_value(component, "                "));
            out.push('\n');
        }
        out.push_str("            ],\n");
        out.push_str("        ),\n");
    }
    out.push_str("    ],\n");
    out.push_str(")\n");
    out
}

fn emit_spawn_tuple(entity: &BevySceneEntityDescriptor) -> String {
    let mut items = Vec::new();
    if let Some(name) = &entity.name {
        items.push(format!("::bevy::prelude::Name::new({name:?})"));
    }
    for component in &entity.components {
        items.push(component.value_expr.clone());
    }

    if items.len() == 1 {
        format!("({},)", items[0])
    } else {
        let joined = items
            .into_iter()
            .map(|item| format!("        {item},"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("(\n{joined}\n    )")
    }
}

fn emit_spawn_code(scene: &BevySceneDescriptor, commands_var: &str) -> String {
    let mut lines = vec![format!("// Scene: {}", scene.name)];
    for resource in &scene.resources {
        lines.push(format!(
            "{commands_var}.insert_resource({});",
            resource.value_expr
        ));
    }
    for entity in &scene.entities {
        lines.push(format!(
            "{commands_var}.spawn({});",
            emit_spawn_tuple(entity)
        ));
    }
    lines.join("\n")
}

#[elicit_tool(
    plugin = "bevy_scene",
    name = "new",
    description = "Create a new Bevy scene descriptor and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_scene(
    ctx: Arc<BevySceneCtx>,
    p: BevySceneNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let descriptor = BevySceneDescriptor {
        name: p.name,
        resources: Vec::new(),
        entities: Vec::new(),
    };
    ctx.lock_items()?.insert(id, descriptor);
    Ok(json_result(&BevySceneNewResult {
        scene_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_scene",
    name = "add_entity",
    description = "Add an entity with optional name and component expressions to a stored scene descriptor.",
    emit = None
)]
#[instrument(skip_all)]
async fn add_entity(
    ctx: Arc<BevySceneCtx>,
    p: BevySceneAddEntityParams,
) -> Result<CallToolResult, ErrorData> {
    validate_add_entity_params(&p)?;
    let id = parse_id(&p.scene_id)?;
    let mut items = ctx.lock_items()?;
    let scene = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown scene id: {id}"), None))?;
    scene.entities.push(BevySceneEntityDescriptor {
        name: p.name,
        components: p.components,
    });
    Ok(ok_text("entity added"))
}

#[elicit_tool(
    plugin = "bevy_scene",
    name = "add_resource",
    description = "Add a resource expression to a stored scene descriptor.",
    emit = None
)]
#[instrument(skip_all)]
async fn add_resource(
    ctx: Arc<BevySceneCtx>,
    p: BevySceneAddResourceParams,
) -> Result<CallToolResult, ErrorData> {
    validate_add_resource_params(&p)?;
    let id = parse_id(&p.scene_id)?;
    let mut items = ctx.lock_items()?;
    let scene = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown scene id: {id}"), None))?;
    scene.resources.push(p.resource);
    Ok(ok_text("resource added"))
}

#[elicit_tool(
    plugin = "bevy_scene",
    name = "emit_ron",
    description = "Emit the stored scene descriptor as a RON manifest.",
    emit = None
)]
#[instrument(skip_all)]
async fn emit_ron(
    ctx: Arc<BevySceneCtx>,
    p: BevySceneEmitRonParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.scene_id)?;
    let items = ctx.lock_items()?;
    let scene = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown scene id: {id}"), None))?;
    Ok(ok_text(emit_scene_ron(scene)))
}

#[elicit_tool(
    plugin = "bevy_scene",
    name = "emit_spawn_code",
    description = "Emit `Commands` code that inserts the scene resources and spawns the described entities.",
    emit = None
)]
#[instrument(skip_all)]
async fn emit_spawn_code_tool(
    ctx: Arc<BevySceneCtx>,
    p: BevySceneEmitSpawnCodeParams,
) -> Result<CallToolResult, ErrorData> {
    validate_spawn_code_params(&p)?;
    let id = parse_id(&p.scene_id)?;
    let items = ctx.lock_items()?;
    let scene = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown scene id: {id}"), None))?;
    Ok(ok_text(emit_spawn_code(scene, &p.commands_var)))
}

/// MCP plugin providing `bevy_scene__*` tools for Bevy scene descriptors.
#[derive(Debug)]
pub struct BevyScenePlugin(Arc<BevySceneCtx>);

impl BevyScenePlugin {
    /// Creates a new `BevyScenePlugin` with an empty registry.
    #[instrument]
    pub fn new() -> Self {
        Self(Arc::new(BevySceneCtx::new()))
    }

    /// Returns a shared reference to the underlying scene registry context.
    #[instrument(skip(self))]
    pub fn ctx(&self) -> Arc<BevySceneCtx> {
        Arc::clone(&self.0)
    }

    /// Convenience helper for tests and direct integration.
    #[instrument(skip(self, args))]
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let bare = name
            .strip_prefix("bevy_scene__")
            .unwrap_or(name)
            .to_string();
        let params = if let Some(map) = args.as_object().cloned() {
            CallToolRequestParams::new(bare.clone()).with_arguments(map)
        } else {
            CallToolRequestParams::new(bare.clone())
        };
        let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_scene")
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

impl Default for BevyScenePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for BevyScenePlugin {
    type Context = BevySceneCtx;

    fn name(&self) -> &'static str {
        "bevy_scene"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_scene")
            .map(|registration| (registration.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_scene")
            .map(|registration| (registration.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}
