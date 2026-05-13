//! `WgpuInitPlugin` — code-generation tools for wgpu device/adapter/surface init.
//!
//! All tools return Rust code strings; no GPU device is created at runtime.
//!
//! # Tool namespace: `wgpu_init__*`
//!
//! | Tool | Params | Emits |
//! |------|--------|-------|
//! | `instance` | `backend` | `wgpu::Instance::new(...)` |
//! | `adapter_request` | `power_preference` | `instance.request_adapter(...)` |
//! | `device_request` | `label` | `adapter.request_device(...)` |
//! | `surface_config` | format, present_mode, alpha_mode, width, height | `wgpu::SurfaceConfiguration { ... }` |

use elicitation::{
    Prop, VerifiedWorkflow, WgpuBackend, WgpuCompositeAlphaMode, WgpuPowerPreference,
    WgpuPresentMode, WgpuTextureFormat, elicit_tool,
};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: wgpu device/surface init code was generated.
#[derive(Prop)]
pub struct WgpuInitialized;

impl VerifiedWorkflow for WgpuInitialized {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `wgpu_init__instance`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InstanceParams {
    /// Backend to target (omit for `Backends::all()`).
    pub backend: Option<WgpuBackend>,
}

/// Parameters for `wgpu_init__adapter_request`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AdapterRequestParams {
    /// Power preference for adapter selection.
    pub power_preference: WgpuPowerPreference,
}

/// Parameters for `wgpu_init__device_request`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeviceRequestParams {
    /// Optional debug label for the device.
    pub label: Option<String>,
}

/// Parameters for `wgpu_init__surface_config`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SurfaceConfigParams {
    /// Surface pixel format.
    pub format: WgpuTextureFormat,
    /// Present mode.
    pub present_mode: WgpuPresentMode,
    /// Composite alpha mode.
    pub alpha_mode: WgpuCompositeAlphaMode,
    /// Surface width in pixels.
    pub width: u32,
    /// Surface height in pixels.
    pub height: u32,
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
    plugin = "wgpu_init",
    name = "wgpu_init__instance",
    description = "Generate code to create a `wgpu::Instance`. \
                   Optionally restrict to a specific backend; defaults to `Backends::all()`."
)]
#[instrument(skip(p))]
async fn init_instance(p: InstanceParams) -> Result<CallToolResult, ErrorData> {
    let backends = match p.backend {
        Some(ref b) => format!("wgpu::Backends::{:?}", **b),
        None => "wgpu::Backends::all()".to_string(),
    };
    text(format!(
        "let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {{\n\
         \x20\x20\x20\x20backends: {backends},\n\
         \x20\x20\x20\x20..Default::default()\n\
         }});"
    ))
}

#[elicit_tool(
    plugin = "wgpu_init",
    name = "wgpu_init__adapter_request",
    description = "Generate `instance.request_adapter(...)` code with the given power preference."
)]
#[instrument(skip(p))]
async fn init_adapter_request(p: AdapterRequestParams) -> Result<CallToolResult, ErrorData> {
    let pref = format!("wgpu::PowerPreference::{:?}", *p.power_preference);
    text(format!(
        "let adapter = instance\n\
         \x20\x20\x20\x20.request_adapter(&wgpu::RequestAdapterOptions {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20power_preference: {pref},\n\
         \x20\x20\x20\x20\x20\x20\x20\x20compatible_surface: Some(&surface),\n\
         \x20\x20\x20\x20\x20\x20\x20\x20..Default::default()\n\
         \x20\x20\x20\x20}})\n\
         \x20\x20\x20\x20.await\n\
         \x20\x20\x20\x20.expect(\"no suitable GPU adapter\");"
    ))
}

#[elicit_tool(
    plugin = "wgpu_init",
    name = "wgpu_init__device_request",
    description = "Generate `adapter.request_device(...)` code that yields a `(Device, Queue)` pair."
)]
#[instrument(skip(p))]
async fn init_device_request(p: DeviceRequestParams) -> Result<CallToolResult, ErrorData> {
    let label = label_opt(&p.label);
    text(format!(
        "let (device, queue) = adapter\n\
         \x20\x20\x20\x20.request_device(\n\
         \x20\x20\x20\x20\x20\x20\x20\x20&wgpu::DeviceDescriptor {{\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20label: {label},\n\
         \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20..Default::default()\n\
         \x20\x20\x20\x20\x20\x20\x20\x20}},\n\
         \x20\x20\x20\x20\x20\x20\x20\x20None,\n\
         \x20\x20\x20\x20)\n\
         \x20\x20\x20\x20.await\n\
         \x20\x20\x20\x20.expect(\"device creation failed\");"
    ))
}

#[elicit_tool(
    plugin = "wgpu_init",
    name = "wgpu_init__surface_config",
    description = "Generate a `wgpu::SurfaceConfiguration` struct literal for swapchain setup."
)]
#[instrument(skip(p))]
async fn init_surface_config(p: SurfaceConfigParams) -> Result<CallToolResult, ErrorData> {
    let fmt = format!("wgpu::TextureFormat::{:?}", *p.format);
    let mode = format!("wgpu::PresentMode::{:?}", *p.present_mode);
    let alpha = format!("wgpu::CompositeAlphaMode::{:?}", *p.alpha_mode);
    text(format!(
        "let config = wgpu::SurfaceConfiguration {{\n\
         \x20\x20\x20\x20usage: wgpu::TextureUsages::RENDER_ATTACHMENT,\n\
         \x20\x20\x20\x20format: {fmt},\n\
         \x20\x20\x20\x20width: {},\n\
         \x20\x20\x20\x20height: {},\n\
         \x20\x20\x20\x20present_mode: {mode},\n\
         \x20\x20\x20\x20alpha_mode: {alpha},\n\
         \x20\x20\x20\x20view_formats: vec![],\n\
         \x20\x20\x20\x20desired_maximum_frame_latency: 2,\n\
         }};",
        p.width, p.height
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: wgpu instance/adapter/device/surface code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wgpu_init")]
pub struct WgpuInitPlugin;

impl WgpuInitPlugin {
    /// Create a new init plugin.
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
            .filter(|r| r.plugin == "wgpu_init")
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

impl Default for WgpuInitPlugin {
    fn default() -> Self {
        Self::new()
    }
}
