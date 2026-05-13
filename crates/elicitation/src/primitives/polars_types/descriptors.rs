//! Polars pipeline descriptor types.
//!
//! Available with the `polars-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::PolarsPipelineOp;

/// A single named step in a polars pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct PolarsPipelineStep {
    /// Step UUID.
    pub step_id: Uuid,
    /// The operation to perform.
    pub op: PolarsPipelineOp,
}

/// Descriptor for a complete polars LazyFrame pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct PolarsPipelineDescriptor {
    /// Pipeline UUID.
    pub pipeline_id: Uuid,
    /// Human-readable pipeline name.
    pub name: String,
    /// Ordered list of pipeline steps.
    pub steps: Vec<PolarsPipelineStep>,
}
