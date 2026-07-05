//! Named-zone resolution and attachment traits.

use elicitation::Established;

use crate::{
    LocalDateTimeDescriptor, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptor,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedTimeZoneUsesIanaIdentifier,
    OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor,
    ResolvedLocalDateTimeAtNamedZoneResult, TemporalResult, TimestampRepresentsFixedInstant,
    ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Resolve and attach named-zone identity without collapsing it to a bare offset.
///
/// Normative sources: RFC 9557 §1.2, §3.4, and §4.1.
/// Informative cross-check: BCP 175 / IANA TZDB naming.
pub trait TemporalZoneFactory: Send + Sync {
    /// Resolve a named IANA time zone into the neutral zone descriptor.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn resolve_named_zone(
        &self,
        identifier: &str,
    ) -> TemporalResult<(
        NamedTimeZoneDescriptor,
        Established<NamedTimeZoneUsesIanaIdentifier>,
    )>;

    /// Resolve a local wall-clock timestamp against a named zone using explicit transition authority.
    ///
    /// The authority descriptor makes repeated and skipped local times lawful
    /// input rather than backend-defined error cases.
    ///
    /// Normative sources: RFC 9557 §1.1, §1.2, and §3.4.
    fn resolve_local_date_time(
        &self,
        timestamp: &LocalDateTimeDescriptor,
        zone: &NamedTimeZoneDescriptor,
        timestamp_proof: Established<LocalDateTimeValid>,
        local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
        zone_identity: Established<NamedTimeZoneUsesIanaIdentifier>,
        resolution_authority: &LocalTimeZoneResolutionAuthorityDescriptor,
    ) -> ResolvedLocalDateTimeAtNamedZoneResult;

    /// Attach a named zone to a fixed-instant timestamp.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn attach_named_zone(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        zone: &NamedTimeZoneDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
        zone_identity: Established<NamedTimeZoneUsesIanaIdentifier>,
    ) -> TemporalResult<(
        ZonedDateTimeDescriptor,
        Established<ZonedDateTimeHasNamedZone>,
        Established<OffsetConsistentWithNamedZone>,
    )>;

    /// Confirm that the attached named zone is interpreted under a concrete TZDB revision.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn confirm_named_zone_revision(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        zone_proof: Established<ZonedDateTimeHasNamedZone>,
    ) -> TemporalResult<Established<NamedTimeZoneInterpretationTracksTzdbRevision>>;
}
