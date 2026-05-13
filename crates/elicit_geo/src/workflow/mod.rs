//! MCP workflow plugins for geo algorithms.

pub mod boolean_ops_plugin;
pub mod calculations_plugin;
pub mod geodesic_plugin;
pub mod measurements_plugin;
pub mod predicates_plugin;
pub mod transformations_plugin;
pub mod validation_plugin;
pub mod workflow_plugin;

pub use boolean_ops_plugin::GeoBooleanOpsPlugin;
pub use calculations_plugin::GeoCalculationsPlugin;
pub use geodesic_plugin::GeoGeodesicPlugin;
pub use measurements_plugin::GeoMeasurementsPlugin;
pub use predicates_plugin::GeoPredicatesPlugin;
pub use transformations_plugin::{GeoTransformationsPlugin, TransformationApplied};
pub use validation_plugin::GeoValidationPlugin;
pub use workflow_plugin::GeoWorkflowPlugin;
