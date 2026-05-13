//! Wrapper for [`geo_types::Triangle<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::Triangle;

use super::coord::GeoCoord;

/// Elicitable representation of [`geo_types::Triangle<f64>`].
///
/// A triangle defined by three 2D vertices.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct GeoTriangle {
    /// First vertex.
    pub v1: GeoCoord,
    /// Second vertex.
    pub v2: GeoCoord,
    /// Third vertex.
    pub v3: GeoCoord,
}

impl From<Triangle<f64>> for GeoTriangle {
    fn from(t: Triangle<f64>) -> Self {
        Self {
            v1: GeoCoord::from(t.v1()),
            v2: GeoCoord::from(t.v2()),
            v3: GeoCoord::from(t.v3()),
        }
    }
}

impl From<GeoTriangle> for Triangle<f64> {
    fn from(t: GeoTriangle) -> Self {
        Triangle(t.v1.into(), t.v2.into(), t.v3.into())
    }
}

crate::default_style!(GeoTriangle => GeoTriangleStyle);

impl Prompt for GeoTriangle {
    fn prompt() -> Option<&'static str> {
        Some("Specify a triangle (three vertices):")
    }
}

impl Elicitation for GeoTriangle {
    type Style = GeoTriangleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoTriangle");
        let v1 = GeoCoord::elicit(communicator).await?;
        let v2 = GeoCoord::elicit(communicator).await?;
        let v3 = GeoCoord::elicit(communicator).await?;
        Ok(Self { v1, v2, v3 })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        // GeoTriangle has three GeoCoord fields (v1, v2, v3) — delegate to compose.
        GeoCoord::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        GeoCoord::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        GeoCoord::creusot_proof()
    }
}

impl ElicitIntrospect for GeoTriangle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Triangle<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "v1",
                        type_name: "GeoCoord",
                        prompt: Some("First vertex:"),
                    },
                    FieldInfo {
                        name: "v2",
                        type_name: "GeoCoord",
                        prompt: Some("Second vertex:"),
                    },
                    FieldInfo {
                        name: "v3",
                        type_name: "GeoCoord",
                        prompt: Some("Third vertex:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoTriangle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoTriangle".to_string(),
            fields: vec![
                ("v1".to_string(), Box::new(GeoCoord::prompt_tree())),
                ("v2".to_string(), Box::new(GeoCoord::prompt_tree())),
                ("v3".to_string(), Box::new(GeoCoord::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoTriangle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v1 = self.v1.to_code_literal();
        let v2 = self.v2.to_code_literal();
        let v3 = self.v3.to_code_literal();
        quote::quote! {
            elicitation::GeoTriangle { v1: #v1, v2: #v2, v3: #v3 }
        }
    }
}
