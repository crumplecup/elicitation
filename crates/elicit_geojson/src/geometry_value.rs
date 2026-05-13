//! `Value` — GeoJSON geometry-value enum wrapper.

use crate::{GeoJsonResult, Geometry};
use tracing::instrument;

elicitation::elicit_newtype!(geojson::GeometryValue, as Value);
elicitation::elicit_newtype_traits!(Value, geojson::GeometryValue, [display]);

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&*self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        serde_json::from_value::<geojson::GeometryValue>(value)
            .map(Self::from)
            .map_err(serde::de::Error::custom)
    }
}

impl From<Value> for geojson::GeometryValue {
    fn from(value: Value) -> Self {
        (*value.0).clone()
    }
}

impl From<&Value> for geojson::GeometryValue {
    fn from(value: &Value) -> Self {
        (*value.0).clone()
    }
}

impl From<&elicit_geo_types::Point> for Value {
    fn from(value: &elicit_geo_types::Point) -> Self {
        let point: geo_types::Point<f64> = (*value.as_ref()).into();
        Self::from(geojson::GeometryValue::from(&point))
    }
}

impl From<&elicit_geo_types::Line> for Value {
    fn from(value: &elicit_geo_types::Line) -> Self {
        let line: geo_types::Line<f64> = (*value.as_ref()).into();
        Self::from(geojson::GeometryValue::from(&line))
    }
}

impl From<&elicit_geo_types::LineString> for Value {
    fn from(value: &elicit_geo_types::LineString) -> Self {
        let line_string: geo_types::LineString<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&line_string))
    }
}

impl From<&elicit_geo_types::Polygon> for Value {
    fn from(value: &elicit_geo_types::Polygon) -> Self {
        let polygon: geo_types::Polygon<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&polygon))
    }
}

impl From<&elicit_geo_types::MultiPoint> for Value {
    fn from(value: &elicit_geo_types::MultiPoint) -> Self {
        let multi_point: geo_types::MultiPoint<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&multi_point))
    }
}

impl From<&elicit_geo_types::MultiLineString> for Value {
    fn from(value: &elicit_geo_types::MultiLineString) -> Self {
        let multi_line_string: geo_types::MultiLineString<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&multi_line_string))
    }
}

impl From<&elicit_geo_types::MultiPolygon> for Value {
    fn from(value: &elicit_geo_types::MultiPolygon) -> Self {
        let multi_polygon: geo_types::MultiPolygon<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&multi_polygon))
    }
}

impl From<&elicit_geo_types::Rect> for Value {
    fn from(value: &elicit_geo_types::Rect) -> Self {
        let rect: geo_types::Rect<f64> = (*value.as_ref()).into();
        Self::from(geojson::GeometryValue::from(&rect))
    }
}

impl From<&elicit_geo_types::Triangle> for Value {
    fn from(value: &elicit_geo_types::Triangle) -> Self {
        let triangle: geo_types::Triangle<f64> = (*value.as_ref()).into();
        Self::from(geojson::GeometryValue::from(&triangle))
    }
}

impl From<&elicit_geo_types::GeometryCollection> for Value {
    fn from(value: &elicit_geo_types::GeometryCollection) -> Self {
        let collection: geo_types::GeometryCollection<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&collection))
    }
}

impl From<&elicit_geo_types::Geometry> for Value {
    fn from(value: &elicit_geo_types::Geometry) -> Self {
        let geometry: geo_types::Geometry<f64> = value.as_ref().clone().into();
        Self::from(geojson::GeometryValue::from(&geometry))
    }
}

impl TryFrom<Value> for elicit_geo_types::Geometry {
    type Error = geojson::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let geometry = geo_types::Geometry::<f64>::try_from(geojson::GeometryValue::from(value))?;
        Ok(crate::helpers::wrap_geo_geometry(geometry))
    }
}

impl Value {
    /// Returns the concrete geometry variant name.
    #[instrument(skip(self))]
    pub fn type_name(&self) -> &'static str {
        self.0.type_name()
    }

    /// Converts a JSON object into a geometry value.
    #[instrument]
    pub fn from_json_object(object: geojson::JsonObject) -> GeoJsonResult<Self> {
        serde_json::from_value(serde_json::Value::Object(object))
            .map(|v: geojson::GeometryValue| Self::from(v))
            .map_err(|e| Box::new(geojson::Error::from(e)))
    }

    /// Converts a JSON value into a geometry value.
    #[instrument]
    pub fn from_json_value(value: serde_json::Value) -> GeoJsonResult<Self> {
        serde_json::from_value(value)
            .map(|v: geojson::GeometryValue| Self::from(v))
            .map_err(|e| Box::new(geojson::Error::from(e)))
    }

    /// Creates a `Point` geometry value.
    #[instrument]
    pub fn point(coordinates: Vec<f64>) -> Self {
        Self::from(geojson::GeometryValue::Point {
            coordinates: coordinates.into(),
        })
    }

    /// Creates a `MultiPoint` geometry value.
    #[instrument]
    pub fn multi_point(coordinates: Vec<Vec<f64>>) -> Self {
        Self::from(geojson::GeometryValue::MultiPoint {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a `LineString` geometry value.
    #[instrument]
    pub fn line_string(coordinates: Vec<Vec<f64>>) -> Self {
        Self::from(geojson::GeometryValue::LineString {
            coordinates: coordinates.into_iter().map(Into::into).collect(),
        })
    }

    /// Creates a `MultiLineString` geometry value.
    #[instrument]
    pub fn multi_line_string(coordinates: Vec<Vec<Vec<f64>>>) -> Self {
        Self::from(geojson::GeometryValue::MultiLineString {
            coordinates: coordinates
                .into_iter()
                .map(|ring| ring.into_iter().map(Into::into).collect())
                .collect(),
        })
    }

    /// Creates a `Polygon` geometry value.
    #[instrument]
    pub fn polygon(coordinates: Vec<Vec<Vec<f64>>>) -> Self {
        Self::from(geojson::GeometryValue::Polygon {
            coordinates: coordinates
                .into_iter()
                .map(|ring| ring.into_iter().map(Into::into).collect())
                .collect(),
        })
    }

    /// Creates a `MultiPolygon` geometry value.
    #[instrument]
    pub fn multi_polygon(coordinates: Vec<Vec<Vec<Vec<f64>>>>) -> Self {
        Self::from(geojson::GeometryValue::MultiPolygon {
            coordinates: coordinates
                .into_iter()
                .map(|poly| {
                    poly.into_iter()
                        .map(|ring| ring.into_iter().map(Into::into).collect())
                        .collect()
                })
                .collect(),
        })
    }

    /// Creates a `GeometryCollection` geometry value.
    #[instrument]
    pub fn geometry_collection(geometries: Vec<Geometry>) -> Self {
        let inner: Vec<geojson::Geometry> = geometries
            .into_iter()
            .map(geojson::Geometry::from)
            .collect();
        Self::from(geojson::GeometryValue::GeometryCollection { geometries: inner })
    }
}

mod emit_impls {
    use super::Value;

    impl elicitation::emit_code::ToCodeLiteral for Value {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(&*self.0).expect("Value is serializable");
            quote::quote! {
                ::elicit_geojson::GeometryValue::from(
                    ::serde_json::from_str::<::geojson::GeometryValue>(#json)
                        .expect("valid Value JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Value {}
