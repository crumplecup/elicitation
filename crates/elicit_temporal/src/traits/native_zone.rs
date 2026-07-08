//! Native higher-order named-zone traits.
//!
//! These traits lift zone attachment and local-to-zone resolution above the
//! descriptor-only compatibility layer so backends can operate directly on
//! their real temporal carriers while still exchanging explicit proof
//! sidecars.

use crate::{
    ConfirmedProvenNamedZoneRevisionResult, LocalTimeZoneResolutionAuthorityDescriptor,
    NativeAttachedProvenNamedZoneResult, NativeResolvedProvenLocalDateTimeAtNamedZoneResult,
    ProvenLocalDateTimeCarrier, ProvenNamedTimeZoneCarrier, ProvenOffsetDateTimeCarrier,
    ProvenZonedDateTimeCarrier, TemporalCivilProps, TemporalInstantProps, TemporalZoneProps,
    ZoneTransitionResolutionAuthorityBundle,
};

/// Resolve and attach named-zone identity using native temporal carriers.
pub trait TemporalNativeZoneFactory:
    TemporalCivilProps + TemporalInstantProps + TemporalZoneProps + Send + Sync
{
    /// Resolve a native local date-time against a native named zone.
    ///
    /// The caller must supply explicit local-time semantics, named-zone
    /// identity, and resolution-authority sidecars. The result re-issues the
    /// fixed-instant, named-zone attachment, offset-consistency, authority,
    /// and ambiguity-versus-gap branch semantics explicitly.
    fn resolve_local_date_time_native(
        &self,
        timestamp: &ProvenLocalDateTimeCarrier<Self::LocalDateTime>,
        zone: &ProvenNamedTimeZoneCarrier<Self::NamedTimeZone>,
        resolution_authority: &LocalTimeZoneResolutionAuthorityDescriptor,
        authority: &ZoneTransitionResolutionAuthorityBundle,
    ) -> NativeResolvedProvenLocalDateTimeAtNamedZoneResult<Self::ZonedDateTime>;

    /// Attach a native named zone to a native fixed-instant timestamp.
    ///
    /// The returned exchange re-issues fixed-instant semantics, named-zone
    /// identity, named-zone attachment, and offset-consistency evidence.
    fn attach_named_zone_native(
        &self,
        timestamp: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        zone: &ProvenNamedTimeZoneCarrier<Self::NamedTimeZone>,
    ) -> NativeAttachedProvenNamedZoneResult<Self::ZonedDateTime>;

    /// Confirm that a native zoned timestamp is interpreted under a concrete TZDB revision.
    fn confirm_named_zone_revision_native(
        &self,
        timestamp: &ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
    ) -> ConfirmedProvenNamedZoneRevisionResult;
}
