//! `elicit_geo` — geometric algorithm MCP tools.
//!
//! Provides 72 MCP tools across 8 plugins operating on geo-types primitives.
//!
//! # Plugins
//!
//! | Plugin | Namespace | Tools |
//! |--------|-----------|-------|
//! | `GeoPredicatesPlugin` | `geo_predicates` | 12 |
//! | `GeoMeasurementsPlugin` | `geo_measurements` | 10 |
//! | `GeoGeodesicPlugin` | `geo_geodesic` | 6 |
//! | `GeoCalculationsPlugin` | `geo_calculations` | 10 |
//! | `GeoTransformationsPlugin` | `geo_transformations` | 10 |
//! | `GeoValidationPlugin` | `geo_validation` | 6 |
//! | `GeoBooleanOpsPlugin` | `geo_boolean_ops` | 8 |
//! | `GeoWorkflowPlugin` | `geo_workflow` | 10 |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    GeoBooleanOpsPlugin, GeoCalculationsPlugin, GeoGeodesicPlugin, GeoMeasurementsPlugin,
    GeoPredicatesPlugin, GeoTransformationsPlugin, GeoValidationPlugin, GeoWorkflowPlugin,
};
