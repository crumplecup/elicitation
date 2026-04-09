//! `elicit_axum` — axum web framework MCP tools.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    AxumExtractBodyPlugin, AxumExtractMiscPlugin, AxumExtractMultipartPlugin,
    AxumExtractPathPlugin, AxumExtractQueryPlugin, AxumHandlerPlugin, AxumHttpTypesPlugin,
    AxumMethodRouterPlugin, AxumMiddlewarePlugin, AxumResponseHeadersPlugin,
    AxumResponseHtmlPlugin, AxumResponseJsonPlugin, AxumResponseSsePlugin, AxumRouterPlugin,
    AxumServePlugin,
};
