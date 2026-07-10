//! Trait re-exports for the descriptor accord and native carrier families.
//!
//! # Three-layer architecture
//!
//! `elicit_temporal` now has two architectural layers:
//!
//! - descriptor-oriented traits for lawful text and neutral-contract exchange
//! - associated native-carrier families for backend-owned runtime values
//! - user-facing proven-carrier seams that aggregate proof sidecars into named
//!   semantic bundles
//!
//! The descriptor layer preserves a shared inter-crate accord. The native
//! layer is the semantic center for backends that work with real upstream time
//! types such as those from `time`, `chrono`, or `jiff`. The proven-carrier
//! layer sits above the raw native bridges so higher-order consumers can move
//! runtime values together with aggregate evidence instead of hand-carrying
//! loose `Established<P>` tokens in every call.
//!
//! # Three-role taxonomy
//!
//! The descriptor-oriented runtime seams still partition into three orthogonal
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
mod native_bridge;
mod native_conversion;
mod native_extension;
mod native_interval;
mod native_props;
mod native_span;
mod native_zone;
mod parse;
mod report;
mod zone;

pub use calconnect::TemporalCalConnectFactory;
pub use conversion::TemporalConversionFactory;
pub use format::TemporalFormatter;
pub use interval::TemporalIntervalFactory;
pub use native_bridge::{
    TemporalCivilNativeBridge, TemporalInstantNativeBridge, TemporalNativeBridge,
    TemporalZoneNativeBridge,
};
pub use native_conversion::TemporalNativeConversionFactory;
pub use native_extension::{
    TemporalDateTimeFormulaNativeBridge, TemporalExplicitDurationNativeBridge,
    TemporalExplicitTemporalFormNativeBridge, TemporalExplicitTimeIntervalNativeBridge,
    TemporalGroupedTimeScaleUnitNativeBridge, TemporalNativeDateTimeFormulaFactory,
    TemporalNativeExtensionBridge, TemporalQualifiedTemporalValueNativeBridge,
    TemporalSetNativeBridge,
};
pub use native_interval::TemporalNativeIntervalFactory;
pub use native_props::{
    TemporalCivilProps, TemporalDateTimeFormulaProps, TemporalDurationProps,
    TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalExtensionProps, TemporalGroupedTimeScaleUnitProps,
    TemporalInstantProps, TemporalNativeProps, TemporalQualifiedTemporalValueProps,
    TemporalRecurringIntervalProps, TemporalSetProps, TemporalSpanProps, TemporalTimeIntervalProps,
    TemporalZoneProps,
};
pub use native_span::{
    TemporalDurationNativeBridge, TemporalNativeSpanBridge, TemporalRecurringIntervalNativeBridge,
    TemporalTimeIntervalNativeBridge,
};
pub use native_zone::TemporalNativeZoneFactory;
pub use parse::TemporalParser;
pub use report::TemporalReporter;
pub use zone::TemporalZoneFactory;

/// Complete descriptor-oriented backend for the initial temporal seam.
///
/// Any type that implements all temporal sub-traits automatically implements
/// `TemporalBackend`.
///
/// # Role in the architecture
///
/// `TemporalBackend` remains a compatibility aggregate for the descriptor-
/// oriented doorway. It is not the semantic center of the long-term temporal
/// architecture: native associated-type families carry that role so backends do
/// not need to collapse into strings or framework-owned replacement time
/// values.
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
