//! Wrapper for [`wkt::types::Coord<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Elicitable representation of [`wkt::types::Coord<f64>`].
///
/// A 2D/3D/4D WKT coordinate with x, y, optional z and optional m values.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktCoord {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
    /// Optional Z (altitude/elevation).
    #[serde(default)]
    pub z: Option<f64>,
    /// Optional M (measure).
    #[serde(default)]
    pub m: Option<f64>,
}

impl From<wkt::types::Coord<f64>> for WktCoord {
    fn from(c: wkt::types::Coord<f64>) -> Self {
        Self {
            x: c.x,
            y: c.y,
            z: c.z,
            m: c.m,
        }
    }
}

impl From<WktCoord> for wkt::types::Coord<f64> {
    fn from(c: WktCoord) -> Self {
        Self {
            x: c.x,
            y: c.y,
            z: c.z,
            m: c.m,
        }
    }
}

crate::default_style!(WktCoord => WktCoordStyle);

impl Prompt for WktCoord {
    fn prompt() -> Option<&'static str> {
        Some("Specify a WKT coordinate (x, y, optional z, optional m):")
    }
}

impl Elicitation for WktCoord {
    type Style = WktCoordStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktCoord");
        let x = f64::elicit(communicator).await?;
        let y = f64::elicit(communicator).await?;
        Ok(Self {
            x,
            y,
            z: None,
            m: None,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f64 as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f64 as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f64 as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for WktCoord {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::Coord<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f64",
                        prompt: Some("X coordinate:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f64",
                        prompt: Some("Y coordinate:"),
                    },
                    FieldInfo {
                        name: "z",
                        type_name: "Option<f64>",
                        prompt: Some("Z coordinate (optional):"),
                    },
                    FieldInfo {
                        name: "m",
                        type_name: "Option<f64>",
                        prompt: Some("M measure (optional):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktCoord {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "WktCoord".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WktCoord {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        let z_tokens = match self.z {
            Some(v) => quote::quote! { Some(#v) },
            None => quote::quote! { None },
        };
        let m_tokens = match self.m {
            Some(v) => quote::quote! { Some(#v) },
            None => quote::quote! { None },
        };
        quote::quote! {
            elicitation::WktCoord { x: #x, y: #y, z: #z_tokens, m: #m_tokens }
        }
    }
}
