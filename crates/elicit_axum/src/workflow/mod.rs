//! MCP workflow plugins for the axum web framework.

pub mod extract_body_plugin;
pub mod extract_misc_plugin;
pub mod extract_multipart_plugin;
pub mod extract_path_plugin;
pub mod extract_query_plugin;
pub mod handler_plugin;
pub mod http_types_plugin;
pub mod method_router_plugin;
pub mod middleware_plugin;
pub mod response_headers_plugin;
pub mod response_html_plugin;
pub mod response_json_plugin;
pub mod response_sse_plugin;
pub mod router_plugin;
pub mod serve_plugin;

pub use extract_body_plugin::AxumExtractBodyPlugin;
pub use extract_misc_plugin::AxumExtractMiscPlugin;
pub use extract_multipart_plugin::AxumExtractMultipartPlugin;
pub use extract_path_plugin::AxumExtractPathPlugin;
pub use extract_query_plugin::AxumExtractQueryPlugin;
pub use handler_plugin::AxumHandlerPlugin;
pub use http_types_plugin::AxumHttpTypesPlugin;
pub use method_router_plugin::AxumMethodRouterPlugin;
pub use middleware_plugin::AxumMiddlewarePlugin;
pub use response_headers_plugin::AxumResponseHeadersPlugin;
pub use response_html_plugin::AxumResponseHtmlPlugin;
pub use response_json_plugin::AxumResponseJsonPlugin;
pub use response_sse_plugin::AxumResponseSsePlugin;
pub use router_plugin::AxumRouterPlugin;
pub use serve_plugin::AxumServePlugin;
