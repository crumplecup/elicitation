//! `elicit_leptos` — MCP tools for Leptos 0.8 reactive web framework.
//!
//! # Plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`LeptosReactivePlugin`] | `leptos_reactive__*` | Signal/memo/action state management |
//! | [`LeptosCodePlugin`] | `leptos_code__*` | Component, view, routing, and app scaffolding |
//!
//! # Renderer
//!
//! [`LeptosRenderer`] implements [`elicit_ui::UiRenderer`] for rendering
//! AccessKit trees to Leptos view descriptors.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod code;
mod error;
mod reactive;
mod renderer;

pub use code::LeptosCodePlugin;
pub use error::{LeptosError, LeptosErrorKind, LeptosResult};
pub use reactive::LeptosReactivePlugin;
pub use renderer::LeptosRenderer;
