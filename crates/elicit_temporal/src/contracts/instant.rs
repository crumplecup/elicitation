//! Instant-semantics propositions.
//!
//! Sources:
//! - ISO 8601-1:2019 — combined date-time and UTC relationship forms
//! - RFC 3339 §4.4, §5.6
//! - RFC 9557 §3.1

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — instant contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — instant contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — instant contract */ }
                }
            }
        };
    }

    /// A timestamp carries an explicit UTC relationship.
    ///
    /// Normative source: RFC 3339 §4.4, §5.6
    pub struct TimestampHasExplicitUtcOffset;
    structural_prop!(
        TimestampHasExplicitUtcOffset,
        "TimestampHasExplicitUtcOffset"
    );

    /// An offset date-time identifies a single fixed instant on the UTC timeline.
    ///
    /// Normative source: ISO 8601-1:2019 — date-time with UTC relationship
    /// Informative cross-check: RFC 3339 §5.6
    pub struct OffsetDateTimeIdentifiesSingleInstant;
    structural_prop!(
        OffsetDateTimeIdentifiesSingleInstant,
        "OffsetDateTimeIdentifiesSingleInstant"
    );

    /// A local date-time requires an offset or zone authority before it can identify an instant.
    ///
    /// Normative source: RFC 3339 §4.4 — Unqualified Local Time
    pub struct LocalDateTimeRequiresZoneOrOffsetForInstant;
    structural_prop!(
        LocalDateTimeRequiresZoneOrOffsetForInstant,
        "LocalDateTimeRequiresZoneOrOffsetForInstant"
    );

    /// A local date-time may be ambiguous at a zone transition boundary.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct LocalDateTimeMayBeAmbiguousAtZoneTransition;
    structural_prop!(
        LocalDateTimeMayBeAmbiguousAtZoneTransition,
        "LocalDateTimeMayBeAmbiguousAtZoneTransition"
    );

    /// A local date-time may fall inside a skipped wall-clock gap at a zone transition.
    ///
    /// Normative source: RFC 9557 §3.1 — Time Zone Identifiers
    pub struct LocalDateTimeMayFallInZoneTransitionGap;
    structural_prop!(
        LocalDateTimeMayFallInZoneTransitionGap,
        "LocalDateTimeMayFallInZoneTransitionGap"
    );

    /// Fixed-instants are ordered on the UTC timeline.
    ///
    /// Normative source: RFC 3339 §5.1 — Ordering
    pub struct UtcTimelineOrderingAppliesToFixedInstants;
    structural_prop!(
        UtcTimelineOrderingAppliesToFixedInstants,
        "UtcTimelineOrderingAppliesToFixedInstants"
    );
}

pub use emit_impls::{
    LocalDateTimeMayBeAmbiguousAtZoneTransition, LocalDateTimeMayFallInZoneTransitionGap,
    LocalDateTimeRequiresZoneOrOffsetForInstant, OffsetDateTimeIdentifiesSingleInstant,
    TimestampHasExplicitUtcOffset, UtcTimelineOrderingAppliesToFixedInstants,
};
