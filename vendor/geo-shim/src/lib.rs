//! Shim crate: re-exports geo 0.33 under the geo 0.32 identity.
//!
//! `surrealdb-core` declares `geo = "^0.32"`. The workspace uses geo 0.33.
//! A `[patch.crates-io]` entry routes surrealdb-core's resolution here,
//! so both end up on the same geo 0.33 types with no conversion needed.
pub use geo_real::*;
