//! Verus verification proofs for elicitation contract types.
//!
//! This module contains Verus executable specifications for formal verification.
//! Proofs are simplified stubs focusing on type-level contracts.
//!
//! # Verification Architecture
//!
//! Complete trenchcoat verification pipeline:
//! - Contract types: User-facing validated types
//! - Trenchcoat types: Internal stdlib wrappers
//! - Stdlib types: Trusted foundation
//!
//! # Usage
//!
//! These proofs are for verification only and don't affect runtime behavior.
//! Run with: `verus crates/elicitation_verus/src/lib.rs`


// Serde boundary consistency theorems
pub mod serde_boundary;

// Contract type proofs (newtypes/wrappers)
pub mod bools;
pub mod chars;
pub mod collections;
pub mod datetimes;
pub mod durations;
pub mod floats;
pub mod integers;
pub mod networks;
pub mod paths;
pub mod regexes;
pub mod strings;
pub mod tuples;
pub mod urls;
pub mod uuids;
pub mod values;

// Trenchcoat verification wrapper modules (internal stdlib wrappers)
pub mod ipaddr_bytes;
pub mod macaddr;
pub mod pathbytes;
pub mod regexbytes;
pub mod socketaddr;
pub mod urlbytes;
pub mod utf8;
pub mod uuid_bytes;

// Base type proofs (stdlib and external crates)
pub mod external_types;
pub mod primitives;
pub mod stdlib_collections;
pub mod clap_types;
pub mod sqlx_types;
pub mod tokio_types;
pub mod egui_types;
pub mod ratatui_types;
pub mod geo_types;
pub mod georaster_types;
pub mod geojson_types;
pub mod rstar_types;
pub mod proj_types;
pub mod wkt_types;
pub mod wkb_types;
pub mod palette_types;
pub mod ui_types;
pub mod winit_types;
pub mod wgpu_types;
