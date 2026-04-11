//! MCP workflow plugins for GeoJSON documents, values, features, and conversions.

mod conversion_plugin;
mod document_plugin;
mod feature_plugin;
mod geometry_plugin;

use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use serde::Serialize;

pub use conversion_plugin::{
    FeatureCollectionFromGeoGeometryCollectionParams, GeoGeometryFromFeatureCollectionParams,
    GeoGeometryFromFeatureParams, GeoGeometryFromGeoJsonParams, GeoGeometryFromGeometryParams,
    GeoGeometryFromValueParams, GeoJsonConversionPlugin, GeoJsonConverted,
    GeometryFromGeoGeometryParams, ValueFromGeoGeometryParams,
};
pub use document_plugin::{
    GeoJsonDocumentParsed, GeoJsonDocumentPlugin, GeoJsonFromJsonValueParams, GeoJsonFromStrParams,
    GeoJsonToJsonValueParams, GeoJsonToStringPrettyParams, GeoJsonVariantParams,
};
pub use feature_plugin::{
    FeatureCollectionFromFeaturesParams, FeatureCollectionFromJsonValueParams,
    FeatureContainsPropertyParams, FeatureFromGeometryParams, FeatureFromJsonValueParams,
    FeatureFromValueParams, FeatureLenPropertiesParams, FeaturePropertyParams,
    FeatureRemovePropertyParams, FeatureSetPropertyParams, GeoJsonFeatureCreated,
    GeoJsonFeaturePlugin, IdNumberParams, IdStringParams,
};
pub use geometry_plugin::{
    GeoJsonGeometryCreated, GeoJsonGeometryPlugin, GeometryFromJsonValueParams, GeometryNewParams,
    ValueFromJsonValueParams, ValueGeometryCollectionParams, ValueLineStringParams,
    ValueMultiLineStringParams, ValueMultiPointParams, ValueMultiPolygonParams, ValuePointParams,
    ValuePolygonParams, ValueTypeNameParams,
};

fn json_result<T: Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(value)
            .map_err(|error| ErrorData::internal_error(error.to_string(), None))?,
    )]))
}

fn text_result(text: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(text.into())])
}
