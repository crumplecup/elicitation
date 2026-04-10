//! `elicit_polars` — MCP tools for the polars DataFrame library.
//!
//! Provides 4 plugins covering ~75 tools:
//! - [`PolarsExprPlugin`] — Expr composition registry (~30 tools)
//! - [`PolarsDataFramePlugin`] — Runtime DataFrame execution (~28 tools)
//! - [`PolarsPipelinePlugin`] — LazyFrame pipeline code generation (~15 tools)
//! - [`PolarsSqlPlugin`] — SQLContext runtime interface (~5 tools)

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod dataframe;
mod expr;
mod pipeline;
mod sql;

pub use dataframe::{PolarsDataFramePlugin, PolarsDfCreated, SharedDfRegistry};
pub use expr::{PolarsExprCreated, PolarsExprPlugin, SharedExprRegistry};
pub use pipeline::{PolarsPipelineCreated, PolarsPipelinePlugin};
pub use sql::{PolarsSqlCreated, PolarsSqlPlugin};
