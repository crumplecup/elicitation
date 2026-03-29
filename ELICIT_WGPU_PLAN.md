# ELICIT_WGPU_PLAN.md

## Goal
Add complete wgpu support to elicitation as GPU alphabet:
1. **Core type integration** — wgpu types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_wgpu` with MCP tools for the entire wgpu API (~200+ operations)

## Architecture Overview

Following established patterns from elicit_chrono, elicit_tokio, elicit_url:
- **Core**: Feature-gated wgpu types with Select enums and Elicitation impls
- **Shadow crate**: 12 workflow plugins covering 125+ types and 203 methods
- **GPU alphabet**: Foundation for windowing (winit) + rendering workflows

## API Coverage

wgpu provides:
- **GPU initialization**: Instance, Adapter, Device, Queue
- **Resource management**: Buffer, Texture, Sampler
- **Shader compilation**: WGSL, SPIR-V
- **Pipeline configuration**: RenderPipeline, ComputePipeline
- **Command encoding**: CommandEncoder, RenderPass, ComputePass
- **Binding resources**: BindGroup, BindGroupLayout
- **Surface presentation**: Surface, SwapChain

**Total API surface**: ~125 types, 203 methods → ~124 MCP tools across 12 plugins

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add wgpu to workspace dependencies**:
```toml
# GPU rendering
wgpu = { version = "27.0", default-features = false, features = ["wgsl", "spirv"] }
```

**1.2 Add elicit_wgpu member**:
```toml
  "crates/elicit_wgpu",
```

**1.3 Add elicit_wgpu workspace dependency**:
```toml
elicit_wgpu = { path = "crates/elicit_wgpu", version = "0.9.1" }
```

**1.4 Add wgpu feature to elicitation**:
- Add optional dependency: `wgpu = { workspace = true, optional = true }`
- Add feature: `wgpu = ["dep:wgpu"]`
- Update `full` feature to include `"wgpu"`

## Phase 2: Core Type Integration

### Files to create/modify:
- `crates/elicitation/src/wgpu_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Simple Enums** (use `select_trenchcoat!` macro):

77 TextureFormat variants:
```rust
select_trenchcoat!(wgpu::TextureFormat, as TextureFormatSelect, serde);
```

Other key enums:
- PresentMode (Fifo, Mailbox, Immediate, FifoRelaxed)
- PowerPreference (None, LowPower, HighPerformance)
- Backend (Vulkan, Metal, Dx12, Gl, BrowserWebGpu, WebGpu)
- CompareFunction (Never, Less, Equal, etc.)
- PrimitiveTopology (PointList, LineList, TriangleList, etc.)
- FrontFace (Ccw, Cw)
- CullMode (Front, Back, None)
- BlendOperation (Add, Subtract, ReverseSubtract, Min, Max)
- TextureDimension (D1, D2, D3)
- TextureViewDimension (D1, D2, D2Array, Cube, CubeArray, D3)
- StencilOperation (~8 variants)
- BindingType (~7 variants)

**Total: ~15 enums with 100+ combined variants**

**2.2 Bitflag Structs** (manual `Elicitation` impl):
- Features (25+ flags)
- ShaderStages (Vertex, Fragment, Compute)
- TextureUsages (CopyDst, CopySrc, RenderAttachment, etc.)
- BufferUsages (MapRead, MapWrite, CopySrc, CopyDst, Uniform, Storage, etc.)
- ColorWrites (Red, Green, Blue, Alpha)

**2.3 Complex Descriptors** (builder-style `Elicitation` impl):
- DeviceDescriptor
- BufferDescriptor
- TextureDescriptor
- SamplerDescriptor
- BindGroupLayoutDescriptor
- RenderPipelineDescriptor
- ComputePipelineDescriptor
- CommandEncoderDescriptor

### Implementation Pattern:

```rust
// crates/elicitation/src/wgpu_types.rs
#![cfg(feature = "wgpu")]

use wgpu::{TextureFormat, PresentMode, PowerPreference};

// Simple enums
select_trenchcoat!(wgpu::TextureFormat, as TextureFormatSelect, serde);
select_trenchcoat!(wgpu::PresentMode, as PresentModeSelect, serde);
select_trenchcoat!(wgpu::PowerPreference, as PowerPreferenceSelect, serde);

// Bitflags - custom Elicitation impl
impl Elicitation for wgpu::Features {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Guided multi-select for feature flags
    }
}

// Complex descriptors - builder pattern
impl Elicitation for wgpu::DeviceDescriptor<'static> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Step-by-step builder configuration
    }
}
```

**2.4 Export from lib.rs**:
```rust
#[cfg(feature = "wgpu")]
pub mod wgpu_types;

#[cfg(feature = "wgpu")]
pub use wgpu_types::{
    TextureFormatSelect, PresentModeSelect, PowerPreferenceSelect,
    // ... all other exports
};
```

## Phase 3: Create elicit_wgpu Shadow Crate

### Directory Structure:

```
crates/elicit_wgpu/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── instance.rs         (Instance wrapper)
│   ├── adapter.rs          (Adapter wrapper)
│   ├── device.rs           (Device wrapper)
│   ├── queue.rs            (Queue wrapper)
│   ├── buffer.rs           (Buffer wrapper)
│   ├── texture.rs          (Texture wrapper + TextureView)
│   ├── sampler.rs          (Sampler wrapper)
│   ├── shader.rs           (ShaderModule wrapper)
│   ├── pipeline.rs         (RenderPipeline, ComputePipeline)
│   ├── binding.rs          (BindGroup, BindGroupLayout)
│   ├── command.rs          (CommandEncoder wrapper)
│   ├── render_pass.rs      (RenderPass wrapper)
│   ├── compute_pass.rs     (ComputePass wrapper)
│   ├── surface.rs          (Surface wrapper)
│   └── workflow/
│       ├── mod.rs
│       ├── init_plugin.rs          (~8 tools: Instance, Adapter, Device setup)
│       ├── resources_plugin.rs     (~10 tools: Buffer, Texture, Sampler)
│       ├── shader_plugin.rs        (~6 tools: ShaderModule, compilation)
│       ├── pipeline_plugin.rs      (~12 tools: RenderPipeline, ComputePipeline)
│       ├── binding_plugin.rs       (~8 tools: BindGroup, BindGroupLayout)
│       ├── command_plugin.rs       (~15 tools: CommandEncoder operations)
│       ├── render_pass_plugin.rs   (~25 tools: RenderPass methods)
│       ├── compute_pass_plugin.rs  (~8 tools: ComputePass methods)
│       ├── surface_plugin.rs       (~6 tools: Surface configuration)
│       ├── queue_plugin.rs         (~4 tools: Queue submit, write)
│       ├── enums_plugin.rs         (~10 tools: Enum selectors)
│       └── workflow_plugin.rs      (~12 tools: High-level workflows)
└── tests/
    └── wgpu_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_wgpu"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled wgpu wrappers with comprehensive MCP tools for GPU operations"
keywords = ["mcp", "wgpu", "gpu", "graphics", "elicitation"]
categories = ["graphics", "rendering", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["wgpu"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
wgpu = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
uuid = { workspace = true }
futures = { workspace = true }

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[dev-dependencies]
tokio = { workspace = true }
pollster = "0.4"

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure:

```rust
//! `elicit_wgpu` — comprehensive wgpu API exposure via MCP tools.
//!
//! Provides complete coverage of wgpu 27.0 API (~203 methods across 125+ types):
//! - GPU initialization (Instance, Adapter, Device, Queue)
//! - Resource management (Buffer, Texture, Sampler)
//! - Shader compilation (WGSL, SPIR-V)
//! - Pipeline configuration (Render, Compute)
//! - Command encoding (CommandEncoder, RenderPass, ComputePass)
//! - Binding resources (BindGroup, BindGroupLayout)
//! - Surface presentation (Surface, SwapChain)
//!
//! # Plugin Organization (12 plugins, ~124 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `WgpuInitPlugin` | 8 | Instance, Adapter, Device, Queue setup |
//! | `WgpuResourcesPlugin` | 10 | Buffer, Texture, Sampler creation |
//! | `WgpuShaderPlugin` | 6 | ShaderModule compilation |
//! | `WgpuPipelinePlugin` | 12 | RenderPipeline, ComputePipeline |
//! | `WgpuBindingPlugin` | 8 | BindGroup, BindGroupLayout |
//! | `WgpuCommandPlugin` | 15 | CommandEncoder operations |
//! | `WgpuRenderPassPlugin` | 25 | RenderPass draw calls |
//! | `WgpuComputePassPlugin` | 8 | ComputePass dispatch |
//! | `WgpuSurfacePlugin` | 6 | Surface configuration |
//! | `WgpuQueuePlugin` | 4 | Queue submit, write |
//! | `WgpuEnumsPlugin` | 10 | Enum/constant selectors |
//! | `WgpuWorkflowPlugin` | 12 | High-level compositions |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod adapter;
mod binding;
mod buffer;
mod command;
mod compute_pass;
mod device;
mod instance;
mod pipeline;
mod queue;
mod render_pass;
mod sampler;
mod shader;
mod surface;
mod texture;
pub mod workflow;

pub use adapter::Adapter;
pub use binding::{BindGroup, BindGroupLayout};
pub use buffer::Buffer;
pub use command::CommandEncoder;
pub use compute_pass::ComputePass;
pub use device::Device;
pub use instance::Instance;
pub use pipeline::{ComputePipeline, RenderPipeline};
pub use queue::Queue;
pub use render_pass::RenderPass;
pub use sampler::Sampler;
pub use shader::ShaderModule;
pub use surface::Surface;
pub use texture::{Texture, TextureView};
pub use workflow::{
    WgpuBindingPlugin, WgpuCommandPlugin, WgpuComputePassPlugin,
    WgpuEnumsPlugin, WgpuInitPlugin, WgpuPipelinePlugin,
    WgpuQueuePlugin, WgpuRenderPassPlugin, WgpuResourcesPlugin,
    WgpuShaderPlugin, WgpuSurfacePlugin, WgpuWorkflowPlugin,
};
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Simple Wrappers (instance.rs, adapter.rs, etc.):

Use `elicit_newtype!` for opaque handles:

```rust
// instance.rs
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(wgpu::Instance, as Instance, serde);

#[reflect_methods]
impl Instance {
    #[instrument(skip(self))]
    pub fn enumerate_adapters(&self) -> Vec<Adapter> {
        self.0.enumerate_adapters(wgpu::Backends::all())
            .into_iter()
            .map(Adapter::from)
            .collect()
    }
}
```

### 4.2 Resource Types (buffer.rs, texture.rs):

```rust
// buffer.rs
elicit_newtype!(wgpu::Buffer, as Buffer, serde);

#[reflect_methods]
impl Buffer {
    #[instrument(skip(self))]
    pub fn size(&self) -> u64 {
        self.0.size()
    }

    #[instrument(skip(self))]
    pub fn usage(&self) -> String {
        format!("{:?}", self.0.usage())
    }
}
```

### 4.3 Command Encoding (render_pass.rs):

```rust
// render_pass.rs - special case: mutable lifetime-bound type
// Cannot use Arc due to lifetime constraints
// Solution: CommandEncoder owns RenderPass, tools operate on it

pub struct RenderPass<'a> {
    inner: wgpu::RenderPass<'a>,
    stats: RenderStats,
}

impl<'a> RenderPass<'a> {
    pub fn set_pipeline(&mut self, pipeline: &RenderPipeline) {
        self.inner.set_pipeline(&pipeline.0);
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.inner.draw(vertices, instances);
        self.stats.draw_calls += 1;
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Init Plugin (workflow/init_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateInstanceParams {
    pub backends: Vec<String>,
}

#[elicit_tool(
    plugin = "wgpu_init",
    name = "wgpu_init__create_instance",
    description = "Create a wgpu Instance for GPU initialization. \
                   Backends: 'vulkan', 'metal', 'dx12', 'gl', 'webgpu'.",
    emit = Auto
)]
async fn init_create_instance(p: CreateInstanceParams) -> Result<CallToolResult, ErrorData> {
    let backends = parse_backends(&p.backends)?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    Ok(CallToolResult::success(vec![
        Content::text(format!("Created wgpu Instance with backends: {:?}", backends))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestAdapterParams {
    pub power_preference: String,
    pub force_fallback: Option<bool>,
}

#[elicit_tool(
    plugin = "wgpu_init",
    name = "wgpu_init__request_adapter",
    description = "Request a GPU adapter. Power preference: 'low_power', 'high_performance', or 'none'.",
    emit = Auto
)]
async fn init_request_adapter(p: RequestAdapterParams) -> Result<CallToolResult, ErrorData> {
    // Emit code for adapter request
    Ok(CallToolResult::success(vec![
        Content::text(format!("Requested adapter with power: {}", p.power_preference))
    ]))
}

// ... 6 more tools: request_device, get_limits, get_features, etc.
```

### 5.2 Resources Plugin (workflow/resources_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateBufferParams {
    pub size: u64,
    pub usage: Vec<String>,  // ["MAP_READ", "COPY_DST", etc.]
    pub mapped_at_creation: Option<bool>,
}

#[elicit_tool(
    plugin = "wgpu_resources",
    name = "wgpu_resources__create_buffer",
    description = "Create a GPU buffer. Usage flags: MAP_READ, MAP_WRITE, COPY_SRC, COPY_DST, UNIFORM, STORAGE, INDEX, VERTEX, INDIRECT.",
    emit = Auto
)]
async fn resources_create_buffer(p: CreateBufferParams) -> Result<CallToolResult, ErrorData> {
    let usage = parse_buffer_usage(&p.usage)?;

    Ok(CallToolResult::success(vec![
        Content::text(format!("Created buffer: {} bytes, usage: {:?}", p.size, usage))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTextureParams {
    pub width: u32,
    pub height: u32,
    pub format: String,  // TextureFormat enum
    pub usage: Vec<String>,
    pub dimension: Option<String>,
}

#[elicit_tool(
    plugin = "wgpu_resources",
    name = "wgpu_resources__create_texture",
    description = "Create a GPU texture. Format examples: 'Rgba8Unorm', 'Bgra8UnormSrgb', 'Depth32Float'.",
    emit = Auto
)]
async fn resources_create_texture(p: CreateTextureParams) -> Result<CallToolResult, ErrorData> {
    // Emit texture creation code
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created texture: {}x{}, format: {}", p.width, p.height, p.format))
    ]))
}

// ... 8 more tools: create_sampler, write_buffer, copy_buffer_to_texture, etc.
```

### 5.3 Render Pass Plugin (workflow/render_pass_plugin.rs):

Most complex plugin with ~25 tools:

```rust
#[elicit_tool(
    plugin = "wgpu_render_pass",
    name = "wgpu_render_pass__set_pipeline",
    description = "Set the active render pipeline for subsequent draw calls.",
    emit = Auto
)]
async fn render_pass_set_pipeline(p: SetPipelineParams) -> Result<CallToolResult, ErrorData> {
    // Emit: render_pass.set_pipeline(&pipeline);
    Ok(CallToolResult::success(vec![Content::text("Pipeline set")]))
}

#[elicit_tool(
    plugin = "wgpu_render_pass",
    name = "wgpu_render_pass__draw",
    description = "Issue a draw call. Draws 'vertices' range with 'instances' range.",
    emit = Auto
)]
async fn render_pass_draw(p: DrawParams) -> Result<CallToolResult, ErrorData> {
    // Emit: render_pass.draw(vertices, instances);
    Ok(CallToolResult::success(vec![
        Content::text(format!("Draw: vertices {}..{}, instances {}..{}",
            p.vertices_start, p.vertices_end, p.instances_start, p.instances_end))
    ]))
}

// ... 23 more tools: draw_indexed, set_vertex_buffer, set_bind_group, set_viewport, etc.
```

### 5.4 Workflow Plugin (workflow/workflow_plugin.rs):

High-level compositions:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SetupTriangleParams {
    pub width: u32,
    pub height: u32,
    pub shader_source: String,
}

#[elicit_tool(
    plugin = "wgpu_workflow",
    name = "wgpu_workflow__setup_triangle",
    description = "Complete workflow: create instance, adapter, device, surface, and render a triangle.",
    emit = Auto
)]
async fn workflow_setup_triangle(p: SetupTriangleParams) -> Result<CallToolResult, ErrorData> {
    // Emit complete main() with triangle rendering
    Ok(CallToolResult::success(vec![
        Content::text(format!("Triangle setup: {}x{}", p.width, p.height))
    ]))
}

// ... 11 more tools: setup_compute, configure_depth_stencil, create_render_pipeline_full, etc.
```

## Phase 6: Testing

### File to create:
- `crates/elicit_wgpu/tests/wgpu_test.rs`

### Test Coverage:

**Type wrappers and serialization**:
```rust
#[test]
fn test_instance_creation() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let wrapped = Instance::from(instance);
    // Test serialization, reflect methods, etc.
}

#[test]
fn test_buffer_descriptor_serialization() {
    let desc = BufferDescriptor {
        size: 1024,
        usage: vec!["UNIFORM".to_string(), "COPY_DST".to_string()],
        mapped_at_creation: Some(false),
    };

    let json = serde_json::to_value(&desc).unwrap();
    assert_eq!(json["size"], 1024);
}
```

**Device creation and limits**:
```rust
#[tokio::test]
async fn test_device_creation() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("Failed to find adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    assert!(device.limits().max_texture_dimension_2d > 0);
}
```

**MCP tool registration**:
```rust
#[test]
fn test_plugin_registration() {
    // Verify all 12 plugins register correctly
    // Test tool discovery via inventory
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_wgpu/README.md`

### Content:

```markdown
# elicit_wgpu

Comprehensive elicitation-enabled wrappers around [`wgpu`](https://docs.rs/wgpu) for GPU operations.

## Purpose

Provides the **GPU alphabet** — foundational MCP tools for:
- Graphics rendering workflows (combine with winit for windowing)
- Compute shader operations
- Custom rendering pipelines
- GPU-accelerated processing

## API Coverage

Exposes the complete wgpu 27.0 API (~203 methods across 125+ types) via 12 plugin namespaces:

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `wgpu_init` | 8 | Instance, Adapter, Device, Queue |
| `wgpu_resources` | 10 | Buffer, Texture, Sampler |
| `wgpu_shader` | 6 | ShaderModule, WGSL/SPIR-V |
| `wgpu_pipeline` | 12 | RenderPipeline, ComputePipeline |
| `wgpu_binding` | 8 | BindGroup, BindGroupLayout |
| `wgpu_command` | 15 | CommandEncoder |
| `wgpu_render_pass` | 25 | RenderPass draw calls |
| `wgpu_compute_pass` | 8 | ComputePass dispatch |
| `wgpu_surface` | 6 | Surface configuration |
| `wgpu_queue` | 4 | Queue submit, write |
| `wgpu_enums` | 10 | Enum/constant selectors |
| `wgpu_workflow` | 12 | High-level workflows |

**Total: ~124 MCP tools**

## Usage

```rust
use elicit_wgpu::{Instance, Device, Queue};

// MCP tools generate this code:
let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
let adapter = instance.request_adapter(&Default::default()).await?;
let (device, queue) = adapter.request_device(&Default::default(), None).await?;
```

## Integration with Windowing

Combine with `elicit_winit` for complete rendering stack:

```rust
// winit creates window + surface
let window = winit::window::Window::new(&event_loop)?;

// wgpu provides GPU operations
let instance = wgpu::Instance::new(Default::default());
let surface = instance.create_surface(&window)?;
let (device, queue) = adapter.request_device(&Default::default(), None).await?;

// Configure and render
surface.configure(&device, &config);
// ... rendering loop using wgpu tools
```
```

## Verification Steps

### After implementation:

**elicit_wgpu shadow crate**:
1. `cargo check -p elicit_wgpu`
2. `cargo test -p elicit_wgpu`
3. `cargo check -p elicitation --no-default-features --features wgpu`
4. `cargo test -p elicit_wgpu --features emit`

**Full workspace**:
1. `cargo check --all-features`
2. `cargo test --workspace --all-features`

### Manual verification:

**MCP tool functionality**:
1. Launch MCP server with elicit_wgpu plugin
2. Call `wgpu_init__create_instance` with backends param
3. Call `wgpu_resources__create_buffer` with size/usage
4. Verify JSON responses and emit mode code generation

**Type integration**:
1. Test `wgpu::TextureFormat` → `TextureFormatSelect` conversion
2. Test `wgpu::Features` → bitflags Elicitation impl
3. Verify serialization round-trip for all wrapper types

## Critical Files

### To create:
- `crates/elicit_wgpu/Cargo.toml`
- `crates/elicit_wgpu/README.md`
- `crates/elicit_wgpu/src/lib.rs`
- `crates/elicit_wgpu/src/instance.rs`
- `crates/elicit_wgpu/src/adapter.rs`
- `crates/elicit_wgpu/src/device.rs`
- `crates/elicit_wgpu/src/queue.rs`
- `crates/elicit_wgpu/src/buffer.rs`
- `crates/elicit_wgpu/src/texture.rs`
- `crates/elicit_wgpu/src/sampler.rs`
- `crates/elicit_wgpu/src/shader.rs`
- `crates/elicit_wgpu/src/pipeline.rs`
- `crates/elicit_wgpu/src/binding.rs`
- `crates/elicit_wgpu/src/command.rs`
- `crates/elicit_wgpu/src/render_pass.rs`
- `crates/elicit_wgpu/src/compute_pass.rs`
- `crates/elicit_wgpu/src/surface.rs`
- `crates/elicit_wgpu/src/workflow/mod.rs`
- `crates/elicit_wgpu/src/workflow/init_plugin.rs`
- `crates/elicit_wgpu/src/workflow/resources_plugin.rs`
- `crates/elicit_wgpu/src/workflow/shader_plugin.rs`
- `crates/elicit_wgpu/src/workflow/pipeline_plugin.rs`
- `crates/elicit_wgpu/src/workflow/binding_plugin.rs`
- `crates/elicit_wgpu/src/workflow/command_plugin.rs`
- `crates/elicit_wgpu/src/workflow/render_pass_plugin.rs`
- `crates/elicit_wgpu/src/workflow/compute_pass_plugin.rs`
- `crates/elicit_wgpu/src/workflow/surface_plugin.rs`
- `crates/elicit_wgpu/src/workflow/queue_plugin.rs`
- `crates/elicit_wgpu/src/workflow/enums_plugin.rs`
- `crates/elicit_wgpu/src/workflow/workflow_plugin.rs`
- `crates/elicit_wgpu/tests/wgpu_test.rs`
- `crates/elicitation/src/wgpu_types.rs`

### To modify:
- `Cargo.toml` — Add workspace members and dependencies
- `crates/elicitation/Cargo.toml` — Add wgpu feature
- `crates/elicitation/src/lib.rs` — Export wgpu types

## Implementation Order

1. **Phase 1**: Workspace configuration (30 min)
2. **Phase 2**: Core type integration in elicitation (2 hours)
3. **Phase 3**: Create elicit_wgpu structure (1 hour)
4. **Phase 4**: Implement type wrappers (4 hours)
5. **Phase 5**: Implement MCP tools (~124 tools) (12-16 hours)
6. **Phase 6**: Testing (1-2 hours)
7. **Phase 7**: Documentation (1 hour)

**Total estimated time**: 21-26 hours

## Notes

### Shadow Crate Design
- **12 plugins**: Organized by functional area (init, resources, shaders, pipelines, etc.)
- **~124 total tools**: Comprehensive coverage of wgpu 27.0 API
- **Emit mode**: All tools support code generation
- **Runtime state**: UUID registry pattern for Device/Queue/Resource handles
- **Composable alphabet**: Low-level GPU operations compose into workflows

### Use Cases
- **Graphics rendering**: Combine with elicit_winit for native windowing
- **Compute shaders**: GPU-accelerated processing
- **Custom pipelines**: Build specialized renderers
- **Learning tool**: Complete wgpu API exposure for experimentation

### Technical Challenges
1. **Lifetime constraints**: RenderPass/ComputePass borrow CommandEncoder mutably
   - Solution: Encapsulate in typed wrappers, tools operate on state
2. **Async device creation**: wgpu uses async for adapter/device requests
   - Solution: MCP tools emit async code, runtime uses pollster for tests
3. **Platform differences**: wgpu abstracts Vulkan/Metal/DX12/WebGPU
   - Solution: Expose backend selection via enum tools, let wgpu handle rest
4. **Shader compilation**: WGSL is text, SPIR-V is binary
   - Solution: ShaderSource enum with both variants, validation in tools
5. **Resource ownership**: Device owns all resources (buffers, textures, pipelines)
   - Solution: UUID registry maps IDs → Arc'd resources for MCP tool access

### Future Integration
This shadow crate provides the GPU alphabet. Future frontend implementations:
- **elicit_egui**: Projects elicit_ui contracts → egui widgets
- **elicit_leptos**: Projects elicit_ui contracts → leptos components
- **elicit_ratatui**: Projects elicit_ui contracts → terminal UI
- **elicit_iced**: Projects elicit_ui contracts → iced widgets (maybe)

These frontends may use wgpu internally (egui, iced) or not (leptos, ratatui).
