//! Shim crate: re-exports chrono 0.4.43 under the chrono 0.4.41 identity.
//!
//! `polars-arrow 0.53` requires `chrono <=0.4.41`. The workspace uses chrono 0.4.43.
//! A `[patch.crates-io]` entry routes the resolution here, so both end up on
//! the same chrono 0.4.43 types with no type boundary or conversion needed.
pub use chrono_real::*;
