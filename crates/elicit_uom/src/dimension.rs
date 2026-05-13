//! Dimension derivation table for cross-registration arithmetic.
//!
//! These pure functions encode the known physical relationships between the 18
//! registered quantity kinds, enabling the arithmetic tools to determine the
//! result registration after a `mul`, `div`, `sqrt`, or `pow` operation.

/// Derive the result registration name for `lhs × rhs`.
///
/// Returns `None` when no known derivation exists.
pub fn derive_mul(lhs: &str, rhs: &str) -> Option<&'static str> {
    // Normalise order: try both directions.
    derive_mul_ordered(lhs, rhs).or_else(|| derive_mul_ordered(rhs, lhs))
}

fn derive_mul_ordered(lhs: &str, rhs: &str) -> Option<&'static str> {
    match (lhs, rhs) {
        ("length", "length") => Some("area"),
        ("area", "length") | ("length", "area") => Some("volume"),
        ("mass", "acceleration") => Some("force"),
        ("force", "length") | ("length", "force") => Some("energy"),
        ("velocity", "time") | ("time", "velocity") => Some("length"),
        ("acceleration", "time") | ("time", "acceleration") => Some("velocity"),
        ("density", "volume") | ("volume", "density") => Some("mass"),
        ("pressure", "area") | ("area", "pressure") => Some("force"),
        ("power", "time") | ("time", "power") => Some("energy"),
        _ => None,
    }
}

/// Derive the result registration name for `lhs ÷ rhs`.
///
/// Returns `None` when no known derivation exists.
pub fn derive_div(lhs: &str, rhs: &str) -> Option<&'static str> {
    match (lhs, rhs) {
        ("length", "time") => Some("velocity"),
        ("velocity", "time") => Some("acceleration"),
        ("energy", "time") => Some("power"),
        ("force", "area") => Some("pressure"),
        ("mass", "volume") => Some("density"),
        ("area", "length") => Some("length"),
        ("volume", "area") => Some("length"),
        ("volume", "length") => Some("area"),
        ("energy", "force") => Some("length"),
        ("force", "mass") => Some("acceleration"),
        ("force", "acceleration") => Some("mass"),
        ("length", "velocity") => Some("time"),
        ("velocity", "acceleration") => Some("time"),
        ("angle", "time") => Some("frequency"),
        ("energy", "mass") => Some("area"), // specific energy → m²/s² approximation skipped
        _ => None,
    }
}

/// Derive the result registration name for `√lhs`.
///
/// Returns `None` when no known derivation exists.
pub fn derive_sqrt(lhs: &str) -> Option<&'static str> {
    match lhs {
        "area" => Some("length"),
        _ => None,
    }
}

/// Derive the result registration name for `lhs ^ n`.
///
/// Returns `None` when no known derivation exists.
pub fn derive_pow(lhs: &str, n: i32) -> Option<&'static str> {
    match (lhs, n) {
        ("length", 2) => Some("area"),
        ("length", 3) => Some("volume"),
        (_, 1) => Some(lhs_static(lhs)?),
        _ => None,
    }
}

/// Derive the result registration name for `1 / lhs` (reciprocal).
///
/// Returns `None` when no known derivation exists.
pub fn derive_recip(lhs: &str) -> Option<&'static str> {
    match lhs {
        "frequency" => Some("time"),
        "time" => Some("frequency"),
        _ => None,
    }
}

/// Map a runtime registration name to a `&'static str` from the known set.
fn lhs_static(lhs: &str) -> Option<&'static str> {
    match lhs {
        "length" => Some("length"),
        "mass" => Some("mass"),
        "time" => Some("time"),
        "temperature" => Some("temperature"),
        "electric_current" => Some("electric_current"),
        "amount_of_substance" => Some("amount_of_substance"),
        "luminous_intensity" => Some("luminous_intensity"),
        "velocity" => Some("velocity"),
        "acceleration" => Some("acceleration"),
        "force" => Some("force"),
        "energy" => Some("energy"),
        "power" => Some("power"),
        "pressure" => Some("pressure"),
        "frequency" => Some("frequency"),
        "area" => Some("area"),
        "volume" => Some("volume"),
        "density" => Some("density"),
        "angle" => Some("angle"),
        _ => None,
    }
}
