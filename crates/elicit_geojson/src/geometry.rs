//! `Geometry` — GeoJSON geometry-object wrapper.

use crate::{GeoJsonResult, Value, helpers::geojson_wrapper};
use tracing::instrument;

geojson_wrapper!(Geometry, geojson::Geometry);
elicitation::elicit_newtype_traits!(Geometry, geojson::Geometry, [display, from_str]);

impl From<Value> for Geometry {
    fn from(value: Value) -> Self {
        Self::from(geojson::Geometry::from(geojson::Value::from(value)))
    }
}

impl From<&elicit_geo_types::Geometry> for Geometry {
    fn from(value: &elicit_geo_types::Geometry) -> Self {
        let geometry: geo_types::Geometry<f64> = value.as_ref().clone().into();
        Self::from(geojson::Geometry::from(&geometry))
    }
}

impl TryFrom<Geometry> for elicit_geo_types::Geometry {
    type Error = geojson::Error;

    fn try_from(value: Geometry) -> Result<Self, Self::Error> {
        let geometry = geo_types::Geometry::<f64>::try_from(geojson::Geometry::from(value))?;
        Ok(crate::helpers::wrap_geo_geometry(geometry))
    }
}

impl Geometry {
    /// Creates a new geometry object from a geometry value.
    #[instrument]
    pub fn new(value: Value) -> Self {
        Self::from(geojson::Geometry::new(geojson::Value::from(value)))
    }

    /// Converts a JSON object into a geometry object.
    #[instrument]
    pub fn from_json_object(object: geojson::JsonObject) -> GeoJsonResult<Self> {
        geojson::Geometry::from_json_object(object)
            .map(Self::from)
            .map_err(Box::new)
    }

    /// Converts a JSON value into a geometry object.
    #[instrument]
    pub fn from_json_value(value: serde_json::Value) -> GeoJsonResult<Self> {
        geojson::Geometry::from_json_value(value)
            .map(Self::from)
            .map_err(Box::new)
    }
}

mod emit_impls {
    use super::Geometry;

    impl elicitation::emit_code::ToCodeLiteral for Geometry {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("Geometry is serializable");
            quote::quote! {
                ::elicit_geojson::Geometry::from(
                    ::serde_json::from_str::<::geojson::Geometry>(#json)
                        .expect("valid Geometry JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Geometry {}
