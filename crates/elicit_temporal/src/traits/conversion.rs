//! Conversion traits for lawful temporal representation changes.

use elicitation::Established;

use crate::{
    LosslessPrecisionAdjustedTimestampResult, LossyConversionAuthorityValid,
    NormalizedUtcTimestampResult, OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor,
    PrecisionDescriptor, StrippedNamedZoneTimestampResult, TimestampRepresentsFixedInstant,
    TruncatedSubsecondsTimestampResult, ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Convert between temporal representations while making losslessness or lossiness explicit.
///
/// Normative sources: RFC 3339 §5.1; ISO 8601-2:2019 reduced-precision semantics.
pub trait TemporalConversionFactory: Send + Sync {
    /// Normalize an offset timestamp to UTC while preserving the represented instant.
    ///
    /// Normative source: RFC 3339 §5.1 - ordering on the UTC timeline.
    fn normalize_to_utc(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> NormalizedUtcTimestampResult;

    /// Drop named-zone identity while retaining offset timestamp semantics.
    ///
    /// Normative source: RFC 9557 §3.1 - named-zone identity is not reducible to offset alone.
    fn strip_named_zone(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        zone_proof: Established<ZonedDateTimeHasNamedZone>,
        consistency_proof: Established<OffsetConsistentWithNamedZone>,
    ) -> StrippedNamedZoneTimestampResult;

    /// Adjust timestamp precision without losing represented information.
    ///
    /// Normative source: ISO 8601-2:2019 - reduced precision and rounding semantics.
    fn adjust_precision_losslessly(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        target_precision: &PrecisionDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> LosslessPrecisionAdjustedTimestampResult;

    /// Reduce subsecond precision under explicit lossy-conversion authority.
    ///
    /// Normative source: ISO 8601-2:2019 - reduced precision and rounding semantics.
    fn truncate_subseconds(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        target_precision: &PrecisionDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
        lossy_authority: Established<LossyConversionAuthorityValid>,
    ) -> TruncatedSubsecondsTimestampResult;
}
