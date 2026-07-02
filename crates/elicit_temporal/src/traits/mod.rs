//! Trait re-exports and the [`TemporalBackend`] supertrait.
//!
//! # Three-role taxonomy
//!
//! The temporal trait interface partitions operations into three orthogonal
//! roles, mirroring the broader interface-crate pattern used across the
//! workspace:
//!
//! | Role | Description | Return type | Example traits |
//! |------|-------------|-------------|----------------|
//! | **1a** (leaf factory) | Parses, formats, converts, or resolves one temporal form and mints fresh proof tokens. | `TemporalResult<(Descriptor, Established<P>, ...)>` | `TemporalParser`, `TemporalCalConnectFactory`, `TemporalFormatter`, `TemporalZoneFactory`, `TemporalConversionFactory`, `TemporalIntervalFactory` |
//! | **1b** (section factory) | Composes upstream proof tokens into aggregate propositions. | `Established<P>` via `ProvableFrom` | `contracts::proof_composition` |
//! | **2** (reporter) | Reports backend capabilities or metadata without minting proofs. | concrete values | `TemporalReporter` |
//!
//! `TemporalBackend` is the aggregate supertrait a fully-capable temporal
//! backend must satisfy. Individual sub-traits remain object-safe and are the
//! preferred dynamic-dispatch boundary.

mod calconnect;
mod conversion;
mod format;
mod interval;
mod parse;
mod report;
mod zone;

pub use calconnect::TemporalCalConnectFactory;
pub use conversion::TemporalConversionFactory;
pub use format::TemporalFormatter;
pub use interval::TemporalIntervalFactory;
pub use parse::TemporalParser;
pub use report::TemporalReporter;
pub use zone::TemporalZoneFactory;

/// Complete temporal backend - blanket supertrait for the initial temporal seam.
///
/// Any type that implements all temporal sub-traits automatically implements
/// `TemporalBackend`.
///
/// # Object safety
///
/// `TemporalBackend` is not itself object-safe as a supertrait aggregate, but
/// each individual sub-trait is object-safe and can be accepted directly as
/// `&dyn TemporalParser`, `&dyn TemporalZoneFactory`, and so on.
pub trait TemporalBackend:
    TemporalParser
    + TemporalCalConnectFactory
    + TemporalFormatter
    + TemporalZoneFactory
    + TemporalConversionFactory
    + TemporalIntervalFactory
    + TemporalReporter
    + Send
    + Sync
{
}

impl<T> TemporalBackend for T where
    T: TemporalParser
        + TemporalCalConnectFactory
        + TemporalFormatter
        + TemporalZoneFactory
        + TemporalConversionFactory
        + TemporalIntervalFactory
        + TemporalReporter
        + Send
        + Sync
{
}
