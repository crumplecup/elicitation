# ELICIT_WGPU_PLAN.md

## Goal
Add wgpu 29 support to elicitation as a GPU code-generation alphabet:
1. **Core type integration** -- serializable wgpu descriptor/enum types in `elicitation` with feature gating
2. **Shadow crate** -- `elicit_wgpu` with MCP tools for GPU code generation (~30 tools across 5 plugins)

## Design Constraint: Code-Generation Only

Unlike `elicit_rstar` or `elicit_proj`, wgpu GPU handles (`Device`, `Buffer`, `Texture`, `Queue`, etc.)
hold live GPU resources -- they cannot be serialized, snapshotted, or stored in a `HashMap<Uuid, T>` registry.

`elicit_wgpu` is **code-generation only**: every tool receives a descriptor (as JSON) and returns a
Rust code string that, when compiled and run in a wgpu app, produces the described GPU resource or pipeline.

Live execution of GPU commands is out of scope.

## Architecture Overview

Following the pattern from `elicit_winit` (code-gen only, no live state):
- **elicitation Phase 2**: Serializable enums and descriptor structs as trenchcoat wrappers
- **elicit_wgpu shadow crate**: 5 workflow plugins, each emitting Rust code strings
- **Proof wiring**: Kani/Creusot/Verus coverage for Phase 2 types

## wgpu Version

**Target: wgpu 29.0.1** (previous plan referenced 27 -- obsolete).

Key API notes verified against 29.0.1:
- `DeviceDescriptor` has fields: `label`, `required_features`, `required_limits`, `memory_hints`, `experimental_features`, `trace`
- `SurfaceConfiguration` has `desired_maximum_frame_latency` and `alpha_mode` fields
- `Face` is the cull-mode enum (not `CullMode`) -- variants `Back` and `Front`
- `PolygonMode` has `Fill`, `Line`, `Point` variants
- Serialized forms (verified with serde feature):
  - `TextureFormat::Rgba8Unorm` -> `"rgba8unorm"` (lowercase)
  - `PrimitiveTopology::TriangleList` -> `"triangle-list"` (kebab)
  - `FrontFace::Ccw` -> `"ccw"`
  - `FilterMode::Nearest` -> `"nearest"`
  - `BufferUsages::UNIFORM` -> `"UNIFORM"` (uppercase)
  - `ShaderStages::VERTEX` -> `"VERTEX"`
  - `Extent3d` -> `{"width":N,"height":N,"depthOrArrayLayers":N}`
  - `Color` -> `{"r":0.0,"g":0.0,"b":0.0,"a":1.0}`
  - `Origin3d` -> `{"x":0,"y":0,"z":0}`
  - `TextureSampleType::Float` -> `{"Float":{"filterable":true}}`

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add wgpu to workspace dependencies**:
```toml
wgpu = { version = "29", default-features = false, features = ["wgsl", "serde"] }
```

**1.2 Add `elicit_wgpu` workspace member** in root `Cargo.toml` members list.

**1.3 Add `elicit_wgpu` workspace dependency entry**:
```toml
elicit_wgpu = { path = "crates/elicit_wgpu", version = "0.10.0" }
```

**1.4 Add `wgpu-types` feature to `elicitation`**:
- Add optional dep: `wgpu = { workspace = true, optional = true }`
- Add feature: `wgpu-types = ["dep:wgpu"]`
- Add `"wgpu-types"` to the `full` feature bundle

## Phase 2: Core Type Integration in `elicitation`

### Files to create:
- `crates/elicitation/src/primitives/wgpu_types/` (directory, 4 files)
- `crates/elicitation/src/type_spec/wgpu_specs.rs`
- `crates/elicitation/tests/wgpu_types_test.rs`

### Files to modify:
- `crates/elicitation/src/lib.rs`
- `crates/elicitation/src/primitives/mod.rs`
- `crates/elicitation/src/type_spec/mod.rs`

### Target types (all have `serde` support in wgpu 29):

**2.1 Pure enums** -- `select_trenchcoat!(T, as WgpuT, serde)`:
- `TextureFormat` -> `WgpuTextureFormat`
- `PresentMode` -> `WgpuPresentMode`
- `PowerPreference` -> `WgpuPowerPreference`
- `TextureDimension` -> `WgpuTextureDimension`
- `TextureViewDimension` -> `WgpuTextureViewDimension`
- `PrimitiveTopology` -> `WgpuPrimitiveTopology`
- `FrontFace` -> `WgpuFrontFace`
- `Face` -> `WgpuFace` (cull mode)
- `PolygonMode` -> `WgpuPolygonMode`
- `CompareFunction` -> `WgpuCompareFunction`
- `BlendFactor` -> `WgpuBlendFactor`
- `BlendOperation` -> `WgpuBlendOperation`
- `IndexFormat` -> `WgpuIndexFormat`
- `StencilOperation` -> `WgpuStencilOperation`
- `VertexStepMode` -> `WgpuVertexStepMode`
- `VertexFormat` -> `WgpuVertexFormat`
- `AddressMode` -> `WgpuAddressMode`
- `FilterMode` -> `WgpuFilterMode`
- `SamplerBorderColor` -> `WgpuSamplerBorderColor`
- `CompositeAlphaMode` -> `WgpuCompositeAlphaMode`
- `Backend` -> `WgpuBackend`

**2.2 Bitflag types** -- also `select_trenchcoat!(T, as WgpuT, serde)` since wgpu bitflags have serde:
- `BufferUsages` -> `WgpuBufferUsages`
- `TextureUsages` -> `WgpuTextureUsages`
- `ShaderStages` -> `WgpuShaderStages`
- `ColorWrites` -> `WgpuColorWrites`

**2.3 Struct types** -- full `Elicitation` / `ElicitIntrospect` / `ElicitPromptTree` / `ToCodeLiteral` impls:
- `Extent3d` -> `WgpuExtent3d`
- `Color` -> `WgpuColor`
- `Origin3d` -> `WgpuOrigin3d`

### Module layout:
```
crates/elicitation/src/primitives/wgpu_types/
|-- mod.rs         # pub use all 27 types
|-- enums.rs       # 21 pure enum select_trenchcoat wrappers
|-- bitflags.rs    # 4 bitflag select_trenchcoat wrappers
````-- structs.rs     # WgpuExtent3d, WgpuColor, WgpuOrigin3d
```

### Type-spec:
`crates/elicitation/src/type_spec/wgpu_specs.rs` -- `ElicitSpec` + `ElicitComplete` registration for all 27 types.

## Phase 3: `elicit_wgpu` Shadow Crate

Code-gen only -- no snapshot registries, no `Arc<Mutex<HashMap<Uuid, T>>>`.

### Directory structure:
```
crates/elicit_wgpu/
|-- Cargo.toml
|-- src/
|   |-- lib.rs
|   `-- workflow/
|       |-- mod.rs
|       |-- init_plugin.rs        # Instance/Adapter/Device/Surface setup code (~8 tools)
|       |-- resource_plugin.rs    # Buffer/Texture/Sampler descriptor code (~8 tools)
|       |-- pipeline_plugin.rs    # RenderPipeline/vertex layout/blend state code (~8 tools)
|       |-- shader_plugin.rs      # WGSL boilerplate and shader module setup (~4 tools)
|       `-- compute_plugin.rs     # ComputePipeline and bind group layout code (~4 tools)
`-- tests/
    `-- workflow_test.rs
```

### No direct dep:wgpu needed -- all types accessed via `elicitation::WgpuTextureFormat` etc.

### Plugin inventory (5 plugins, ~32 tools):

**WgpuInitPlugin** (`wgpu_init__*`, ~8 tools):
- `wgpu_init__instance_descriptor` -- emit `InstanceDescriptor{backends: Backends::all()}`
- `wgpu_init__instance_new` -- emit `wgpu::Instance::new(InstanceDescriptor{...})`
- `wgpu_init__adapter_options` -- emit `RequestAdapterOptions{power_preference, ...}`
- `wgpu_init__request_adapter` -- emit `instance.request_adapter(RequestAdapterOptions{...}).await`
- `wgpu_init__device_descriptor` -- emit `DeviceDescriptor{required_features, required_limits, ...}`
- `wgpu_init__request_device` -- emit `adapter.request_device(&DeviceDescriptor{...}, None).await`
- `wgpu_init__surface_config` -- emit full `SurfaceConfiguration{...}` block
- `wgpu_init__surface_configure` -- emit `surface.configure(&device, config)`

**WgpuResourcePlugin** (`wgpu_resource__*`, ~8 tools):
- `wgpu_resource__buffer_descriptor` -- emit `BufferDescriptor{size, usage, ...}`
- `wgpu_resource__create_buffer` -- emit `device.create_buffer(&BufferDescriptor{...})`
- `wgpu_resource__texture_descriptor` -- emit `TextureDescriptor{size, format, usage, ...}`
- `wgpu_resource__create_texture` -- emit `device.create_texture(&TextureDescriptor{...})`
- `wgpu_resource__texture_view` -- emit `texture.create_view(&TextureViewDescriptor::default())`
- `wgpu_resource__sampler_descriptor` -- emit `SamplerDescriptor{address_mode_*, filter, ...}`
- `wgpu_resource__create_sampler` -- emit `device.create_sampler(&SamplerDescriptor{...})`
- `wgpu_resource__write_buffer` -- emit `queue.write_buffer(&buffer, 0, bytemuck::cast_slice(data))`

**WgpuPipelinePlugin** (`wgpu_pipeline__*`, ~8 tools):
- `wgpu_pipeline__vertex_attribute` -- emit `VertexAttribute{format, offset, shader_location}`
- `wgpu_pipeline__vertex_buffer_layout` -- emit `VertexBufferLayout{array_stride, step_mode, attributes: &[...]}`
- `wgpu_pipeline__blend_component` -- emit `BlendComponent{src_factor, dst_factor, operation}`
- `wgpu_pipeline__blend_state` -- emit `BlendState{color: BlendComponent{...}, alpha: BlendComponent{...}}`
- `wgpu_pipeline__primitive_state` -- emit `PrimitiveState{topology, cull_mode, front_face, polygon_mode}`
- `wgpu_pipeline__depth_stencil_state` -- emit `DepthStencilState{format, depth_write_enabled, depth_compare}`
- `wgpu_pipeline__color_target_state` -- emit `ColorTargetState{format, blend, write_mask}`
- `wgpu_pipeline__render_pipeline` -- emit full `RenderPipelineDescriptor{...}`

**WgpuShaderPlugin** (`wgpu_shader__*`, ~4 tools):
- `wgpu_shader__module_descriptor` -- emit `ShaderModuleDescriptor{label, source: ShaderSource::Wgsl(code)}`
- `wgpu_shader__vertex_entry` -- emit WGSL `@vertex` fn scaffold
- `wgpu_shader__fragment_entry` -- emit WGSL `@fragment` fn scaffold
- `wgpu_shader__compute_entry` -- emit WGSL `@compute @workgroup_size(x,y,z)` fn scaffold

**WgpuComputePlugin** (`wgpu_compute__*`, ~4 tools):
- `wgpu_compute__bind_group_layout_entry` -- emit `BindGroupLayoutEntry{binding, visibility, ty, count}`
- `wgpu_compute__bind_group_layout` -- emit `device.create_bind_group_layout(&BindGroupLayoutDescriptor{...})`
- `wgpu_compute__compute_pipeline` -- emit full `ComputePipelineDescriptor{layout, module, entry_point}`
- `wgpu_compute__dispatch_workgroups` -- emit `compute_pass.dispatch_workgroups(x, y, z)`

### Proposition types:
- `WgpuInitialized` -- Instance + Device setup code generated
- `WgpuResourceDescribed` -- Buffer or Texture descriptor code generated
- `WgpuPipelineBuilt` -- RenderPipeline or ComputePipeline descriptor code generated

### Workflow test:
`tests/workflow_test.rs` -- inventory smoke tests + `invoke_tool` calls covering at least one tool per plugin.

## Phase 4: Proof Wiring

Following the pattern from `elicit_winit`:

**4.1 Kani harnesses** (`crates/elicitation_kani/src/wgpu_types.rs`):
- Harness for `WgpuExtent3d` field invariants
- Harness for `WgpuColor` channel encoding
- Harness for `WgpuOrigin3d` field encoding

**4.2 Creusot axioms** (`crates/elicitation_creusot/src/wgpu_types.rs`):
- Trusted axioms for the same three struct types

**4.3 Verus shadow structs** (`crates/elicitation_verus/src/wgpu_types.rs`):
- Shadow struct field proofs for `WgpuExtent3d`, `WgpuColor`, `WgpuOrigin3d`

**4.4 Feature wiring**:
- `wgpu-types = ["elicitation/wgpu-types"]` feature in `elicitation_kani/Cargo.toml` and `elicitation_creusot/Cargo.toml`
- `pub mod wgpu_types;` in `elicitation_verus/src/lib.rs`

**4.5 Runner entries**:
- `runner.rs` -- 3 Kani harness entries
- `creusot_runner.rs` -- `wgpu-types` module entry
- `verus_runner.rs` -- 3 Verus entries

**4.6 `proof_non_empty_test`**:
- Add `wgpu_tests` module covering all 27 Phase 2 types

## Implementation Order

1. Phase 1: workspace wiring
2. Phase 2: `elicitation` wgpu-types (enums + bitflags + structs + type-spec + test)
3. Phase 3: `elicit_wgpu` scaffold + 5 plugins + workflow test
4. Phase 4: proof wiring (Kani + Creusot + Verus + runners + non-empty test)

## Key Constraints

- **No `dep:wgpu` in proof crates or in `elicit_wgpu`** -- types accessed via `elicitation::Wgpu*`
- **No live GPU state** -- all tools return `String` code snippets
- **No `emit` feature needed** -- tools return `String` directly
- **Orphan rule fix for params structs**: Any `#[elicit_tool]` params struct that embeds a wgpu wrapper type needs a local wrapper (same pattern as `WindowAttributesParams` in winit)
