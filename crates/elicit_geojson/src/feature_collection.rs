//! `FeatureCollection` — GeoJSON feature-collection wrapper.

use crate::{Feature, GeoJsonResult, helpers::geojson_wrapper};
use tracing::instrument;

geojson_wrapper!(FeatureCollection, geojson::FeatureCollection);
elicitation::elicit_newtype_traits!(
    FeatureCollection,
    geojson::FeatureCollection,
    [display, from_str]
);

impl FromIterator<Feature> for FeatureCollection {
    fn from_iter<T: IntoIterator<Item = Feature>>(iter: T) -> Self {
        let inner: geojson::FeatureCollection =
            iter.into_iter().map(geojson::Feature::from).collect();
        Self::from(inner)
    }
}

impl From<&elicit_geo_types::GeometryCollection> for FeatureCollection {
    fn from(value: &elicit_geo_types::GeometryCollection) -> Self {
        let collection: geo_types::GeometryCollection<f64> = value.as_ref().clone().into();
        Self::from(geojson::FeatureCollection::from(&collection))
    }
}

impl TryFrom<FeatureCollection> for elicit_geo_types::Geometry {
    type Error = geojson::Error;

    fn try_from(value: FeatureCollection) -> Result<Self, Self::Error> {
        let geometry =
            geo_types::Geometry::<f64>::try_from(geojson::FeatureCollection::from(value))?;
        Ok(crate::helpers::wrap_geo_geometry(geometry))
    }
}

impl FeatureCollection {
    /// Converts a JSON object into a feature collection.
    #[instrument]
    pub fn from_json_object(object: geojson::JsonObject) -> GeoJsonResult<Self> {
        geojson::FeatureCollection::from_json_object(object)
            .map(Self::from)
            .map_err(Box::new)
    }

    /// Converts a JSON value into a feature collection.
    #[instrument]
    pub fn from_json_value(value: serde_json::Value) -> GeoJsonResult<Self> {
        geojson::FeatureCollection::from_json_value(value)
            .map(Self::from)
            .map_err(Box::new)
    }
}

mod emit_impls {
    use super::FeatureCollection;

    impl elicitation::emit_code::ToCodeLiteral for FeatureCollection {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("FeatureCollection is serializable");
            quote::quote! {
                ::elicit_geojson::FeatureCollection::from(
                    ::serde_json::from_str::<::geojson::FeatureCollection>(#json)
                        .expect("valid FeatureCollection JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for FeatureCollection {}
