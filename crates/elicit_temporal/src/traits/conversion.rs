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
/// Normative sources: ISO 8601-1:2019, 3.1.3, 5.2, 5.3, 5.4, 5.5, and 5.6;
/// ISO 8601-1:2019/Amd 1:2022, 5.3.1.4; ISO 8601-2:2019, 7.11, 7.12, 7.13,
/// 14.2, 14.3, and 14.4; CalConnect CC 18011:2018,
/// representations-precision, representations-decimal,
/// representations-reduced-precision, §8.2, and §8.3-§8.5; RFC 3339 §5.1.
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
    /// Normative source: RFC 9557 §1.2 - named-zone identity is not reducible to offset alone.
    fn strip_named_zone(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        zone_proof: Established<ZonedDateTimeHasNamedZone>,
        consistency_proof: Established<OffsetConsistentWithNamedZone>,
    ) -> StrippedNamedZoneTimestampResult;

    /// Adjust timestamp precision without losing represented information.
    ///
    /// Normative sources: ISO 8601-1:2019/Amd 1:2022, 5.3.1.4;
    /// ISO 8601-2:2019, 7.11, 7.12, and 7.13.
    fn adjust_precision_losslessly(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        target_precision: &PrecisionDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
    ) -> LosslessPrecisionAdjustedTimestampResult;

    /// Reduce subsecond precision under explicit lossy-conversion authority.
    ///
    /// Normative basis: ISO 8601-2:2019, 7.13, 14.2, 14.3, and 14.4.
    /// Informative cross-check: CalConnect CC 18011:2018 §8.2 and §8.3-§8.5.
    /// The authority sidecar is layered on top of the standard truncation
    /// regimes so loss remains explicit at the seam.
    fn truncate_subseconds(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        target_precision: &PrecisionDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
        lossy_authority: Established<LossyConversionAuthorityValid>,
    ) -> TruncatedSubsecondsTimestampResult;
}
