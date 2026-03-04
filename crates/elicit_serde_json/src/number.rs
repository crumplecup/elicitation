//! `JsonNumber` — elicitation-enabled wrapper around `serde_json::Number`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

elicit_newtype!(serde_json::Number, as JsonNumber, serde);

#[reflect_methods]
impl JsonNumber {
    /// Returns true if this number is representable as an `i64`.
    pub fn is_i64(&self) -> bool {
        self.0.is_i64()
    }

    /// Returns true if this number is representable as a `u64`.
    pub fn is_u64(&self) -> bool {
        self.0.is_u64()
    }

    /// Returns true if this number is a float (`f64`).
    pub fn is_f64(&self) -> bool {
        self.0.is_f64()
    }

    /// Returns the value as `i64` if representable, or `None`.
    pub fn as_i64(&self) -> Option<i64> {
        self.0.as_i64()
    }

    /// Returns the value as `u64` if representable, or `None`.
    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }

    /// Returns the value as `f64`, or `None` if it cannot be represented.
    pub fn as_f64(&self) -> Option<f64> {
        self.0.as_f64()
    }

    /// Serialize this number to its JSON string representation.
    pub fn to_json_string(&self) -> String {
        self.0.to_string()
    }
}
