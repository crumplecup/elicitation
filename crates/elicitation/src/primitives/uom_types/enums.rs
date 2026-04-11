//! UOM quantity kind and unit system enums.
//!
//! Available with the `uom-types` feature.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The 18 registered physical quantity kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UomQuantityKind {
    /// Base: metre (SI).
    Length,
    /// Base: kilogram (SI).
    Mass,
    /// Base: second (SI).
    Time,
    /// Base: kelvin (SI).
    Temperature,
    /// Base: ampere (SI).
    ElectricCurrent,
    /// Base: mole (SI).
    AmountOfSubstance,
    /// Base: candela (SI).
    LuminousIntensity,
    /// Derived: m/s.
    Velocity,
    /// Derived: m/s².
    Acceleration,
    /// Derived: kg⋅m/s² = N.
    Force,
    /// Derived: kg⋅m²/s² = J.
    Energy,
    /// Derived: J/s = W.
    Power,
    /// Derived: N/m² = Pa.
    Pressure,
    /// Derived: 1/s = Hz.
    Frequency,
    /// Derived: m².
    Area,
    /// Derived: m³.
    Volume,
    /// Derived: kg/m³.
    Density,
    /// Derived: rad.
    Angle,
}

/// Unit system used for a quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UomUnitSystem {
    /// SI (International System of Units).
    Si,
    /// Imperial / US customary.
    Imperial,
    /// Natural units.
    Natural,
}
