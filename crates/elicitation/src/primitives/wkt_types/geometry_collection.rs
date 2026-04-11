//! Wrapper for [`wkt::types::GeometryCollection<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::wkt_geom::WktGeom;

/// Elicitable representation of [`wkt::types::GeometryCollection<f64>`].
///
/// A heterogeneous collection of WKT geometry values.
/// Uses `Vec<WktGeom>` instead of `Vec<wkt::Wkt<f64>>` to avoid a recursive cycle.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktGeometryCollection {
    /// Collection of WKT geometries.
    pub geometries: Vec<WktGeom>,
}

impl From<wkt::types::GeometryCollection<f64>> for WktGeometryCollection {
    fn from(gc: wkt::types::GeometryCollection<f64>) -> Self {
        Self {
            geometries: gc.0.into_iter().map(WktGeom::from).collect(),
        }
    }
}

impl From<WktGeometryCollection> for wkt::types::GeometryCollection<f64> {
    fn from(gc: WktGeometryCollection) -> Self {
        wkt::types::GeometryCollection(gc.geometries.into_iter().map(wkt::Wkt::from).collect())
    }
}

crate::default_style!(WktGeometryCollection => WktGeometryCollectionStyle);

impl Prompt for WktGeometryCollection {
    fn prompt() -> Option<&'static str> {
        Some("Build a WKT geometry collection (add geometries one by one):")
    }
}

impl Elicitation for WktGeometryCollection {
    type Style = WktGeometryCollectionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktGeometryCollection");
        let geometries = Vec::<WktGeom>::elicit(communicator).await?;
        Ok(Self { geometries })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<WktGeom>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<WktGeom>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<WktGeom>::creusot_proof()
    }
}

impl ElicitIntrospect for WktGeometryCollection {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::GeometryCollection<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "geometries",
                    type_name: "Vec<WktGeom>",
                    prompt: Some("Geometry collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktGeometryCollection {
    fn prompt_tree() -> crate::PromptTree {
        WktGeom::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for WktGeometryCollection {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self
            .geometries
            .iter()
            .map(|g| g.to_code_literal())
            .collect();
        quote::quote! {
            elicitation::WktGeometryCollection { geometries: vec![#(#items),*] }
        }
    }
}
