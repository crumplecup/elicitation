//! Wrapper for [`geo_types::GeometryCollection<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::GeometryCollection;

use super::geometry::GeoGeometry;

/// Elicitable representation of [`geo_types::GeometryCollection<f64>`].
///
/// A heterogeneous collection of geometry values.
/// Elicitation collects geometries iteratively until the user stops adding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoGeometryCollection(pub Vec<GeoGeometry>);

impl From<GeometryCollection<f64>> for GeoGeometryCollection {
    fn from(gc: GeometryCollection<f64>) -> Self {
        Self(gc.0.into_iter().map(GeoGeometry::from).collect())
    }
}

impl From<GeoGeometryCollection> for GeometryCollection<f64> {
    fn from(gc: GeoGeometryCollection) -> Self {
        GeometryCollection(gc.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(GeoGeometryCollection => GeoGeometryCollectionStyle);

impl Prompt for GeoGeometryCollection {
    fn prompt() -> Option<&'static str> {
        Some("Build a geometry collection (add geometries one by one):")
    }
}

impl Elicitation for GeoGeometryCollection {
    type Style = GeoGeometryCollectionStyle;

    // Use Box::pin + concrete Pin<Box<dyn Future>> return type (same pattern as
    // GeoGeometry::elicit) to break the compile-time Send-bound inference cycle.
    // async fn would return an opaque impl Future, forcing the compiler to
    // recurse through GeoGeometry→GeoGeometryCollection→Vec<GeoGeometry>→…
    // indefinitely to verify Send. With dyn Future + Send the bound is trivially
    // satisfied at the await site without inspecting the body.
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            tracing::debug!("Eliciting GeoGeometryCollection");
            let geometries = Vec::<GeoGeometry>::elicit(communicator).await?;
            Ok(Self(geometries))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<GeoGeometry>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<GeoGeometry>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<GeoGeometry>::creusot_proof()
    }
}

impl ElicitIntrospect for GeoGeometryCollection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::GeometryCollection<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "geometries",
                    type_name: "Vec<GeoGeometry>",
                    prompt: Some("Geometry collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoGeometryCollection {
    fn prompt_tree() -> crate::PromptTree {
        GeoGeometry::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for GeoGeometryCollection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.0.iter().map(|g| g.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoGeometryCollection(vec![#(#items),*])
        }
    }
}
