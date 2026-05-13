//! `elicit_proj` — MCP workflow tools for the [`proj`] coordinate transformation library.
//!
//! Provides a serializable [`ProjTransform`] snapshot wrapper around [`proj::Proj`] and MCP
//! workflow tools for creating transforms, converting coordinates, projecting geometries, and
//! transforming bounding boxes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod error;
mod transform;
pub mod workflow;

pub use error::{ProjResult, ProjTransformError, ProjTransformErrorKind};
pub use transform::{ProjSpec, ProjTransform};
pub use workflow::{ProjCreated, ProjTransformPlugin};
