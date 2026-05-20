//! Leptos + Axum SSR integration descriptor types.
//!
//! Available with the `leptos-types` feature.

use elicitation_derive::ToCodeLiteral;
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LeptosCustomRouteDescriptor {
    /// HTTP method in lowercase (`"get"`, `"post"`, `"put"`, `"delete"`, `"any"`).
    pub method: String,
    /// URL path pattern (e.g. `"/api/health"`).
    pub path: String,
    /// Handler expression — a function name or async closure literal.
    pub handler: String,
}

/// A response header to add to all responses via a Tower middleware layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
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

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use strum::IntoEnumIterator;

// --- LeptosClientMode -------------------------------------------------------

impl Prompt for LeptosClientMode {
    fn prompt() -> Option<&'static str> { Some("Choose the Leptos client rendering mode:") }
}

impl Select for LeptosClientMode {
    fn options() -> Vec<Self> { LeptosClientMode::iter().collect() }
    fn labels() -> Vec<String> { LeptosClientMode::iter().map(|v| v.to_string()).collect() }
    fn from_label(label: &str) -> Option<Self> {
        LeptosClientMode::iter().find(|v| v.to_string() == label)
    }
}

crate::default_style!(LeptosClientMode => LeptosClientModeStyle);

impl Elicitation for LeptosClientMode {
    type Style = LeptosClientModeStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosClientMode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose client mode:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid LeptosClientMode: {label}")))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("LeptosClientMode", "Hydrate")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("LeptosClientMode", "Hydrate")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("LeptosClientMode", "Hydrate")
    }
}

impl ElicitIntrospect for LeptosClientMode {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosClientMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for LeptosClientMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose client mode:").to_string(),
            type_name: "LeptosClientMode".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- LeptosAxumMode ---------------------------------------------------------

impl Prompt for LeptosAxumMode {
    fn prompt() -> Option<&'static str> { Some("Choose the Leptos Axum rendering/serving mode:") }
}

impl Select for LeptosAxumMode {
    fn options() -> Vec<Self> { LeptosAxumMode::iter().collect() }
    fn labels() -> Vec<String> { LeptosAxumMode::iter().map(|v| v.to_string()).collect() }
    fn from_label(label: &str) -> Option<Self> {
        LeptosAxumMode::iter().find(|v| v.to_string() == label)
    }
}

crate::default_style!(LeptosAxumMode => LeptosAxumModeStyle);

impl Elicitation for LeptosAxumMode {
    type Style = LeptosAxumModeStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosAxumMode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose Axum mode:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid LeptosAxumMode: {label}")))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("LeptosAxumMode", "StaticHtml")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("LeptosAxumMode", "StaticHtml")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("LeptosAxumMode", "StaticHtml")
    }
}

impl ElicitIntrospect for LeptosAxumMode {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosAxumMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for LeptosAxumMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose Axum mode:").to_string(),
            type_name: "LeptosAxumMode".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- LeptosDisplayMode ------------------------------------------------------

impl Prompt for LeptosDisplayMode {
    fn prompt() -> Option<&'static str> { Some("Choose the display shell mode:") }
}

impl Select for LeptosDisplayMode {
    fn options() -> Vec<Self> { LeptosDisplayMode::iter().collect() }
    fn labels() -> Vec<String> { LeptosDisplayMode::iter().map(|v| v.to_string()).collect() }
    fn from_label(label: &str) -> Option<Self> {
        LeptosDisplayMode::iter().find(|v| v.to_string() == label)
    }
}

crate::default_style!(LeptosDisplayMode => LeptosDisplayModeStyle);

impl Elicitation for LeptosDisplayMode {
    type Style = LeptosDisplayModeStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosDisplayMode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose display mode:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid LeptosDisplayMode: {label}")))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("LeptosDisplayMode", "Standard")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("LeptosDisplayMode", "Standard")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("LeptosDisplayMode", "Standard")
    }
}

impl ElicitIntrospect for LeptosDisplayMode {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosDisplayMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for LeptosDisplayMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose display mode:").to_string(),
            type_name: "LeptosDisplayMode".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- LeptosCustomRouteDescriptor --------------------------------------------

impl Prompt for LeptosCustomRouteDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe a custom Axum route:") }
}
crate::default_style!(LeptosCustomRouteDescriptor => LeptosCustomRouteDescriptorStyle);
impl Elicitation for LeptosCustomRouteDescriptor {
    type Style = LeptosCustomRouteDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosCustomRouteDescriptor");
        let method = String::elicit(communicator).await?;
        let path = String::elicit(communicator).await?;
        let handler = String::elicit(communicator).await?;
        Ok(Self { method, path, handler })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosCustomRouteDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosCustomRouteDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "method", type_name: "String", prompt: None },
                FieldInfo { name: "path", type_name: "String", prompt: None },
                FieldInfo { name: "handler", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosCustomRouteDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosCustomRouteDescriptor".to_string(),
            fields: vec![
                ("method".to_string(), Box::new(String::prompt_tree())),
                ("path".to_string(), Box::new(String::prompt_tree())),
                ("handler".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- LeptosResponseHeaderDescriptor -----------------------------------------

impl Prompt for LeptosResponseHeaderDescriptor {
    fn prompt() -> Option<&'static str> { Some("Describe a response header:") }
}
crate::default_style!(LeptosResponseHeaderDescriptor => LeptosResponseHeaderDescriptorStyle);
impl Elicitation for LeptosResponseHeaderDescriptor {
    type Style = LeptosResponseHeaderDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosResponseHeaderDescriptor");
        let name = String::elicit(communicator).await?;
        let value = String::elicit(communicator).await?;
        Ok(Self { name, value })
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosResponseHeaderDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosResponseHeaderDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "name", type_name: "String", prompt: None },
                FieldInfo { name: "value", type_name: "String", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosResponseHeaderDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosResponseHeaderDescriptor".to_string(),
            fields: vec![
                ("name".to_string(), Box::new(String::prompt_tree())),
                ("value".to_string(), Box::new(String::prompt_tree())),
            ],
        }
    }
}

// --- LeptosAxumDescriptor ---------------------------------------------------

impl Prompt for LeptosAxumDescriptor {
    fn prompt() -> Option<&'static str> { Some("Configure the Leptos + Axum application descriptor:") }
}
crate::default_style!(LeptosAxumDescriptor => LeptosAxumDescriptorStyle);
impl Elicitation for LeptosAxumDescriptor {
    type Style = LeptosAxumDescriptorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosAxumDescriptor");
        let app_component = String::elicit(communicator).await?;
        let mode = LeptosAxumMode::elicit(communicator).await?;
        let site_addr = String::elicit(communicator).await?;
        let client_mode = LeptosClientMode::elicit(communicator).await?;
        let static_file_handler = matches!(mode, LeptosAxumMode::FullSsr | LeptosAxumMode::WasmShell);
        let mut desc = LeptosAxumDescriptor::new(app_component, mode, site_addr);
        desc.client_mode = client_mode;
        desc.static_file_handler = static_file_handler;
        Ok(desc)
    }
    fn kani_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::kani_proof() }
    fn verus_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <String as crate::Elicitation>::creusot_proof() }
}
impl ElicitIntrospect for LeptosAxumDescriptor {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosAxumDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey { fields: vec![
                FieldInfo { name: "app_component", type_name: "String", prompt: None },
                FieldInfo { name: "mode", type_name: "LeptosAxumMode", prompt: None },
                FieldInfo { name: "site_addr", type_name: "String", prompt: None },
                FieldInfo { name: "client_mode", type_name: "LeptosClientMode", prompt: None },
            ]},
        }
    }
}
impl crate::ElicitPromptTree for LeptosAxumDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "LeptosAxumDescriptor".to_string(),
            fields: vec![
                ("app_component".to_string(), Box::new(String::prompt_tree())),
                ("mode".to_string(), Box::new(LeptosAxumMode::prompt_tree())),
                ("site_addr".to_string(), Box::new(String::prompt_tree())),
                ("client_mode".to_string(), Box::new(LeptosClientMode::prompt_tree())),
            ],
        }
    }
}
