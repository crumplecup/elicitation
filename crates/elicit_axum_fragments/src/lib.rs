//! `elicit_axum_fragments` — axum code generation MCP tools.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    AxumErrorGenPlugin, AxumHandlerGenPlugin, AxumMiddlewareGenPlugin, AxumRouteGenPlugin,
    AxumServiceGenPlugin, AxumStateGenPlugin, AxumTestGenPlugin,
};
