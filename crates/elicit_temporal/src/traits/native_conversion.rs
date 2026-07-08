//! Native higher-order conversion traits.
//!
//! These traits let backends perform temporal conversions on their upstream
//! carrier types instead of round-tripping through descriptor text at the
//! semantic center of the architecture.

use crate::{
    LossyConversionAuthorityBundle, NativeLosslessPrecisionAdjustedProvenTimestampResult,
    NativeNormalizedProvenUtcTimestampResult, NativeStrippedProvenNamedZoneTimestampResult,
    NativeTruncatedProvenSubsecondsTimestampResult, PrecisionDescriptor,
    ProvenOffsetDateTimeCarrier, ProvenZonedDateTimeCarrier, TemporalInstantProps,
    TemporalZoneProps,
};

/// Convert native temporal carriers while keeping losslessness and lossiness explicit.
pub trait TemporalNativeConversionFactory:
    TemporalInstantProps + TemporalZoneProps + Send + Sync
{
    /// Normalize a native offset timestamp to UTC while preserving the represented instant.
    fn normalize_to_utc_native(
        &self,
        timestamp: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
    ) -> NativeNormalizedProvenUtcTimestampResult<Self::OffsetDateTime>;

    /// Drop native named-zone identity while retaining fixed-instant offset semantics.
    fn strip_named_zone_native(
        &self,
        timestamp: &ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
    ) -> NativeStrippedProvenNamedZoneTimestampResult<Self::OffsetDateTime>;

    /// Adjust native timestamp precision without losing represented information.
    fn adjust_precision_losslessly_native(
        &self,
        timestamp: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        target_precision: &PrecisionDescriptor,
    ) -> NativeLosslessPrecisionAdjustedProvenTimestampResult<Self::OffsetDateTime>;

    /// Reduce native timestamp subsecond precision under explicit lossy-conversion authority.
    fn truncate_subseconds_native(
        &self,
        timestamp: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        target_precision: &PrecisionDescriptor,
        lossy_authority: &LossyConversionAuthorityBundle,
    ) -> NativeTruncatedProvenSubsecondsTimestampResult<Self::OffsetDateTime>;
}
