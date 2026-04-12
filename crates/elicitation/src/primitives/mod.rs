//! Primitive type implementations.
//!
//! This module provides `Elicitation` implementations for all Rust primitive types:
//! - Integer types: i8, i16, i32, i64, u8, u16, u32, u64
//! - Floating-point types: f32, f64
//! - Boolean: bool
//! - Character: char
//! - String: String
//! - PathBuf: std::path::PathBuf
//! - Network types: IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, etc.
//! - Duration: std::time::Duration
//! - Atomics: AtomicBool, AtomicI8..AtomicI64, AtomicU8..AtomicU64, AtomicIsize, AtomicUsize
//! - Tuples: (T1, T2), (T1, T2, T3), up to arity 12
//!
//! All primitive types implement:
//! - [`Prompt`](crate::Prompt) - Provides default prompts
//! - [`Elicitation`](crate::Elicitation) - Async elicitation via MCP
//!
//! Integer and float types use generic macros to eliminate duplication.

mod atomics;
mod boolean;
mod char;
pub mod duration;
pub mod errors;
mod floats;
mod integers;
mod network;
mod pathbuf;
mod string;

pub use string::StringStyle;
pub mod systemtime;
mod tuples;
pub mod unit_structs;

#[cfg(feature = "url")]
pub mod url;

#[cfg(feature = "uuid")]
pub mod uuid;

#[cfg(feature = "reqwest")]
pub mod http;

#[cfg(feature = "clap-types")]
pub mod clap_types;

#[cfg(feature = "sqlx-types")]
pub mod sqlx_types;

#[cfg(feature = "accesskit")]
pub mod accesskit_types;

#[cfg(feature = "egui-types")]
pub mod egui_types;

#[cfg(feature = "ratatui")]
pub mod ratatui_types;

#[cfg(feature = "geo-types")]
pub mod geo_types;

#[cfg(feature = "proj-types")]
pub mod proj_types;

#[cfg(feature = "rstar-types")]
pub mod rstar_types;

#[cfg(feature = "georaster-types")]
pub mod georaster_types;

#[cfg(feature = "geojson-types")]
pub mod geojson_types;

#[cfg(feature = "wkt-types")]
pub mod wkt_types;

#[cfg(feature = "wkb-types")]
pub mod wkb_types;

#[cfg(feature = "winit-types")]
pub mod winit_types;

#[cfg(feature = "palette")]
pub mod palette_types;

#[cfg(feature = "tower-types")]
pub mod tower_types;

#[cfg(feature = "axum-types")]
pub mod axum_types;

#[cfg(feature = "polars-types")]
mod polars_types;
#[cfg(feature = "polars-types")]
pub use polars_types::{
    PolarsDType, PolarsJoinType, PolarsPipelineDescriptor, PolarsPipelineOp, PolarsPipelineStep,
};

#[cfg(feature = "uom-types")]
mod uom_types;
#[cfg(feature = "uom-types")]
pub use uom_types::{UomFormula, UomQuantityKind, UomStep, UomUnitSystem};

#[cfg(feature = "leptos-types")]
mod leptos_types;
#[cfg(feature = "leptos-types")]
pub use leptos_types::{
    LeptosAppDescriptor, LeptosComponentDescriptor, LeptosHtmlTag, LeptosMode,
    LeptosPropDescriptor, LeptosRouteDescriptor, LeptosViewNode,
};
