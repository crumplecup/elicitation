//! `WgpuShaderPlugin` — code-generation tools for wgpu shader module and stage descriptors.
//!
//! # Tool namespace: `wgpu_shader__*`
//!
//! | Tool | Emits |
//! |------|-------|
//! | `module_inline` | `ShaderModuleDescriptor` with inline WGSL |
//! | `vertex_state` | `wgpu::VertexState { ... }` |
//! | `fragment_state` | `wgpu::FragmentState { ... }` with color targets |

use elicitation::{Prop, VerifiedWorkflow, WgpuTextureFormat, elicit_tool};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: wgpu shader stage code was generated.
#[derive(Prop)]
pub struct WgpuShaderStaged;

impl VerifiedWorkflow for WgpuShaderStaged {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `wgpu_shader__module_inline`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ShaderModuleInlineParams {
    /// Variable name to assign the module to.
    pub var_name: String,
    /// Debug label.
    pub label: Option<String>,
    /// Inline WGSL shader source.
    pub wgsl_source: String,
}

/// Parameters for `wgpu_shader__vertex_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct VertexStateParams {
    /// Variable name of the shader module.
    pub module_var: String,
    /// Shader entry point function name.
    pub entry_point: String,
}

/// Parameters for `wgpu_shader__fragment_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FragmentStateParams {
    /// Variable name of the shader module.
    pub module_var: String,
    /// Shader entry point function name.
    pub entry_point: String,
    /// Color attachment formats for the fragment output targets.
    pub targets: Vec<WgpuTextureFormat>,
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
    plugin = "wgpu_shader",
    name = "wgpu_shader__module_inline",
    description = "Generate code to create a `wgpu::ShaderModule` from an inline WGSL string. \
                   The WGSL source is embedded via `wgpu::include_wgsl!` equivalent."
)]
#[instrument(skip(p))]
async fn shader_module_inline(p: ShaderModuleInlineParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    let source = p.wgsl_source.replace('\\', "\\\\").replace('"', "\\\"");
    text(format!(
        "let {var} = device.create_shader_module(wgpu::ShaderModuleDescriptor {{\n\
         \x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(\"{source}\")),\n\
         }});",
        var = p.var_name
    ))
}

#[elicit_tool(
    plugin = "wgpu_shader",
    name = "wgpu_shader__vertex_state",
    description = "Generate a `wgpu::VertexState` referencing the given shader module variable \
                   and entry point. Includes an empty vertex buffer layout list."
)]
#[instrument(skip(p))]
async fn shader_vertex_state(p: VertexStateParams) -> Result<CallToolResult, ErrorData> {
    text(format!(
        "wgpu::VertexState {{\n\
         \x20\x20\x20\x20module: &{module},\n\
         \x20\x20\x20\x20entry_point: Some({entry:?}),\n\
         \x20\x20\x20\x20compilation_options: wgpu::PipelineCompilationOptions::default(),\n\
         \x20\x20\x20\x20buffers: &[],\n\
         }}",
        module = p.module_var,
        entry = p.entry_point,
    ))
}

#[elicit_tool(
    plugin = "wgpu_shader",
    name = "wgpu_shader__fragment_state",
    description = "Generate a `wgpu::FragmentState` referencing the given shader module and \
                   entry point with the supplied color attachment formats as `Some(ColorTargetState)`."
)]
#[instrument(skip(p))]
async fn shader_fragment_state(p: FragmentStateParams) -> Result<CallToolResult, ErrorData> {
    let targets: Vec<String> = p
        .targets
        .iter()
        .map(|f| {
            format!(
                "Some(wgpu::ColorTargetState {{\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20format: wgpu::TextureFormat::{:?},\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20blend: Some(wgpu::BlendState::REPLACE),\n\
                 \x20\x20\x20\x20\x20\x20\x20\x20write_mask: wgpu::ColorWrites::ALL,\n\
                 \x20\x20\x20\x20}})",
                **f
            )
        })
        .collect();
    let targets_joined = targets.join(",\n\x20\x20\x20\x20\x20\x20\x20\x20");
    text(format!(
        "wgpu::FragmentState {{\n\
         \x20\x20\x20\x20module: &{module},\n\
         \x20\x20\x20\x20entry_point: Some({entry:?}),\n\
         \x20\x20\x20\x20compilation_options: wgpu::PipelineCompilationOptions::default(),\n\
         \x20\x20\x20\x20targets: &[{targets_joined}],\n\
         }}",
        module = p.module_var,
        entry = p.entry_point,
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: wgpu shader module and pipeline stage code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wgpu_shader")]
pub struct WgpuShaderPlugin;

impl WgpuShaderPlugin {
    /// Create a new shader plugin.
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
            .filter(|r| r.plugin == "wgpu_shader")
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

impl Default for WgpuShaderPlugin {
    fn default() -> Self {
        Self::new()
    }
}
