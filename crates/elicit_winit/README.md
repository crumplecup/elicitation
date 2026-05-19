# elicit_winit

`elicit_winit` is the [elicitation] shadow crate for [winit]. All tools are **emit-only**: they
generate idiomatic Rust code snippets for native windowing applications. No windows are created at
runtime — no `winit::Window` or `EventLoop` lives in the MCP server process.

## Plugins

| Plugin | Namespace | Tools | Coverage |
|---|---|---|---|
| `WinitWindowPlugin` | `winit_window__*` | 8 | Window creation and runtime configuration |
| `WinitEventPlugin` | `winit_event__*` | 7 | Event loop and `ApplicationHandler` impl |
| `WinitInputPlugin` | `winit_input__*` | 7 | Keyboard, mouse, touch, and cursor input |

## Tool reference

### `winit_window__*`

| Tool | Description |
|---|---|
| `attributes` | Generate a `WindowAttributes` builder chain |
| `set_title` | Generate code to update a window's title bar text at runtime |
| `set_visible` | Generate code to show or hide a window |
| `set_resizable` | Generate code to enable or disable user resizing |
| `set_decorations` | Generate code to toggle window decorations |
| `set_fullscreen` | Generate code to enter or exit fullscreen |
| `set_inner_size` | Generate code to resize the window's client area |
| `set_min_inner_size` | Generate code to constrain the minimum window size |

### `winit_event__*`

| Tool | Description |
|---|---|
| `app_skeleton` | Generate a complete `ApplicationHandler` impl skeleton |
| `event_loop` | Generate the `main` function body creating an `EventLoop` and running an application |
| `resumed_handler` | Generate a `resumed` handler body |
| `window_event_handler` | Generate a `window_event` handler body |
| `device_event_handler` | Generate a `device_event` handler body |
| `about_to_wait_handler` | Generate an `about_to_wait` handler body |
| `exiting_handler` | Generate an `exiting` handler body |

### `winit_input__*`

| Tool | Description |
|---|---|
| `keyboard_handler` | Generate a `WindowEvent::KeyboardInput` match arm for a physical key code |
| `named_key_handler` | Generate a `WindowEvent::KeyboardInput` match arm for a `NamedKey` variant |
| `mouse_button_handler` | Generate a `WindowEvent::MouseInput` match arm |
| `cursor_moved_handler` | Generate a `WindowEvent::CursorMoved` handler |
| `mouse_wheel_handler` | Generate a `WindowEvent::MouseWheel` handler |
| `touch_handler` | Generate a `WindowEvent::Touch` handler |
| `ime_handler` | Generate a `WindowEvent::Ime` handler |

## Usage

```toml
[dependencies]
elicit_winit = "0.11"
```

```rust
use elicit_winit::{WinitWindowPlugin, WinitEventPlugin, WinitInputPlugin};

let server = server
    .register_plugin(WinitWindowPlugin::new())
    .register_plugin(WinitEventPlugin::new())
    .register_plugin(WinitInputPlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[winit]: https://crates.io/crates/winit
