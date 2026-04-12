//! Archive workflow plugins.
//!
//! Each plugin composes calls to the existing `elicit_sqlx`, `elicit_polars`,
//! and `elicit_geo` tools — no direct driver calls are made here.

pub mod browse;
pub mod query;
pub mod render;
pub mod spatial;
