//! `WgpuResourcePlugin` — code-generation tools for wgpu buffer/texture/sampler descriptors.
//!
//! # Tool namespace: `wgpu_resource__*`
//!
//! | Tool | Emits |
//! |------|-------|
//! | `buffer_desc` | `wgpu::BufferDescriptor { ... }` |
//! | `texture_desc` | `wgpu::TextureDescriptor { ... }` |
//! | `sampler_desc` | `wgpu::SamplerDescriptor { ... }` |

use elicitation::{
    Prop, VerifiedWorkflow, WgpuAddressMode, WgpuBufferUsages, WgpuExtent3d, WgpuFilterMode,
    WgpuSamplerBorderColor, WgpuTextureDimension, WgpuTextureFormat, WgpuTextureUsages,
    elicit_tool,
};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a wgpu resource descriptor was generated.
#[derive(Prop)]
pub struct WgpuResourceDescribed;

impl VerifiedWorkflow for WgpuResourceDescribed {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `wgpu_resource__buffer_desc`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BufferDescParams {
    /// Debug label.
    pub label: Option<String>,
    /// Buffer size in bytes.
    pub size: u64,
    /// Buffer usage flags.
    pub usage: WgpuBufferUsages,
    /// Whether to map the buffer on creation.
    pub mapped_at_creation: bool,
}

/// Parameters for `wgpu_resource__texture_desc`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TextureDescParams {
    /// Debug label.
    pub label: Option<String>,
    /// Texture extent (width, height, depth/layers).
    pub size: WgpuExtent3d,
    /// Dimensionality.
    pub dimension: WgpuTextureDimension,
    /// Pixel format.
    pub format: WgpuTextureFormat,
    /// Usage flags.
    pub usage: WgpuTextureUsages,
    /// Number of mip levels.
    pub mip_level_count: u32,
    /// Sample count (1 = no MSAA).
    pub sample_count: u32,
}

/// Parameters for `wgpu_resource__sampler_desc`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SamplerDescParams {
    /// Debug label.
    pub label: Option<String>,
    /// Address mode for u/v/w (all set to the same value).
    pub address_mode: WgpuAddressMode,
    /// Magnification filter.
    pub mag_filter: WgpuFilterMode,
    /// Minification filter.
    pub min_filter: WgpuFilterMode,
    /// Border color for `ClampToBorder` address mode.
    pub border_color: Option<WgpuSamplerBorderColor>,
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
    plugin = "wgpu_resource",
    name = "wgpu_resource__buffer_desc",
    description = "Generate a `wgpu::BufferDescriptor` struct literal. \
                   Pass `device.create_buffer(&desc)` to allocate the buffer."
)]
#[instrument(skip(p))]
async fn resource_buffer_desc(p: BufferDescParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    let usage = format!("wgpu::BufferUsages::{:?}", *p.usage);
    text(format!(
        "let desc = wgpu::BufferDescriptor {{\n\
         \x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20size: {},\n\
         \x20\x20\x20\x20usage: {usage},\n\
         \x20\x20\x20\x20mapped_at_creation: {},\n\
         }};",
        p.size, p.mapped_at_creation
    ))
}

#[elicit_tool(
    plugin = "wgpu_resource",
    name = "wgpu_resource__texture_desc",
    description = "Generate a `wgpu::TextureDescriptor` struct literal. \
                   Pass to `device.create_texture(&desc)`."
)]
#[instrument(skip(p))]
async fn resource_texture_desc(p: TextureDescParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    let dim = format!("wgpu::TextureDimension::{:?}", *p.dimension);
    let fmt = format!("wgpu::TextureFormat::{:?}", *p.format);
    let usage = format!("wgpu::TextureUsages::{:?}", *p.usage);
    let size = format!(
        "wgpu::Extent3d {{ width: {}, height: {}, depth_or_array_layers: {} }}",
        p.size.width, p.size.height, p.size.depth_or_array_layers
    );
    text(format!(
        "let desc = wgpu::TextureDescriptor {{\n\
         \x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20size: {size},\n\
         \x20\x20\x20\x20mip_level_count: {},\n\
         \x20\x20\x20\x20sample_count: {},\n\
         \x20\x20\x20\x20dimension: {dim},\n\
         \x20\x20\x20\x20format: {fmt},\n\
         \x20\x20\x20\x20usage: {usage},\n\
         \x20\x20\x20\x20view_formats: &[],\n\
         }};",
        p.mip_level_count, p.sample_count
    ))
}

#[elicit_tool(
    plugin = "wgpu_resource",
    name = "wgpu_resource__sampler_desc",
    description = "Generate a `wgpu::SamplerDescriptor` struct literal. \
                   Pass to `device.create_sampler(&desc)`."
)]
#[instrument(skip(p))]
async fn resource_sampler_desc(p: SamplerDescParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    let addr = format!("wgpu::AddressMode::{:?}", *p.address_mode);
    let mag = format!("wgpu::FilterMode::{:?}", *p.mag_filter);
    let min = format!("wgpu::FilterMode::{:?}", *p.min_filter);
    let border = match p.border_color {
        Some(ref c) => format!("Some(wgpu::SamplerBorderColor::{:?})", **c),
        None => "None".to_string(),
    };
    text(format!(
        "let desc = wgpu::SamplerDescriptor {{\n\
         \x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20address_mode_u: {addr},\n\
         \x20\x20\x20\x20address_mode_v: {addr},\n\
         \x20\x20\x20\x20address_mode_w: {addr},\n\
         \x20\x20\x20\x20mag_filter: {mag},\n\
         \x20\x20\x20\x20min_filter: {min},\n\
         \x20\x20\x20\x20border_color: {border},\n\
         \x20\x20\x20\x20..Default::default()\n\
         }};"
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: wgpu buffer/texture/sampler descriptor code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wgpu_resource")]
pub struct WgpuResourcePlugin;

impl WgpuResourcePlugin {
    /// Create a new resource plugin.
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
            .filter(|r| r.plugin == "wgpu_resource")
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

impl Default for WgpuResourcePlugin {
    fn default() -> Self {
        Self::new()
    }
}
