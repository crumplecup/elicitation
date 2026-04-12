//! DPI / sizing types and serializable `WindowAttributes` config for winit.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{WinitThemeSelect, WinitWindowLevelSelect};

/// Serializable physical-pixel size (integer coordinates).
///
/// Maps to `winit::dpi::PhysicalSize<u32>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WinitPhysicalSize {
    /// Horizontal extent in physical pixels.
    pub width: u32,
    /// Vertical extent in physical pixels.
    pub height: u32,
}

impl From<WinitPhysicalSize> for winit::dpi::PhysicalSize<u32> {
    fn from(s: WinitPhysicalSize) -> Self {
        Self::new(s.width, s.height)
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for WinitPhysicalSize {
    fn from(s: winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

/// Serializable logical (DPI-scaled) size.
///
/// Maps to `winit::dpi::LogicalSize<f64>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WinitLogicalSize {
    /// Horizontal extent in logical pixels.
    pub width: f64,
    /// Vertical extent in logical pixels.
    pub height: f64,
}

impl From<WinitLogicalSize> for winit::dpi::LogicalSize<f64> {
    fn from(s: WinitLogicalSize) -> Self {
        Self::new(s.width, s.height)
    }
}

impl From<winit::dpi::LogicalSize<f64>> for WinitLogicalSize {
    fn from(s: winit::dpi::LogicalSize<f64>) -> Self {
        Self {
            width: s.width,
            height: s.height,
        }
    }
}

/// Serializable logical position.
///
/// Maps to `winit::dpi::LogicalPosition<f64>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WinitLogicalPosition {
    /// Horizontal offset in logical pixels.
    pub x: f64,
    /// Vertical offset in logical pixels.
    pub y: f64,
}

impl From<WinitLogicalPosition> for winit::dpi::LogicalPosition<f64> {
    fn from(p: WinitLogicalPosition) -> Self {
        Self::new(p.x, p.y)
    }
}

/// Flat, serializable mirror of `winit::window::WindowAttributes`.
///
/// Used as the param type for window-creation code-generation tools.  All
/// fields are optional (except `title`) so users can set only what they care
/// about; unset fields fall back to winit defaults.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WinitWindowAttributes {
    /// Window title bar text.
    pub title: String,
    /// Initial inner (content area) size in logical pixels.
    pub inner_size: Option<WinitLogicalSize>,
    /// Minimum inner size the user can resize to.
    pub min_inner_size: Option<WinitLogicalSize>,
    /// Maximum inner size the user can resize to.
    pub max_inner_size: Option<WinitLogicalSize>,
    /// Initial position of the window's top-left corner.
    pub position: Option<WinitLogicalPosition>,
    /// Whether the user can resize the window. Defaults to `true`.
    pub resizable: Option<bool>,
    /// Whether the window starts maximized.
    pub maximized: Option<bool>,
    /// Whether the window is initially visible.
    pub visible: Option<bool>,
    /// Whether the window background is transparent.
    pub transparent: Option<bool>,
    /// Whether the window has OS decorations (title bar, borders).
    pub decorations: Option<bool>,
    /// Whether the window starts in borderless-fullscreen mode.
    pub fullscreen: Option<bool>,
    /// Window stacking level relative to other windows.
    pub window_level: Option<WinitWindowLevelSelect>,
    /// Preferred OS color theme.
    pub theme: Option<WinitThemeSelect>,
    /// Whether the window requests input focus on creation.
    pub active: Option<bool>,
}

// ── WinitPhysicalSize elicitation ─────────────────────────────────────────────

impl Prompt for WinitPhysicalSize {
    fn prompt() -> Option<&'static str> {
        Some("Specify a physical-pixel size (integer width × height):")
    }
}

crate::default_style!(WinitPhysicalSize => WinitPhysicalSizeStyle);

impl Elicitation for WinitPhysicalSize {
    type Style = WinitPhysicalSizeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WinitPhysicalSize");
        Ok(Self {
            width: u32::elicit(communicator).await?,
            height: u32::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::kani_proof();
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::verus_proof();
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::creusot_proof();
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WinitPhysicalSize {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WinitPhysicalSize",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        type_name: "u32",
                        prompt: Some("Width in physical pixels:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "u32",
                        prompt: Some("Height in physical pixels:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WinitPhysicalSize {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WinitPhysicalSize".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(u32::prompt_tree())),
                ("height".to_string(), Box::new(u32::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for WinitPhysicalSize {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let w = self.width;
        let h = self.height;
        quote::quote! {
            ::elicitation::WinitPhysicalSize { width: #w, height: #h }
        }
    }
}

// ── WinitLogicalSize elicitation ──────────────────────────────────────────────

impl Prompt for WinitLogicalSize {
    fn prompt() -> Option<&'static str> {
        Some("Specify a logical (DPI-scaled) size (f64 width × height):")
    }
}

crate::default_style!(WinitLogicalSize => WinitLogicalSizeStyle);

impl Elicitation for WinitLogicalSize {
    type Style = WinitLogicalSizeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WinitLogicalSize");
        Ok(Self {
            width: f64::elicit(communicator).await?,
            height: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WinitLogicalSize {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WinitLogicalSize",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        type_name: "f64",
                        prompt: Some("Width in logical pixels:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "f64",
                        prompt: Some("Height in logical pixels:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WinitLogicalSize {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WinitLogicalSize".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(f64::prompt_tree())),
                ("height".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for WinitLogicalSize {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let w = self.width;
        let h = self.height;
        quote::quote! {
            ::elicitation::WinitLogicalSize { width: #w, height: #h }
        }
    }
}

// ── WinitLogicalPosition elicitation ─────────────────────────────────────────

impl Prompt for WinitLogicalPosition {
    fn prompt() -> Option<&'static str> {
        Some("Specify a logical (DPI-scaled) position (f64 x, y):")
    }
}

crate::default_style!(WinitLogicalPosition => WinitLogicalPositionStyle);

impl Elicitation for WinitLogicalPosition {
    type Style = WinitLogicalPositionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WinitLogicalPosition");
        Ok(Self {
            x: f64::elicit(communicator).await?,
            y: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WinitLogicalPosition {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WinitLogicalPosition",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f64",
                        prompt: Some("Horizontal offset in logical pixels:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f64",
                        prompt: Some("Vertical offset in logical pixels:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WinitLogicalPosition {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WinitLogicalPosition".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for WinitLogicalPosition {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! {
            ::elicitation::WinitLogicalPosition { x: #x, y: #y }
        }
    }
}

// ── WinitWindowAttributes elicitation ─────────────────────────────────────────

impl Prompt for WinitWindowAttributes {
    fn prompt() -> Option<&'static str> {
        Some("Configure window attributes (title required; all other fields optional):")
    }
}

crate::default_style!(WinitWindowAttributes => WinitWindowAttributesStyle);

impl Elicitation for WinitWindowAttributes {
    type Style = WinitWindowAttributesStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WinitWindowAttributes");
        Ok(Self {
            title: String::elicit(communicator).await?,
            inner_size: Option::<WinitLogicalSize>::elicit(communicator).await?,
            min_inner_size: Option::<WinitLogicalSize>::elicit(communicator).await?,
            max_inner_size: Option::<WinitLogicalSize>::elicit(communicator).await?,
            position: Option::<WinitLogicalPosition>::elicit(communicator).await?,
            resizable: Option::<bool>::elicit(communicator).await?,
            maximized: Option::<bool>::elicit(communicator).await?,
            visible: Option::<bool>::elicit(communicator).await?,
            transparent: Option::<bool>::elicit(communicator).await?,
            decorations: Option::<bool>::elicit(communicator).await?,
            fullscreen: Option::<bool>::elicit(communicator).await?,
            window_level: Option::<WinitWindowLevelSelect>::elicit(communicator).await?,
            theme: Option::<WinitThemeSelect>::elicit(communicator).await?,
            active: Option::<bool>::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        // Compose proof from representative fields: title string + logical size
        let mut ts = <String as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <String as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WinitWindowAttributes {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WinitWindowAttributes",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "title",
                        type_name: "String",
                        prompt: Some("Window title:"),
                    },
                    FieldInfo {
                        name: "inner_size",
                        type_name: "Option<WinitLogicalSize>",
                        prompt: Some("Inner size (optional):"),
                    },
                    FieldInfo {
                        name: "resizable",
                        type_name: "Option<bool>",
                        prompt: Some("Resizable (optional):"),
                    },
                    FieldInfo {
                        name: "decorations",
                        type_name: "Option<bool>",
                        prompt: Some("Decorations (optional):"),
                    },
                    FieldInfo {
                        name: "transparent",
                        type_name: "Option<bool>",
                        prompt: Some("Transparent (optional):"),
                    },
                    FieldInfo {
                        name: "fullscreen",
                        type_name: "Option<bool>",
                        prompt: Some("Fullscreen (optional):"),
                    },
                    FieldInfo {
                        name: "window_level",
                        type_name: "Option<WinitWindowLevelSelect>",
                        prompt: Some("Window level (optional):"),
                    },
                    FieldInfo {
                        name: "theme",
                        type_name: "Option<WinitThemeSelect>",
                        prompt: Some("Theme (optional):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WinitWindowAttributes {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WinitWindowAttributes".to_string(),
            fields: vec![
                ("title".to_string(), Box::new(String::prompt_tree())),
                (
                    "inner_size".to_string(),
                    Box::new(Option::<WinitLogicalSize>::prompt_tree()),
                ),
                (
                    "min_inner_size".to_string(),
                    Box::new(Option::<WinitLogicalSize>::prompt_tree()),
                ),
                (
                    "max_inner_size".to_string(),
                    Box::new(Option::<WinitLogicalSize>::prompt_tree()),
                ),
                (
                    "position".to_string(),
                    Box::new(Option::<WinitLogicalPosition>::prompt_tree()),
                ),
                (
                    "resizable".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "maximized".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "visible".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "transparent".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "decorations".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "fullscreen".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
                (
                    "window_level".to_string(),
                    Box::new(Option::<WinitWindowLevelSelect>::prompt_tree()),
                ),
                (
                    "theme".to_string(),
                    Box::new(Option::<WinitThemeSelect>::prompt_tree()),
                ),
                (
                    "active".to_string(),
                    Box::new(Option::<bool>::prompt_tree()),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for WinitWindowAttributes {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let title = &self.title;
        let inner_size = self.inner_size.to_code_literal();
        let min_inner_size = self.min_inner_size.to_code_literal();
        let max_inner_size = self.max_inner_size.to_code_literal();
        let position = self.position.to_code_literal();
        let resizable = self.resizable.to_code_literal();
        let maximized = self.maximized.to_code_literal();
        let visible = self.visible.to_code_literal();
        let transparent = self.transparent.to_code_literal();
        let decorations = self.decorations.to_code_literal();
        let fullscreen = self.fullscreen.to_code_literal();
        let window_level = self.window_level.to_code_literal();
        let theme = self.theme.to_code_literal();
        let active = self.active.to_code_literal();
        quote::quote! {
            ::elicitation::WinitWindowAttributes {
                title: #title.to_string(),
                inner_size: #inner_size,
                min_inner_size: #min_inner_size,
                max_inner_size: #max_inner_size,
                position: #position,
                resizable: #resizable,
                maximized: #maximized,
                visible: #visible,
                transparent: #transparent,
                decorations: #decorations,
                fullscreen: #fullscreen,
                window_level: #window_level,
                theme: #theme,
                active: #active,
            }
        }
    }
}
