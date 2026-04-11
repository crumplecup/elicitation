//! `#[reflect_trait]` factories for WKT conversion traits.
//!
//! The upstream `wkt::ToWkt<T>` and `wkt::TryFromWkt<T>` traits are generic
//! over the coordinate scalar type `T`. This crate fixes that scalar to `f64`,
//! which is the concrete instantiation used throughout the surrounding
//! geospatial crates in this workspace.
//!
//! ## Omitted upstream methods
//!
//! Two upstream methods are intentionally not exposed as MCP tools:
//! - `ToWkt::write_wkt` takes an `impl Write`
//! - `TryFromWkt::try_from_wkt_reader` takes an `impl Read`
//!
//! Those I/O traits are not serializable across the MCP boundary. The string
//! based methods remain available with their original names.

use crate::WktItem;
use elicitation_macros::reflect_trait;

/// Monomorphized `wkt::ToWkt<f64>` adapter.
pub trait ToWktF64 {
    /// Converts `self` into a structured WKT value.
    fn to_wkt(&self) -> WktItem;

    /// Serializes `self` directly to a WKT string.
    fn wkt_string(&self) -> String;
}

impl<T> ToWktF64 for T
where
    T: wkt::ToWkt<f64>,
{
    fn to_wkt(&self) -> WktItem {
        WktItem::from(elicitation::WktGeom::from(wkt::ToWkt::to_wkt(self)))
    }

    fn wkt_string(&self) -> String {
        wkt::ToWkt::wkt_string(self)
    }
}

/// Monomorphized `wkt::TryFromWkt<f64>` adapter.
pub trait TryFromWktF64: Sized {
    /// Parses `wkt_str` into `Self`.
    fn try_from_wkt_str(wkt_str: &str) -> Result<Self, String>;
}

impl<T> TryFromWktF64 for T
where
    T: wkt::TryFromWkt<f64>,
    <T as wkt::TryFromWkt<f64>>::Error: std::fmt::Debug,
{
    fn try_from_wkt_str(wkt_str: &str) -> Result<Self, String> {
        <T as wkt::TryFromWkt<f64>>::try_from_wkt_str(wkt_str).map_err(|error| format!("{error:?}"))
    }
}

/// Expose `wkt::ToWkt<f64>` as dynamic MCP tools.
#[reflect_trait(crate::trait_factories::ToWktF64)]
pub trait ToWktTools {
    /// Converts `self` into a structured WKT value.
    fn to_wkt(&self) -> WktItem;

    /// Serializes `self` to a WKT string.
    fn wkt_string(&self) -> String;
}

/// Expose `wkt::TryFromWkt<f64>` as dynamic MCP tools.
#[reflect_trait(crate::trait_factories::TryFromWktF64)]
pub trait TryFromWktTools {
    /// Parses a WKT string into `Self`.
    fn try_from_wkt_str(wkt_str: &str) -> Result<Self, String>
    where
        Self: Sized;
}
