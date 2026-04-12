# ELICIT_WINIT_PLAN.md

## Goal

Add winit support to elicitation as the windowing/input alphabet:
1. **Core type integration** — winit types in `elicitation` with `winit-types` feature
2. **Shadow crate** — `elicit_winit` with code-generation MCP tools for windowing and input
3. **Proof wiring** — Kani / Creusot / Verus structural proofs for the core types

## Key Constraint: Emit-Only

`winit` manages OS windows, event loops, and raw input. None of these can be
created in an MCP server context (no display, no OS window manager access).
All `elicit_winit` tools are therefore **emit-only** — they generate Rust code
that the user pastes into their native application.  This is identical to how
`elicit_tokio` treats `spawn` / `spawn_blocking`.

---

## Phase 1: Workspace Configuration

### 1.1 Add winit to workspace deps (root `Cargo.toml`)

```toml
# Windowing
winit = { version = "0.30", features = ["serde"] }
```

Note: the `serde` feature cascades to `cursor-icon/serde`, `dpi/serde`, and
`smol_str/serde`, enabling `Serialize` / `Deserialize` on nearly all winit
enum types.

### 1.2 Add workspace member + dependency

```toml
# members
"crates/elicit_winit",

# [workspace.dependencies]
elicit_winit = { path = "crates/elicit_winit", version = "0.10.0" }
```

---

## Phase 2: Core Type Integration in `elicitation`

### 2.1 Cargo.toml changes (`crates/elicitation`)

```toml
# optional dependency
winit = { workspace = true, optional = true }

# feature
winit-types = ["dep:winit"]
```

Include `"winit-types"` in the `full` bundle.

### 2.2 Module layout

```
crates/elicitation/src/primitives/winit_types/
├── mod.rs            ← pub use all + module doc
├── trenchcoats.rs    ← select_trenchcoat! wrappers for all serde enums
└── dpi.rs            ← WinitPhysicalSize, WinitLogicalSize, WinitWindowAttributes
```

### 2.3 Select-trenchcoat wrappers (`trenchcoats.rs`)

All of these use the `serde` variant — the underlying winit types already
derive `Serialize` / `Deserialize` when `winit/serde` is enabled.

| Wrapper | Source type |
|---------|-------------|
| `WinitCursorIconSelect` | `winit::window::CursorIcon` (re-exported from cursor-icon) |
| `WinitWindowLevelSelect` | `winit::window::WindowLevel` |
| `WinitThemeSelect` | `winit::window::Theme` |
| `WinitMouseButtonSelect` | `winit::event::MouseButton` |
| `WinitElementStateSelect` | `winit::event::ElementState` |
| `WinitTouchPhaseSelect` | `winit::event::TouchPhase` |
| `WinitKeyCodeSelect` | `winit::keyboard::KeyCode` |

Pattern (same as `egui_types/trenchcoats.rs`):
```rust
crate::select_trenchcoat!(winit::window::WindowLevel, as WinitWindowLevelSelect, serde);
crate::select_trenchcoat_traits!(WinitWindowLevelSelect, winit::window::WindowLevel, [copy, eq, hash]);
```

### 2.4 DPI and attributes structs (`dpi.rs`)

`dpi::PhysicalSize<P>` is generic — the macro can't wrap it directly.  Define
independent serializable structs instead:

```rust
/// Serializable physical-pixel size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WinitPhysicalSize { pub width: u32, pub height: u32 }

impl From<WinitPhysicalSize> for winit::dpi::PhysicalSize<u32> {
    fn from(s: WinitPhysicalSize) -> Self { Self::new(s.width, s.height) }
}

/// Serializable logical (DPI-aware) size.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WinitLogicalSize { pub width: f64, pub height: f64 }

impl From<WinitLogicalSize> for winit::dpi::LogicalSize<f64> {
    fn from(s: WinitLogicalSize) -> Self { Self::new(s.width, s.height) }
}

/// Serializable logical position.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WinitLogicalPosition { pub x: f64, pub y: f64 }

/// Flat serializable window-creation config (mirrors `winit::window::WindowAttributes`).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WinitWindowAttributes {
    pub title: String,
    pub inner_size: Option<WinitLogicalSize>,
    pub min_inner_size: Option<WinitLogicalSize>,
    pub max_inner_size: Option<WinitLogicalSize>,
    pub position: Option<WinitLogicalPosition>,
    pub resizable: Option<bool>,
    pub maximized: Option<bool>,
    pub visible: Option<bool>,
    pub transparent: Option<bool>,
    pub decorations: Option<bool>,
    pub fullscreen: Option<bool>,
    pub window_level: Option<WinitWindowLevelSelect>,
    pub theme: Option<WinitThemeSelect>,
    pub active: Option<bool>,
}
```

`WinitWindowAttributes` implements `ToCodeLiteral` (generates a
`WindowAttributes::default().with_title(...)` builder chain) but does NOT
implement runtime conversion — it's only a code-generation input.

### 2.5 Type-spec and tests

- `crates/elicitation/src/type_spec/winit_specs.rs` — TypeSpec entries for each type
- `crates/elicitation/tests/winit_types_test.rs` — roundtrip tests
- `crates/elicitation/tests/proof_non_empty_test.rs` — winit_tests module

---

## Phase 3: Shadow Crate `elicit_winit`

### 3.1 Directory layout

```
crates/elicit_winit/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── error.rs          ← WinitCodeError / WinitCodeResult
│   └── workflow/
│       ├── mod.rs        ← json_result + pub use
│       ├── window_plugin.rs    ← 8 tools: winit_window__*
│       ├── event_plugin.rs     ← 7 tools: winit_event__*
│       └── input_plugin.rs     ← 7 tools: winit_input__*
└── tests/
    └── workflow_test.rs
```

### 3.2 Cargo.toml

```toml
[dependencies]
elicitation = { workspace = true, features = ["winit-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
winit = { workspace = true }         # for type references in code output
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
derive_more = { workspace = true, features = ["display", "error"] }
```

### 3.3 Plugin surface

#### `winit_window` (8 tools)

All return emitted Rust code strings.

| Tool | Params | Emits |
|------|--------|-------|
| `window__attributes` | `WinitWindowAttributes` | `WindowAttributes::default()...` builder chain |
| `window__set_title` | `title: String` | `window.set_title(...)` |
| `window__set_visible` | `visible: bool` | `window.set_visible(...)` |
| `window__set_resizable` | `resizable: bool` | `window.set_resizable(...)` |
| `window__set_decorations` | `decorations: bool` | `window.set_decorations(...)` |
| `window__set_window_level` | `WinitWindowLevelSelect` | `window.set_window_level(...)` |
| `window__set_fullscreen` | `enabled: bool` | fullscreen/windowed toggle code |
| `window__request_redraw` | — | `window.request_redraw()` |

#### `winit_event` (7 tools)

| Tool | Params | Emits |
|------|--------|-------|
| `event__app_skeleton` | `struct_name: String` | full `ApplicationHandler` impl skeleton |
| `event__event_loop` | `app_name: String` | `EventLoop::new()` + `run_app` boilerplate |
| `event__resumed` | `struct_name: String` | `fn resumed(...)` body with window creation |
| `event__window_event_dispatch` | `events: Vec<String>` | `fn window_event(...)` match arms |
| `event__about_to_wait` | `body: String` | `fn about_to_wait(...)` body |
| `event__close_requested` | — | `WindowEvent::CloseRequested` handler |
| `event__resized` | — | `WindowEvent::Resized(size)` handler |

#### `winit_input` (7 tools)

| Tool | Params | Emits |
|------|--------|-------|
| `input__keyboard_handler` | `key: WinitKeyCodeSelect, body: String` | keyboard match arm for a key code |
| `input__named_key_handler` | `key_name: String, body: String` | named-key match arm |
| `input__mouse_button_handler` | `button: WinitMouseButtonSelect, state: WinitElementStateSelect, body: String` | mouse button match arm |
| `input__cursor_moved` | `body: String` | `CursorMoved` match arm |
| `input__scroll_handler` | `body: String` | `MouseWheel` match arm |
| `input__touch_handler` | `phase: WinitTouchPhaseSelect, body: String` | touch match arm |
| `input__set_cursor_icon` | `icon: WinitCursorIconSelect` | `window.set_cursor(CursorIcon::...)` |

### 3.4 Propositions

```rust
#[derive(Prop)] pub struct WinitWindowConfigured;
#[derive(Prop)] pub struct WinitEventLoopReady;
#[derive(Prop)] pub struct WinitInputHandlerReady;
```

---

## Phase 4: Proof Wiring

Following the `rstar`/`proj` pattern:

- `crates/elicitation_kani/src/winit_types.rs` — proofs for WinitPhysicalSize, WinitLogicalSize, WinitWindowAttributes
- `crates/elicitation_creusot/src/winit_types.rs` — trusted axioms
- `crates/elicitation_verus/src/winit_types.rs` — u32/u64 shadow structs

Feature: `winit-types = ["dep:winit", "elicitation/winit-types"]` in kani and creusot.

Runner entries in `runner.rs`, `creusot_runner.rs`, `verus_runner.rs`.

---

## Phase 5: Validation

```bash
just check elicit_winit
just test-package elicit_winit
just check-all elicit_winit
cargo check -p elicitation --features winit-types
cargo test -p elicitation --features winit-types --test winit_types_test
cargo test -p elicitation --features winit-types --test proof_non_empty_test
cargo check -p elicitation_kani --features 'kani,winit-types'
cargo check -p elicitation_creusot --features 'winit-types'
cargo check --manifest-path crates/elicitation_verus/Cargo.toml
```

---

## Implementation Order

1. Phase 1: workspace config
2. Phase 2: `elicitation` winit-types primitives
3. Phase 3: `elicit_winit` shadow crate (3 plugins, 22 tools)
4. Phase 4: proof wiring
5. Phase 5: validate + commit


## Goal
Add complete winit support to elicitation as windowing alphabet:
1. **Core type integration** — winit types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_winit` with MCP tools for windowing and input

## Architecture Overview

Following established patterns from elicit_chrono, elicit_tokio, elicit_url:
- **Core**: Feature-gated winit types with Select enums and Elicitation impls
- **Shadow crate**: ~6-8 workflow plugins covering window management, events, input
- **Windowing alphabet**: Foundation for native PC applications (pair with elicit_wgpu for rendering)

## API Coverage

winit provides:
- **Window management** (~40 methods): creation, configuration, sizing, decorations
- **Event loop** (~15 types): WindowEvent, DeviceEvent, lifecycle
- **Input handling** (~25 types): keyboard, mouse, touch, gamepad
- **Platform integration** (~20 methods): monitor info, cursor, fullscreen

**Total API surface**: ~100 types/methods → ~40-60 MCP tools across 6-8 plugins

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add winit to workspace dependencies**:
```toml
# Windowing
winit = { version = "0.30" }
```

**1.2 Add elicit_winit member**:
```toml
  "crates/elicit_winit",
```

**1.3 Add elicit_winit workspace dependency**:
```toml
elicit_winit = { path = "crates/elicit_winit", version = "0.9.1" }
```

**1.4 Add winit feature to elicitation**:
- Add optional dependency: `winit = { workspace = true, optional = true }`
- Add feature: `winit = ["dep:winit"]`
- Update `full` feature to include `"winit"`

## Phase 2: Core Type Integration

### Files to create/modify:
- `crates/elicitation/src/winit_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Simple Enums** (use `select_trenchcoat!` macro):
- CursorIcon (~20 variants)
- WindowLevel (AlwaysOnBottom, Normal, AlwaysOnTop)
- Fullscreen (Exclusive, Borderless)
- Theme (Light, Dark)
- MouseButton (Left, Right, Middle, Other)
- ElementState (Pressed, Released)

**2.2 Complex Types** (manual `Elicitation` impl):
- WindowAttributes (builder pattern)
- PhysicalSize/PhysicalPosition
- LogicalSize/LogicalPosition
- KeyEvent (with modifiers)
- MouseScrollDelta

### Implementation Pattern:

```rust
// crates/elicitation/src/winit_types.rs
#![cfg(feature = "winit")]

use winit::window::CursorIcon;

// Simple enums
select_trenchcoat!(winit::window::CursorIcon, as CursorIconSelect, serde);
select_trenchcoat!(winit::window::WindowLevel, as WindowLevelSelect, serde);

// Complex types
impl Elicitation for winit::dpi::PhysicalSize<u32> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for width, height
    }
}
```

**2.3 Export from lib.rs**:
```rust
#[cfg(feature = "winit")]
pub mod winit_types;

#[cfg(feature = "winit")]
pub use winit_types::{CursorIconSelect, WindowLevelSelect, /* ... */};
```

## Phase 3: Create elicit_winit Shadow Crate

### Directory Structure:

```
crates/elicit_winit/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── window.rs           (Window wrapper)
│   ├── event_loop.rs       (EventLoop wrapper)
│   ├── monitor.rs          (MonitorHandle wrapper)
│   └── workflow/
│       ├── mod.rs
│       ├── window_plugin.rs        (~12 tools: create, configure, resize, etc.)
│       ├── event_plugin.rs         (~10 tools: event loop, polling)
│       ├── input_plugin.rs         (~8 tools: keyboard, mouse events)
│       ├── cursor_plugin.rs        (~6 tools: cursor control, grab)
│       ├── monitor_plugin.rs       (~5 tools: monitor info, video modes)
│       └── workflow_plugin.rs      (~8 tools: common patterns)
└── tests/
    └── winit_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_winit"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled winit wrappers with MCP tools for windowing and input"
keywords = ["mcp", "winit", "windowing", "input", "elicitation"]
categories = ["gui", "os", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["winit"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
winit = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure:

```rust
//! `elicit_winit` — comprehensive winit API exposure via MCP tools.
//!
//! Provides windowing alphabet for native applications:
//! - Window creation and management
//! - Event loop and input handling
//! - Monitor and display configuration
//! - Cursor control
//!
//! # Plugin Organization (6 plugins, ~49 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `WinitWindowPlugin` | 12 | Window creation, configuration, sizing |
//! | `WinitEventPlugin` | 10 | Event loop, polling, dispatch |
//! | `WinitInputPlugin` | 8 | Keyboard, mouse, touch events |
//! | `WinitCursorPlugin` | 6 | Cursor control, grab, visibility |
//! | `WinitMonitorPlugin` | 5 | Monitor info, video modes |
//! | `WinitWorkflowPlugin` | 8 | Common patterns and compositions |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod event_loop;
mod monitor;
mod window;
pub mod workflow;

pub use event_loop::EventLoop;
pub use monitor::MonitorHandle;
pub use window::Window;
pub use workflow::{
    WinitCursorPlugin, WinitEventPlugin, WinitInputPlugin,
    WinitMonitorPlugin, WinitWindowPlugin, WinitWorkflowPlugin,
};
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Window wrapper (window.rs):

```rust
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(winit::window::Window, as Window, serde);

#[reflect_methods]
impl Window {
    #[instrument(skip(self))]
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    #[instrument(skip(self))]
    pub fn set_visible(&self, visible: bool) {
        self.0.set_visible(visible);
    }

    // ... more methods
}
```

### 4.2 EventLoop wrapper (event_loop.rs):

```rust
elicit_newtype!(winit::event_loop::EventLoop<()>, as EventLoop, serde);

#[reflect_methods]
impl EventLoop {
    #[instrument]
    pub fn new() -> Result<Self, EventLoopError> {
        winit::event_loop::EventLoop::new()
            .map(Self::from)
            .map_err(|e| EventLoopError::new(format!("{:?}", e)))
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Window Plugin (workflow/window_plugin.rs):

```rust
use elicitation_derive::elicit_tool;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateWindowParams {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: Option<bool>,
    pub decorations: Option<bool>,
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__create",
    description = "Create a new window with specified dimensions and properties.",
    emit = Auto
)]
async fn window_create(p: CreateWindowParams) -> Result<CallToolResult, ErrorData> {
    // Emit window creation code
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created window: {}x{} '{}'", p.width, p.height, p.title))
    ]))
}

// ... 11 more tools: set_title, resize, maximize, minimize, etc.
```

### 5.2 Event Plugin (workflow/event_plugin.rs):

```rust
#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__run_loop",
    description = "Start the event loop with a callback handler.",
    emit = Auto
)]
async fn event_run_loop(p: RunLoopParams) -> Result<CallToolResult, ErrorData> {
    // Emit event loop setup code
    Ok(CallToolResult::success(vec![
        Content::text("Event loop started")
    ]))
}

// ... 9 more tools: poll_events, handle_window_event, etc.
```

### 5.3 Input Plugin (workflow/input_plugin.rs):

```rust
#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__handle_keyboard",
    description = "Handle keyboard input events with key codes and modifiers.",
    emit = Auto
)]
async fn input_handle_keyboard(p: KeyboardParams) -> Result<CallToolResult, ErrorData> {
    // Emit keyboard handling code
    Ok(CallToolResult::success(vec![
        Content::text("Keyboard handler registered")
    ]))
}

// ... 7 more tools: handle_mouse, handle_touch, etc.
```

## Phase 6: Testing

### File to create:
- `crates/elicit_winit/tests/winit_test.rs`

### Test Coverage:

```rust
#[test]
fn test_window_creation() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let wrapped = Window::from(window);
    // Test serialization, reflect methods
}

#[test]
fn test_window_attributes_serialization() {
    let attrs = WindowAttributesParams {
        title: "Test Window".to_string(),
        width: 800,
        height: 600,
        resizable: true,
    };

    let json = serde_json::to_value(&attrs).unwrap();
    assert_eq!(json["width"], 800);
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_winit/README.md`

### Content:

```markdown
# elicit_winit

Elicitation-enabled wrappers around [`winit`](https://docs.rs/winit) for windowing and input.

## Purpose

Provides the **windowing alphabet** — foundational MCP tools for:
- Native window creation and management
- Event loop and input handling
- Monitor and display configuration
- Cross-platform windowing workflows

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `winit_window` | 12 | Window creation, configuration, sizing |
| `winit_event` | 10 | Event loop, polling, dispatch |
| `winit_input` | 8 | Keyboard, mouse, touch events |
| `winit_cursor` | 6 | Cursor control, grab, visibility |
| `winit_monitor` | 5 | Monitor info, video modes |
| `winit_workflow` | 8 | Common patterns |

**Total: ~49 MCP tools**

## Usage

```rust
use elicit_winit::{EventLoop, Window};

// MCP tools generate this code:
let event_loop = EventLoop::new()?;
let window = Window::builder()
    .with_title("My App")
    .with_inner_size(LogicalSize::new(800, 600))
    .build(&event_loop)?;
```

## Integration with wgpu

Combine with `elicit_wgpu` for complete rendering:

```rust
// winit provides window + event loop
let event_loop = EventLoop::new()?;
let window = Window::new(&event_loop)?;

// wgpu provides GPU rendering
let instance = wgpu::Instance::new(Default::default());
let surface = instance.create_surface(&window)?;

// Event loop
event_loop.run(move |event, target| {
    match event {
        Event::WindowEvent { event, .. } => {
            // Handle window events
        }
        Event::AboutToWait => {
            // Render frame
        }
        _ => {}
    }
})?;
```
```

## Verification Steps

**After implementation**:
1. `cargo check -p elicit_winit`
2. `cargo test -p elicit_winit`
3. `cargo check -p elicitation --no-default-features --features winit`
4. `cargo check --all-features`

**Manual verification**:
1. Launch MCP server with elicit_winit
2. Call `winit_window__create` with params
3. Verify window creation code emission

## Critical Files

### To create:
- `crates/elicit_winit/Cargo.toml`
- `crates/elicit_winit/README.md`
- `crates/elicit_winit/src/lib.rs`
- `crates/elicit_winit/src/window.rs`
- `crates/elicit_winit/src/event_loop.rs`
- `crates/elicit_winit/src/monitor.rs`
- `crates/elicit_winit/src/workflow/mod.rs`
- `crates/elicit_winit/src/workflow/window_plugin.rs`
- `crates/elicit_winit/src/workflow/event_plugin.rs`
- `crates/elicit_winit/src/workflow/input_plugin.rs`
- `crates/elicit_winit/src/workflow/cursor_plugin.rs`
- `crates/elicit_winit/src/workflow/monitor_plugin.rs`
- `crates/elicit_winit/src/workflow/workflow_plugin.rs`
- `crates/elicit_winit/tests/winit_test.rs`
- `crates/elicitation/src/winit_types.rs`

### To modify:
- `Cargo.toml`
- `crates/elicitation/Cargo.toml`
- `crates/elicitation/src/lib.rs`

## Implementation Order

1. **Phase 1**: Workspace configuration (20 min)
2. **Phase 2**: Core type integration (1-2 hours)
3. **Phase 3**: Create elicit_winit structure (30 min)
4. **Phase 4**: Implement type wrappers (2 hours)
5. **Phase 5**: Implement MCP tools (~49 tools) (6-8 hours)
6. **Phase 6**: Testing (1 hour)
7. **Phase 7**: Documentation (30 min)

**Total estimated time**: 11-14 hours

## Notes

### Shadow Crate Design
- **6 plugins**: Organized by functional area
- **~49 total tools**: Complete winit API coverage
- **Emit mode**: All tools support code generation
- **Cross-platform**: Works on Windows, macOS, Linux, Web (wasm)

### Use Cases
- **Native applications**: Window creation for desktop apps
- **Game development**: Input handling, fullscreen, cursor control
- **Pair with wgpu**: Foundation for GPU-accelerated rendering
- **Cross-platform UI**: Single codebase for multiple platforms
