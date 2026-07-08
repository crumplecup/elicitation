//! Native descriptor/carrier bridge traits for temporal backends.
//!
//! These traits are intentionally not object-safe architectural centers. Their
//! purpose is to let backends exchange real upstream temporal carrier types
//! while preserving the same explicit proof-sidecar grammar used everywhere
//! else in `elicit_temporal`.
//!
//! The bridge layer is the first user-facing aggregation point above the raw
//! `Established<P>` sidecars: descriptors cross the seam together with named
//! semantic bundles, and native carriers return wrapped in proven carrier
//! aggregates rather than loose proof tuples.

use crate::{
    LocalDateTimeDescriptor, LocalDateTimeSemanticBundle, NamedTimeZoneDescriptor,
    NamedTimeZoneSemanticBundle, OffsetDateTimeDescriptor, OffsetDateTimeSemanticBundle,
    ProvenLocalDateTimeCarrier, ProvenNamedTimeZoneCarrier, ProvenOffsetDateTimeCarrier,
    ProvenZonedDateTimeCarrier, RealizedProvenLocalDateTimeResult,
    RealizedProvenNamedTimeZoneResult, RealizedProvenOffsetDateTimeResult,
    RealizedProvenZonedDateTimeResult, ReflectedProvenLocalDateTimeResult,
    ReflectedProvenNamedTimeZoneResult, ReflectedProvenOffsetDateTimeResult,
    ReflectedProvenZonedDateTimeResult, TemporalCivilProps, TemporalInstantProps,
    TemporalZoneProps, ZonedDateTimeDescriptor, ZonedDateTimeSemanticBundle,
};

/// Realize and reflect civil local date-time carriers.
pub trait TemporalCivilNativeBridge: TemporalCivilProps + Send + Sync {
    /// Realize a validated neutral local date-time descriptor as a proven backend-native carrier.
    fn realize_local_date_time(
        &self,
        timestamp: &LocalDateTimeDescriptor,
        semantics: &LocalDateTimeSemanticBundle,
    ) -> RealizedProvenLocalDateTimeResult<Self::LocalDateTime>;

    /// Reflect a proven backend-native local date-time carrier into the neutral descriptor accord.
    fn reflect_local_date_time(
        &self,
        timestamp: &ProvenLocalDateTimeCarrier<Self::LocalDateTime>,
    ) -> ReflectedProvenLocalDateTimeResult;
}

/// Realize and reflect fixed-instant offset date-time carriers.
pub trait TemporalInstantNativeBridge: TemporalInstantProps + Send + Sync {
    /// Realize a validated neutral offset date-time descriptor as a proven backend-native carrier.
    fn realize_offset_date_time(
        &self,
        timestamp: &OffsetDateTimeDescriptor,
        semantics: &OffsetDateTimeSemanticBundle,
    ) -> RealizedProvenOffsetDateTimeResult<Self::OffsetDateTime>;

    /// Reflect a proven backend-native offset date-time carrier into the neutral descriptor accord.
    fn reflect_offset_date_time(
        &self,
        timestamp: &ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
    ) -> ReflectedProvenOffsetDateTimeResult;
}

/// Realize and reflect named-zone and zoned date-time carriers.
pub trait TemporalZoneNativeBridge: TemporalInstantProps + TemporalZoneProps + Send + Sync {
    /// Realize a validated neutral named-zone descriptor as a proven backend-native carrier.
    fn realize_named_time_zone(
        &self,
        zone: &NamedTimeZoneDescriptor,
        semantics: &NamedTimeZoneSemanticBundle,
    ) -> RealizedProvenNamedTimeZoneResult<Self::NamedTimeZone>;

    /// Reflect a proven backend-native named-zone carrier into the neutral descriptor accord.
    fn reflect_named_time_zone(
        &self,
        zone: &ProvenNamedTimeZoneCarrier<Self::NamedTimeZone>,
    ) -> ReflectedProvenNamedTimeZoneResult;

    /// Realize a validated neutral zoned date-time descriptor as a proven backend-native carrier.
    fn realize_zoned_date_time(
        &self,
        timestamp: &ZonedDateTimeDescriptor,
        semantics: &ZonedDateTimeSemanticBundle,
    ) -> RealizedProvenZonedDateTimeResult<Self::ZonedDateTime>;

    /// Reflect a proven backend-native zoned date-time carrier into the neutral descriptor accord.
    fn reflect_zoned_date_time(
        &self,
        timestamp: &ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
    ) -> ReflectedProvenZonedDateTimeResult;
}

/// Aggregate native bridge for the first civil/instant/zone vertical slice.
pub trait TemporalNativeBridge:
    TemporalCivilNativeBridge + TemporalInstantNativeBridge + TemporalZoneNativeBridge
{
}

impl<T> TemporalNativeBridge for T where
    T: TemporalCivilNativeBridge + TemporalInstantNativeBridge + TemporalZoneNativeBridge
{
}
