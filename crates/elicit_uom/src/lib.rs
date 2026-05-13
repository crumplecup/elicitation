//! `elicit_uom` — MCP tools for the `uom` units-of-measurement library.
//!
//! Provides two plugins:
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`UomQuantityPlugin`] | `uom_{name}__*`, `uom_qty__*` | Quantity creation, arithmetic, conversion |
//! | [`UomCodePlugin`] | `uom_code__*` | Code emission and catalog queries |
//!
//! # Quick start
//!
//! ```rust,no_run
//! use elicit_uom::{UomQuantityPlugin, UomCodePlugin};
//!
//! let qty = UomQuantityPlugin::new();
//! let code = UomCodePlugin::with_bus(qty.bus());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod bus;
mod code;
mod dimension;
mod error;
mod quantity;

pub use bus::{QuantityBus, QuantityBusEntry, new_bus};
pub use code::UomCodePlugin;
pub use dimension::{derive_div, derive_mul, derive_pow, derive_recip, derive_sqrt};
pub use error::{UomError, UomErrorKind};
pub use quantity::{
    ALL_REGISTRATIONS, UomQuantityPlugin, convert_to_unit, parse_any, parse_params, rust_type_name,
    si_unit_name, supported_units,
};
