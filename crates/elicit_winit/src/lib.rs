//! `elicit_winit` — code-generation MCP tools for winit windowing and input.
//!
//! All tools are **emit-only**: they generate Rust code snippets for native
//! windowing applications.  No windows are created at runtime.
//!
//! # Plugins
//!
//! | Plugin | Prefix | Tools | Coverage |
//! |--------|--------|-------|---------|
//! | [`WinitWindowPlugin`] | `winit_window__` | 8 | Window creation and configuration |
//! | [`WinitEventPlugin`] | `winit_event__` | 7 | Event loop and ApplicationHandler |
//! | [`WinitInputPlugin`] | `winit_input__` | 7 | Keyboard, mouse, touch, cursor |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    WinitEventLoopScaffolded, WinitEventPlugin, WinitInputHandled, WinitInputPlugin,
    WinitWindowConfigured, WinitWindowPlugin,
};
