//! `GeoJson` — top-level GeoJSON document wrapper.

use crate::{Feature, FeatureCollection, GeoJsonResult, Geometry};
use std::io::Read;
use tracing::instrument;

use crate::helpers::geojson_wrapper;

geojson_wrapper!(GeoJson, geojson::GeoJson);
elicitation::elicit_newtype_traits!(GeoJson, geojson::GeoJson, [display, from_str]);

impl From<Geometry> for GeoJson {
    fn from(value: Geometry) -> Self {
        Self::from(geojson::GeoJson::from(geojson::Geometry::from(value)))
    }
}

impl From<Feature> for GeoJson {
    fn from(value: Feature) -> Self {
        Self::from(geojson::GeoJson::from(geojson::Feature::from(value)))
    }
}

impl From<FeatureCollection> for GeoJson {
    fn from(value: FeatureCollection) -> Self {
        Self::from(geojson::GeoJson::from(geojson::FeatureCollection::from(
            value,
        )))
    }
}

impl From<Vec<Feature>> for GeoJson {
    fn from(value: Vec<Feature>) -> Self {
        let features: Vec<geojson::Feature> =
            value.into_iter().map(geojson::Feature::from).collect();
        Self::from(geojson::GeoJson::from(features))
    }
}

impl TryFrom<GeoJson> for Geometry {
    type Error = geojson::Error;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        geojson::Geometry::try_from(geojson::GeoJson::from(value)).map(Self::from)
    }
}

impl TryFrom<GeoJson> for Feature {
    type Error = geojson::Error;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        geojson::Feature::try_from(geojson::GeoJson::from(value)).map(Self::from)
    }
}

impl TryFrom<GeoJson> for FeatureCollection {
    type Error = geojson::Error;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        geojson::FeatureCollection::try_from(geojson::GeoJson::from(value)).map(Self::from)
    }
}

impl TryFrom<GeoJson> for elicit_geo_types::Geometry {
    type Error = geojson::Error;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        let geometry = geo_types::Geometry::<f64>::try_from(geojson::GeoJson::from(value))?;
        Ok(crate::helpers::wrap_geo_geometry(geometry))
    }
}

impl GeoJson {
    /// Converts a JSON object into a GeoJSON document.
    #[instrument]
    pub fn from_json_object(object: geojson::JsonObject) -> GeoJsonResult<Self> {
        serde_json::from_value(serde_json::Value::Object(object))
            .map(|g: geojson::GeoJson| Self::from(g))
            .map_err(|e| Box::new(geojson::Error::from(e)))
    }

    /// Converts a JSON value into a GeoJSON document.
    #[instrument]
    pub fn from_json_value(value: serde_json::Value) -> GeoJsonResult<Self> {
        serde_json::from_value(value)
            .map(|g: geojson::GeoJson| Self::from(g))
            .map_err(|e| Box::new(geojson::Error::from(e)))
    }

    /// Converts this document into a JSON value.
    #[instrument(skip(self))]
    pub fn to_json_value(self) -> serde_json::Value {
        serde_json::to_value(geojson::GeoJson::from(self)).unwrap_or(serde_json::Value::Null)
    }

    /// Reads a GeoJSON document from a reader.
    #[instrument(skip(rdr))]
    pub fn from_reader<R>(rdr: R) -> serde_json::Result<Self>
    where
        R: Read,
    {
        geojson::GeoJson::from_reader(rdr).map(Self::from)
    }

    /// Serializes this document to pretty-printed JSON.
    #[instrument(skip(self))]
    pub fn to_string_pretty(self) -> GeoJsonResult<String> {
        geojson::GeoJson::from(self)
            .to_string_pretty()
            .map_err(Box::new)
    }
}

mod emit_impls {
    use super::GeoJson;

    impl elicitation::emit_code::ToCodeLiteral for GeoJson {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("GeoJson is serializable");
            quote::quote! {
                ::elicit_geojson::GeoJson::from(
                    ::serde_json::from_str::<::geojson::GeoJson>(#json)
                        .expect("valid GeoJson JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for GeoJson {}
