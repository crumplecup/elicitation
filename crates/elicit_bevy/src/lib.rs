//! `elicit_bevy` — code-generation MCP tools for Bevy ECS game development.
//!
//! All tools are **emit-only**: they generate Rust code snippets for Bevy
//! applications. No Bevy world or ECS runtime is created at tool execution time.
//!
//! # Plugins
//!
//! | Plugin | Prefix | Coverage |
//! |--------|--------|---------|
//! | [`BevyDerivePlugin`] | `bevy_derive__` | ECS derive attributes |
//! | [`BevyEcsPlugin`] | `bevy_ecs__` | System and app wiring |
//! | [`BevyAppPlugin`] | `bevy_app__` | App/Plugin/Scene descriptors |
//! | [`BevyQueryPlugin`] | `bevy_query__` | Generic ECS type factories |
//! | [`BevyRenderPlugin`] | `bevy_render__` | Material/light/camera descriptors |
//! | [`BevyUiPlugin`] | `bevy_ui__` | UI layout tools |

#![forbid(unsafe_code)]
#![warn(missing_docs)]
