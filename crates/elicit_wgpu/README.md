# elicit_wgpu

`elicit_wgpu` is the [elicitation] shadow crate for [wgpu]. All tools are **emit-only**: they
generate idiomatic Rust code snippets for GPU applications. No GPU device is created or accessed
at runtime — no `wgpu::Device`, surface, or buffer lives in the MCP server process.

## Plugins

| Plugin | Namespace | Tools | Coverage |
|---|---|---|---|
| `WgpuInitPlugin` | `wgpu_init__*` | 4 | Instance, adapter, device, surface configuration |
| `WgpuResourcePlugin` | `wgpu_resource__*` | 3 | Buffer, texture, and sampler descriptors |
| `WgpuPipelinePlugin` | `wgpu_pipeline__*` | 4 | Render pipeline state descriptors |
| `WgpuShaderPlugin` | `wgpu_shader__*` | 3 | Shader module and vertex/fragment stage |
| `WgpuComputePlugin` | `wgpu_compute__*` | 3 | Compute pipeline, dispatch, bind group layout |

## Tool reference

### `wgpu_init__*`

| Tool | Description |
|---|---|
| `instance` | Generate code to create a `wgpu::Instance` |
| `adapter_request` | Generate `instance.request_adapter(...)` code |
| `device_request` | Generate `adapter.request_device(...)` code yielding `(Device, Queue)` |
| `surface_config` | Generate a `wgpu::SurfaceConfiguration` struct literal |

### `wgpu_resource__*`

| Tool | Description |
|---|---|
| `buffer_desc` | Generate a `wgpu::BufferDescriptor` struct literal |
| `texture_desc` | Generate a `wgpu::TextureDescriptor` struct literal |
| `sampler_desc` | Generate a `wgpu::SamplerDescriptor` struct literal |

### `wgpu_pipeline__*`

| Tool | Description |
|---|---|
| `primitive_state` | Generate a `wgpu::PrimitiveState` for rasterization configuration |
| `blend_state` | Generate a `wgpu::BlendState` with color and alpha blend components |
| `color_target_state` | Generate a `wgpu::ColorTargetState` for a render pass attachment |
| `render_pipeline_desc` | Generate a `wgpu::RenderPipelineDescriptor` block |

### `wgpu_shader__*`

| Tool | Description |
|---|---|
| `module_inline` | Generate code to create a `wgpu::ShaderModule` from an inline WGSL string |
| `vertex_state` | Generate a `wgpu::VertexState` referencing a shader module variable |
| `fragment_state` | Generate a `wgpu::FragmentState` referencing a shader module and targets |

### `wgpu_compute__*`

| Tool | Description |
|---|---|
| `pipeline_desc` | Generate a `wgpu::ComputePipelineDescriptor` code block |
| `dispatch` | Generate a `compute_pass.dispatch_workgroups(x, y, z)` call |
| `bind_group_layout_entry` | Generate a `wgpu::BindGroupLayoutEntry` for a buffer binding |

## Usage

```toml
[dependencies]
elicit_wgpu = "0.11"
```

```rust
use elicit_wgpu::{
    WgpuInitPlugin, WgpuResourcePlugin,
    WgpuPipelinePlugin, WgpuShaderPlugin, WgpuComputePlugin,
};

let server = server
    .register_plugin(WgpuInitPlugin::new())
    .register_plugin(WgpuResourcePlugin::new())
    .register_plugin(WgpuPipelinePlugin::new())
    .register_plugin(WgpuShaderPlugin::new())
    .register_plugin(WgpuComputePlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[wgpu]: https://crates.io/crates/wgpu
