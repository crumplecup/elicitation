use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

pub struct ShadowGeoJsonValue {
    pub is_point: bool,
}

pub fn make_geojson_point_value() -> (result: ShadowGeoJsonValue)
    ensures result.is_point,
{
    ShadowGeoJsonValue { is_point: true }
}

pub fn verify_geojson_value_point_type_name() -> (result: ShadowGeoJsonValue)
    ensures result.is_point,
{
    make_geojson_point_value()
}

pub struct ShadowGeoJsonGeometry {
    pub has_point_value: bool,
}

pub fn make_geojson_point_geometry() -> (result: ShadowGeoJsonGeometry)
    ensures result.has_point_value,
{
    ShadowGeoJsonGeometry { has_point_value: true }
}

pub fn verify_geojson_geometry_new_point() -> (result: ShadowGeoJsonGeometry)
    ensures result.has_point_value,
{
    make_geojson_point_geometry()
}

pub struct ShadowGeoJsonFeature {
    pub has_property: bool,
    pub property_count: u32,
}

pub fn make_geojson_feature(has_property: bool, property_count: u32) -> (result: ShadowGeoJsonFeature)
    ensures
        result.has_property == has_property,
        result.property_count == property_count,
{
    ShadowGeoJsonFeature { has_property, property_count }
}

pub fn verify_geojson_feature_property_access() -> (result: ShadowGeoJsonFeature)
    ensures
        result.has_property,
        result.property_count == 1,
{
    make_geojson_feature(true, 1u32)
}

pub struct ShadowGeoJsonFeatureCollection {
    pub feature_count: u32,
}

pub fn make_geojson_feature_collection(feature_count: u32) -> (result: ShadowGeoJsonFeatureCollection)
    ensures result.feature_count == feature_count,
{
    ShadowGeoJsonFeatureCollection { feature_count }
}

pub fn verify_geojson_feature_collection_len() -> (result: ShadowGeoJsonFeatureCollection)
    ensures result.feature_count == 2,
{
    make_geojson_feature_collection(2u32)
}

pub struct ShadowGeoJsonId {
    pub is_string: bool,
}

pub fn make_geojson_string_id() -> (result: ShadowGeoJsonId)
    ensures result.is_string,
{
    ShadowGeoJsonId { is_string: true }
}

pub fn verify_geojson_id_string_variant() -> (result: ShadowGeoJsonId)
    ensures result.is_string,
{
    make_geojson_string_id()
}

}
