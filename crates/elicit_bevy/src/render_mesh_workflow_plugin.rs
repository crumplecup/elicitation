//! `BevyRenderMeshWorkflowPlugin` — stateful mesh-entity authoring workflows.
//!
//! This plugin stores mesh entity descriptors server-side and emits Bevy
//! `Commands` spawn code on demand. It covers constructor-shaped render
//! components that are awkward to treat as a single literal authoring step.

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

/// Supported stored mesh workflow kinds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyRenderMeshWorkflowKind {
    /// A 3D mesh entity workflow.
    #[serde(rename = "mesh_3d")]
    Mesh3d,
    /// A 2D mesh entity workflow.
    #[serde(rename = "mesh_2d")]
    Mesh2d,
}

/// Stored state for a mesh-entity workflow.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowDescriptor {
    /// The authored mesh workflow kind.
    pub kind: BevyRenderMeshWorkflowKind,
    /// Optional human-readable label used in emitted comments.
    #[serde(default)]
    pub name: Option<String>,
    /// Optional handle expression used for `Mesh3d` / `Mesh2d`.
    #[serde(default)]
    pub mesh_expr: Option<String>,
    /// Optional typed material handle expression.
    #[serde(default)]
    pub material_expr: Option<String>,
    /// Optional explicit wireframe material handle expression.
    #[serde(default)]
    pub wireframe_material_expr: Option<String>,
    /// Optional transform expression inserted into the spawn tuple.
    #[serde(default)]
    pub transform_expr: Option<String>,
}

/// Shared registry context for `bevy_render_mesh_workflow__*` tools.
#[derive(Debug)]
pub struct BevyRenderMeshWorkflowCtx {
    items: Mutex<HashMap<Uuid, BevyRenderMeshWorkflowDescriptor>>,
}

impl BevyRenderMeshWorkflowCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn lock_items(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, BevyRenderMeshWorkflowDescriptor>>, ErrorData> {
        self.items.lock().map_err(|_| {
            ErrorData::internal_error("bevy_render_mesh_workflow registry lock poisoned", None)
        })
    }
}

impl PluginContext for BevyRenderMeshWorkflowCtx {}

/// Shared params for creating a stored mesh workflow.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowNewParams {
    /// Optional human-readable label for the stored workflow.
    #[serde(default)]
    pub name: Option<String>,
}

/// Result returned by `bevy_render_mesh_workflow__new_mesh_*`.
#[derive(Debug, Serialize)]
pub struct BevyRenderMeshWorkflowNewResult {
    /// UUID handle for the stored mesh descriptor.
    pub mesh_id: String,
}

/// Parameters for `bevy_render_mesh_workflow__set_mesh`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowSetMeshParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
    /// Optional mesh handle expression. Use `null` to clear it.
    #[serde(default)]
    pub mesh_expr: Option<String>,
}

/// Parameters for `bevy_render_mesh_workflow__set_material`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowSetMaterialParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
    /// Optional typed material handle expression. Use `null` to clear it.
    #[serde(default)]
    pub material_expr: Option<String>,
}

/// Parameters for `bevy_render_mesh_workflow__set_wireframe_material`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowSetWireframeMaterialParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
    /// Optional wireframe material handle expression. Use `null` to clear it.
    #[serde(default)]
    pub wireframe_material_expr: Option<String>,
}

/// Parameters for `bevy_render_mesh_workflow__set_transform`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowSetTransformParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
    /// Optional transform expression. Use `null` to clear it.
    #[serde(default)]
    pub transform_expr: Option<String>,
}

/// Parameters for `bevy_render_mesh_workflow__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowDescribeParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
}

/// Parameters for `bevy_render_mesh_workflow__emit_spawn_code`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderMeshWorkflowEmitSpawnCodeParams {
    /// UUID returned by `new_mesh_3d` or `new_mesh_2d`.
    pub mesh_id: String,
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
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

fn parse_expr(src: &str, context: &str) -> Result<(), ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map(|_| ())
        .map_err(|error| {
            ErrorData::invalid_params(format!("invalid {context} `{src}`: {error}"), None)
        })
}

fn validate_optional_expr(src: &Option<String>, context: &str) -> Result<(), ErrorData> {
    if let Some(src) = src {
        parse_expr(src, context)?;
    }
    Ok(())
}

fn validate_set_mesh_params(params: &BevyRenderMeshWorkflowSetMeshParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.mesh_expr, "mesh handle")
}

fn validate_set_material_params(
    params: &BevyRenderMeshWorkflowSetMaterialParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.material_expr, "material handle")
}

fn validate_set_wireframe_material_params(
    params: &BevyRenderMeshWorkflowSetWireframeMaterialParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.wireframe_material_expr, "wireframe material handle")
}

fn validate_set_transform_params(
    params: &BevyRenderMeshWorkflowSetTransformParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.transform_expr, "transform")
}

fn validate_emit_spawn_code_params(
    params: &BevyRenderMeshWorkflowEmitSpawnCodeParams,
) -> Result<(), ErrorData> {
    parse_expr(&params.commands_var, "commands receiver")
}

fn emit_mesh_spawn_tuple(mesh: &BevyRenderMeshWorkflowDescriptor) -> Result<String, ErrorData> {
    let mesh_expr = mesh.mesh_expr.as_ref().ok_or_else(|| {
        ErrorData::invalid_params(
            "mesh workflow must include mesh_expr before emitting spawn code",
            None,
        )
    })?;
    let mut items = Vec::new();
    match mesh.kind {
        BevyRenderMeshWorkflowKind::Mesh3d => {
            items.push(format!("::bevy::mesh::Mesh3d({mesh_expr})"));
            if let Some(material_expr) = &mesh.material_expr {
                items.push(format!("::bevy::pbr::MeshMaterial3d({material_expr})"));
            }
            if let Some(wireframe_expr) = &mesh.wireframe_material_expr {
                items.push(format!("::bevy::pbr::Mesh3dWireframe({wireframe_expr})"));
            }
        }
        BevyRenderMeshWorkflowKind::Mesh2d => {
            items.push(format!("::bevy::mesh::Mesh2d({mesh_expr})"));
            if let Some(material_expr) = &mesh.material_expr {
                items.push(format!(
                    "::bevy::sprite_render::MeshMaterial2d({material_expr})"
                ));
            }
            if let Some(wireframe_expr) = &mesh.wireframe_material_expr {
                items.push(format!(
                    "::bevy::sprite_render::Mesh2dWireframe({wireframe_expr})"
                ));
            }
        }
    }
    if let Some(transform_expr) = &mesh.transform_expr {
        items.push(transform_expr.clone());
    }

    if items.len() == 1 {
        Ok(format!("({},)", items[0]))
    } else {
        let joined = items
            .into_iter()
            .map(|item| format!("        {item},"))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(format!("(\n{joined}\n    )"))
    }
}

fn emit_spawn_code(
    mesh: &BevyRenderMeshWorkflowDescriptor,
    commands_var: &str,
) -> Result<String, ErrorData> {
    let tuple = emit_mesh_spawn_tuple(mesh)?;
    let mut lines = Vec::new();
    if let Some(name) = &mesh.name {
        lines.push(format!("// Render mesh: {name}"));
    }
    lines.push(format!("{commands_var}.spawn({tuple});"));
    Ok(lines.join("\n"))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "new_mesh_3d",
    description = "Create a stored 3D mesh workflow and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_mesh_3d(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(
        id,
        BevyRenderMeshWorkflowDescriptor {
            kind: BevyRenderMeshWorkflowKind::Mesh3d,
            name: p.name,
            mesh_expr: None,
            material_expr: None,
            wireframe_material_expr: None,
            transform_expr: None,
        },
    );
    Ok(json_result(&BevyRenderMeshWorkflowNewResult {
        mesh_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "new_mesh_2d",
    description = "Create a stored 2D mesh workflow and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_mesh_2d(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(
        id,
        BevyRenderMeshWorkflowDescriptor {
            kind: BevyRenderMeshWorkflowKind::Mesh2d,
            name: p.name,
            mesh_expr: None,
            material_expr: None,
            wireframe_material_expr: None,
            transform_expr: None,
        },
    );
    Ok(json_result(&BevyRenderMeshWorkflowNewResult {
        mesh_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "set_mesh",
    description = "Set or clear the mesh handle expression for a stored mesh workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_mesh(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowSetMeshParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_mesh_params(&p)?;
    let id = parse_id(&p.mesh_id)?;
    let mut items = ctx.lock_items()?;
    let mesh = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    mesh.mesh_expr = p.mesh_expr;
    Ok(ok_text("mesh updated"))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "set_material",
    description = "Set or clear the typed material handle expression for a stored mesh workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_material(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowSetMaterialParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_material_params(&p)?;
    let id = parse_id(&p.mesh_id)?;
    let mut items = ctx.lock_items()?;
    let mesh = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    mesh.material_expr = p.material_expr;
    Ok(ok_text("material updated"))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "set_wireframe_material",
    description = "Set or clear the explicit wireframe material handle for a stored mesh workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_wireframe_material(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowSetWireframeMaterialParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_wireframe_material_params(&p)?;
    let id = parse_id(&p.mesh_id)?;
    let mut items = ctx.lock_items()?;
    let mesh = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    mesh.wireframe_material_expr = p.wireframe_material_expr;
    Ok(ok_text("wireframe material updated"))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "set_transform",
    description = "Set or clear the transform expression for a stored mesh workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_transform(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowSetTransformParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_transform_params(&p)?;
    let id = parse_id(&p.mesh_id)?;
    let mut items = ctx.lock_items()?;
    let mesh = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    mesh.transform_expr = p.transform_expr;
    Ok(ok_text("transform updated"))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "describe",
    description = "Return the stored mesh workflow descriptor as JSON.",
    emit = None
)]
#[instrument(skip_all)]
async fn describe(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.mesh_id)?;
    let items = ctx.lock_items()?;
    let mesh = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    Ok(json_result(mesh))
}

#[elicit_tool(
    plugin = "bevy_render_mesh_workflow",
    name = "emit_spawn_code",
    description = "Emit `Commands` spawn code for a stored mesh workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn emit_spawn_code_tool(
    ctx: Arc<BevyRenderMeshWorkflowCtx>,
    p: BevyRenderMeshWorkflowEmitSpawnCodeParams,
) -> Result<CallToolResult, ErrorData> {
    validate_emit_spawn_code_params(&p)?;
    let id = parse_id(&p.mesh_id)?;
    let items = ctx.lock_items()?;
    let mesh = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown mesh id: {id}"), None))?;
    Ok(ok_text(emit_spawn_code(mesh, &p.commands_var)?))
}

/// MCP plugin providing `bevy_render_mesh_workflow__*` stateful mesh-authoring tools.
#[derive(Debug)]
pub struct BevyRenderMeshWorkflowPlugin(Arc<BevyRenderMeshWorkflowCtx>);

impl BevyRenderMeshWorkflowPlugin {
    /// Creates a new `BevyRenderMeshWorkflowPlugin` with an empty registry.
    #[instrument]
    pub fn new() -> Self {
        Self(Arc::new(BevyRenderMeshWorkflowCtx::new()))
    }

    /// Returns a shared reference to the underlying mesh workflow registry context.
    #[instrument(skip(self))]
    pub fn ctx(&self) -> Arc<BevyRenderMeshWorkflowCtx> {
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
            .strip_prefix("bevy_render_mesh_workflow__")
            .unwrap_or(name)
            .to_string();
        let params = if let Some(map) = args.as_object().cloned() {
            CallToolRequestParams::new(bare.clone()).with_arguments(map)
        } else {
            CallToolRequestParams::new(bare.clone())
        };
        let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_mesh_workflow")
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

impl Default for BevyRenderMeshWorkflowPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for BevyRenderMeshWorkflowPlugin {
    type Context = BevyRenderMeshWorkflowCtx;

    fn name(&self) -> &'static str {
        "bevy_render_mesh_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_mesh_workflow")
            .map(|registration| (registration.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_mesh_workflow")
            .map(|registration| (registration.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}
