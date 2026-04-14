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

#[cfg(feature = "emit")]
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

#[cfg(feature = "emit")]
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
