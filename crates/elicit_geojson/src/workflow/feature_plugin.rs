//! `GeoJsonFeaturePlugin` — construct and inspect GeoJSON features.

use crate::{Feature, FeatureCollection, Geometry, Id, Value, helpers};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::{json_result, text_result};

/// Proposition: a GeoJSON feature, feature collection, or identifier was created.
#[derive(Prop)]
pub struct GeoJsonFeatureCreated;

impl VerifiedWorkflow for GeoJsonFeatureCreated {}

/// Parameters for constructing a feature from a geometry object.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureFromGeometryParams {
    /// Geometry to wrap as a feature.
    pub geometry: Geometry,
}

/// Parameters for constructing a feature from a geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureFromValueParams {
    /// Geometry value to wrap as a feature.
    pub value: Value,
}

/// Parameters for constructing a feature from raw JSON.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureFromJsonValueParams {
    /// JSON value containing a feature object.
    pub value: serde_json::Value,
}

/// Parameters for constructing a feature collection from raw JSON.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureCollectionFromJsonValueParams {
    /// JSON value containing a feature collection object.
    pub value: serde_json::Value,
}

/// Parameters for constructing a feature collection from features.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureCollectionFromFeaturesParams {
    /// Features to collect.
    pub features: Vec<Feature>,
}

/// Parameters for constructing a string feature identifier.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IdStringParams {
    /// Identifier text.
    pub value: String,
}

/// Parameters for constructing a numeric feature identifier.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IdNumberParams {
    /// Identifier numeric literal.
    pub value: String,
}

/// Parameters for looking up a feature property.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeaturePropertyParams {
    /// Feature to inspect.
    pub feature: Feature,
    /// Property key.
    pub key: String,
}

/// Parameters for testing whether a feature property exists.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureContainsPropertyParams {
    /// Feature to inspect.
    pub feature: Feature,
    /// Property key.
    pub key: String,
}

/// Parameters for setting a feature property.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureSetPropertyParams {
    /// Feature to update.
    pub feature: Feature,
    /// Property key.
    pub key: String,
    /// Property value.
    pub value: serde_json::Value,
}

/// Parameters for removing a feature property.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureRemovePropertyParams {
    /// Feature to update.
    pub feature: Feature,
    /// Property key.
    pub key: String,
}

/// Parameters for counting feature properties.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FeatureLenPropertiesParams {
    /// Feature to inspect.
    pub feature: Feature,
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_from_geometry",
    description = "Construct a GeoJSON feature from a geometry object. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_from_geometry(
    p: FeatureFromGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let feature = Feature::from(p.geometry);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&feature)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_from_value",
    description = "Construct a GeoJSON feature from a geometry value. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_from_value(
    p: FeatureFromValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let feature = Feature::from(p.value);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&feature)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_from_json_value",
    description = "Construct a GeoJSON feature from raw JSON. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_from_json_value(
    p: FeatureFromJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let feature = Feature::from_json_value(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&feature)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_collection_from_json_value",
    description = "Construct a GeoJSON feature collection from raw JSON. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_collection_from_json_value(
    p: FeatureCollectionFromJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let collection = FeatureCollection::from_json_value(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&collection)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_collection_from_features",
    description = "Construct a GeoJSON feature collection from features. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_collection_from_features(
    p: FeatureCollectionFromFeaturesParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let collection: FeatureCollection = p.features.into_iter().collect();
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&collection)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "id_string",
    description = "Construct a string GeoJSON feature identifier. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn id_string(p: IdStringParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let id = Id::string(p.value);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&id)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "id_number",
    description = "Construct a numeric GeoJSON feature identifier. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn id_number(p: IdNumberParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let number =
        helpers::json_number(&p.value).map_err(|error| ErrorData::invalid_params(error, None))?;
    let id = Id::number(number);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&id)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_property",
    description = "Return a GeoJSON feature property value by key."
)]
#[instrument]
async fn feature_property(
    p: FeaturePropertyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.feature.property(p.key).cloned())
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_contains_property",
    description = "Return whether a GeoJSON feature property key is present."
)]
#[instrument]
async fn feature_contains_property(
    p: FeatureContainsPropertyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(text_result(p.feature.contains_property(p.key).to_string()))
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_set_property",
    description = "Set a GeoJSON feature property. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_set_property(
    p: FeatureSetPropertyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let mut feature = p.feature;
    feature.set_property(p.key, p.value);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&feature)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_remove_property",
    description = "Remove a GeoJSON feature property and return the updated feature. Establishes: GeoJsonFeatureCreated."
)]
#[instrument]
async fn feature_remove_property(
    p: FeatureRemovePropertyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let mut feature = p.feature;
    let _removed = feature.remove_property(p.key);
    let _proof = Established::<GeoJsonFeatureCreated>::assert();
    json_result(&feature)
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "feature_len_properties",
    description = "Return the number of properties stored on a GeoJSON feature."
)]
#[instrument]
async fn feature_len_properties(
    p: FeatureLenPropertiesParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(text_result(p.feature.len_properties().to_string()))
}

/// The GeoJSON feature MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geojson_feature")]
pub struct GeoJsonFeaturePlugin;

impl GeoJsonFeaturePlugin {
    /// Creates a new GeoJSON feature plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoJsonFeaturePlugin {
    fn default() -> Self {
        Self::new()
    }
}
