//! Integration tests for elicit_geo plugins.

use elicit_geo::{
    GeoBooleanOpsPlugin, GeoCalculationsPlugin, GeoGeodesicPlugin, GeoMeasurementsPlugin,
    GeoPredicatesPlugin, GeoTransformationsPlugin, GeoValidationPlugin, GeoWorkflowPlugin,
};

#[test]
fn plugins_exist() {
    let _ = GeoPredicatesPlugin;
    let _ = GeoMeasurementsPlugin;
    let _ = GeoGeodesicPlugin;
    let _ = GeoCalculationsPlugin;
    let _ = GeoTransformationsPlugin;
    let _ = GeoValidationPlugin;
    let _ = GeoBooleanOpsPlugin;
    let _ = GeoWorkflowPlugin;
}
