//! Shared quantity bus for cross-registration arithmetic.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

/// A single entry stored on the [`QuantityBus`].
#[derive(Debug, Clone)]
pub struct QuantityBusEntry {
    /// Which registration owns this value (e.g. `"length"`, `"velocity"`).
    pub registration: &'static str,
    /// The SI base value (e.g. metres for length, m/s for velocity).
    pub si_value: f64,
    /// Rust code snippet used by `{name}__emit`.
    pub code_snippet: String,
}

/// Thread-safe UUID-keyed store of all created quantity values.
///
/// Shared between [`crate::UomQuantityPlugin`] and [`crate::UomCodePlugin`] to enable
/// cross-registration arithmetic and code emission.
pub type QuantityBus = Arc<Mutex<HashMap<Uuid, QuantityBusEntry>>>;

/// Create a fresh, empty [`QuantityBus`].
pub fn new_bus() -> QuantityBus {
    Arc::new(Mutex::new(HashMap::new()))
}
