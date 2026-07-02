//! ISO 8601 interval, duration, and recurrence propositions.
//!
//! Sources:
//! - ISO 8601-1:2019, *Date and time - Representations for information interchange - Part 1: Basic rules*
//! - ISO 8601-2:2019, *Date and time - Representations for information interchange - Part 2: Extensions*
//!
//! Core interval and recurrence laws below now carry exact clause references
//! from the locally staged ISO extracts. Where public clause-addressable
//! CalConnect text mirrors the same law, those aligned subsection references
//! are embedded below as informative cross-checks.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 interval contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 interval contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 interval contract */ }
                }
            }
        };
    }

    /// A duration representation begins with the duration designator `P`.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.2 b)
    /// Informative cross-check: CalConnect CC 18011:2018 §7.3 — Representations
    pub struct DurationUsesPeriodDesignator;
    structural_prop!(DurationUsesPeriodDesignator, "DurationUsesPeriodDesignator");

    /// A duration time component is introduced by the time designator `T`.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.3.2
    /// Informative cross-check: CalConnect CC 18011:2018 §7.3 — Representations
    pub struct DurationTimeComponentsFollowTimeDesignator;
    structural_prop!(
        DurationTimeComponentsFollowTimeDesignator,
        "DurationTimeComponentsFollowTimeDesignator"
    );

    /// A week-only duration uses the week unit without mixing calendar or clock units.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.3.2
    /// Informative cross-check: CalConnect CC 18011:2018 §7.3 — Representations
    pub struct DurationWeekFormUsesSingleWeekUnit;
    structural_prop!(
        DurationWeekFormUsesSingleWeekUnit,
        "DurationWeekFormUsesSingleWeekUnit"
    );

    /// A week-form duration is not mixed with year, month, day, hour, minute, or second units.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.3.2
    /// Informative cross-check: CalConnect CC 18011:2018 §7.3 — Representations
    pub struct DurationWeekFormNotMixedWithCalendarOrClockUnits;
    structural_prop!(
        DurationWeekFormNotMixedWithCalendarOrClockUnits,
        "DurationWeekFormNotMixedWithCalendarOrClockUnits"
    );

    /// A time interval separates its two components with the solidus `/`.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.2 a)
    /// Informative cross-check: CalConnect CC 18011:2018 §6.14 — Time interval
    pub struct TimeIntervalUsesSolidusSeparator;
    structural_prop!(
        TimeIntervalUsesSolidusSeparator,
        "TimeIntervalUsesSolidusSeparator"
    );

    /// A time interval carries exactly two top-level components.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.4.1; 4.4.2 a)
    /// Informative cross-check: CalConnect CC 18011:2018 §6.14 — Time interval
    pub struct TimeIntervalHasTwoComponents;
    structural_prop!(TimeIntervalHasTwoComponents, "TimeIntervalHasTwoComponents");

    /// A time interval declares whether it is start/end, start/duration, or duration/end.
    ///
    /// Normative source: ISO 8601-1:2019, 4.4.1
    /// Informative cross-check: CalConnect CC 18011:2018 §6.14 — Time interval
    pub struct TimeIntervalBoundaryOrDurationFormDeclared;
    structural_prop!(
        TimeIntervalBoundaryOrDurationFormDeclared,
        "TimeIntervalBoundaryOrDurationFormDeclared"
    );

    /// An interval endpoint pair is ordered from earlier to later on the relevant timeline.
    ///
    /// Normative sources: ISO 8601-1:2019, 3.1.1.6; 3.1.1.8
    /// Informative cross-check: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration
    pub struct IntervalStartPrecedesEnd;
    structural_prop!(IntervalStartPrecedesEnd, "IntervalStartPrecedesEnd");

    /// An interval duration does not represent a negative span.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8
    /// Informative cross-check: CalConnect CC 18011:2018 §8 — Evaluation of date and time with duration
    pub struct IntervalDurationIsNonNegative;
    structural_prop!(
        IntervalDurationIsNonNegative,
        "IntervalDurationIsNonNegative"
    );

    /// A recurring interval begins with the repetition designator `R`.
    ///
    /// Normative source: ISO 8601-1:2019, 4.5.2
    /// Informative cross-check: CalConnect CC 18012:2018 §6.3 — Repeat rule; §6.4 — Complete representation
    pub struct RecurringIntervalUsesRepeatDesignator;
    structural_prop!(
        RecurringIntervalUsesRepeatDesignator,
        "RecurringIntervalUsesRepeatDesignator"
    );

    /// A bounded recurring interval recurrence count is non-negative.
    ///
    /// Normative source: ISO 8601-1:2019, 4.5.1
    /// Informative cross-check: CalConnect CC 18012:2018 §6.3 — Repeat rule
    pub struct RecurringIntervalCountIsNonNegativeWhenBounded;
    structural_prop!(
        RecurringIntervalCountIsNonNegativeWhenBounded,
        "RecurringIntervalCountIsNonNegativeWhenBounded"
    );

    /// An omitted recurring interval recurrence count denotes unbounded occurrences.
    ///
    /// Normative source: ISO 8601-1:2019, 4.5.1
    /// Informative cross-check: CalConnect CC 18012:2018 §6.4 — Complete representation
    pub struct RecurringIntervalOmittedCountDenotesUnboundedOccurrences;
    structural_prop!(
        RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
        "RecurringIntervalOmittedCountDenotesUnboundedOccurrences"
    );

    /// A recurring interval carries an interval component after the repetition prefix.
    ///
    /// Normative sources: ISO 8601-1:2019, 4.5.1; 4.5.2
    /// Informative cross-check: CalConnect CC 18012:2018 §6.4 — Complete representation
    pub struct RecurringIntervalCarriesIntervalComponent;
    structural_prop!(
        RecurringIntervalCarriesIntervalComponent,
        "RecurringIntervalCarriesIntervalComponent"
    );
}

pub use emit_impls::{
    DurationTimeComponentsFollowTimeDesignator, DurationUsesPeriodDesignator,
    DurationWeekFormNotMixedWithCalendarOrClockUnits, DurationWeekFormUsesSingleWeekUnit,
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    RecurringIntervalCarriesIntervalComponent, RecurringIntervalCountIsNonNegativeWhenBounded,
    RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    RecurringIntervalUsesRepeatDesignator, TimeIntervalBoundaryOrDurationFormDeclared,
    TimeIntervalHasTwoComponents, TimeIntervalUsesSolidusSeparator,
};
