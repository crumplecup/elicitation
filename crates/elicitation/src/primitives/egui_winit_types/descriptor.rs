//! Descriptor types for a native egui + winit application.
//!
//! Agents build an [`EguiWinitDescriptor`] incrementally via
//! `egui_winit__*` tools, then call `egui_winit__emit` to produce a
//! complete `main.rs` scaffold.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// GPU rendering backend for the egui native app.
///
/// `Wgpu` is the cross-platform choice and requires `egui-wgpu`.
/// `Glow` uses OpenGL via `egui-glow` (simpler, less portable on modern
/// systems).
#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum EguiWinitRenderer {
    /// Cross-platform wgpu (WebGPU) backend — the recommended choice.
    #[display("wgpu")]
    #[default]
    Wgpu,
    /// OpenGL via glow — simpler setup, fewer dependencies.
    #[display("glow")]
    Glow,
}

/// Colour theme preference for the egui context.
#[derive(
    Debug,
    Clone,
    Copy,
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
pub enum EguiWinitTheme {
    /// Force dark mode regardless of OS setting.
    #[display("dark")]
    #[default]
    Dark,
    /// Force light mode regardless of OS setting.
    #[display("light")]
    Light,
    /// Follow the OS / system preference.
    #[display("system")]
    System,
}

/// Descriptor for a native egui + winit application.
///
/// Agents build this incrementally via `egui_winit__*` tools, then call
/// `egui_winit__emit` to recover a complete `main.rs` entry-point with:
/// - winit [`EventLoop`] using the [`ApplicationHandler`] pattern
/// - [`egui_winit::State`] for input integration
/// - A wgpu or glow render loop stub
/// - User-defined update closure placeholder
///
/// [`EventLoop`]: winit::event_loop::EventLoop
/// [`ApplicationHandler`]: winit::application::ApplicationHandler
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EguiWinitDescriptor {
    /// Name of the user's application struct (PascalCase, e.g. `"MyApp"`).
    pub app_struct: String,
    /// Window title shown in the OS title bar.
    pub title: String,
    /// Initial window width in logical pixels.
    pub width: u32,
    /// Initial window height in logical pixels.
    pub height: u32,
    /// GPU rendering backend.
    #[serde(default)]
    pub renderer: EguiWinitRenderer,
    /// Colour theme preference.
    #[serde(default)]
    pub theme: EguiWinitTheme,
    /// Whether to enable vertical sync.
    #[serde(default = "default_true")]
    pub vsync: bool,
    /// Whether to show OS window decorations (title bar, borders).
    #[serde(default = "default_true")]
    pub decorations: bool,
    /// Whether the window can be resized by the user.
    #[serde(default = "default_true")]
    pub resizable: bool,
    /// Whether the window starts maximised.
    #[serde(default)]
    pub maximized: bool,
    /// Whether the window background is transparent.
    #[serde(default)]
    pub transparent: bool,
}

fn default_true() -> bool {
    true
}

impl EguiWinitDescriptor {
    /// Create a descriptor with sensible defaults.
    pub fn new(
        app_struct: impl Into<String>,
        title: impl Into<String>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            app_struct: app_struct.into(),
            title: title.into(),
            width,
            height,
            renderer: EguiWinitRenderer::default(),
            theme: EguiWinitTheme::default(),
            vsync: true,
            decorations: true,
            resizable: true,
            maximized: false,
            transparent: false,
        }
    }
}

impl crate::emit_code::ToCodeLiteral for EguiWinitRenderer {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            EguiWinitRenderer::Wgpu => {
                quote::quote! { elicitation::EguiWinitRenderer::Wgpu }
            }
            EguiWinitRenderer::Glow => {
                quote::quote! { elicitation::EguiWinitRenderer::Glow }
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::EguiWinitRenderer }
    }
}

impl crate::emit_code::ToCodeLiteral for EguiWinitTheme {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            EguiWinitTheme::Dark => {
                quote::quote! { elicitation::EguiWinitTheme::Dark }
            }
            EguiWinitTheme::Light => {
                quote::quote! { elicitation::EguiWinitTheme::Light }
            }
            EguiWinitTheme::System => {
                quote::quote! { elicitation::EguiWinitTheme::System }
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::EguiWinitTheme }
    }
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
};

// --- EguiWinitRenderer -------------------------------------------------------

impl Prompt for EguiWinitRenderer {
    fn prompt() -> Option<&'static str> {
        Some("Choose GPU rendering backend for the egui native app:")
    }
}

impl Select for EguiWinitRenderer {
    fn options() -> Vec<Self> {
        vec![EguiWinitRenderer::Wgpu, EguiWinitRenderer::Glow]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(EguiWinitRenderer => EguiWinitRendererStyle);

impl Elicitation for EguiWinitRenderer {
    type Style = EguiWinitRendererStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiWinitRenderer");
        let params = crate::mcp::select_params(
            Self::prompt().unwrap_or("Choose renderer:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(crate::mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid EguiWinitRenderer: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "elicitation::EguiWinitRenderer",
            "wgpu",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum(
            "elicitation::EguiWinitRenderer",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum(
            "elicitation::EguiWinitRenderer",
        )
    }
}

impl ElicitIntrospect for EguiWinitRenderer {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::EguiWinitRenderer",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiWinitRenderer {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "EguiWinitRenderer".to_string(),
            type_name: "EguiWinitRenderer".to_string(),
            options: Self::labels(),
            branches: vec![None, None],
        }
    }
}

// --- EguiWinitTheme ----------------------------------------------------------

impl Prompt for EguiWinitTheme {
    fn prompt() -> Option<&'static str> {
        Some("Choose colour theme for the egui context:")
    }
}

impl Select for EguiWinitTheme {
    fn options() -> Vec<Self> {
        vec![EguiWinitTheme::Dark, EguiWinitTheme::Light, EguiWinitTheme::System]
    }

    fn labels() -> Vec<String> {
        Self::options()
            .iter()
            .map(|v| {
                serde_json::to_string(v)
                    .unwrap()
                    .trim_matches('"')
                    .to_string()
            })
            .collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        serde_json::from_str(&format!("\"{}\"", label)).ok()
    }
}

crate::default_style!(EguiWinitTheme => EguiWinitThemeStyle);

impl Elicitation for EguiWinitTheme {
    type Style = EguiWinitThemeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiWinitTheme");
        let params = crate::mcp::select_params(
            Self::prompt().unwrap_or("Choose theme:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(crate::mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = crate::mcp::extract_value(result)?;
        let label = crate::mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid EguiWinitTheme: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_multi_variant_enum(
            "elicitation::EguiWinitTheme",
            "dark",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_multi_variant_enum("elicitation::EguiWinitTheme")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_multi_variant_enum(
            "elicitation::EguiWinitTheme",
        )
    }
}

impl ElicitIntrospect for EguiWinitTheme {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::EguiWinitTheme",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiWinitTheme {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: "EguiWinitTheme".to_string(),
            type_name: "EguiWinitTheme".to_string(),
            options: Self::labels(),
            branches: vec![None, None, None],
        }
    }
}

// --- EguiWinitDescriptor -----------------------------------------------------

impl Prompt for EguiWinitDescriptor {
    fn prompt() -> Option<&'static str> {
        Some("Describe the native egui + winit application:")
    }
}

crate::default_style!(EguiWinitDescriptor => EguiWinitDescriptorStyle);

impl Elicitation for EguiWinitDescriptor {
    type Style = EguiWinitDescriptorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiWinitDescriptor");
        let app_struct = String::elicit(communicator).await?;
        let title = String::elicit(communicator).await?;
        let width = u32::elicit(communicator).await?;
        let height = u32::elicit(communicator).await?;
        let renderer = EguiWinitRenderer::elicit(communicator).await?;
        let theme = EguiWinitTheme::elicit(communicator).await?;
        let vsync = bool::elicit(communicator).await?;
        let decorations = bool::elicit(communicator).await?;
        let resizable = bool::elicit(communicator).await?;
        let maximized = bool::elicit(communicator).await?;
        let transparent = bool::elicit(communicator).await?;
        Ok(Self {
            app_struct,
            title,
            width,
            height,
            renderer,
            theme,
            vsync,
            decorations,
            resizable,
            maximized,
            transparent,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for EguiWinitDescriptor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::EguiWinitDescriptor",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo { name: "app_struct", type_name: "String", prompt: Some("Application struct name (PascalCase, e.g. \"MyApp\"):") },
                    FieldInfo { name: "title", type_name: "String", prompt: Some("Window title:") },
                    FieldInfo { name: "width", type_name: "u32", prompt: Some("Initial window width (logical pixels):") },
                    FieldInfo { name: "height", type_name: "u32", prompt: Some("Initial window height (logical pixels):") },
                    FieldInfo { name: "renderer", type_name: "EguiWinitRenderer", prompt: Some("GPU rendering backend:") },
                    FieldInfo { name: "theme", type_name: "EguiWinitTheme", prompt: Some("Colour theme:") },
                    FieldInfo { name: "vsync", type_name: "bool", prompt: Some("Enable vertical sync?") },
                    FieldInfo { name: "decorations", type_name: "bool", prompt: Some("Show OS window decorations?") },
                    FieldInfo { name: "resizable", type_name: "bool", prompt: Some("Allow window resizing?") },
                    FieldInfo { name: "maximized", type_name: "bool", prompt: Some("Start maximised?") },
                    FieldInfo { name: "transparent", type_name: "bool", prompt: Some("Transparent window background?") },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiWinitDescriptor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiWinitDescriptor".to_string(),
            fields: vec![
                ("app_struct".to_string(), Box::new(String::prompt_tree())),
                ("title".to_string(), Box::new(String::prompt_tree())),
                ("width".to_string(), Box::new(u32::prompt_tree())),
                ("height".to_string(), Box::new(u32::prompt_tree())),
                ("renderer".to_string(), Box::new(EguiWinitRenderer::prompt_tree())),
                ("theme".to_string(), Box::new(EguiWinitTheme::prompt_tree())),
                ("vsync".to_string(), Box::new(bool::prompt_tree())),
                ("decorations".to_string(), Box::new(bool::prompt_tree())),
                ("resizable".to_string(), Box::new(bool::prompt_tree())),
                ("maximized".to_string(), Box::new(bool::prompt_tree())),
                ("transparent".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for EguiWinitDescriptor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let app_struct = &self.app_struct;
        let title = &self.title;
        let width = self.width;
        let height = self.height;
        let renderer = self.renderer.to_code_literal();
        let theme = self.theme.to_code_literal();
        let vsync = self.vsync;
        let decorations = self.decorations;
        let resizable = self.resizable;
        let maximized = self.maximized;
        let transparent = self.transparent;
        quote::quote! {
            elicitation::EguiWinitDescriptor {
                app_struct: #app_struct.to_string(),
                title: #title.to_string(),
                width: #width,
                height: #height,
                renderer: #renderer,
                theme: #theme,
                vsync: #vsync,
                decorations: #decorations,
                resizable: #resizable,
                maximized: #maximized,
                transparent: #transparent,
            }
        }
    }

    fn type_tokens() -> proc_macro2::TokenStream {
        quote::quote! { elicitation::EguiWinitDescriptor }
    }
}
