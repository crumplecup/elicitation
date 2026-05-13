//! `WgpuComputePlugin` — code-generation tools for wgpu compute pass setup.
//!
//! # Tool namespace: `wgpu_compute__*`
//!
//! | Tool | Emits |
//! |------|-------|
//! | `pipeline_desc` | `ComputePipelineDescriptor` code |
//! | `dispatch` | `compute_pass.dispatch_workgroups(x, y, z)` |
//! | `bind_group_layout_entry` | `wgpu::BindGroupLayoutEntry { ... }` |

use elicitation::{Prop, VerifiedWorkflow, WgpuShaderStages, elicit_tool};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: wgpu compute pass code was generated.
#[derive(Prop)]
pub struct WgpuComputeDispatched;

impl VerifiedWorkflow for WgpuComputeDispatched {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `wgpu_compute__pipeline_desc`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ComputePipelineDescParams {
    /// Debug label.
    pub label: Option<String>,
    /// Variable name of the shader module.
    pub module_var: String,
    /// Compute entry point name.
    pub entry_point: String,
}

/// Parameters for `wgpu_compute__dispatch`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DispatchParams {
    /// Workgroup count in X dimension.
    pub x: u32,
    /// Workgroup count in Y dimension.
    pub y: u32,
    /// Workgroup count in Z dimension.
    pub z: u32,
}

/// Parameters for `wgpu_compute__bind_group_layout_entry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BindGroupLayoutEntryParams {
    /// Binding slot index.
    pub binding: u32,
    /// Shader stage visibility.
    pub visibility: WgpuShaderStages,
    /// Whether this is a storage buffer (vs uniform).
    pub is_storage: bool,
    /// Whether the storage buffer is read-only.
    pub read_only: bool,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text(s: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

fn label_opt(opt: &Option<String>) -> String {
    match opt {
        Some(s) => format!("Some({s:?})"),
        None => "None".to_string(),
    }
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "wgpu_compute",
    name = "wgpu_compute__pipeline_desc",
    description = "Generate a `wgpu::ComputePipelineDescriptor` code block. \
                   Pass to `device.create_compute_pipeline(&desc)` to build the pipeline."
)]
#[instrument(skip(p))]
async fn compute_pipeline_desc(p: ComputePipelineDescParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    text(format!(
        "let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {{\n\
         \x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20layout: None,\n\
         \x20\x20\x20\x20module: &{module},\n\
         \x20\x20\x20\x20entry_point: Some({entry:?}),\n\
         \x20\x20\x20\x20compilation_options: wgpu::PipelineCompilationOptions::default(),\n\
         \x20\x20\x20\x20cache: None,\n\
         }});",
        module = p.module_var,
        entry = p.entry_point,
    ))
}

#[elicit_tool(
    plugin = "wgpu_compute",
    name = "wgpu_compute__dispatch",
    description = "Generate a `compute_pass.dispatch_workgroups(x, y, z)` call for the given \
                   workgroup grid dimensions."
)]
#[instrument(skip(p))]
async fn compute_dispatch(p: DispatchParams) -> Result<CallToolResult, ErrorData> {
    text(format!(
        "compute_pass.dispatch_workgroups({}, {}, {});",
        p.x, p.y, p.z
    ))
}

#[elicit_tool(
    plugin = "wgpu_compute",
    name = "wgpu_compute__bind_group_layout_entry",
    description = "Generate a `wgpu::BindGroupLayoutEntry` for a buffer binding. \
                   Generates either a uniform or storage buffer entry."
)]
#[instrument(skip(p))]
async fn compute_bind_group_layout_entry(
    p: BindGroupLayoutEntryParams,
) -> Result<CallToolResult, ErrorData> {
    let vis = format!("wgpu::ShaderStages::{:?}", *p.visibility);
    let ty = if p.is_storage {
        format!(
            "wgpu::BindingType::Buffer {{\n\
             \x20\x20\x20\x20\x20\x20\x20\x20ty: wgpu::BufferBindingType::Storage {{ read_only: {} }},\n\
             \x20\x20\x20\x20\x20\x20\x20\x20has_dynamic_offset: false,\n\
             \x20\x20\x20\x20\x20\x20\x20\x20min_binding_size: None,\n\
             \x20\x20\x20\x20}}",
            p.read_only
        )
    } else {
        "wgpu::BindingType::Buffer {\n\
             \x20\x20\x20\x20\x20\x20\x20\x20ty: wgpu::BufferBindingType::Uniform,\n\
             \x20\x20\x20\x20\x20\x20\x20\x20has_dynamic_offset: false,\n\
             \x20\x20\x20\x20\x20\x20\x20\x20min_binding_size: None,\n\
             \x20\x20\x20\x20}"
            .to_string()
    };
    text(format!(
        "wgpu::BindGroupLayoutEntry {{\n\
         \x20\x20\x20\x20binding: {},\n\
         \x20\x20\x20\x20visibility: {vis},\n\
         \x20\x20\x20\x20ty: {ty},\n\
         \x20\x20\x20\x20count: None,\n\
         }}",
        p.binding
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: wgpu compute pass and pipeline code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wgpu_compute")]
pub struct WgpuComputePlugin;

impl WgpuComputePlugin {
    /// Create a new compute plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }

    /// Convenience dispatcher for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        use elicitation::{NoContext, PluginToolRegistration, inventory};
        let found = inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "wgpu_compute")
            .find(|r| r.name == name)
            .map(|r| (r.constructor)());
        let params = if let Some(m) = args.as_object().cloned() {
            rmcp::model::CallToolRequestParams::new(name.to_string()).with_arguments(m)
        } else {
            rmcp::model::CallToolRequestParams::new(name.to_string())
        };
        match found {
            Some(descriptor) => {
                descriptor
                    .dispatch(std::sync::Arc::new(NoContext), params)
                    .await
            }
            None => Err(rmcp::ErrorData::invalid_params(
                format!("unknown tool: {name}"),
                None,
            )),
        }
    }
}

impl Default for WgpuComputePlugin {
    fn default() -> Self {
        Self::new()
    }
}
