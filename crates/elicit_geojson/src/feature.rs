//! `Feature` — GeoJSON feature-object wrapper.

use crate::{GeoJsonResult, Geometry, Id, Value, helpers::geojson_wrapper};
use tracing::instrument;

geojson_wrapper!(Feature, geojson::Feature);
elicitation::elicit_newtype_traits!(Feature, geojson::Feature, [display, from_str]);

impl From<Geometry> for Feature {
    fn from(value: Geometry) -> Self {
        Self::from(geojson::Feature::from(geojson::Geometry::from(value)))
    }
}

impl From<Value> for Feature {
    fn from(value: Value) -> Self {
        Self::from(geojson::Feature::from(geojson::Value::from(value)))
    }
}

impl TryFrom<Feature> for elicit_geo_types::Geometry {
    type Error = geojson::Error;

    fn try_from(value: Feature) -> Result<Self, Self::Error> {
        let geometry = geo_types::Geometry::<f64>::try_from(geojson::Feature::from(value))?;
        Ok(crate::helpers::wrap_geo_geometry(geometry))
    }
}

impl Feature {
    /// Converts a JSON object into a feature object.
    #[instrument]
    pub fn from_json_object(object: geojson::JsonObject) -> GeoJsonResult<Self> {
        geojson::Feature::from_json_object(object)
            .map(Self::from)
            .map_err(Box::new)
    }

    /// Converts a JSON value into a feature object.
    #[instrument]
    pub fn from_json_value(value: serde_json::Value) -> GeoJsonResult<Self> {
        geojson::Feature::from_json_value(value)
            .map(Self::from)
            .map_err(Box::new)
    }

    /// Returns the property value stored at `key`, if present.
    #[instrument(skip(self, key))]
    pub fn property(&self, key: impl AsRef<str>) -> Option<&geojson::JsonValue> {
        self.0.property(key)
    }

    /// Returns `true` when `key` is present in the feature properties map.
    #[instrument(skip(self, key))]
    pub fn contains_property(&self, key: impl AsRef<str>) -> bool {
        self.0.contains_property(key)
    }

    /// Sets the property at `key` to `value`, creating the properties map if needed.
    #[instrument(skip(self, key, value))]
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<geojson::JsonValue>) {
        std::sync::Arc::make_mut(&mut self.0).set_property(key, value);
    }

    /// Removes the property at `key`, if present.
    #[instrument(skip(self, key))]
    pub fn remove_property(&mut self, key: impl AsRef<str>) -> Option<geojson::JsonValue> {
        std::sync::Arc::make_mut(&mut self.0).remove_property(key)
    }

    /// Returns the number of properties stored on this feature.
    #[instrument(skip(self))]
    pub fn len_properties(&self) -> usize {
        self.0.len_properties()
    }

    /// Returns an iterator over all feature properties.
    #[instrument(skip(self))]
    pub fn properties_iter(
        &self,
    ) -> Box<dyn ExactSizeIterator<Item = (&String, &geojson::JsonValue)> + '_> {
        self.0.properties_iter()
    }

    /// Returns the feature identifier, if present.
    #[instrument(skip(self))]
    pub fn id(&self) -> Option<Id> {
        self.0.id.clone().map(Id::from)
    }
}

mod emit_impls {
    use super::Feature;

    impl elicitation::emit_code::ToCodeLiteral for Feature {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("Feature is serializable");
            quote::quote! {
                ::elicit_geojson::Feature::from(
                    ::serde_json::from_str::<::geojson::Feature>(#json)
                        .expect("valid Feature JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Feature {}
