//! Elicitation wrappers for wgpu 29 plain-struct types:
//! [`WgpuExtent3d`], [`WgpuColor`], and [`WgpuOrigin3d`].
//!
//! These are our own serializable structs (with serde + JsonSchema) that mirror
//! the wgpu types and convert to/from them.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── WgpuExtent3d ──────────────────────────────────────────────────────────────

/// Serializable 3-D extent used for textures and copy regions.
///
/// Maps to `wgpu::Extent3d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WgpuExtent3d {
    /// Width in texels.
    pub width: u32,
    /// Height in texels.
    pub height: u32,
    /// Depth (or array layer count for 2-D arrays).
    #[serde(rename = "depthOrArrayLayers")]
    pub depth_or_array_layers: u32,
}

impl Default for WgpuExtent3d {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }
    }
}

impl From<WgpuExtent3d> for wgpu::Extent3d {
    fn from(e: WgpuExtent3d) -> Self {
        Self {
            width: e.width,
            height: e.height,
            depth_or_array_layers: e.depth_or_array_layers,
        }
    }
}

impl From<wgpu::Extent3d> for WgpuExtent3d {
    fn from(e: wgpu::Extent3d) -> Self {
        Self {
            width: e.width,
            height: e.height,
            depth_or_array_layers: e.depth_or_array_layers,
        }
    }
}

impl Prompt for WgpuExtent3d {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 3-D extent (width × height × depth/layers):")
    }
}

crate::default_style!(WgpuExtent3d => WgpuExtent3dStyle);

impl Elicitation for WgpuExtent3d {
    type Style = WgpuExtent3dStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WgpuExtent3d");
        Ok(Self {
            width: u32::elicit(communicator).await?,
            height: u32::elicit(communicator).await?,
            depth_or_array_layers: u32::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::kani_proof();
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::verus_proof();
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::creusot_proof();
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WgpuExtent3d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WgpuExtent3d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        type_name: "u32",
                        prompt: Some("Width in texels:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "u32",
                        prompt: Some("Height in texels:"),
                    },
                    FieldInfo {
                        name: "depth_or_array_layers",
                        type_name: "u32",
                        prompt: Some("Depth (or number of array layers):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WgpuExtent3d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WgpuExtent3d".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(u32::prompt_tree())),
                ("height".to_string(), Box::new(u32::prompt_tree())),
                (
                    "depth_or_array_layers".to_string(),
                    Box::new(u32::prompt_tree()),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for WgpuExtent3d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let w = self.width;
        let h = self.height;
        let d = self.depth_or_array_layers;
        quote::quote! {
            wgpu::Extent3d { width: #w, height: #h, depth_or_array_layers: #d }
        }
    }
}

// ── WgpuColor ─────────────────────────────────────────────────────────────────

/// Serializable RGBA color with f64 channels in [0.0, 1.0].
///
/// Maps to `wgpu::Color`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WgpuColor {
    /// Red channel.
    pub r: f64,
    /// Green channel.
    pub g: f64,
    /// Blue channel.
    pub b: f64,
    /// Alpha channel.
    pub a: f64,
}

impl Default for WgpuColor {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl From<WgpuColor> for wgpu::Color {
    fn from(c: WgpuColor) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<wgpu::Color> for WgpuColor {
    fn from(c: wgpu::Color) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl Prompt for WgpuColor {
    fn prompt() -> Option<&'static str> {
        Some("Specify an RGBA color (f64 channels, 0.0–1.0):")
    }
}

crate::default_style!(WgpuColor => WgpuColorStyle);

impl Elicitation for WgpuColor {
    type Style = WgpuColorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WgpuColor");
        Ok(Self {
            r: f64::elicit(communicator).await?,
            g: f64::elicit(communicator).await?,
            b: f64::elicit(communicator).await?,
            a: f64::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::kani_proof();
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts.extend(<f64 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::verus_proof();
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts.extend(<f64 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f64 as Elicitation>::creusot_proof();
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts.extend(<f64 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WgpuColor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WgpuColor",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "r",
                        type_name: "f64",
                        prompt: Some("Red channel (0.0–1.0):"),
                    },
                    FieldInfo {
                        name: "g",
                        type_name: "f64",
                        prompt: Some("Green channel (0.0–1.0):"),
                    },
                    FieldInfo {
                        name: "b",
                        type_name: "f64",
                        prompt: Some("Blue channel (0.0–1.0):"),
                    },
                    FieldInfo {
                        name: "a",
                        type_name: "f64",
                        prompt: Some("Alpha channel (0.0=transparent, 1.0=opaque):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WgpuColor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WgpuColor".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(f64::prompt_tree())),
                ("g".to_string(), Box::new(f64::prompt_tree())),
                ("b".to_string(), Box::new(f64::prompt_tree())),
                ("a".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for WgpuColor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;
        quote::quote! {
            wgpu::Color { r: #r, g: #g, b: #b, a: #a }
        }
    }
}

// ── WgpuOrigin3d ──────────────────────────────────────────────────────────────

/// Serializable 3-D texel origin used for copy operations.
///
/// Maps to `wgpu::Origin3d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WgpuOrigin3d {
    /// X offset in texels.
    pub x: u32,
    /// Y offset in texels.
    pub y: u32,
    /// Z offset in texels.
    pub z: u32,
}

impl Default for WgpuOrigin3d {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl From<WgpuOrigin3d> for wgpu::Origin3d {
    fn from(o: WgpuOrigin3d) -> Self {
        Self {
            x: o.x,
            y: o.y,
            z: o.z,
        }
    }
}

impl From<wgpu::Origin3d> for WgpuOrigin3d {
    fn from(o: wgpu::Origin3d) -> Self {
        Self {
            x: o.x,
            y: o.y,
            z: o.z,
        }
    }
}

impl Prompt for WgpuOrigin3d {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 3-D texel origin (x, y, z offsets):")
    }
}

crate::default_style!(WgpuOrigin3d => WgpuOrigin3dStyle);

impl Elicitation for WgpuOrigin3d {
    type Style = WgpuOrigin3dStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WgpuOrigin3d");
        Ok(Self {
            x: u32::elicit(communicator).await?,
            y: u32::elicit(communicator).await?,
            z: u32::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::kani_proof();
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts.extend(<u32 as Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::verus_proof();
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts.extend(<u32 as Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <u32 as Elicitation>::creusot_proof();
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts.extend(<u32 as Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for WgpuOrigin3d {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WgpuOrigin3d",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "u32",
                        prompt: Some("X offset in texels:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "u32",
                        prompt: Some("Y offset in texels:"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "u32",
                        prompt: Some("Z offset in texels:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WgpuOrigin3d {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "WgpuOrigin3d".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(u32::prompt_tree())),
                ("y".to_string(), Box::new(u32::prompt_tree())),
                ("z".to_string(), Box::new(u32::prompt_tree())),
            ],
        }
    }
}

impl ToCodeLiteral for WgpuOrigin3d {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        quote::quote! {
            wgpu::Origin3d { x: #x, y: #y, z: #z }
        }
    }
}
