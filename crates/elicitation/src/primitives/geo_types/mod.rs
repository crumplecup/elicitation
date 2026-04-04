//! Elicitation implementations for [`geo_types`] spatial primitives.
//!
//! Provides [`Elicitation`](crate::Elicitation) for geo-types 0.7 types
//! that can be interactively constructed from an agent — composite types
//! via Survey (field-by-field elicitation).
//!
//! # Enabled by the `geo-types` feature
//!
//! ```toml
//! elicitation = { version = "*", features = ["geo-types"] }
//! ```

// ── Composite struct modules ─────────────────────────────────────────
mod coord;
mod line;
mod rect;

// ── Composite struct wrapper re-exports ──────────────────────────────
pub use coord::{GeoCoord, GeoCoordStyle};
pub use line::{GeoLine, GeoLineStyle};
pub use rect::{GeoRect, GeoRectStyle};
