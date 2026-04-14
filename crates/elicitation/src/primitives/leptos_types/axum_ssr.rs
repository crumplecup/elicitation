//! Leptos + Axum SSR integration descriptor types.
//!
//! Available with the `leptos-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Client-side Leptos rendering mode for WASM builds.
///
/// Controls the feature flag emitted in the client `Cargo.toml` and the
/// entry-point call in the client `lib.rs`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
    Default,
)]
#[serde(rename_all = "snake_case")]
pub enum LeptosClientMode {
    /// Pure client-side rendering — no SSR, browser runs everything.
    ///
    /// Uses `leptos = { features = ["csr"] }`.  Best for SPAs that don't
    /// need server-rendered HTML.
    #[display("csr")]
    Csr,
    /// Hydration mode — browser picks up where the server left off.
    ///
    /// Uses `leptos = { features = ["hydrate"] }`.  Pair this with
    /// [`LeptosAxumMode::FullSsr`] or [`LeptosAxumMode::WasmShell`].
    #[display("hydrate")]
    #[default]
    Hydrate,
}

/// Rendering + serving mode for a Leptos + Axum application.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
)]
#[serde(rename_all = "snake_case")]
pub enum LeptosAxumMode {
    /// Serve pre-rendered HTML strings via [`elicit_leptos::LeptosRenderer`].
    ///
    /// No live `leptos` runtime required.  The server builds a `VerifiedTree`
    /// per-request, calls `LeptosRenderer::html()`, and returns
    /// `axum::response::Html(string)`.  Ideal for archive/read-heavy UIs.
    #[display("static_html")]
    StaticHtml,
    /// Full SSR with `leptos_axum` integration.
    ///
    /// Requires `leptos` and `leptos_axum` runtime deps.  Uses
    /// `generate_route_list` + `.leptos_routes()` + `file_and_error_handler`.
    #[display("full_ssr")]
    FullSsr,
    /// Serve a WASM shell: axum delivers the `index.html` + `/pkg` assets;
    /// the browser hydrates with the compiled WASM bundle.
    ///
    /// Server function calls are forwarded to
    /// `leptos_axum::handle_server_fns`.
    #[display("wasm_shell")]
    WasmShell,
}

/// A custom axum route added before the Leptos routes.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct LeptosCustomRouteDescriptor {
    /// HTTP method in lowercase (`"get"`, `"post"`, `"put"`, `"delete"`, `"any"`).
    pub method: String,
    /// URL path pattern (e.g. `"/api/health"`).
    pub path: String,
    /// Handler expression — a function name or async closure literal.
    pub handler: String,
}

/// A response header to add to all responses via a Tower middleware layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct LeptosResponseHeaderDescriptor {
    /// Header name (e.g. `"Cache-Control"`).
    pub name: String,
    /// Header value (e.g. `"no-store"`).
    pub value: String,
}

/// Descriptor for a Leptos + Axum SSR server configuration.
///
/// Agents build this incrementally via `leptos_axum__*` tools, then call
/// `leptos_axum__emit` to recover a complete `main.rs` for the mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LeptosAxumDescriptor {
    /// Top-level app component name (PascalCase, e.g. `"App"`).
    pub app_component: String,
    /// Serving mode.
    pub mode: LeptosAxumMode,
    /// Socket address to bind (e.g. `"0.0.0.0:3000"`).
    pub site_addr: String,
    /// Relative path to the compiled WASM/JS package directory.
    ///
    /// Used only for [`LeptosAxumMode::WasmShell`] and
    /// [`LeptosAxumMode::FullSsr`].  Defaults to `"pkg"`.
    pub pkg_dir: String,
    /// Custom routes added before Leptos routes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_routes: Vec<LeptosCustomRouteDescriptor>,
    /// Additional response headers applied via `tower_http::set_header`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub response_headers: Vec<LeptosResponseHeaderDescriptor>,
    /// Path prefix for the server-function handler endpoint.
    ///
    /// Defaults to `"/api/leptos"`.  Only used in `FullSsr` and `WasmShell`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_fn_route: Option<String>,
    /// Whether to include `file_and_error_handler` for static asset serving.
    ///
    /// Defaults to `true` for `FullSsr` and `WasmShell`.
    pub static_file_handler: bool,
    /// Client-side WASM rendering mode — `Csr` or `Hydrate`.
    ///
    /// Controls the feature flag in the emitted client `Cargo.toml` and
    /// the entry-point call in the emitted client `lib.rs`.
    /// Defaults to [`LeptosClientMode::Hydrate`].
    #[serde(default)]
    pub client_mode: LeptosClientMode,
    /// Rust crate name for the client WASM library (e.g. `"archive_client"`).
    ///
    /// Used in emitted `index.html` asset paths, client `lib.rs` imports, and
    /// client `Cargo.toml` package name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    /// Page `<title>` for the emitted `index.html`.
    ///
    /// Defaults to `"Leptos App"` if not set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
}

impl LeptosAxumDescriptor {
    /// Create a new descriptor with sensible defaults.
    pub fn new(
        app_component: impl Into<String>,
        mode: LeptosAxumMode,
        site_addr: impl Into<String>,
    ) -> Self {
        let static_file_handler =
            matches!(mode, LeptosAxumMode::FullSsr | LeptosAxumMode::WasmShell);
        Self {
            app_component: app_component.into(),
            mode,
            site_addr: site_addr.into(),
            pkg_dir: "pkg".to_string(),
            custom_routes: vec![],
            response_headers: vec![],
            server_fn_route: None,
            static_file_handler,
            client_mode: LeptosClientMode::default(),
            pkg_name: None,
            app_title: None,
        }
    }
}

/// Display mode controlling which shell component wraps the Leptos view tree.
///
/// Passed to the bridge plugin to select the outer HTML wrapper / theme
/// applied around the rendered component.  When emitting code the bridge
/// matches on this enum to expand the correct shell.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
)]
#[serde(rename_all = "snake_case")]
pub enum LeptosDisplayMode {
    /// Minimal wrapper — renders the component with no extra chrome.
    #[display("bare")]
    Bare,
    /// Standard responsive shell with a nav header and content area.
    #[default]
    #[display("standard")]
    Standard,
    /// Full-page dashboard layout with sidebar and main pane.
    #[display("dashboard")]
    Dashboard,
}

#[cfg(feature = "emit")]
impl crate::emit_code::ToCodeLiteral for LeptosAxumMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            LeptosAxumMode::StaticHtml => {
                quote::quote! { elicitation::LeptosAxumMode::StaticHtml }
            }
            LeptosAxumMode::FullSsr => {
                quote::quote! { elicitation::LeptosAxumMode::FullSsr }
            }
            LeptosAxumMode::WasmShell => {
                quote::quote! { elicitation::LeptosAxumMode::WasmShell }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::LeptosAxumMode }
    }
}

#[cfg(feature = "emit")]
impl crate::emit_code::ToCodeLiteral for LeptosClientMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            LeptosClientMode::Csr => {
                quote::quote! { elicitation::LeptosClientMode::Csr }
            }
            LeptosClientMode::Hydrate => {
                quote::quote! { elicitation::LeptosClientMode::Hydrate }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::LeptosClientMode }
    }
}

#[cfg(feature = "emit")]
impl crate::emit_code::ToCodeLiteral for LeptosDisplayMode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            LeptosDisplayMode::Bare => {
                quote::quote! { elicitation::LeptosDisplayMode::Bare }
            }
            LeptosDisplayMode::Standard => {
                quote::quote! { elicitation::LeptosDisplayMode::Standard }
            }
            LeptosDisplayMode::Dashboard => {
                quote::quote! { elicitation::LeptosDisplayMode::Dashboard }
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::LeptosDisplayMode }
    }
}
