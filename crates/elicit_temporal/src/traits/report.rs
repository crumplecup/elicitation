//! Reporter traits for backend capability and metadata inspection.

use crate::SerializationProfile;

/// Report backend temporal capabilities without minting or consuming proofs.
pub trait TemporalReporter: Send + Sync {
    /// Serialization profiles this backend can emit directly.
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile>;

    /// Maximum fractional-second precision this backend can preserve directly.
    fn max_fractional_second_digits(&self) -> Option<u8>;

    /// Whether this backend can represent leap-second inputs distinctly.
    fn supports_leap_seconds(&self) -> bool;

    /// Whether this backend preserves RFC 3339 unknown-local-offset semantics.
    fn supports_unknown_local_offset(&self) -> bool;

    /// Whether this backend can round-trip named-zone identity without flattening to an offset.
    fn supports_named_zone_round_trip(&self) -> bool;

    /// Whether this backend supports the end-of-day `24:00:00` form.
    fn supports_end_of_day_twenty_four(&self) -> bool;

    /// Whether this backend emits or preserves IXDTF calendar annotations.
    fn supports_ixdtf_calendar_annotations(&self) -> bool;

    /// Whether this backend preserves or enforces RFC 9557 critical suffix semantics.
    fn supports_ixdtf_critical_annotations(&self) -> bool;

    /// Additional IXDTF suffix keys this backend can round-trip or interpret.
    fn supported_ixdtf_annotation_keys(&self) -> Vec<String>;

    /// Current TZDB revision visible to the backend, when applicable.
    fn current_tzdb_revision(&self) -> Option<String>;
}
