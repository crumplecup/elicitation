# ELICIT_BEVY_PLAN.md

## Goal

Complete elicitation coverage for Bevy 0.18.x — the dominant Rust game/simulation engine.

This enables MCP-driven game development, procedural scene authoring, ECS application
construction, and accessible tooling around Bevy's full API surface.

Three layers:

1. **Primitive integration** — serializable Bevy enums and structs in `crates/elicitation/`
   with a `bevy-types` feature gate
2. **Shadow crate** — `crates/elicit_bevy/` with MCP tools for ECS code generation, app
   construction, material/light/camera descriptors, and factory patterns for generic ECS params
3. **Proof coverage** — Kani/Creusot/Verus proofs for all Phase 2 types

## Version

**Target: Bevy 0.18.1** (latest stable as of this document).

Significant upstream API decisions:
- `Style` was folded into `Node` in 0.15 — all UI layout fields live on `Node`
- `Color` is an enum with distinct color-space variants (each a struct): `Srgba`, `LinearRgba`,
  `Hsla`, `Hsva`, `Hwba`, `Laba`, `Lcha`, `Oklaba`, `Oklcha`, `Xyza`
- Bevy's math is re-exported from `glam`; math types support serde already
- ECS derive macros (`Component`, `Resource`, `Bundle`, `Asset`, `Event`, `States`,
  `SystemSet`, `ScheduleLabel`) produce `Reflect`-compatible code when combined with `#[derive(Reflect)]`
- `RenderTarget` is now a **required component** (since 0.18) — not a field on `Camera`.
  `commands.spawn((Camera3d::default(), RenderTarget::Image(...)))` — affects 3G render tools.
- Camera, lights, mesh, and shader are now separate Bevy feature crates: `bevy_camera`,
  `bevy_light`, `bevy_mesh`, `bevy_shader` — must be listed individually in feature deps.
- Post-process effects live in `bevy_post_process` (bloom, depth of field, chromatic aberration).
- New high-level Cargo feature collections: `2d`, `3d`, `ui` — can be used instead of fine-grained lists.
- Feature renames in 0.18: `animation` → `gltf_animation`; `bevy_sprite_picking_backend` →
  `sprite_picking`; `bevy_ui_picking_backend` → `ui_picking`; `bevy_mesh_picking_backend` →
  `mesh_picking`.
- `#[reflect(...)]` now only supports parentheses (not braces/brackets) — code-gen tools must
  emit `#[reflect(Clone)]` not `#[reflect[Clone]]`.
- `SimpleExecutor` was removed in 0.18 — emit `SingleThreadedExecutor` for single-threaded code-gen.
- New 0.18 types to cover: `ScatteringMedium` asset, `Atmosphere` component,
  `FullscreenMaterial` trait, `FullscreenMaterialPlugin`, `RenderTarget` component.
- Picking is now first-party (`bevy_picking`) with sub-features `mesh_picking`, `sprite_picking`,
  `ui_picking` — picking types belong in Phase 2.

## Architecture: Three Elicitation Layers

### Why Bevy is Different

Bevy's ECS is the dominant runtime pattern. Most user code is not a self-contained struct —
it's a *system* (a function), a *plugin* (a bundle of systems), or an *entity* (a collection of
components). These cannot be serialized or snapshot-stored in a `HashMap<Uuid, T>` registry.

`elicit_bevy` is therefore a **code-generation + descriptor tool**, not a runtime state inspector.
The MCP tools produce Rust code strings that, when compiled, produce the described game behaviour.

### Five Elicitation Mechanisms Applied to Bevy

| Mechanism | Bevy Pattern | Examples |
|-----------|-------------|---------|
| **Select** | Fixed-variant enums | `WindowMode`, `PresentMode`, `KeyCode`, `MouseButton`, `BlendFactor`, `AlphaMode`, `TimerMode`, `Anchor`, `JustifyText` |
| **Survey** | Config structs with multiple fields | `Transform`, `Window`, `StandardMaterial`, `Timer`, `PlaybackSettings`, `DirectionalLight`, `Sprite`, `TextStyle` |
| **Fragment** | ECS macro/wiring code snippets | `#[derive(Component)]`, `app.add_systems()`, `commands.spawn()`, `app.insert_resource()` |
| **Factory** | Generic parameterized ECS types | `Query<D, F>`, `Handle<A>`, `Res<R>`, `ResMut<R>`, `EventWriter<E>`, `Time<T>`, `Local<T>` |
| **Descriptor-registry** | App-builder/plugin-registry types | `App`, `Plugin`, `PluginGroup`, `Schedule`, asset manifests, scene descriptors |

### Trenchcoat Pattern

Bevy types that lack `schemars::JsonSchema` (due to orphan rules) require local newtypes.
This applies to any `bevy::math` type that isn't already JSON-schema-capable and to ECS generics.
The trenchcoat pattern: wrap in a local newtype, impl the required traits on the wrapper,
then convert to/from the upstream type. See `crates/elicitation/src/primitives/rstar_types/`
for the reference implementation.

---

## Phase 1: Workspace Configuration

### 1.1 Add `bevy` to workspace dependencies

**File:** `Cargo.toml` (workspace root)

```toml
[workspace.dependencies]
bevy = { version = "0.18", default-features = false }
```

Use `default-features = false` — Bevy's default feature set pulls in rendering, windowing,
audio, and the full engine. Each consumer crate explicitly opts in to the features it needs.

`elicitation` needs only `bevy_math`, `bevy_color`, `bevy_transform`, and the relevant
sub-crates for the types it shadows. `elicit_bevy` includes the full engine surface.

### 1.2 Add `elicit_bevy` as a workspace member

**File:** `Cargo.toml` (workspace root)

```toml
[workspace]
members = [
    # ...existing members...
    "crates/elicit_bevy",
]
```

### 1.3 Create `crates/elicit_bevy/` skeleton

```
crates/elicit_bevy/
├── Cargo.toml
├── src/
│   ├── lib.rs            # mod + pub use only
│   ├── math.rs           # math type newtypes
│   ├── ecs.rs            # ECS fragment tools + derive factories
│   ├── app.rs            # App/Plugin descriptor-registry plugin
│   ├── render.rs         # Material/mesh/shader descriptor plugin
│   ├── ui.rs             # UI layout survey plugin
│   ├── scene.rs          # Scene descriptor plugin
│   └── factory.rs        # Query/Handle/Res factory plugin
└── tests/
    ├── math_test.rs
    ├── ecs_test.rs
    ├── app_test.rs
    ├── render_test.rs
    ├── ui_test.rs
    └── scene_test.rs
```

---

## Phase 2: elicitation Primitives (`crates/elicitation/`)

All new types live under `crates/elicitation/src/primitives/bevy_types/` and are
feature-gated with `#[cfg(feature = "bevy-types")]`. TypeSpec entries go in
`crates/elicitation/src/type_spec/bevy_specs.rs`.

### 2.1 bevy_math — Vector, Matrix, and Geometric Types

All `glam`-based math types re-exported from `bevy::math`. Most already implement serde
(verify at impl time). May need trenchcoat wrappers for schemars.

**Files:** `primitives/bevy_types/vec.rs`, `mat.rs`, `quat.rs`, `affine.rs`, `ray.rs`, `shapes.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Vec2` | Survey | x, y: f32 |
| `Vec3` | Survey | x, y, z: f32 |
| `Vec3A` | Survey | SIMD-aligned; trenchcoat needed |
| `Vec4` | Survey | x, y, z, w: f32 |
| `DVec2` | Survey | f64 variant |
| `DVec3` | Survey | f64 variant |
| `DVec4` | Survey | f64 variant |
| `IVec2` | Survey | i32 variant |
| `IVec3` | Survey | i32 variant |
| `IVec4` | Survey | i32 variant |
| `UVec2` | Survey | u32 variant |
| `UVec3` | Survey | u32 variant |
| `UVec4` | Survey | u32 variant |
| `BVec2` | Survey | bool variant |
| `BVec3` | Survey | bool variant |
| `BVec4` | Survey | bool variant |
| `Mat2` | Survey | 2x2 f32 matrix (col-major) |
| `Mat3` | Survey | 3x3 f32 matrix |
| `Mat3A` | Survey | SIMD-aligned 3x3; trenchcoat needed |
| `Mat4` | Survey | 4x4 f32 matrix |
| `DMat2` | Survey | 2x2 f64 |
| `DMat3` | Survey | 3x3 f64 |
| `DMat4` | Survey | 4x4 f64 |
| `Quat` | Survey | x, y, z, w: f32 |
| `DQuat` | Survey | f64 quaternion |
| `Affine2` | Survey | 2D affine transform |
| `Affine3A` | Survey | 3D affine transform; trenchcoat needed |
| `DAffine2` | Survey | f64 2D affine |
| `DAffine3` | Survey | f64 3D affine |
| `Ray2d` | Survey | origin: Vec2, direction: Dir2 |
| `Ray3d` | Survey | origin: Vec3, direction: Dir3 |
| `Dir2` | Survey | Unit-vector wrapper; trenchcoat needed |
| `Dir3` | Survey | Unit-vector wrapper; trenchcoat needed |
| `Dir3A` | Survey | SIMD unit-vector; trenchcoat needed |
| `Circle` | Survey | radius: f32 |
| `Ellipse` | Survey | half_size: Vec2 |
| `Triangle2d` | Survey | vertices: [Vec2; 3] |
| `Rectangle` | Survey | half_size: Vec2 |
| `RegularPolygon` | Survey | circumradius: f32, sides: usize |
| `Capsule2d` | Survey | radius: f32, half_length: f32 |
| `Capsule3d` | Survey | radius: f32, half_length: f32 |
| `Sphere` | Survey | radius: f32 |
| `Cuboid` | Survey | half_size: Vec3 |
| `Cylinder` | Survey | radius: f32, half_height: f32 |
| `Cone` | Survey | radius: f32, height: f32 |
| `ConicalFrustum` | Survey | top_radius, bottom_radius, height: f32 |
| `Torus` | Survey | minor_radius: f32, major_radius: f32 |
| `Line2d` | Survey | direction: Dir2 |
| `Line3d` | Survey | direction: Dir3 |
| `Segment2d` | Survey | direction: Dir2, half_length: f32 |
| `Segment3d` | Survey | two Vec3 endpoints |
| `Plane2d` | Survey | normal: Dir2 |
| `Plane3d` | Survey | normal: Dir3, half_size: Vec2 |

### 2.2 bevy_transform — Transform Types

**File:** `primitives/bevy_types/transform.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Transform` | Survey | translation: Vec3, rotation: Quat, scale: Vec3 |
| `GlobalTransform` | Survey | Affine3A internally; code-gen only (computed at runtime) |

### 2.3 bevy_color — Color Space Types

**File:** `primitives/bevy_types/color.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Color` | Select (outer enum) + Survey (inner structs) | Variants: Srgba, LinearRgba, Hsla, Hsva, Hwba, Laba, Lcha, Oklaba, Oklcha, Xyza |
| `Srgba` | Survey | red, green, blue, alpha: f32 |
| `LinearRgba` | Survey | red, green, blue, alpha: f32 |
| `Hsla` | Survey | hue, saturation, lightness, alpha: f32 |
| `Hsva` | Survey | hue, saturation, value, alpha: f32 |
| `Hwba` | Survey | hue, whiteness, blackness, alpha: f32 |
| `Laba` | Survey | lightness, a, b, alpha: f32 |
| `Lcha` | Survey | lightness, chroma, hue, alpha: f32 |
| `Oklaba` | Survey | lightness, a, b, alpha: f32 |
| `Oklcha` | Survey | lightness, chroma, hue, alpha: f32 |
| `Xyza` | Survey | x, y, z, alpha: f32 |
| `ClearColor` | Survey | wraps Color |
| `ClearColorConfig` | Survey (owned enum) | None / Custom(Color) — data variant needs owned enum trenchcoat |
| `LegacyColor` | deferred | deprecated; skip |

### 2.4 bevy_render — GPU Configuration Enums

**File:** `primitives/bevy_types/render_enums.rs`

These are the same wgpu-aligned enums that appear in both Bevy and wgpu. Do not duplicate
with `elicitation`'s `wgpu-types` impls — verify if Bevy re-exports from wgpu or defines
its own. Where they are the same type, the wgpu impl satisfies the Bevy need.

| Type | Elicitation | Notes |
|------|-------------|-------|
| `BlendFactor` | Select | vertex blend factors |
| `BlendOperation` | Select | Add, Subtract, ReverseSubtract, Min, Max |
| `BlendState` | Survey | src_factor, dst_factor, operation (color + alpha) |
| `BlendComponent` | Survey | factor pair + operation |
| `ColorWrites` | Survey | bitflags; trenchcoat needed |
| `CompareFunction` | Select | Never/Less/Equal/LessEqual/Greater/NotEqual/GreaterEqual/Always |
| `Face` | Select | Front/Back |
| `FrontFace` | Select | Ccw/Cw |
| `PrimitiveTopology` | Select | PointList/LineList/LineStrip/TriangleList/TriangleStrip |
| `PolygonMode` | Select | Fill/Line/Point |
| `TextureFormat` | Select | ~80 variants; re-use wgpu impl if same type |
| `TextureDimension` | Select | D1/D2/D3 |
| `TextureUsages` | Survey | bitflags; trenchcoat needed |
| `AddressMode` | Select | ClampToEdge/Repeat/MirrorRepeat/ClampToBorder |
| `FilterMode` | Select | Nearest/Linear |
| `VertexStepMode` | Select | Vertex/Instance |
| `IndexFormat` | Select | Uint16/Uint32 |
| `Tonemapping` | Select | None/Reinhard/ReinhardLuminance/AcesFitted/AgX/Blender/TonyMcMapface/BlenderFilmic |
| `AlphaMode` | Survey (owned enum) | Mask(f32) is a data variant — owned enum trenchcoat; `#[serde(tag="type", content="value")]` |
| `RenderLayers` | Survey | bitmask of render layers |

### 2.5 bevy_window — Window Configuration

**File:** `primitives/bevy_types/window.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `WindowMode` | Select | Windowed/BorderlessFullscreen/SizedFullscreen/Fullscreen |
| `PresentMode` | Select | AutoVsync/AutoNoVsync/Fifo/FifoRelaxed/Mailbox/Immediate |
| `WindowLevel` | Select | AlwaysOnBottom/Normal/AlwaysOnTop |
| `WindowTheme` | Select | Light/Dark |
| `CursorIcon` | Select | ~30 variants (Default/Crosshair/Hand/Text/etc.) |
| `MonitorSelection` | Survey (owned enum) | Index(usize)/Entity(Entity) carry data — owned enum trenchcoat |
| `WindowPosition` | Survey (owned enum) | At(IVec2) carries data — owned enum trenchcoat |
| `CompositeAlphaMode` | Select | Auto/Opaque/PreMultiplied/PostMultiplied/Inherit |
| `WindowResizeConstraints` | Survey | min/max width/height: f32 |
| `WindowResolution` | Survey | width, height: f32, scale_factor: f64 |
| `Window` | Survey | resolution, mode, present_mode, title, cursor, etc. |
| `EnabledButtons` | Survey | minimize, maximize, close: bool |
| `InternalWindowState` | deferred | internal; skip |

### 2.6 bevy_input — Input Configuration

**File:** `primitives/bevy_types/input.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `KeyCode` | Select | ~200 variants; verify 0.18 naming |
| `MouseButton` | Select | Left/Right/Middle/Back/Forward/Other(u16) |
| `ButtonState` | Select | Pressed/Released |
| `GamepadButtonType` | Select | South/East/North/West/LeftTrigger/RightTrigger/etc. |
| `GamepadAxisType` | Select | LeftStickX/Y/RightStickX/Y/LeftZ/RightZ |
| `TouchPhase` | Select | Started/Moved/Ended/Cancelled |
| `GamepadButton` | Survey | gamepad: Gamepad, button_type: GamepadButtonType |
| `GamepadAxis` | Survey | gamepad: Gamepad, axis_type: GamepadAxisType |
| `Gamepad` | Survey | id: usize |

### 2.7 bevy_ui — Layout Configuration

In Bevy 0.15+, `Style` was merged into `Node`. All layout fields live on `Node`.

**File:** `primitives/bevy_types/ui.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Val` | Survey (owned enum) | Px(f32)/Percent(f32)/Vw/Vh/VMin/VMax all carry f32 — owned enum trenchcoat |
| `Display` | Select | Flex/Grid/Block/None |
| `PositionType` | Select | Relative/Absolute |
| `FlexDirection` | Select | Row/Column/RowReverse/ColumnReverse |
| `FlexWrap` | Select | NoWrap/Wrap/WrapReverse |
| `AlignItems` | Select | Default/Start/End/Center/Baseline/Stretch |
| `AlignContent` | Select | Default/Start/End/Center/Stretch/SpaceBetween/SpaceEvenly/SpaceAround/FlexStart/FlexEnd |
| `AlignSelf` | Select | Auto/Start/End/Center/Baseline/Stretch |
| `JustifyContent` | Select | Default/Start/End/Center/Stretch/SpaceBetween/SpaceEvenly/SpaceAround/FlexStart/FlexEnd |
| `JustifyItems` | Select | Default/Start/End/Center/Baseline/Stretch |
| `JustifySelf` | Select | Auto/Start/End/Center/Baseline/Stretch |
| `Overflow` | Survey | x: OverflowAxis, y: OverflowAxis |
| `OverflowAxis` | Select | Visible/Clip/Hidden/Scroll |
| `Direction` | Select | Inherit/LeftToRight/RightToLeft |
| `GridAutoFlow` | Select | Row/Column/RowDense/ColumnDense |
| `UiRect` | Survey | left/right/top/bottom: Val |
| `BorderRadius` | Survey | top_left/top_right/bottom_left/bottom_right: Val |
| `BackgroundColor` | Survey | 0: Color |
| `BorderColor` | Survey | 0: Color |
| `Outline` | Survey | width/offset: Val, color: Color |
| `ZIndex` | Survey (owned enum) | Local(i32)/Global(i32) carry data — owned enum trenchcoat |
| `FocusPolicy` | Select | Block/Pass |
| `Visibility` | Select | Inherited/Hidden/Visible |
| `GridPlacement` | Survey | start, span, end: Option<i16> |
| `RepeatedGridTrack` | Survey | repetition, track: GridTrack — complex |
| `Node` | Survey | display, position_type, flex_direction, width, height, margin, padding, border, ... (many fields) |

### 2.8 bevy_time — Time and Timer Types

**File:** `primitives/bevy_types/time.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `TimerMode` | Select | Once/Repeating |
| `Timer` | Survey | duration: Duration, mode: TimerMode, elapsed: Duration, finished: bool |
| `Stopwatch` | Survey | elapsed: Duration, paused: bool |

### 2.9 bevy_audio — Audio Configuration

**File:** `primitives/bevy_types/audio.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `PlaybackSettings` | Survey | mode, volume, speed, paused, spatial, spatial_scale |
| `PlaybackMode` | Select | Once/Loop/Despawn/Remove |
| `Volume` | Survey | amplitude: f32 |
| `SpatialScale` | Survey | 0: Vec3 |
| `SpatialSettings` | Survey | gap, inner_angle, outer_angle (all f32) |

### 2.10 bevy_animation — Animation Configuration

**File:** `primitives/bevy_types/animation.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `RepeatAnimation` | Select | Never/Forever/Count(u32) |
| `TimingFunction` | Select | Linear/EaseIn/EaseOut/EaseInOut/CubicBezier/Steps |
| `AnimationTargetId` | Survey | uuid wrapper; code-gen only |

### 2.11 bevy_pbr — PBR Material and Light Types

**File:** `primitives/bevy_types/pbr.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `AlphaMode` | Select | (see render section — same type) |
| `CascadeShadowConfig` | Survey | bounds, overlap_proportion, minimum_distance, ... |
| `CascadeShadowConfigBuilder` | Survey | builder for CascadeShadowConfig |
| `FogFalloff` | Select | Linear/Exponential/ExponentialSquared/Atmospheric |
| `FogSettings` | Survey | color: Color, falloff: FogFalloff, directional_light_color, directional_light_exponent |
| `BloomSettings` | Survey | intensity, low_frequency_boost, high_pass_frequency, ... |
| `BloomCompositeMode` | Select | EnergyConserving/Additive |
| `ScreenSpaceAmbientOcclusionSettings` | Survey | quality_level: SsaoQuality |
| `SsaoQuality` | Select | Low/Medium/High/Ultra/Custom |
| `TemporalAntiAliasSettings` | Survey | reset_accumulation: bool |
| `StandardMaterial` | Survey | base_color, emissive, perceptual_roughness, metallic, alpha_mode, ... (many fields) |
| `DirectionalLight` | Survey | color: Color, illuminance: f32 |
| `PointLight` | Survey | color, intensity, range, radius, shadows_enabled, shadow_depth_bias, ... |
| `SpotLight` | Survey | color, intensity, range, radius, inner/outer_angle, shadows_enabled, ... |
| `AmbientLight` | Survey | color: Color, brightness: f32 |
| `DirectionalLightShadowMap` | Survey | size: usize |
| `PointLightShadowMap` | Survey | size: usize |
| `EnvironmentMapLight` | Survey | diffuse_map, specular_map handles + intensity |

### 2.12 bevy_sprite — 2D Sprite Types

**File:** `primitives/bevy_types/sprite.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Anchor` | Survey (owned enum) | Custom(Vec2) carries data — owned enum trenchcoat |
| `ImageScaleMode` | Select | Stretched/Tiled |
| `Sprite` | Survey | color: Color, custom_size: Option<Vec2>, flip_x/y: bool, anchor: Anchor |

### 2.13 bevy_text — Text Layout Types

**File:** `primitives/bevy_types/text.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `JustifyText` | Select | Left/Center/Right/Justified |
| `BreakLineOn` | Select | WordBoundary/AnyCharacter/NoWrap |
| `TextStyle` | Survey | font_size: f32, color: Color |

### 2.14 bevy_core_pipeline — Camera and Projection Types

**File:** `primitives/bevy_types/camera.rs`

Note: `RenderTarget` in 0.18 is a required component, not a `Camera` field. It is
`RenderTarget::Window(WindowRef::Primary)` by default but must be spawned explicitly
when targeting image handles or texture views.

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Viewport` | Survey | physical_position: UVec2, physical_size: UVec2, depth: Range<f32> |
| `OrthographicProjection` | Survey | near, far, scale, viewport_origin, scaling_mode, area |
| `PerspectiveProjection` | Survey | fov, aspect_ratio, near, far |
| `ScalingMode` | Select | Fixed/WindowSize/AutoMin/AutoMax/FixedVertical/FixedHorizontal |
| `RenderTarget` | Select | Window(WindowRef)/Image(Handle<Image>)/TextureView — now a required component |
| `WindowRef` | Select | Primary/Entity |
| `DepthCalculation` | Select | Distance/ZDifference |

### 2.15 bevy_pbr — New 0.18 Atmosphere Types

**File:** `primitives/bevy_types/atmosphere.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `Atmosphere` | Survey | wraps Handle<ScatteringMedium>; use earthlike() constructor |
| `ScatteringMedium` | Descriptor | complex asset with ScatteringTerm list; code-gen only |
| `ScatteringTerm` | Survey | absorption/scattering: Vec3, falloff: Falloff, phase: PhaseFunction |
| `Falloff` | Select | Exponential/Tent/Linear (with f32 params) |
| `PhaseFunction` | Select | Rayleigh/Mie(asymmetry)/Isotropic |

### 2.16 bevy_picking — Picking System Types (built-in since 0.15)

**File:** `primitives/bevy_types/picking.rs`

| Type | Elicitation | Notes |
|------|-------------|-------|
| `PointerButton` | Select | Primary/Secondary/Middle |
| `PickingBehavior` | Survey | should_block_lower: bool, is_hoverable: bool |
| `HitData` | Survey | camera: Entity, depth: f32, position/normal: Option<Vec3> |

### 2.17 TypeSpec module

**File:** `crates/elicitation/src/type_spec/bevy_specs.rs`

Contains `ElicitSpec` implementations for the complete inventory above. Follows same
pattern as `wgpu_specs.rs` — Select specs list variants, Survey specs list fields.

---

## Phase 3: Shadow Crate (`crates/elicit_bevy/`)

### Cargo.toml

```toml
[package]
name = "elicit_bevy"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
description = "Elicitation-enabled Bevy ECS tools for code generation and game development"
keywords = ["mcp", "bevy", "ecs", "game", "elicitation"]
categories = ["game-engines", "development-tools"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }

[dependencies]
elicitation = { workspace = true, features = ["bevy-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
bevy = { workspace = true, features = [
    "bevy_math",
    "bevy_color",
    "bevy_transform",
    "bevy_render",
    "bevy_camera",          # NEW in 0.18: camera + visibility types
    "bevy_light",           # NEW in 0.18: point/directional/spot lights
    "bevy_mesh",            # NEW in 0.18: mesh format
    "bevy_shader",          # NEW in 0.18: shaders as assets
    "bevy_window",
    "bevy_input",
    "bevy_ui",
    "bevy_time",
    "bevy_audio",
    "bevy_animation",
    "gltf_animation",       # RENAMED in 0.18 from "animation"
    "bevy_pbr",
    "bevy_post_process",    # NEW in 0.18: bloom, depth of field, chromatic aberration
    "bevy_anti_alias",
    "bevy_sprite",
    "bevy_text",
    "bevy_core_pipeline",
    "bevy_asset",
    "bevy_scene",
    "bevy_reflect",
    "bevy_picking",         # first-party picking, added in 0.15 stabilised in 0.18
    "bevy_state",
    "serialize",            # enables serde impls on bevy types
] }
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
derive_more = { workspace = true, features = ["display", "error"] }

[dev-dependencies]
tokio.workspace = true
serde_json.workspace = true
```

### Phase 3A: Core Type Conversions

Newtypes and `Into<bevy::*>` / `From<bevy::*>` conversions for all Phase 2 types
that required trenchcoats (Vec3A, Mat3A, Affine3A, Dir2/3/Dir3A, ColorWrites, TextureUsages).

The trenchcoat wrappers live in `src/math.rs` and use `elicit_newtype!`.

### Phase 3B: ECS Derive Macro Factories (`#[reflect_trait]`)

These tools receive a struct/enum name and optional fields, and emit Rust code adding
ECS derive traits to that type.

**Plugin:** `BevyDerivePlugin` (prefix: `bevy_derive__`)

| Tool | Description |
|------|-------------|
| `bevy_derive__component` | Emit `#[derive(Component)]` for a named struct/enum |
| `bevy_derive__resource` | Emit `#[derive(Resource)]` |
| `bevy_derive__asset` | Emit `#[derive(Asset, TypePath)]` |
| `bevy_derive__event` | Emit `#[derive(Event)]` |
| `bevy_derive__bundle` | Emit `#[derive(Bundle)]` with field list |
| `bevy_derive__states` | Emit `#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]` |
| `bevy_derive__system_set` | Emit `#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]` |
| `bevy_derive__schedule_label` | Emit `#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]` |
| `bevy_derive__reflect` | Emit `#[derive(Reflect)]` with optional type_path |
| `bevy_derive__animate` | Emit `#[derive(Animatable)]` for interpolatable types |

Each tool returns a `CodeBlock` containing the derive attributes, with an optional
full struct skeleton when the user also supplies field definitions.

**0.18 reflect syntax note**: `#[reflect(...)]` now only accepts parentheses.
All code-gen tools must emit `#[reflect(Clone)]` — never `#[reflect[Clone]]` or `#[reflect{Clone}]`.

### Phase 3C: ECS Workflow Plugin — System and App Wiring

Fragment tools for common ECS wiring patterns. Each tool receives parameters and
emits the Rust code snippet.

**Plugin:** `BevyEcsPlugin` (prefix: `bevy_ecs__`)

| Tool | Description |
|------|-------------|
| `bevy_ecs__add_systems` | `app.add_systems(Schedule, system_fn)` with ordering/condition |
| `bevy_ecs__add_plugins` | `app.add_plugins(PluginType)` single or tuple |
| `bevy_ecs__insert_resource` | `app.insert_resource(ResourceValue)` |
| `bevy_ecs__init_resource` | `app.init_resource::<T>()` |
| `bevy_ecs__add_event` | `app.add_event::<EventType>()` |
| `bevy_ecs__register_type` | `app.register_type::<T>()` |
| `bevy_ecs__spawn_entity` | `commands.spawn((Component1, Component2, ...))` |
| `bevy_ecs__spawn_bundle` | `commands.spawn(BundleType { ... })` |
| `bevy_ecs__with_children` | `commands.spawn(...).with_children(|parent| { ... })` |
| `bevy_ecs__insert_component` | `commands.entity(id).insert(component)` |
| `bevy_ecs__remove_component` | `commands.entity(id).remove::<T>()` |
| `bevy_ecs__despawn` | `commands.entity(id).despawn()` / `despawn_recursive()` |
| `bevy_ecs__query_for` | Type signature for a `Query<(&T, &mut U), With<V>>` system parameter |
| `bevy_ecs__run_criteria` | `system.run_if(condition)` |
| `bevy_ecs__in_set` | `system.in_set(MySet::Name)` |
| `bevy_ecs__chain` | `.chain()` system combinator |
| `bevy_ecs__pipe` | `system_a.pipe(system_b)` |
| `bevy_ecs__observer` | `app.add_observer(on_event_fn)` |
| `bevy_ecs__trigger` | `commands.trigger(EventValue)` |

### Phase 3D: EmitCode for All Workflow Tools

`EmitCode` impls for all 3C tools. Each returns Rust source code as a string, with
imports included at the top. Following the `elicit_wgpu` pattern — tools are code-gen,
not runtime execution.

### Phase 3E: Descriptor-Registry Plugin — App, Plugin, Schedule

Registry-style plugin where MCP calls build up a `HashMap<String, AppDescriptor>`
and can emit the full app initialization code.

**Plugin:** `BevyAppPlugin` (prefix: `bevy_app__`)

| Tool | Description |
|------|-------------|
| `bevy_app__new` | Create a new App descriptor with a given name |
| `bevy_app__add_default_plugins` | Add `DefaultPlugins` with WindowPlugin configuration |
| `bevy_app__add_plugin` | Register a named plugin in the app descriptor |
| `bevy_app__add_schedule` | Define a custom schedule and its ordering |
| `bevy_app__set_runner` | Set the app runner function |
| `bevy_app__emit` | Emit the complete `fn main()` + `App::new()` code block |
| `bevy_app__plugin_struct` | Emit a `struct MyPlugin; impl Plugin for MyPlugin { ... }` skeleton |
| `bevy_app__plugin_group` | Emit a `struct MyGroup; impl PluginGroup for MyGroup { ... }` skeleton |
| `bevy_app__state_machine` | Emit a `States` enum + `OnEnter/OnExit` system wiring |

**Plugin:** `BevyScenePlugin` (prefix: `bevy_scene__`)

| Tool | Description |
|------|-------------|
| `bevy_scene__new` | Create a new DynamicScene descriptor |
| `bevy_scene__add_entity` | Add an entity with components to the scene |
| `bevy_scene__add_resource` | Add a resource to the scene |
| `bevy_scene__emit_ron` | Emit the scene as RON format |
| `bevy_scene__emit_spawn_code` | Emit `commands.spawn_scene()` code |

### Phase 3F: Factory Plugins for Generic ECS Types

**Plugin:** `BevyQueryPlugin` (prefix: `bevy_query__`)

These factories work like `elicit_rstar`'s factory: the user registers a type set
at runtime, and the plugin emits typed code for `Query<>`, `Res<>`, `Handle<>` etc.

| Tool | Description |
|------|-------------|
| `bevy_query__define_component_query` | Emit a `Query<&T>` or `Query<(&T, &U)>` type annotation |
| `bevy_query__define_resource` | Emit a `Res<T>` or `ResMut<T>` system parameter |
| `bevy_query__define_event_reader` | Emit an `EventReader<E>` system parameter |
| `bevy_query__define_event_writer` | Emit an `EventWriter<E>` system parameter |
| `bevy_query__define_handle` | Emit a `Handle<A>` field declaration |
| `bevy_query__define_local` | Emit a `Local<T>` system parameter |
| `bevy_query__define_time` | Emit `Res<Time>`, `Res<Time<Fixed>>`, etc. |
| `bevy_query__system_signature` | Emit a complete system function signature with multiple params |
| `bevy_query__filter` | Emit a query filter: `With<T>`, `Without<T>`, `Added<T>`, `Changed<T>` |

### Phase 3G: Render and Material Plugin

**Plugin:** `BevyRenderPlugin` (prefix: `bevy_render__`)

Survey tools for render configuration types.

Note: In 0.18, `RenderTarget` is a **required component** spawned alongside `Camera3d`/`Camera2d`,
not a field on `Camera`. Code-gen tools must emit `commands.spawn((Camera3d::default(), RenderTarget::Window(WindowRef::Primary)))` pattern.

| Tool | Description |
|------|-------------|
| `bevy_render__standard_material` | Survey for StandardMaterial (30+ fields) |
| `bevy_render__directional_light` | Survey for DirectionalLight |
| `bevy_render__point_light` | Survey for PointLight |
| `bevy_render__spot_light` | Survey for SpotLight |
| `bevy_render__ambient_light` | Survey for AmbientLight |
| `bevy_render__fog_settings` | Survey for FogSettings |
| `bevy_render__bloom_settings` | Survey for BloomSettings (now in bevy_post_process) |
| `bevy_render__camera_3d` | Survey for Camera3d + PerspectiveProjection + RenderTarget |
| `bevy_render__camera_2d` | Survey for Camera2d + OrthographicProjection + RenderTarget |
| `bevy_render__viewport` | Survey for Viewport |
| `bevy_render__color` | Survey for Color (color-space selector + fields) |
| `bevy_render__sprite` | Survey for Sprite |
| `bevy_render__text_style` | Survey for TextStyle |
| `bevy_render__tonemapping` | Select for Tonemapping algorithm |
| `bevy_render__alpha_mode` | Select/Survey for AlphaMode |
| `bevy_render__atmosphere` | Survey for Atmosphere component + ScatteringMedium asset (0.18) |
| `bevy_render__fullscreen_material` | Fragment: impl FullscreenMaterial boilerplate (0.18) |
| `bevy_render__render_target` | Survey for RenderTarget component (Window/Image/TextureView) |

### Phase 3H: UI Layout Plugin

**Plugin:** `BevyUiPlugin` (prefix: `bevy_ui__`)

| Tool | Description |
|------|-------------|
| `bevy_ui__node` | Survey for Node (all layout CSS fields) |
| `bevy_ui__ui_rect` | Survey for UiRect (margin, padding, border) |
| `bevy_ui__grid_placement` | Survey for GridPlacement |
| `bevy_ui__text` | Survey for Text component |
| `bevy_ui__image` | Survey for UiImage component |
| `bevy_ui__button_bundle` | Emit code for a Button entity with Node + interaction |
| `bevy_ui__flex_container` | Emit code for a flex container Node |
| `bevy_ui__grid_container` | Emit code for a CSS grid Node |

---

## Phase 4: Kani Proofs

**Directory:** `crates/elicitation_kani/src/bevy/`

Coverage targets:
- All math type constructors (Vec2/3/4, Mat2/3/4, Quat): roundtrip serde, NaN/inf guards
- Transform composition: `compose_from_parts` vs `Transform::from_xyz` equivalence
- Color channel bounds: all f32 fields in [0.0, 1.0] after construction
- UiRect::all / UiRect::axes: correct field assignment
- Timer: elapsed never exceeds duration

---

## Phase 5: Creusot Proofs

**Directory:** `crates/elicitation_creusot/src/bevy/`

Post-conditions for Survey types:
- `Vec3::normalize`: output length in [0.999, 1.001]
- `Transform::looking_at`: forward direction aligns with (target - self.translation)
- `Timer::tick`: elapsed increases monotonically
- `Window::default()`: present_mode == AutoVsync, mode == Windowed

---

## Phase 6: Verus Proofs

**Directory:** `crates/elicitation_verus/src/bevy/`

Invariants for config structs:
- `StandardMaterial`: perceptual_roughness in [0.0, 1.0], metallic in [0.0, 1.0]
- `PlaybackSettings`: speed > 0.0
- `Viewport`: physical_size both non-zero

---

## Type Count Summary

| Module | Select | Survey | Fragment | Factory | Descriptor | Total |
|--------|--------|--------|----------|---------|------------|-------|
| bevy_math | 0 | 47 | 0 | 0 | 0 | 47 |
| bevy_transform | 0 | 2 | 0 | 0 | 0 | 2 |
| bevy_color | 1 | 10 | 0 | 0 | 0 | 11 |
| bevy_render enums | 9 | 6 | 0 | 0 | 0 | 15 |
| bevy_window | 6 | 5 | 0 | 0 | 0 | 11 |
| bevy_input | 5 | 3 | 0 | 0 | 0 | 8 |
| bevy_ui | 17 | 10 | 0 | 0 | 0 | 27 |
| bevy_time | 1 | 2 | 0 | 0 | 0 | 3 |
| bevy_audio | 1 | 3 | 0 | 0 | 0 | 4 |
| bevy_animation | 2 | 1 | 0 | 0 | 0 | 3 |
| bevy_pbr | 2 | 11 | 0 | 0 | 0 | 13 |
| bevy_sprite | 1 | 2 | 0 | 0 | 0 | 3 |
| bevy_text | 2 | 1 | 0 | 0 | 0 | 3 |
| bevy_camera | 3 | 3 | 0 | 0 | 0 | 6 |
| Shadow crate — ECS derive | 0 | 0 | 10 | 0 | 0 | 10 |
| Shadow crate — ECS wiring | 0 | 0 | 20 | 0 | 0 | 20 |
| Shadow crate — App/Scene | 0 | 0 | 0 | 0 | 14 | 14 |
| Shadow crate — Factory | 0 | 0 | 0 | 9 | 0 | 9 |
| Shadow crate — Render | 0 | 15 | 0 | 0 | 0 | 15 |
| Shadow crate — UI | 0 | 8 | 0 | 0 | 0 | 8 |
| **Total** | **50** | **129** | **30** | **9** | **14** | **232** |

---

## Deferred / Out of Scope

| Item | Reason |
|------|--------|
| Live ECS World inspection | Runtime state cannot be serialized through MCP |
| `bevy_ecs::world::World` as a live tool | Not serializable; use descriptor-registry pattern only |
| Bevy Reflect runtime queries | Requires live Bevy app process — out of scope |
| WASM/WebGL rendering | Platform-specific; out of scope for code-gen tools |
| Bevy Editor integration | Future work; blocked on official Editor stability |
| `bevy_egui` | Separate `elicit_bevy_egui` crate if needed |
| Async system tasks (`Task<T>`) | Requires runtime; deferred |
| `bevy_lunex` / third-party Bevy plugins | Out of scope for core `elicit_bevy` |
| Physics (`bevy_rapier`, `avian`) | Separate elicit crates |
| Networking (`bevy_replicon`, `lightyear`) | Separate elicit crates |
| `bevy_solari` (ray-traced renderer) | Experimental; defer until stable |
| `SimpleExecutor` | Removed in Bevy 0.18; emit `SingleThreadedExecutor` if needed |
| Bevy Remote Protocol (`bevy_remote`) | Live network protocol; out of scope |
| `bevy_gizmos` | Debug-only; low priority for first release |

---

## Implementation Order

Follow phases strictly. Each phase unblocks the next:

1. **Phase 1** (workspace) — enables cargo check to pass for the new crate
2. **Phase 2.1** (math) — foundational; transform/color/render all depend on Vec/Mat/Quat
3. **Phase 2.2** (transform) — depends on Phase 2.1
4. **Phase 2.3** (color) — independent; unblocks pbr/render/ui
5. **Phase 2.4–2.15** (render, window, input, ui, time, audio, animation, pbr, sprite, text, camera) — parallel
6. **Phase 3A** (newtypes) — depends on Phase 2 complete
7. **Phase 3B–3H** (shadow crate plugins) — parallel within Phase 3
8. **Phases 4–6** (proofs) — depend on Phase 2 types

Within Phase 2, the ordering is:

```
math → transform → color → render_enums → window → input → ui → time → audio → animation → pbr → sprite → text → camera → typespec
```

---

## Key Design Decisions

### 1. Code-gen only, no runtime state

Bevy's ECS World state cannot be serialized as JSON for MCP transmission. All `elicit_bevy`
tools are code generators — they produce Rust source code, not live system interactions.
The exception is static config types (StandardMaterial, Transform, etc.) which are fine.

### 2. Trenchcoat wrappers for SIMD and bitflag types

`Vec3A`, `Mat3A`, `Affine3A`, `Dir2/3/3A`, `ColorWrites`, `TextureUsages`, `RenderLayers`
all need trenchcoat wrappers to satisfy `schemars::JsonSchema` and `Serialize`/`Deserialize`
under orphan rules.

### 3. Color is both Select and Survey

`Color` the enum has a Select mechanism (choosing the color space), but each variant
(e.g. `Color::Srgba(Srgba { ... })`) is a Survey of its fields. The elicitation should
present a two-step process: first select the color space, then survey the component values.

### 4. `Node` (UI) is the merged successor to `Style`

In Bevy 0.15+, the `Style` component was merged into `Node`. The `Node` survey tool
covers all CSS flexbox/grid layout fields. Do not create separate style/node tools.

### 5. Factory pattern for ECS generics

`Query<D, F>`, `Handle<A>`, `Res<R>` cannot have static JSON schemas — they are generic.
The factory tools use string parameters for type names and emit correctly typed code.
No runtime registry (unlike `elicit_rstar`) is needed because these are code-gen outputs.

---

## Reference Implementations

| What | Where | Why |
|------|-------|-----|
| Code-gen only, no live state | `elicit_wgpu` | Same pattern: GPU descriptors are code-gen |
| Descriptor-registry | `elicit_tower` / `elicit_sqlx` | App/Plugin registry pattern |
| Factory with generic ECS params | `elicit_rstar` | Factory seam: T: ElicitComplete + bound |
| Trenchcoat for orphan types | `crates/elicitation/src/primitives/rstar_types/` | Newtype + trait impl pattern |
| Comprehensive enum coverage | `elicit_winit` | Large enum (KeyCode, etc.) coverage |
