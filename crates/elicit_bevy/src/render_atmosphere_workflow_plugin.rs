//! `BevyRenderAtmosphereWorkflowPlugin` — stateful atmosphere authoring.
//!
//! This plugin stores atmosphere descriptors server-side and emits the same
//! block expression as `bevy_render__atmosphere` on demand. It is intended for
//! authoring flows where scattering terms are accumulated incrementally.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::render_plugin::{BevyAtmosphereParams, validate_atmosphere};
use elicitation::{
    BevyAtmosphere, BevyScatteringTerm, PluginContext, PluginToolRegistration, StatefulPlugin,
    ToolDescriptor, elicit_tool, emit_code::EmitCode,
};
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

/// Shared registry context for `bevy_render_atmosphere_workflow__*` tools.
#[derive(Debug)]
pub struct BevyRenderAtmosphereWorkflowCtx {
    items: Mutex<HashMap<Uuid, BevyAtmosphereParams>>,
}

impl BevyRenderAtmosphereWorkflowCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    fn lock_items(&self) -> Result<MutexGuard<'_, HashMap<Uuid, BevyAtmosphereParams>>, ErrorData> {
        self.items.lock().map_err(|_| {
            ErrorData::internal_error(
                "bevy_render_atmosphere_workflow registry lock poisoned",
                None,
            )
        })
    }
}

impl PluginContext for BevyRenderAtmosphereWorkflowCtx {}

/// Parameters for `bevy_render_atmosphere_workflow__new_atmosphere`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowNewParams {}

/// Result returned by `bevy_render_atmosphere_workflow__new_atmosphere`.
#[derive(Debug, Serialize)]
pub struct BevyRenderAtmosphereWorkflowNewResult {
    /// UUID handle for the stored atmosphere descriptor.
    pub atmosphere_id: String,
}

/// Parameters for `bevy_render_atmosphere_workflow__set_scattering_media_var`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowSetScatteringMediaVarParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// `Assets<ScatteringMedium>` resource expression used to add the generated medium.
    pub scattering_media_var: String,
}

/// Parameters for `bevy_render_atmosphere_workflow__set_atmosphere`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowSetAtmosphereParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Optional explicit atmosphere component fields. Use `null` to restore `Atmosphere::earthlike`.
    #[serde(default)]
    pub atmosphere: Option<BevyAtmosphere>,
}

/// Parameters for `bevy_render_atmosphere_workflow__set_medium_label`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowSetMediumLabelParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Optional label applied to the generated scattering medium. Use `null` to clear it.
    #[serde(default)]
    pub medium_label: Option<String>,
}

/// Parameters for `bevy_render_atmosphere_workflow__set_resolutions`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowSetResolutionsParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Resolution used to sample each term's falloff distribution.
    pub falloff_resolution: u32,
    /// Resolution used to sample each term's phase function.
    pub phase_resolution: u32,
}

/// Parameters for `bevy_render_atmosphere_workflow__set_density_multiplier`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowSetDensityMultiplierParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Optional density multiplier applied after building the medium. Use `null` to clear it.
    #[serde(default)]
    pub density_multiplier: Option<f32>,
}

/// Parameters for `bevy_render_atmosphere_workflow__add_term`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowAddTermParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Scattering term appended to the stored medium.
    pub term: BevyScatteringTerm,
}

/// Parameters for `bevy_render_atmosphere_workflow__replace_term`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowReplaceTermParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Zero-based term index to replace.
    pub index: usize,
    /// Replacement scattering term.
    pub term: BevyScatteringTerm,
}

/// Parameters for `bevy_render_atmosphere_workflow__remove_term`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowRemoveTermParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
    /// Zero-based term index to remove.
    pub index: usize,
}

/// Parameters for `bevy_render_atmosphere_workflow__clear_terms`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowClearTermsParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
}

/// Parameters for `bevy_render_atmosphere_workflow__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowDescribeParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
}

/// Parameters for `bevy_render_atmosphere_workflow__emit_code`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BevyRenderAtmosphereWorkflowEmitCodeParams {
    /// UUID returned by `new_atmosphere`.
    pub atmosphere_id: String,
}

fn default_descriptor() -> BevyAtmosphereParams {
    BevyAtmosphereParams {
        scattering_media_var: "scattering_media".to_string(),
        atmosphere: None,
        medium_label: None,
        falloff_resolution: 256,
        phase_resolution: 256,
        terms: Vec::new(),
        density_multiplier: None,
    }
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

fn load_descriptor<'a>(
    items: &'a HashMap<Uuid, BevyAtmosphereParams>,
    id: &Uuid,
) -> Result<&'a BevyAtmosphereParams, ErrorData> {
    items
        .get(id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown atmosphere id: {id}"), None))
}

fn load_descriptor_mut<'a>(
    items: &'a mut HashMap<Uuid, BevyAtmosphereParams>,
    id: &Uuid,
) -> Result<&'a mut BevyAtmosphereParams, ErrorData> {
    items
        .get_mut(id)
        .ok_or_else(|| ErrorData::invalid_params(format!("unknown atmosphere id: {id}"), None))
}

fn validate_updated_descriptor(descriptor: &BevyAtmosphereParams) -> Result<(), ErrorData> {
    validate_atmosphere(descriptor)
}

fn checked_replacement<F>(
    current: &BevyAtmosphereParams,
    update: F,
) -> Result<BevyAtmosphereParams, ErrorData>
where
    F: FnOnce(&mut BevyAtmosphereParams) -> Result<(), ErrorData>,
{
    let mut next = current.clone();
    update(&mut next)?;
    validate_updated_descriptor(&next)?;
    Ok(next)
}

fn replace_term(
    descriptor: &BevyAtmosphereParams,
    index: usize,
    term: BevyScatteringTerm,
) -> Result<BevyAtmosphereParams, ErrorData> {
    checked_replacement(descriptor, |next| {
        if index >= next.terms.len() {
            return Err(ErrorData::invalid_params(
                format!(
                    "term index {index} is out of bounds for {} stored terms",
                    next.terms.len()
                ),
                None,
            ));
        }
        next.terms[index] = term;
        Ok(())
    })
}

fn remove_term(
    descriptor: &BevyAtmosphereParams,
    index: usize,
) -> Result<BevyAtmosphereParams, ErrorData> {
    checked_replacement(descriptor, |next| {
        if index >= next.terms.len() {
            return Err(ErrorData::invalid_params(
                format!(
                    "term index {index} is out of bounds for {} stored terms",
                    next.terms.len()
                ),
                None,
            ));
        }
        next.terms.remove(index);
        Ok(())
    })
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "new_atmosphere",
    description = "Create a stored atmosphere workflow and return its UUID handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn new_atmosphere(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowNewParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    let id = Uuid::new_v4();
    ctx.lock_items()?.insert(id, default_descriptor());
    Ok(json_result(&BevyRenderAtmosphereWorkflowNewResult {
        atmosphere_id: id.to_string(),
    }))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "set_scattering_media_var",
    description = "Set the `Assets<ScatteringMedium>` expression used by a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_scattering_media_var(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowSetScatteringMediaVarParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.scattering_media_var = p.scattering_media_var;
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("scattering media updated"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "set_atmosphere",
    description = "Set or clear the explicit `Atmosphere` component fields for a stored workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_atmosphere(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowSetAtmosphereParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.atmosphere = p.atmosphere;
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("atmosphere updated"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "set_medium_label",
    description = "Set or clear the generated scattering-medium label for a stored workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_medium_label(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowSetMediumLabelParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.medium_label = p.medium_label;
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("medium label updated"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "set_resolutions",
    description = "Set the falloff and phase resolutions for a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_resolutions(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowSetResolutionsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.falloff_resolution = p.falloff_resolution;
        descriptor.phase_resolution = p.phase_resolution;
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("resolutions updated"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "set_density_multiplier",
    description = "Set or clear the density multiplier for a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn set_density_multiplier(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowSetDensityMultiplierParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.density_multiplier = p.density_multiplier;
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("density multiplier updated"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "add_term",
    description = "Append a scattering term to a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn add_term(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowAddTermParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.terms.push(p.term);
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("term added"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "replace_term",
    description = "Replace one scattering term in a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn replace_term_tool(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowReplaceTermParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = replace_term(current, p.index, p.term)?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("term replaced"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "remove_term",
    description = "Remove one scattering term from a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn remove_term_tool(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowRemoveTermParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = remove_term(current, p.index)?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("term removed"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "clear_terms",
    description = "Clear all custom scattering terms from a stored atmosphere workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn clear_terms(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowClearTermsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let mut items = ctx.lock_items()?;
    let current = load_descriptor(&items, &id)?;
    let next = checked_replacement(current, |descriptor| {
        descriptor.terms.clear();
        Ok(())
    })?;
    *load_descriptor_mut(&mut items, &id)? = next;
    Ok(ok_text("terms cleared"))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "describe",
    description = "Return the stored atmosphere workflow descriptor as JSON.",
    emit = None
)]
#[instrument(skip_all)]
async fn describe(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let items = ctx.lock_items()?;
    let descriptor = load_descriptor(&items, &id)?;
    validate_updated_descriptor(descriptor)?;
    Ok(json_result(descriptor))
}

#[elicit_tool(
    plugin = "bevy_render_atmosphere_workflow",
    name = "emit_code",
    description = "Emit the Bevy atmosphere block expression for a stored workflow.",
    emit = None
)]
#[instrument(skip_all)]
async fn emit_code_tool(
    ctx: Arc<BevyRenderAtmosphereWorkflowCtx>,
    p: BevyRenderAtmosphereWorkflowEmitCodeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.atmosphere_id)?;
    let items = ctx.lock_items()?;
    let descriptor = load_descriptor(&items, &id)?;
    validate_updated_descriptor(descriptor)?;
    Ok(ok_text(descriptor.emit_code().to_string()))
}

/// MCP plugin providing `bevy_render_atmosphere_workflow__*` stateful atmosphere tools.
#[derive(Debug)]
pub struct BevyRenderAtmosphereWorkflowPlugin(Arc<BevyRenderAtmosphereWorkflowCtx>);

impl BevyRenderAtmosphereWorkflowPlugin {
    /// Creates a new `BevyRenderAtmosphereWorkflowPlugin` with an empty registry.
    #[instrument]
    pub fn new() -> Self {
        Self(Arc::new(BevyRenderAtmosphereWorkflowCtx::new()))
    }

    /// Returns a shared reference to the underlying atmosphere workflow registry context.
    #[instrument(skip(self))]
    pub fn ctx(&self) -> Arc<BevyRenderAtmosphereWorkflowCtx> {
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
            .strip_prefix("bevy_render_atmosphere_workflow__")
            .unwrap_or(name)
            .to_string();
        let params = if let Some(map) = args.as_object().cloned() {
            CallToolRequestParams::new(bare.clone()).with_arguments(map)
        } else {
            CallToolRequestParams::new(bare.clone())
        };
        let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_atmosphere_workflow")
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

impl Default for BevyRenderAtmosphereWorkflowPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulPlugin for BevyRenderAtmosphereWorkflowPlugin {
    type Context = BevyRenderAtmosphereWorkflowCtx;

    fn name(&self) -> &'static str {
        "bevy_render_atmosphere_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_atmosphere_workflow")
            .map(|registration| (registration.constructor)().as_tool())
            .collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        elicitation::inventory::iter::<PluginToolRegistration>()
            .filter(|registration| registration.plugin == "bevy_render_atmosphere_workflow")
            .map(|registration| (registration.constructor)())
            .collect()
    }

    fn context(&self) -> Arc<Self::Context> {
        self.0.clone()
    }
}
