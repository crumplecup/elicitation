//! `WgpuPipelinePlugin` — code-generation tools for wgpu render pipeline descriptors.
//!
//! # Tool namespace: `wgpu_pipeline__*`
//!
//! | Tool | Emits |
//! |------|-------|
//! | `primitive_state` | `wgpu::PrimitiveState { ... }` |
//! | `blend_state` | `wgpu::BlendState { ... }` |
//! | `color_target_state` | `wgpu::ColorTargetState { ... }` |
//! | `depth_stencil_state` | `wgpu::DepthStencilState { ... }` |

use elicitation::{
    Prop, VerifiedWorkflow, WgpuBlendFactor, WgpuBlendOperation, WgpuColorWrites,
    WgpuCompareFunctionSelect, WgpuFace, WgpuFrontFace, WgpuIndexFormat, WgpuPolygonMode,
    WgpuPrimitiveTopology, WgpuTextureFormat, elicit_tool,
};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a wgpu render pipeline descriptor was generated.
#[derive(Prop)]
pub struct WgpuPipelineBuilt;

impl VerifiedWorkflow for WgpuPipelineBuilt {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `wgpu_pipeline__primitive_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PrimitiveStateParams {
    /// Primitive assembly topology.
    pub topology: WgpuPrimitiveTopology,
    /// Strip index format (required for strip topologies).
    pub strip_index_format: Option<WgpuIndexFormat>,
    /// Front face winding order.
    pub front_face: WgpuFrontFace,
    /// Face to cull (`None` disables culling).
    pub cull_mode: Option<WgpuFace>,
    /// Polygon fill mode.
    pub polygon_mode: WgpuPolygonMode,
}

/// Parameters for `wgpu_pipeline__blend_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BlendStateParams {
    /// Source blend factor.
    pub src_factor: WgpuBlendFactor,
    /// Destination blend factor.
    pub dst_factor: WgpuBlendFactor,
    /// Blend operation.
    pub operation: WgpuBlendOperation,
}

/// Parameters for `wgpu_pipeline__color_target_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ColorTargetStateParams {
    /// Attachment texture format.
    pub format: WgpuTextureFormat,
    /// Color write mask.
    pub write_mask: WgpuColorWrites,
}

/// Parameters for `wgpu_pipeline__depth_stencil_state`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DepthStencilStateParams {
    /// Depth/stencil texture format.
    pub format: WgpuTextureFormat,
    /// Whether depth writes are enabled.
    pub depth_write_enabled: bool,
    /// Depth comparison function.
    pub depth_compare: WgpuCompareFunctionSelect,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text(s: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "wgpu_pipeline",
    name = "wgpu_pipeline__primitive_state",
    description = "Generate a `wgpu::PrimitiveState` struct literal for rasterization configuration."
)]
#[instrument(skip(p))]
async fn pipeline_primitive_state(p: PrimitiveStateParams) -> Result<CallToolResult, ErrorData> {
    let topo = format!("wgpu::PrimitiveTopology::{:?}", *p.topology);
    let strip = match p.strip_index_format {
        Some(ref f) => format!("Some(wgpu::IndexFormat::{:?})", **f),
        None => "None".to_string(),
    };
    let ff = format!("wgpu::FrontFace::{:?}", *p.front_face);
    let cull = match p.cull_mode {
        Some(ref f) => format!("Some(wgpu::Face::{:?})", **f),
        None => "None".to_string(),
    };
    let poly = format!("wgpu::PolygonMode::{:?}", *p.polygon_mode);
    text(format!(
        "wgpu::PrimitiveState {{\n\
         \x20\x20\x20\x20topology: {topo},\n\
         \x20\x20\x20\x20strip_index_format: {strip},\n\
         \x20\x20\x20\x20front_face: {ff},\n\
         \x20\x20\x20\x20cull_mode: {cull},\n\
         \x20\x20\x20\x20polygon_mode: {poly},\n\
         \x20\x20\x20\x20..Default::default()\n\
         }}"
    ))
}

#[elicit_tool(
    plugin = "wgpu_pipeline",
    name = "wgpu_pipeline__blend_state",
    description = "Generate a `wgpu::BlendState` with identical color and alpha blend components."
)]
#[instrument(skip(p))]
async fn pipeline_blend_state(p: BlendStateParams) -> Result<CallToolResult, ErrorData> {
    let src = format!("wgpu::BlendFactor::{:?}", *p.src_factor);
    let dst = format!("wgpu::BlendFactor::{:?}", *p.dst_factor);
    let op = format!("wgpu::BlendOperation::{:?}", *p.operation);
    let component =
        format!("wgpu::BlendComponent {{ src_factor: {src}, dst_factor: {dst}, operation: {op} }}");
    text(format!(
        "wgpu::BlendState {{\n\
         \x20\x20\x20\x20color: {component},\n\
         \x20\x20\x20\x20alpha: {component},\n\
         }}"
    ))
}

#[elicit_tool(
    plugin = "wgpu_pipeline",
    name = "wgpu_pipeline__color_target_state",
    description = "Generate a `wgpu::ColorTargetState` for a render pass color attachment."
)]
#[instrument(skip(p))]
async fn pipeline_color_target_state(
    p: ColorTargetStateParams,
) -> Result<CallToolResult, ErrorData> {
    let fmt = format!("wgpu::TextureFormat::{:?}", *p.format);
    let mask = format!("wgpu::ColorWrites::{:?}", *p.write_mask);
    text(format!(
        "wgpu::ColorTargetState {{\n\
         \x20\x20\x20\x20format: {fmt},\n\
         \x20\x20\x20\x20blend: Some(wgpu::BlendState::REPLACE),\n\
         \x20\x20\x20\x20write_mask: {mask},\n\
         }}"
    ))
}

#[elicit_tool(
    plugin = "wgpu_pipeline",
    name = "wgpu_pipeline__depth_stencil_state",
    description = "Generate a `wgpu::DepthStencilState` for depth testing with default stencil."
)]
#[instrument(skip(p))]
async fn pipeline_depth_stencil_state(
    p: DepthStencilStateParams,
) -> Result<CallToolResult, ErrorData> {
    let fmt = format!("wgpu::TextureFormat::{:?}", *p.format);
    let cmp = format!("wgpu::CompareFunction::{:?}", *p.depth_compare);
    text(format!(
        "wgpu::DepthStencilState {{\n\
         \x20\x20\x20\x20format: {fmt},\n\
         \x20\x20\x20\x20depth_write_enabled: {},\n\
         \x20\x20\x20\x20depth_compare: {cmp},\n\
         \x20\x20\x20\x20stencil: wgpu::StencilState::default(),\n\
         \x20\x20\x20\x20bias: wgpu::DepthBiasState::default(),\n\
         }}",
        p.depth_write_enabled
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: wgpu render pipeline descriptor code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wgpu_pipeline")]
pub struct WgpuPipelinePlugin;

impl WgpuPipelinePlugin {
    /// Create a new pipeline plugin.
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
            .filter(|r| r.plugin == "wgpu_pipeline")
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

impl Default for WgpuPipelinePlugin {
    fn default() -> Self {
        Self::new()
    }
}
