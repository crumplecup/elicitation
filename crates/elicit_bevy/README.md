# elicit\_bevy

> **Complete MCP coverage for Bevy 0.18 — every user-authored type, method, trait, and macro, via the Model Context Protocol.**

[![Crates.io](https://img.shields.io/crates/v/elicit_bevy.svg)](https://crates.io/crates/elicit_bevy)
[![Documentation](https://docs.rs/elicit_bevy/badge.svg)](https://docs.rs/elicit_bevy)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Bevy](https://img.shields.io/badge/bevy-0.18-orange.svg)](https://bevyengine.org)

---

## Why This Exists

Bevy is built around two hard-to-serialize abstractions:

- **ECS**: entities, components, systems, and resources exist only at runtime inside a running `App`
- **Generics**: `Query<D, F>`, `Handle<A>`, `Res<R>`, `MeshMaterial3d<M>` — parameterized by types the agent does not know at compile time

Connecting an AI agent to a live Bevy runtime over MCP is therefore the wrong abstraction.
The agent cannot inspect what it cannot serialize, and it cannot round-trip a `World` state
through a JSON schema.

`elicit_bevy` takes a different stance: **the agent authors code, not runtime state**.
Every tool in the crate produces a fragment of source Rust that, when compiled into the
user's project, creates the described game behavior. There is no runtime bridge to Bevy;
there is a comprehensive library of source-code generators that understands the full Bevy
0.18 API surface.

This approach has three advantages:

1. **No runtime dependency** — the MCP server requires only `elicit_bevy` as a library; it does
   not link against a running game or embed a Bevy instance.
2. **Exact fidelity** — generated code matches what a skilled Bevy developer would write.
   There is no lossy mapping through an intermediate IR; the output is idiomatic Rust.
3. **Complete coverage** — because the target is code generation rather than runtime
   inspection, every Bevy type is reachable, including generics, traits, and macros.

---

## Coverage at a Glance

| Metric | Count |
|--------|-------|
| Source lines | ~32,000 |
| Unique exported symbols | ~556 |
| Shadow type wrappers (`elicit_newtype!`) | 142 |
| Method-surface blocks (`#[reflect_methods]`) | 153 |
| Methods exposed as MCP tools via shadow types | ~640 |
| Plugin-level `#[elicit_tool]` tools | 179 |
| MCP plugins | 10 |
| Stateful workflow plugins | 5 |
| Generator implementations | 3 |
| Bevy subcrates covered | 31 |

### Bevy Subcrates Covered

`bevy_animation` · `bevy_anti_alias` · `bevy_app` · `bevy_asset` · `bevy_audio` ·
`bevy_camera` · `bevy_color` · `bevy_core_pipeline` · `bevy_ecs` · `bevy_gizmos` ·
`bevy_image` · `bevy_input` · `bevy_input_focus` · `bevy_light` · `bevy_math` ·
`bevy_mesh` · `bevy_pbr` · `bevy_picking` · `bevy_post_process` · `bevy_reflect` ·
`bevy_render` · `bevy_scene` · `bevy_shader` · `bevy_sprite` · `bevy_sprite_render` ·
`bevy_state` · `bevy_text` · `bevy_time` · `bevy_transform` · `bevy_ui` · `bevy_window`

---

## Coverage Strategy

Coverage is not achieved through a single mechanism. Bevy's API surface requires five
complementary patterns, each chosen for a different kind of type or authoring surface.

### 1. Shadow Types (`elicit_newtype!`)

For concrete Bevy structs and enums whose fields are user-authored values — `Transform`,
`Window`, `StandardMaterial`, `AmbientLight`, `Sprite`, `TextFont`, etc. — the primary
pattern is a **shadow newtype**.

```rust
elicit_newtype!(bevy::transform::components::Transform, as Transform);
```

The macro wraps the upstream type in a local newtype, derives `serde::Serialize`,
`serde::Deserialize`, and `schemars::JsonSchema` (so the AI can construct it), and implements
`ToCodeLiteral` (so the wrapper can emit source Rust that reconstructs the upstream value
at the call site).

**Why a newtype instead of implementing on the upstream type directly?**
Rust's orphan rules prohibit implementing `schemars::JsonSchema` for a type defined in
another crate. The trenchcoat newtype is the standard solution — wrap the type locally,
impl the foreign traits on the wrapper, then convert back via the `From` impl that the macro
generates for free.

**142 newtypes** cover the full catalog of user-authored Bevy value types.

### 2. Method Surface (`#[reflect_methods]`)

A shadow newtype alone only lets an agent *construct* a value. To also let the agent *mutate*
it — calling methods like `with_translation`, `with_rotation`, `looking_at`, etc. — each
shadow type carries a `#[reflect_methods]` impl block.

```rust
#[reflect_methods]
impl Transform {
    pub fn with_translation(&self, x: f32, y: f32, z: f32) -> Self { ... }
    pub fn looking_at(&self, target_x: f32, target_y: f32, target_z: f32, ...) -> Self { ... }
    // ...
}
```

The `#[reflect_methods]` macro expands each method into a registered MCP tool named
`{module}__{method}` (e.g., `transform__with_translation`). The tool accepts JSON-serialized
parameters, applies the method to the wrapped value, and returns the updated wrapper.

**153 reflect_methods blocks** expose approximately **640 instance methods** as live MCP tools
across all shadow types.

### 3. Unit Markers (`unit_elicitation!`)

Bevy makes extensive use of zero-field marker components: `Camera2d`, `Camera3d`, `LightProbe`,
`MsaaWriteback`, `Wireframe`, etc. These types carry no data — their presence on an entity
*is* the configuration.

```rust
unit_elicitation!(bevy::camera::Camera3d);
```

The `unit_elicitation!` macro generates an empty shadow struct, derives `Elicit`, and emits
`TypeName` (the unit constructor) as the code literal.

### 4. Shadow Enums (`shadow_elicitation!`)

Bevy enums with complex variant shapes — particularly those where upstream `serde` derives
are feature-gated — require their own schema, emitter, and `From` impl. The
`shadow_elicitation!` macro handles the boilerplate:

- Derives `serde::Serialize`, `serde::Deserialize`, `schemars::JsonSchema`
- Implements `ElicitComplete` and `ToCodeLiteral` on the shadow enum
- Provides a `From<ShadowEnum> for UpstreamEnum` conversion

Examples: `WindowPosition`, `VideoModeSelection`, `MonitorSelection`.

### 5. Generators (Generator Trait)

Some Bevy types are not authored by the user — they are *constructed* from other artifacts:
fonts loaded from disk, texture slices computed from atlas coordinates, etc. These types have
no sensible JSON schema because their canonical representation is an external resource
reference or a derived computation, not a set of fields.

For these, `elicit_bevy` provides **Generator implementations**:

```rust
/// Load a font from disk.
#[derive(Elicit)]
pub struct FontGenerator {
    /// Path to the font file (relative to the Bevy asset directory).
    pub path: String,
}

impl Generator for FontGenerator {
    type Target = bevy::text::Font;

    fn generate(&self) -> Self::Target {
        // reads the file, returns Font::try_from_bytes(...)
    }
}
```

The generator struct itself is the elicitable surface. Its `ToCodeLiteral` impl emits the
correct Bevy asset-loading expression (`asset_server.load(path)`) rather than trying to
inline the binary font data.

**3 generators**: `FontGenerator`, `TextureSliceGenerator`, and the atmosphere descriptor.

---

## Plugin Architecture

Plugin-level tools handle the parts of Bevy that cannot be reached through value-type
construction: **ECS wiring, app assembly, codegen macros, and incremental authoring workflows**.

### Stateless Fragment Plugins

These plugins emit code fragments. Each tool accepts parameters describing what to emit and
returns a `String` of Rust source code.

| Plugin | Tools | What It Covers |
|--------|-------|----------------|
| `BevyDerivePlugin` | 9 | `#[derive(Component)]`, `#[derive(Resource)]`, `#[derive(Bundle)]`, `#[derive(Asset)]`, `#[derive(Event)]`, `#[derive(States)]`, `#[derive(SystemSet)]`, `#[derive(ScheduleLabel)]`, `#[derive(Reflect)]` |
| `BevyEcsPlugin` | 19 | `app.add_systems()`, `commands.spawn()`, `commands.insert()`, query signatures, system combinator chains, resource initialization, event registration, system sets, run conditions |
| `BevyQueryPlugin` | 9 | `Query<D, F>`, `Res<R>`, `ResMut<R>`, `EventReader<E>`, `EventWriter<E>`, `Handle<A>`, `Local<T>`, `Time<T>`, full system signature assembly |
| `BevyRenderPlugin` | 89 | `StandardMaterial`, all light types, cameras, tonemapping, post-process settings, bloom, SSAO, TAA, atmosphere, volumetric fog, wireframe, mesh materials, color emission |
| `BevyUiPlugin` | 8 | `Node`, `UiRect`, `GridPlacement`, UI text/image components, button scaffolds, flex/grid container emitters |

### Stateful Workflow Plugins

These plugins maintain a **descriptor store** across multiple tool calls. The agent builds up
a descriptor incrementally, then emits the final `commands.spawn(...)` call when ready.
They implement `StatefulPlugin` and are backed by `Arc<Mutex<Ctx>>`.

| Plugin | Tools | What It Covers |
|--------|-------|----------------|
| `BevyAppPlugin` | 10 | `App` assembly: `add_plugin`, `add_systems`, `insert_resource`, `add_event`, `run` scaffolding; `DefaultPlugins` descriptors; full app skeleton emission |
| `BevyScenePlugin` | 5 | Scene manifests: stored entity/component descriptors, RON emission, `Commands` spawn-code emission for saved scenes |
| `BevyRenderWorkflowPlugin` | 10 | 3D/2D camera descriptors: configure `Camera3d`, projection, `RenderTarget`, `Tonemapping`, MSAA, bloom, depth-of-field; emit spawn tuple |
| `BevyRenderMeshWorkflowPlugin` | 8 | Mesh entity descriptors: configure `Mesh3d`/`Mesh2d`, typed material handles, wireframe; emit spawn tuple |
| `BevyRenderAtmosphereWorkflowPlugin` | 12 | `Atmosphere` + `ScatteringMedium` descriptors: ordered scattering-term edits, JSON inspection, block-expression emission |

### Trait Factories

`BevyTraitFactories` (`trait_factories.rs`) provides the marker-trait bridge for user-defined
types. When an agent declares `MyComponent` as a Bevy `Component`, it calls:

```rust
prime_bevy_component::<MyComponent>();
registry.register_type::<MyComponent>("my_component").await;
```

Factories are provided for: `Component`, `Resource`, `Asset`, `Bundle`, `Event`, `States`,
and `std::default::Default`.

---

## Intentional Exclusions

A small set of Bevy symbols are intentionally not shadowed:

| Excluded | Reason |
|----------|--------|
| `App`, `Plugin`, `PluginGroup` | Infrastructure types; `BevyAppPlugin` covers their usage surface via descriptor-registry tools instead |
| Schedule labels (`Update`, `Startup`, etc.) | Authoring surface covered by `BevyEcsPlugin` string-form schedule selectors; adding 40+ unit wrappers would add noise without capability |
| `Material`, `MaterialPlugin<M>`, `MeshMaterial3d<M>` | Generic traits; covered by the factory and workflow plugins that emit correctly-typed code without requiring monomorphization in the shadow layer |
| `ViewportNode` | Runtime-internal render graph type; not user-authored |

---

## Usage Example

An AI agent that wants to author a Bevy scene with a camera, a directional light, and a
textured mesh might proceed as follows:

```
// Step 1: Build the camera descriptor
bevy_render_workflow__create_camera_3d({})
bevy_render_workflow__set_tonemapping({ "tonemapping": "AcesFitted" })
bevy_render_workflow__set_bloom({ "intensity": 0.15 })

// Step 2: Add a directional light
bevy_render__directional_light({
    "illuminance": 10000.0,
    "shadows_enabled": true,
    "color": "LinearRgba(1.0, 0.98, 0.95, 1.0)"
})

// Step 3: Emit the spawn call
bevy_render_workflow__emit_spawn_code({ "var": "commands" })
// → commands.spawn((Camera3d::default(), ...))

// Step 4: Author the StandardMaterial
bevy_render__standard_material({
    "base_color": "LinearRgba(0.8, 0.6, 0.4, 1.0)",
    "metallic": 0.0,
    "perceptual_roughness": 0.7
})
```

Shadow type tools operate directly on the wrapper value. Plugin tools emit code strings.
Workflow plugins accumulate state and emit the final spawn expression.

---

## Relationship to the Elicitation Framework

`elicit_bevy` is a **shadow crate** in the elicitation framework. The layers are:

1. **`crates/elicitation`** — the core framework: `Elicit` derive, `ElicitComplete` trait,
   `ToCodeLiteral` trait, `Generator` trait, `StatefulPlugin` trait, `#[reflect_methods]`
   macro, `elicit_newtype!` macro
2. **`crates/elicitation/src/primitives/`** — primitive Bevy type support (math, color,
   transform) with `bevy-types` feature gate; these are trenchcoat wrappers for types that
   lack `JsonSchema` under orphan rules
3. **`crates/elicit_bevy`** (this crate) — the full shadow layer; 31 Bevy subcrates covered,
   ~32,000 lines, 10 MCP plugins, ~820 total agent-callable tools

See [`SHADOW_CRATE_MOTIVATION.md`](../../SHADOW_CRATE_MOTIVATION.md) and
[`THIRD_PARTY_SUPPORT_GUIDE.md`](../../THIRD_PARTY_SUPPORT_GUIDE.md) for the design
rationale behind the shadow crate architecture.

---

## License

Licensed under either of [Apache License, Version 2.0](../../LICENSE-APACHE) or
[MIT License](../../LICENSE-MIT) at your option.
