//! `BevyRenderWorkflowPlugin` — stateful render-authoring workflows.
//!
//! This plugin stores camera descriptors server-side and emits Bevy `Commands`
//! spawn code on demand. It is intended for authoring flows where the exact
//! source shape depends on a sequence of incremental choices rather than a
//! single literal value.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::render_plugin::{
    validate_orthographic_projection, validate_perspective_projection, validate_render_target,
};
use crate::{
    BevyOrthographicProjectionParams, BevyPerspectiveProjectionParams, BevyRenderTargetParams,
};
use elicitation::{
    PluginContext, PluginToolRegistration, StatefulPlugin, ToolDescriptor, elicit_tool,
    emit_code::EmitCode,
};
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

/// Supported stored camera workflow kinds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyRenderWorkflowCameraKind {
    /// A `Camera3d` spawn workflow.
    #[serde(rename = "camera_3d")]
    Camera3d,
    /// A `Camera2d` spawn workflow.
    #[serde(rename = "camera_2d")]
    Camera2d,
}

/// Stored state for a stateful render camera workflow.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowCameraDescriptor {
    /// The authored camera kind.
    pub kind: BevyRenderWorkflowCameraKind,
    /// Optional human-readable label used in emitted comments.
    #[serde(default)]
    pub name: Option<String>,
    /// Optional transform expression inserted into the spawn tuple.
    #[serde(default)]
    pub transform_expr: Option<String>,
    /// Optional explicit `Camera { hdr: ... }` override.
    #[serde(default)]
    pub hdr: Option<bool>,
    /// Optional render-target override.
    #[serde(default)]
    pub render_target: Option<BevyRenderTargetParams>,
    /// Optional tonemapping expression.
    #[serde(default)]
    pub tonemapping_expr: Option<String>,
    /// Optional stored perspective projection for 3D cameras.
    #[serde(default)]
    pub perspective: Option<BevyPerspectiveProjectionParams>,
    /// Optional stored orthographic projection for 2D cameras.
    #[serde(default)]
    pub orthographic: Option<BevyOrthographicProjectionParams>,
}

/// Shared registry context for `bevy_render_workflow__*` tools.
#[derive(Debug)]
pub struct BevyRenderWorkflowCtx {
    items: Mutex<HashMap<Uuid, BevyRenderWorkflowCameraDescriptor>>,
}

impl BevyRenderWorkflowCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn lock_items(
        &self,
    ) -> Result<MutexGuard<'_, HashMap<Uuid, BevyRenderWorkflowCameraDescriptor>>, ErrorData> {
        self.items.lock().map_err(|_| {
            ErrorData::internal_error("bevy_render_workflow registry lock poisoned", None)
        })
    }
}

impl PluginContext for BevyRenderWorkflowCtx {}

/// Shared params for creating a stored camera workflow.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowNewCameraParams {
    /// Optional human-readable label for the stored camera workflow.
    #[serde(default)]
    pub name: Option<String>,
}

/// Result returned by `bevy_render_workflow__new_camera_*`.
#[derive(Debug, Serialize)]
pub struct BevyRenderWorkflowNewResult {
    /// UUID handle for the stored camera descriptor.
    pub camera_id: String,
}

/// Parameters for `bevy_render_workflow__set_transform`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetTransformParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
    /// Optional transform expression. Use `null` to clear it.
    #[serde(default)]
    pub transform_expr: Option<String>,
}

/// Parameters for `bevy_render_workflow__set_hdr`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetHdrParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
    /// Optional HDR override. Use `null` to clear it.
    #[serde(default)]
    pub hdr: Option<bool>,
}

/// Parameters for `bevy_render_workflow__set_render_target`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetRenderTargetParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
    /// Optional render target override. Use `null` to restore the primary window.
    #[serde(default)]
    pub render_target: Option<BevyRenderTargetParams>,
}

/// Parameters for `bevy_render_workflow__set_tonemapping`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetTonemappingParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
    /// Optional tonemapping expression. Use `null` to clear it.
    #[serde(default)]
    pub tonemapping_expr: Option<String>,
}

/// Parameters for `bevy_render_workflow__set_perspective_projection`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetPerspectiveProjectionParams {
    /// UUID returned by `new_camera_3d`.
    pub camera_id: String,
    /// Perspective projection descriptor stored for the camera.
    pub projection: BevyPerspectiveProjectionParams,
}

/// Parameters for `bevy_render_workflow__set_orthographic_projection`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowSetOrthographicProjectionParams {
    /// UUID returned by `new_camera_2d`.
    pub camera_id: String,
    /// Orthographic projection descriptor stored for the camera.
    pub projection: BevyOrthographicProjectionParams,
}

/// Parameters for `bevy_render_workflow__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowDescribeParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
}

/// Parameters for `bevy_render_workflow__emit_spawn_code`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderWorkflowEmitSpawnCodeParams {
    /// UUID returned by `new_camera_3d` or `new_camera_2d`.
    pub camera_id: String,
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

fn validate_set_transform_params(
    params: &BevyRenderWorkflowSetTransformParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.transform_expr, "transform")
}

fn validate_set_render_target_params(
    params: &BevyRenderWorkflowSetRenderTargetParams,
) -> Result<(), ErrorData> {
    if let Some(render_target) = &params.render_target {
        validate_render_target(render_target)?;
    }
    Ok(())
}

fn validate_set_tonemapping_params(
    params: &BevyRenderWorkflowSetTonemappingParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.tonemapping_expr, "tonemapping")
}

fn validate_emit_spawn_code_params(
    params: &BevyRenderWorkflowEmitSpawnCodeParams,
) -> Result<(), ErrorData> {
    parse_expr(&params.commands_var, "commands receiver")
}

fn ensure_camera_kind(
    camera: &BevyRenderWorkflowCameraDescriptor,
    expected: BevyRenderWorkflowCameraKind,
    context: &str,
) -> Result<(), ErrorData> {
    if camera.kind != expected {
        return Err(ErrorData::invalid_params(
            format!("{context} is only valid for {:?} workflows", expected),
            None,
        ));
    }
    Ok(())
}

fn emit_camera_spawn_tuple(camera: &BevyRenderWorkflowCameraDescriptor) -> String {
    let mut items = Vec::new();
    match camera.kind {
        BevyRenderWorkflowCameraKind::Camera3d => {
            items.push("::bevy::camera::Camera3d::default()".to_string());
            let projection = camera
                .perspective
                .as_ref()
                .map(|projection| projection.emit_code().to_string())
                .unwrap_or_else(|| "::bevy::camera::PerspectiveProjection::default()".to_string());
            items.push(format!(
                "::bevy::camera::Projection::Perspective({projection})"
            ));
        }
        BevyRenderWorkflowCameraKind::Camera2d => {
            items.push("::bevy::camera::Camera2d::default()".to_string());
            let projection = camera
                .orthographic
                .as_ref()
                .map(|projection| projection.emit_code().to_string())
                .unwrap_or_else(|| {
                    "::bevy::camera::OrthographicProjection::default_2d()".to_string()
                });
            items.push(format!(
                "::bevy::camera::Projection::Orthographic({projection})"
            ));
        }
    }

    let render_target = camera
        .render_target
        .as_ref()
        .map(|target| target.emit_code().to_string())
        .unwrap_or_else(|| {
            "::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary)".to_string()
        });
    items.push(render_target);

    if let Some(tonemapping_expr) = &camera.tonemapping_expr {
        items.push(tonemapping_expr.clone());
    }
    if let Some(hdr) = camera.hdr {
        items.push(format!(
            "::bevy::camera::Camera {{ hdr: {hdr}, ..::std::default::Default::default() }}"
        ));
    }
    if let Some(transform_expr) = &camera.transform_expr {
        items.push(transform_expr.clone());
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

fn emit_spawn_code(camera: &BevyRenderWorkflowCameraDescriptor, commands_var: &str) -> String {
    let tuple = emit_camera_spawn_tuple(camera);
    let mut lines = Vec::new();
    if let Some(name) = &camera.name {
        lines.push(format!("// Render camera: {name}"));
    }
    lines.push(format!("{commands_var}.spawn({tuple});"));
    lines.join("\n")
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "new_camera_3d",
    description = "Create a stored 3D camera workflow and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_camera_3d(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowNewCameraParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(
        id,
        BevyRenderWorkflowCameraDescriptor {
            kind: BevyRenderWorkflowCameraKind::Camera3d,
            name: p.name,
            transform_expr: None,
            hdr: None,
            render_target: None,
            tonemapping_expr: None,
            perspective: None,
            orthographic: None,
        },
    );
    Ok(json_result(&BevyRenderWorkflowNewResult {
        camera_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "new_camera_2d",
    description = "Create a stored 2D camera workflow and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_camera_2d(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowNewCameraParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(
        id,
        BevyRenderWorkflowCameraDescriptor {
            kind: BevyRenderWorkflowCameraKind::Camera2d,
            name: p.name,
            transform_expr: None,
            hdr: None,
            render_target: None,
            tonemapping_expr: None,
            perspective: None,
            orthographic: None,
        },
    );
    Ok(json_result(&BevyRenderWorkflowNewResult {
        camera_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_transform",
    description = "Set or clear the transform expression for a stored camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_transform(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowSetTransformParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_transform_params(&p)?;
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    camera.transform_expr = p.transform_expr;
    Ok(ok_text("transform updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_hdr",
    description = "Set or clear the explicit HDR override for a stored camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_hdr(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowSetHdrParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    camera.hdr = p.hdr;
    Ok(ok_text("hdr updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_render_target",
    description = "Set or clear the render target for a stored camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_render_target(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowSetRenderTargetParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_render_target_params(&p)?;
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    camera.render_target = p.render_target;
    Ok(ok_text("render target updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_tonemapping",
    description = "Set or clear the tonemapping expression for a stored camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_tonemapping(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowSetTonemappingParams,
) -> Result<CallToolResult, ErrorData> {
    validate_set_tonemapping_params(&p)?;
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    camera.tonemapping_expr = p.tonemapping_expr;
    Ok(ok_text("tonemapping updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_perspective_projection",
    description = "Set the perspective projection stored for a 3D camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_perspective_projection(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowSetPerspectiveProjectionParams,
) -> Result<CallToolResult, ErrorData> {
    validate_perspective_projection(&p.projection)?;
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    ensure_camera_kind(
        camera,
        BevyRenderWorkflowCameraKind::Camera3d,
        "set_perspective_projection",
    )?;
    camera.perspective = Some(p.projection);
    Ok(ok_text("perspective projection updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "set_orthographic_projection",
    description = "Set the orthographic projection stored for a 2D camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_orthographic_projection(
    ctx: Arc<BevyRenderWorkflowCtx>,
    mut p: BevyRenderWorkflowSetOrthographicProjectionParams,
) -> Result<CallToolResult, ErrorData> {
    if p.projection.fields.use_2d_defaults.is_none() {
        p.projection.fields.use_2d_defaults = Some(true);
    }
    validate_orthographic_projection(&p.projection)?;
    let id = parse_id(&p.camera_id)?;
    let mut items = ctx.lock_items()?;
    let camera = items
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    ensure_camera_kind(
        camera,
        BevyRenderWorkflowCameraKind::Camera2d,
        "set_orthographic_projection",
    )?;
    camera.orthographic = Some(p.projection);
    Ok(ok_text("orthographic projection updated"))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "describe",
    description = "Return the stored camera workflow descriptor as JSON.",
    emit = None
)]
#[instrument(skip_all)]
async fn describe(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.camera_id)?;
    let items = ctx.lock_items()?;
    let camera = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    Ok(json_result(camera))
}

#[elicit_tool(
    plugin = "bevy_render_workflow",
    name = "emit_spawn_code",
    description = "Emit `Commands` spawn code for a stored camera workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn emit_spawn_code_tool(
    ctx: Arc<BevyRenderWorkflowCtx>,
    p: BevyRenderWorkflowEmitSpawnCodeParams,
) -> Result<CallToolResult, ErrorData> {
    validate_emit_spawn_code_params(&p)?;
    let id = parse_id(&p.camera_id)?;
    let items = ctx.lock_items()?;
    let camera = items
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown camera id: {id}"), None))?;
    Ok(ok_text(emit_spawn_code(camera, &p.commands_var)))
}

/// MCP plugin providing `bevy_render_workflow__*` stateful camera-authoring tools.
#[derive(Debug)]
pub struct BevyRenderWorkflowPlugin(Arc<BevyRenderWorkflowCtx>);

impl BevyRenderWorkflowPlugin {
    /// Creates a new `BevyRenderWorkflowPlugin` with an empty registry.
    #[instrument]
    pub fn new() -> Self {
        Self(Arc::new(BevyRenderWorkflowCtx::new()))
    }

    /// Returns a shared reference to the underlying workflow registry context.
    #[instrument(skip(self))]
    pub fn ctx(&self) -> Arc<BevyRenderWorkflowCtx> {
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
            .strip_prefix("bevy_render_workflow__")
            .unwrap_or(name)
            .to_string();
        let params = if let Some(map) = args.as_object().cloned() {
            CallToolRequestParams::new(bare.clone()).with_arguments(map)
        } else {
            CallToolRequestParams::new(bare.clone())
        };
        let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_workflow")
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

impl Default for BevyRenderWorkflowPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for BevyRenderWorkflowPlugin {
    type Context = BevyRenderWorkflowCtx;

    fn name(&self) -> &'static str {
        "bevy_render_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_workflow")
            .map(|registration| (registration.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_workflow")
            .map(|registration| (registration.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}
