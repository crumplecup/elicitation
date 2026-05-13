//! UOM workflow descriptor types.
//!
//! Available with the `uom-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::UomQuantityKind;

/// A single step in a unit-of-measurement computation workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct UomStep {
    /// Human-readable description of the step.
    pub description: String,
    /// The quantity kind produced by this step.
    pub kind: UomQuantityKind,
    /// Rust code snippet that implements this step.
    pub code_snippet: String,
}

/// Descriptor for a physics formula involving quantities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct UomFormula {
    /// Formula name (e.g. `"KineticEnergy"`).
    pub name: String,
    /// Symbolic formula string (e.g. `"E = ½mv²"`).
    pub formula: String,
    /// Description of what the formula computes.
    pub description: String,
    /// Ordered list of parameter names and their quantity kinds.
    pub params: Vec<(String, UomQuantityKind)>,
    /// The quantity kind of the result.
    pub result_kind: UomQuantityKind,
}
