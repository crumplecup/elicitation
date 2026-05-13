//! Internal helpers for the GeoJSON shadow crate.

/// Declares a common Arc-backed wrapper around an upstream GeoJSON type.
macro_rules! geojson_wrapper {
    ($wrapper:ident, $inner:path) => {
        elicitation::elicit_newtype!($inner, as $wrapper);

        impl ::serde::Serialize for $wrapper {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&*self.0, serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $wrapper {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <$inner as ::serde::Deserialize>::deserialize(deserializer).map(Self::from)
            }
        }

        impl From<$wrapper> for $inner {
            fn from(value: $wrapper) -> Self {
                (*value.0).clone()
            }
        }

        impl From<&$wrapper> for $inner {
            fn from(value: &$wrapper) -> Self {
                (*value.0).clone()
            }
        }
    };
}

pub(crate) use geojson_wrapper;

/// Returns the JSON type name for a serde JSON value.
pub(crate) fn json_type_name(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

/// Parses a JSON number from a textual numeric literal.
pub(crate) fn json_number(value: &str) -> Result<serde_json::Number, String> {
    let parsed = serde_json::from_str::<serde_json::Value>(value)
        .map_err(|error| format!("invalid JSON number: {error}"))?;
    parsed
        .as_number()
        .cloned()
        .ok_or_else(|| format!("expected JSON number, got {}", json_type_name(&parsed)))
}

/// Wraps a `geo_types::Geometry<f64>` as an `elicit_geo_types::Geometry`.
pub(crate) fn wrap_geo_geometry(geometry: geo_types::Geometry<f64>) -> elicit_geo_types::Geometry {
    elicit_geo_types::Geometry::from(elicitation::GeoGeometry::from(geometry))
}
