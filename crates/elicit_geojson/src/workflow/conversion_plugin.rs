//! `GeoJsonConversionPlugin` — explicit GeoJSON/geo-types conversion tools.

use crate::{Feature, FeatureCollection, GeoJson, Geometry, Value};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a GeoJSON/geo-types conversion succeeded.
#[derive(Prop)]
pub struct GeoJsonConverted;

impl VerifiedWorkflow for GeoJsonConverted {}

/// Parameters for converting an `elicit_geo_types::Geometry` to a GeoJSON geometry object.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryFromGeoGeometryParams {
    /// Geometry wrapper to convert.
    pub geometry: elicit_geo_types::Geometry,
}

/// Parameters for converting an `elicit_geo_types::Geometry` to a GeoJSON geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueFromGeoGeometryParams {
    /// Geometry wrapper to convert.
    pub geometry: elicit_geo_types::Geometry,
}

/// Parameters for converting an `elicit_geo_types::GeometryCollection` to a feature collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureCollectionFromGeoGeometryCollectionParams {
    /// Geometry collection wrapper to convert.
    pub collection: elicit_geo_types::GeometryCollection,
}

/// Parameters for converting a GeoJSON document to an `elicit_geo_types::Geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoGeometryFromGeoJsonParams {
    /// GeoJSON document to convert.
    pub geojson: GeoJson,
}

/// Parameters for converting a GeoJSON geometry object to an `elicit_geo_types::Geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoGeometryFromGeometryParams {
    /// GeoJSON geometry object to convert.
    pub geometry: Geometry,
}

/// Parameters for converting a GeoJSON geometry value to an `elicit_geo_types::Geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoGeometryFromValueParams {
    /// GeoJSON geometry value to convert.
    pub value: Value,
}

/// Parameters for converting a GeoJSON feature to an `elicit_geo_types::Geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoGeometryFromFeatureParams {
    /// GeoJSON feature to convert.
    pub feature: Feature,
}

/// Parameters for converting a GeoJSON feature collection to an `elicit_geo_types::Geometry`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoGeometryFromFeatureCollectionParams {
    /// GeoJSON feature collection to convert.
    pub collection: FeatureCollection,
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geometry_from_geo_geometry",
    description = "Convert an elicit_geo_types geometry wrapper to a GeoJSON geometry object. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geometry_from_geo_geometry(
    p: GeometryFromGeoGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = Geometry::from(&p.geometry);
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "value_from_geo_geometry",
    description = "Convert an elicit_geo_types geometry wrapper to a GeoJSON geometry value. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn value_from_geo_geometry(
    p: ValueFromGeoGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::from(&p.geometry);
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "feature_collection_from_geo_geometry_collection",
    description = "Convert an elicit_geo_types geometry collection wrapper to a GeoJSON feature collection. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn feature_collection_from_geo_geometry_collection(
    p: FeatureCollectionFromGeoGeometryCollectionParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let collection = FeatureCollection::from(&p.collection);
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&collection)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geo_geometry_from_geojson",
    description = "Convert a GeoJSON document to an elicit_geo_types geometry wrapper. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geo_geometry_from_geojson(
    p: GeoGeometryFromGeoJsonParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = elicit_geo_types::Geometry::try_from(p.geojson)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geo_geometry_from_geometry",
    description = "Convert a GeoJSON geometry object to an elicit_geo_types geometry wrapper. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geo_geometry_from_geometry(
    p: GeoGeometryFromGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = elicit_geo_types::Geometry::try_from(p.geometry)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geo_geometry_from_value",
    description = "Convert a GeoJSON geometry value to an elicit_geo_types geometry wrapper. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geo_geometry_from_value(
    p: GeoGeometryFromValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = elicit_geo_types::Geometry::try_from(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geo_geometry_from_feature",
    description = "Convert a GeoJSON feature to an elicit_geo_types geometry wrapper. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geo_geometry_from_feature(
    p: GeoGeometryFromFeatureParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = elicit_geo_types::Geometry::try_from(p.feature)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geo_geometry_from_feature_collection",
    description = "Convert a GeoJSON feature collection to an elicit_geo_types geometry wrapper. Establishes: GeoJsonConverted."
)]
#[instrument]
async fn geo_geometry_from_feature_collection(
    p: GeoGeometryFromFeatureCollectionParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = elicit_geo_types::Geometry::try_from(p.collection)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonConverted>::assert();
    json_result(&geometry)
}

/// The GeoJSON conversion MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geojson_conversion")]
pub struct GeoJsonConversionPlugin;

impl GeoJsonConversionPlugin {
    /// Creates a new GeoJSON conversion plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoJsonConversionPlugin {
    fn default() -> Self {
        Self::new()
    }
}
