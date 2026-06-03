//! `elicit_winit` — shadow crate for `winit` windowing.
//!
//! Exposes OS window management as MCP tools via a [`WinitPlugin`].
//! The embedding app owns the winit `EventLoop` on the main thread and
//! injects an `EventLoopProxy<WinitCmd>` into the plugin.
//!
//! # Integration pattern
//!
//! ```no_run
//! # use winit::event_loop::EventLoop;
//! # use elicit_winit::{WinitCmd, WinitPlugin};
//! let event_loop = EventLoop::<WinitCmd>::with_user_event()
//!     .build()
//!     .expect("event loop");
//! let proxy = event_loop.create_proxy();
//! let plugin = WinitPlugin::new(proxy);
//! // register plugin with your MCP server, then run event_loop.run_app(...)
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod context;
mod plugin;
mod window;

pub use context::{WinitCmd, WinitCtx, WinitError};
pub use plugin::WinitPlugin;
pub use window::Window;
