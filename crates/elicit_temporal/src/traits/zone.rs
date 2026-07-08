//! Named-zone resolution and attachment traits.

use elicitation::Established;

use crate::{
    AttachedNamedZoneResult, ConfirmedNamedZoneRevisionResult,
    ConfirmedZoneTransitionResolutionAuthorityResult, LocalDateTimeDescriptor,
    LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptor,
    NamedTimeZoneIdentityValid, OffsetDateTimeDescriptor, ResolvedLocalDateTimeAtNamedZoneResult,
    ResolvedNamedTimeZoneResult, TimestampRepresentsFixedInstant,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Resolve and attach named-zone identity without collapsing it to a bare offset.
///
/// Normative sources: RFC 9557 §1.2, §3.4, and §4.1.
/// Informative cross-check: BCP 175 / IANA TZDB naming.
pub trait TemporalZoneFactory: Send + Sync {
    /// Resolve a named IANA time zone into the neutral zone descriptor.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn resolve_named_zone(&self, identifier: &str) -> ResolvedNamedTimeZoneResult;

    /// Confirm that a local-to-zone resolution policy is explicit and lawful before use.
    ///
    /// This keeps the authority descriptor itself in the leaf-constructor role:
    /// higher-order zone resolution can then consume the established authority
    /// sidecar instead of minting it implicitly.
    ///
    /// Normative sources: RFC 9557 §1.1, §1.2, §3.4, and §4.1.
    fn confirm_local_time_zone_resolution_authority(
        &self,
        zone: &NamedTimeZoneDescriptor,
        resolution_authority: &LocalTimeZoneResolutionAuthorityDescriptor,
        zone_identity: Established<NamedTimeZoneIdentityValid>,
    ) -> ConfirmedZoneTransitionResolutionAuthorityResult;

    /// Resolve a local wall-clock timestamp against a named zone using explicit transition authority.
    ///
    /// The caller must supply both named-zone identity and explicit
    /// resolution-authority sidecars, and the returned exchange re-issues the
    /// named-zone attachment, offset-consistency, authority, and
    /// ambiguity-versus-gap branch semantics explicitly.
    ///
    /// Normative sources: RFC 9557 §1.1, §1.2, and §3.4.
    fn resolve_local_date_time(
        &self,
        timestamp: &LocalDateTimeDescriptor,
        zone: &NamedTimeZoneDescriptor,
        resolution_authority: &LocalTimeZoneResolutionAuthorityDescriptor,
        timestamp_proof: Established<LocalDateTimeValid>,
        local_semantics: Established<LocalDateTimeDoesNotIdentifyFixedInstant>,
        zone_identity: Established<NamedTimeZoneIdentityValid>,
        resolution_authority_proof: Established<ZoneTransitionResolutionAuthorityValid>,
    ) -> ResolvedLocalDateTimeAtNamedZoneResult;

    /// Attach a named zone to a fixed-instant timestamp.
    ///
    /// The caller supplies the fixed-instant and named-zone identity sidecars;
    /// the result re-issues generic named-zone attachment and
    /// offset-consistency evidence.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn attach_named_zone(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        zone: &NamedTimeZoneDescriptor,
        fixed_instant: Established<TimestampRepresentsFixedInstant>,
        zone_identity: Established<NamedTimeZoneIdentityValid>,
    ) -> AttachedNamedZoneResult;

    /// Confirm that the attached named zone is interpreted under a concrete TZDB revision.
    ///
    /// The returned exchange keeps revision-tracking semantics explicit rather
    /// than collapsing them into one bare proposition token.
    ///
    /// Normative sources: RFC 9557 §1.2 and §4.1.
    fn confirm_named_zone_revision(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        zone_proof: Established<ZonedDateTimeHasNamedZone>,
    ) -> ConfirmedNamedZoneRevisionResult;
}
