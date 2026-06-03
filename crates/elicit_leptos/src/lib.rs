//! `elicit_leptos` — MCP tools for Leptos 0.8 reactive web framework.
//!
//! # Plugins
//!
//! | Plugin | Namespace | Description |
//! |------|-----------|-------------|
//! | [`LeptosReactivePlugin`] | `leptos_reactive__*` | Signal/memo/action state management |
//! | [`LeptosCodePlugin`] | `leptos_code__*` | Component, view, routing, and app scaffolding |
//! | [`LeptosAxumPlugin`] | `leptos_axum__*` | Axum SSR server configuration (static HTML, full SSR, WASM shell) |
//! | [`LeptosAxumBridgePlugin`] | `leptos_axum_bridge__*` | Bridge from leptos descriptor → axum router descriptor |
//!
//! # Renderer
//!
//! [`LeptosRenderer`] implements [`elicit_ui::UiRenderer`] for rendering
//! AccessKit trees to either semantic HTML5 or Leptos `view!` macro source.
//!
//! | Mode | Output | Use case |
//! |------|--------|----------|
//! | [`leptos_accesskit_convert::LeptosRenderMode::Html`] | Semantic HTML5 string | SSR via axum/tower |
//! | [`leptos_accesskit_convert::LeptosRenderMode::ViewMacro`] | Leptos `view!` body | CSR/WASM or codegen |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod axum_ssr;
pub mod bridge;
mod code;
mod error;
pub mod leptos_accesskit_convert;
mod reactive;
pub mod render_context;
pub mod renderer;

pub use axum_ssr::LeptosAxumPlugin;
pub use bridge::LeptosAxumBridgePlugin;
pub use code::LeptosCodePlugin;
pub use error::{LeptosError, LeptosErrorKind, LeptosResult};
pub use leptos_accesskit_convert::{LeptosRenderMode, render_tree, render_tree_with_stats};
pub use reactive::LeptosReactivePlugin;
pub use render_context::{LeptosRenderArea, LeptosRenderContext};
pub use renderer::LeptosRenderer;
