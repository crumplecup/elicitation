//! ISO 8601 temporal propositions.
//!
//! Sources:
//! - ISO 8601-1:2019, *Date and time - Representations for information interchange - Part 1: Basic rules*
//! - ISO 8601-1:2019/Amd 1:2022, *Date and time - Representations for information interchange - Part 1: Basic rules — Amendment 1: Technical corrections*
//! - ISO 8601-2:2019, *Date and time - Representations for information interchange - Part 2: Extensions*
//!
//! This module carries clause-level citations grounded in the local licensed
//! extracts and open working-draft cross-checks used throughout the temporal
//! standards worksheets.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601 contract */ }
                }
            }
        };
    }

    /// A calendar date uses the proleptic Gregorian calendar representation.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.2.1 and 4.1.2.1.
    pub struct CalendarDateUsesGregorianCalendar;
    structural_prop!(
        CalendarDateUsesGregorianCalendar,
        "CalendarDateUsesGregorianCalendar"
    );

    /// A time axis represents the succession in time of instantaneous events along a unique axis.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.4
    pub struct TimeAxisOrdersTimePointsByTemporalPosition;
    structural_prop!(
        TimeAxisOrdersTimePointsByTemporalPosition,
        "TimeAxisOrdersTimePointsByTemporalPosition"
    );

    /// A time scale is a system of ordered marks attributed to instants on the time axis.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.5
    pub struct TimeScaleAssociatesTimePointsWithOrderedMeasure;
    structural_prop!(
        TimeScaleAssociatesTimePointsWithOrderedMeasure,
        "TimeScaleAssociatesTimePointsWithOrderedMeasure"
    );

    /// A time is a mark attributed to an instant or time interval on a specified time scale.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.2
    pub struct TimeIsMarkOnSpecifiedTimeScale;
    structural_prop!(
        TimeIsMarkOnSpecifiedTimeScale,
        "TimeIsMarkOnSpecifiedTimeScale"
    );

    /// An instant is a point on the time axis.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.3
    pub struct InstantIsPointOnTimeAxis;
    structural_prop!(InstantIsPointOnTimeAxis, "InstantIsPointOnTimeAxis");

    /// A date is time on the calendar time scale.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.1
    pub struct DateIdentifiesPositionWithinCalendar;
    structural_prop!(
        DateIdentifiesPositionWithinCalendar,
        "DateIdentifiesPositionWithinCalendar"
    );

    /// A time of day is time occurring within a calendar day.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.16
    pub struct TimeOfDayOccursWithinCalendarDay;
    structural_prop!(
        TimeOfDayOccursWithinCalendarDay,
        "TimeOfDayOccursWithinCalendarDay"
    );

    /// A calendar date carries year, month, and day components.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.1.9 and 4.1.2.2.
    pub struct CalendarDateHasYearMonthDay;
    structural_prop!(CalendarDateHasYearMonthDay, "CalendarDateHasYearMonthDay");

    /// A reduced-precision calendar date carries a calendar year component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.3.
    pub struct ReducedCalendarDateHasYearComponent;
    structural_prop!(
        ReducedCalendarDateHasYearComponent,
        "ReducedCalendarDateHasYearComponent"
    );

    /// A reduced-precision calendar date may use the year-only form.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.3 b).
    pub struct ReducedCalendarDateUsesYearOnlyRepresentation;
    structural_prop!(
        ReducedCalendarDateUsesYearOnlyRepresentation,
        "ReducedCalendarDateUsesYearOnlyRepresentation"
    );

    /// A reduced-precision calendar date may use the year-month form.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.3 a).
    pub struct ReducedCalendarDateUsesYearMonthRepresentation;
    structural_prop!(
        ReducedCalendarDateUsesYearMonthRepresentation,
        "ReducedCalendarDateUsesYearMonthRepresentation"
    );

    /// A reduced calendar date omits lower-order digits starting from the extreme right-hand side.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.3.
    pub struct ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight;
    structural_prop!(
        ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight,
        "ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight"
    );

    /// A calendar-date month value is in the range 01 through 12.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.2.1 and 4.1.2.1.
    pub struct CalendarMonthInRangeOneToTwelve;
    structural_prop!(
        CalendarMonthInRangeOneToTwelve,
        "CalendarMonthInRangeOneToTwelve"
    );

    /// A calendar-date day value lies within the valid bounds for the given month and year.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.2.1 and 4.1.2.1.
    pub struct CalendarDayWithinMonthBounds;
    structural_prop!(CalendarDayWithinMonthBounds, "CalendarDayWithinMonthBounds");

    /// February 29 appears only in a leap year.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.21 note 1; ISO/WD 8601-1:2016(E), 3.2.1.
    pub struct LeapDayOccursOnlyInLeapYear;
    structural_prop!(LeapDayOccursOnlyInLeapYear, "LeapDayOccursOnlyInLeapYear");

    /// A Gregorian calendar decade ordinal is in the range `000` through `999`.
    ///
    /// Normative source: ISO 8601-1:2019/Amd 1:2022, 4.3.11.
    pub struct DecadeOrdinalInRangeZeroToNineHundredNinetyNine;
    structural_prop!(
        DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
        "DecadeOrdinalInRangeZeroToNineHundredNinetyNine"
    );

    /// A Gregorian calendar century ordinal is in the range `00` through `99`.
    ///
    /// Normative source: ISO 8601-1:2019/Amd 1:2022, 4.3.12.
    pub struct CenturyOrdinalInRangeZeroToNinetyNine;
    structural_prop!(
        CenturyOrdinalInRangeZeroToNinetyNine,
        "CenturyOrdinalInRangeZeroToNinetyNine"
    );

    /// An ordinal date carries a year and day-of-year component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.1.10 and 4.1.3.2.
    pub struct OrdinalDateHasYearAndDayOfYear;
    structural_prop!(
        OrdinalDateHasYearAndDayOfYear,
        "OrdinalDateHasYearAndDayOfYear"
    );

    /// An ordinal day is in the range 001 through 365 or 366, depending on the year.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.2.1 and 4.1.3.1.
    pub struct OrdinalDayInRangeOneToThreeHundredSixtySix;
    structural_prop!(
        OrdinalDayInRangeOneToThreeHundredSixtySix,
        "OrdinalDayInRangeOneToThreeHundredSixtySix"
    );

    /// An ordinal day-of-year component is represented by three decimal digits.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.3.1.
    pub struct OrdinalDayOfYearUsesThreeDigits;
    structural_prop!(
        OrdinalDayOfYearUsesThreeDigits,
        "OrdinalDayOfYearUsesThreeDigits"
    );

    /// A week date carries week-year, week number, and weekday components.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.1.11 and 4.1.4.2.
    pub struct WeekDateHasWeekYearWeekAndWeekday;
    structural_prop!(
        WeekDateHasWeekYearWeekAndWeekday,
        "WeekDateHasWeekYearWeekAndWeekday"
    );

    /// A week number is in the range 01 through 53.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.4.1.
    pub struct WeekNumberInRangeOneToFiftyThree;
    structural_prop!(
        WeekNumberInRangeOneToFiftyThree,
        "WeekNumberInRangeOneToFiftyThree"
    );

    /// A calendar week begins on Monday.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.2.8 and 4.1.4.1.
    pub struct CalendarWeekStartsOnMonday;
    structural_prop!(CalendarWeekStartsOnMonday, "CalendarWeekStartsOnMonday");

    /// A calendar week number is assigned using the first-Thursday rule.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.23; ISO/WD 8601-1:2016(E), 2.2.10.
    pub struct CalendarWeekNumberUsesFirstThursdayRule;
    structural_prop!(
        CalendarWeekNumberUsesFirstThursdayRule,
        "CalendarWeekNumberUsesFirstThursdayRule"
    );

    /// A weekday is in the range 1 through 7.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.4.1.
    pub struct WeekdayInRangeOneToSeven;
    structural_prop!(WeekdayInRangeOneToSeven, "WeekdayInRangeOneToSeven");

    /// A reduced-accuracy week date omits the weekday component and identifies a specific week.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.4.3.
    pub struct ReducedAccuracyWeekDateOmitsWeekdayComponent;
    structural_prop!(
        ReducedAccuracyWeekDateOmitsWeekdayComponent,
        "ReducedAccuracyWeekDateOmitsWeekdayComponent"
    );

    /// A local time carries hour, minute, and second components.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.2.2.
    pub struct LocalTimeHasHourMinuteSecond;
    structural_prop!(LocalTimeHasHourMinuteSecond, "LocalTimeHasHourMinuteSecond");

    /// A reduced-accuracy local-time representation may identify a specific hour and minute.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.2.3 a).
    pub struct ReducedAccuracyLocalTimeUsesHourMinuteRepresentation;
    structural_prop!(
        ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
        "ReducedAccuracyLocalTimeUsesHourMinuteRepresentation"
    );

    /// A reduced-accuracy local-time representation may identify a specific hour only.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.2.3 b).
    pub struct ReducedAccuracyLocalTimeUsesHourOnlyRepresentation;
    structural_prop!(
        ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
        "ReducedAccuracyLocalTimeUsesHourOnlyRepresentation"
    );

    /// An hour value is in the range 00 through 24.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.1; ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 and 5.3.2.
    pub struct HourInRangeZeroToTwentyFour;
    structural_prop!(HourInRangeZeroToTwentyFour, "HourInRangeZeroToTwentyFour");

    /// A minute value is in the range 00 through 59.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.1.
    pub struct MinuteInRangeZeroToFiftyNine;
    structural_prop!(MinuteInRangeZeroToFiftyNine, "MinuteInRangeZeroToFiftyNine");

    /// A second value is in the range 00 through 60.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.1.
    pub struct SecondInRangeZeroToSixty;
    structural_prop!(SecondInRangeZeroToSixty, "SecondInRangeZeroToSixty");

    /// The value 24 for the hour component is reserved for the end of a calendar day.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.1 and 4.2.3; ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 and 5.3.2.
    pub struct TwentyFourHourReservedForEndOfDay;
    structural_prop!(
        TwentyFourHourReservedForEndOfDay,
        "TwentyFourHourReservedForEndOfDay"
    );

    /// When the hour component is 24, the minute, second, and fraction components are zero.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.3; ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 and 5.3.2.
    pub struct TwentyFourHourRequiresZeroMinuteSecondAndFraction;
    structural_prop!(
        TwentyFourHourRequiresZeroMinuteSecondAndFraction,
        "TwentyFourHourRequiresZeroMinuteSecondAndFraction"
    );

    /// A leap second may be used only at a UTC boundary where UTC permits it.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.1.
    pub struct LeapSecondOccursOnlyAtUtcBoundary;
    structural_prop!(
        LeapSecondOccursOnlyAtUtcBoundary,
        "LeapSecondOccursOnlyAtUtcBoundary"
    );

    /// A second is the base unit of measurement of time in the International System of Units.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.1 second
    pub struct SecondIsSiBaseUnitOfTime;
    structural_prop!(SecondIsSiBaseUnitOfTime, "SecondIsSiBaseUnitOfTime");

    /// A second is the base unit for expressing duration.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.1 second
    pub struct SecondIsBaseUnitForExpressingDuration;
    structural_prop!(
        SecondIsBaseUnitForExpressingDuration,
        "SecondIsBaseUnitForExpressingDuration"
    );

    /// A minute is a unit of time equal to sixty seconds.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.3 minute
    pub struct MinuteEqualsSixtySeconds;
    structural_prop!(MinuteEqualsSixtySeconds, "MinuteEqualsSixtySeconds");

    /// An hour is a unit of time equal to sixty minutes.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.4 hour
    pub struct HourEqualsSixtyMinutes;
    structural_prop!(HourEqualsSixtyMinutes, "HourEqualsSixtyMinutes");

    /// A day as a unit of time is equal to twenty-four hours.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.5 day
    pub struct DayEqualsTwentyFourHours;
    structural_prop!(DayEqualsTwentyFourHours, "DayEqualsTwentyFourHours");

    /// A duration equals the difference between the final and initial instants of a time interval.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8.
    pub struct DurationEqualsDifferenceBetweenIntervalEndpoints;
    structural_prop!(
        DurationEqualsDifferenceBetweenIntervalEndpoints,
        "DurationEqualsDifferenceBetweenIntervalEndpoints"
    );

    /// A calendar day is the interval associated with one calendar-date advance.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.2.6.
    pub struct CalendarDayIsIntervalOfSingleCalendarDateAdvance;
    structural_prop!(
        CalendarDayIsIntervalOfSingleCalendarDateAdvance,
        "CalendarDayIsIntervalOfSingleCalendarDateAdvance"
    );

    /// A calendar-day duration may be modified by leap seconds or local-time-scale adjustments.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.6 calendar day
    pub struct CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts;
    structural_prop!(
        CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts,
        "CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts"
    );

    /// A day as a nominal duration may vary from exact elapsed-time interpretations.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8 note 3; ISO/WD 8601-1:2016(E), 2.2.7.
    pub struct NominalDayDurationMayDifferFromExactElapsedTime;
    structural_prop!(
        NominalDayDurationMayDifferFromExactElapsedTime,
        "NominalDayDurationMayDifferFromExactElapsedTime"
    );

    /// A day duration may span from a given time of day to the same time of day on the next calendar day.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.7 day
    pub struct DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays;
    structural_prop!(
        DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays,
        "DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays"
    );

    /// A calendar week is a seven-day calendar interval beginning on Monday.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.2.8.
    pub struct CalendarWeekIsSevenDayIntervalBeginningOnMonday;
    structural_prop!(
        CalendarWeekIsSevenDayIntervalBeginningOnMonday,
        "CalendarWeekIsSevenDayIntervalBeginningOnMonday"
    );

    /// A week as a nominal duration is distinct from an exact elapsed-duration measure.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8 note 3; ISO/WD 8601-1:2016(E), 2.2.9.
    pub struct NominalWeekDurationIsDistinctFromExactElapsedTime;
    structural_prop!(
        NominalWeekDurationIsDistinctFromExactElapsedTime,
        "NominalWeekDurationIsDistinctFromExactElapsedTime"
    );

    /// A week duration may span from a given time of day to the same weekday and time of day in the next calendar week.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.9 week
    pub struct WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek;
    structural_prop!(
        WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek,
        "WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek"
    );

    /// A calendar month is the named month interval within a calendar year.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.2.11.
    pub struct CalendarMonthIsNamedIntervalWithinCalendarYear;
    structural_prop!(
        CalendarMonthIsNamedIntervalWithinCalendarYear,
        "CalendarMonthIsNamedIntervalWithinCalendarYear"
    );

    /// A month as a nominal duration depends on calendar context.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8 note 3; ISO/WD 8601-1:2016(E), 2.2.12.
    pub struct NominalMonthDurationDependsOnCalendarContext;
    structural_prop!(
        NominalMonthDurationDependsOnCalendarContext,
        "NominalMonthDurationDependsOnCalendarContext"
    );

    /// A month duration spans twenty-eight, twenty-nine, thirty, or thirty-one calendar days according to calendar context.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.12 month
    pub struct MonthDurationInRangeTwentyEightToThirtyOneCalendarDays;
    structural_prop!(
        MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
        "MonthDurationInRangeTwentyEightToThirtyOneCalendarDays"
    );

    /// A month duration may require an agreed ending calendar day when the same day does not exist in the next month.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.12 month
    pub struct MonthDurationMayRequireAgreedEndingCalendarDay;
    structural_prop!(
        MonthDurationMayRequireAgreedEndingCalendarDay,
        "MonthDurationMayRequireAgreedEndingCalendarDay"
    );

    /// Certain applications may treat a month as a duration of thirty calendar days.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.12 month
    pub struct MonthMayBeConsideredThirtyCalendarDaysInCertainApplications;
    structural_prop!(
        MonthMayBeConsideredThirtyCalendarDaysInCertainApplications,
        "MonthMayBeConsideredThirtyCalendarDaysInCertainApplications"
    );

    /// A calendar year is the interval formed by its successive calendar months.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.2.13.
    pub struct CalendarYearIsIntervalOfSuccessiveCalendarMonths;
    structural_prop!(
        CalendarYearIsIntervalOfSuccessiveCalendarMonths,
        "CalendarYearIsIntervalOfSuccessiveCalendarMonths"
    );

    /// A year as a nominal duration depends on calendar context.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8 note 3; ISO/WD 8601-1:2016(E), 2.2.14.
    pub struct NominalYearDurationDependsOnCalendarContext;
    structural_prop!(
        NominalYearDurationDependsOnCalendarContext,
        "NominalYearDurationDependsOnCalendarContext"
    );

    /// A year duration spans three hundred sixty-five or three hundred sixty-six calendar days according to calendar context.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.14 year
    pub struct YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays;
    structural_prop!(
        YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
        "YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays"
    );

    /// A year duration may require an agreed ending calendar date when the same date does not exist in the next year.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 2.2.14 year
    pub struct YearDurationMayRequireAgreedEndingCalendarDate;
    structural_prop!(
        YearDurationMayRequireAgreedEndingCalendarDate,
        "YearDurationMayRequireAgreedEndingCalendarDate"
    );

    /// A nominal duration depends on calendar context rather than only fixed elapsed seconds.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.8 note 3.
    pub struct NominalDurationDependsOnCalendarContext;
    structural_prop!(
        NominalDurationDependsOnCalendarContext,
        "NominalDurationDependsOnCalendarContext"
    );

    /// A decimal fraction uses an ISO 8601 decimal sign.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.2.4.
    pub struct FractionUsesDecimalSign;
    structural_prop!(FractionUsesDecimalSign, "FractionUsesDecimalSign");

    /// A decimal fraction applies to the lowest-order present date or time component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.6 and 4.2.2.4.
    pub struct FractionAppliesToLowestOrderComponent;
    structural_prop!(
        FractionAppliesToLowestOrderComponent,
        "FractionAppliesToLowestOrderComponent"
    );

    /// A combined date-time representation uses the time designator between date and time parts.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.2 and 4.3.3.
    pub struct CombinedDateTimeUsesTimeDesignator;
    structural_prop!(
        CombinedDateTimeUsesTimeDesignator,
        "CombinedDateTimeUsesTimeDesignator"
    );

    /// A combined date-time representation may use a complete calendar-date component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.2 a).
    pub struct CombinedDateTimePermitsCalendarDateComponent;
    structural_prop!(
        CombinedDateTimePermitsCalendarDateComponent,
        "CombinedDateTimePermitsCalendarDateComponent"
    );

    /// A combined date-time representation may use a complete ordinal-date component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.2 b).
    pub struct CombinedDateTimePermitsOrdinalDateComponent;
    structural_prop!(
        CombinedDateTimePermitsOrdinalDateComponent,
        "CombinedDateTimePermitsOrdinalDateComponent"
    );

    /// A combined date-time representation may use a complete week-date component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.2 c).
    pub struct CombinedDateTimePermitsWeekDateComponent;
    structural_prop!(
        CombinedDateTimePermitsWeekDateComponent,
        "CombinedDateTimePermitsWeekDateComponent"
    );

    /// A combined date-time representation does not use a reduced-accuracy date component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.3 c).
    pub struct CombinedDateTimeDateComponentMustNotUseReducedAccuracy;
    structural_prop!(
        CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
        "CombinedDateTimeDateComponentMustNotUseReducedAccuracy"
    );

    /// A combined date-time representation uses one format family across its date and time parts.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.3.3 d).
    pub struct CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents;
    structural_prop!(
        CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
        "CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents"
    );

    /// The UTC designator is the uppercase letter `Z`.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.3
    pub struct UtcDesignatorIsUppercaseZ;
    structural_prop!(UtcDesignatorIsUppercaseZ, "UtcDesignatorIsUppercaseZ");

    /// UTC is the reference time scale used across UTC-related representations.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.12
    pub struct UtcIsReferenceTimeScale;
    structural_prop!(UtcIsReferenceTimeScale, "UtcIsReferenceTimeScale");

    /// UTC of day is time of day in UTC.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.13
    pub struct UtcOfDayIdentifiesTimeWithinUtcCalendarDay;
    structural_prop!(
        UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
        "UtcOfDayIdentifiesTimeWithinUtcCalendarDay"
    );

    /// A UTC-of-day representation uses a local-time representation followed immediately by `Z`.
    ///
    /// Normative source: ISO 8601-1:2019 Clause 4.2.4 UTC of day
    pub struct UtcOfDayUsesTrailingZuluDesignatorImmediately;
    structural_prop!(
        UtcOfDayUsesTrailingZuluDesignatorImmediately,
        "UtcOfDayUsesTrailingZuluDesignatorImmediately"
    );

    /// A numeric UTC offset carries a sign, an hour component, and an optional minute component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1
    pub struct UtcOffsetCarriesSignHourAndOptionalMinute;
    structural_prop!(
        UtcOffsetCarriesSignHourAndOptionalMinute,
        "UtcOffsetCarriesSignHourAndOptionalMinute"
    );

    /// A UTC-offset hour is in the range 00 through 23.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1.
    pub struct UtcOffsetHourInRangeZeroToTwentyThree;
    structural_prop!(
        UtcOffsetHourInRangeZeroToTwentyThree,
        "UtcOffsetHourInRangeZeroToTwentyThree"
    );

    /// A UTC-offset minute is in the range 00 through 59.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1.
    pub struct UtcOffsetMinuteInRangeZeroToFiftyNine;
    structural_prop!(
        UtcOffsetMinuteInRangeZeroToFiftyNine,
        "UtcOffsetMinuteInRangeZeroToFiftyNine"
    );

    /// Standard time is a time scale derived from UTC by a time shift established in a given location.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.14
    pub struct StandardTimeDerivedFromUtcByLocalShift;
    structural_prop!(
        StandardTimeDerivedFromUtcByLocalShift,
        "StandardTimeDerivedFromUtcByLocalShift"
    );

    /// Standard time of day marks an instant within a calendar day relative to local standard time.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.1.15
    pub struct StandardTimeOfDayUsesStandardTimeScale;
    structural_prop!(
        StandardTimeOfDayUsesStandardTimeScale,
        "StandardTimeOfDayUsesStandardTimeScale"
    );

    /// This Rust `LocalTime*` exchange family models ISO 8601-1:2019 local time of day.
    ///
    /// A local time of day is time of day in a local time scale.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.17
    pub struct LocalTimeUsesLocallyApplicableTimeScale;
    structural_prop!(
        LocalTimeUsesLocallyApplicableTimeScale,
        "LocalTimeUsesLocallyApplicableTimeScale"
    );

    /// A local time scale may be a standard time scale or another non-UTC-based local scale.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.15
    pub struct LocalTimeScaleMayBeStandardOrNonUtcBased;
    structural_prop!(
        LocalTimeScaleMayBeStandardOrNonUtcBased,
        "LocalTimeScaleMayBeStandardOrNonUtcBased"
    );

    /// A time shift is a constant duration difference between two time scales.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.25
    pub struct TimeShiftIsConstantDurationBetweenTimeScales;
    structural_prop!(
        TimeShiftIsConstantDurationBetweenTimeScales,
        "TimeShiftIsConstantDurationBetweenTimeScales"
    );

    /// A common year contains 365 calendar days.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.20.
    pub struct CommonYearHasThreeHundredSixtyFiveCalendarDays;
    structural_prop!(
        CommonYearHasThreeHundredSixtyFiveCalendarDays,
        "CommonYearHasThreeHundredSixtyFiveCalendarDays"
    );

    /// A leap year contains 366 calendar days.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.21.
    pub struct LeapYearHasThreeHundredSixtySixCalendarDays;
    structural_prop!(
        LeapYearHasThreeHundredSixtySixCalendarDays,
        "LeapYearHasThreeHundredSixtySixCalendarDays"
    );

    /// A centennial year is a year number divisible by 100.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.22.
    pub struct CentennialYearDivisibleByOneHundred;
    structural_prop!(
        CentennialYearDivisibleByOneHundred,
        "CentennialYearDivisibleByOneHundred"
    );

    /// A Gregorian leap year follows the divisible-by-four rule with the centennial divisible-by-four-hundred exception.
    ///
    /// Normative source: ISO 8601-1:2019, 3.1.1.21 note 1; ISO/WD 8601-1:2016(E), 3.2.1.
    pub struct GregorianLeapYearUsesDivisibleByFourAndFourHundredException;
    structural_prop!(
        GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
        "GregorianLeapYearUsesDivisibleByFourAndFourHundredException"
    );

    /// A non-expanded calendar year value lies within the range `0000` through `9999`.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.1.
    pub struct CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine;
    structural_prop!(
        CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
        "CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine"
    );

    /// Calendar-year values from `0000` through `1582` require mutual agreement in information interchange.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.1.2.1.
    pub struct CalendarYearThrough1582RequiresMutualAgreement;
    structural_prop!(
        CalendarYearThrough1582RequiresMutualAgreement,
        "CalendarYearThrough1582RequiresMutualAgreement"
    );

    /// Proleptic Gregorian dates before the 1582 introduction point require mutual agreement.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.2.1.
    pub struct ProlepticGregorianDatesBefore1583RequireMutualAgreement;
    structural_prop!(
        ProlepticGregorianDatesBefore1583RequireMutualAgreement,
        "ProlepticGregorianDatesBefore1583RequireMutualAgreement"
    );

    /// A complete representation carries all date and time components associated with the expression.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.5
    pub struct CompleteRepresentationCarriesAllRequiredComponents;
    structural_prop!(
        CompleteRepresentationCarriesAllRequiredComponents,
        "CompleteRepresentationCarriesAllRequiredComponents"
    );

    /// A date-time representation identifies a time point, time interval, or recurring time interval.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.1
    pub struct DateTimeRepresentationIdentifiesPointIntervalOrRecurrence;
    structural_prop!(
        DateTimeRepresentationIdentifiesPointIntervalOrRecurrence,
        "DateTimeRepresentationIdentifiesPointIntervalOrRecurrence"
    );

    /// A date-time format representation describes the format of a group of date-time representations.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.2
    pub struct DateTimeFormatRepresentationDescribesRepresentationFamily;
    structural_prop!(
        DateTimeFormatRepresentationDescribesRepresentationFamily,
        "DateTimeFormatRepresentationDescribesRepresentationFamily"
    );

    /// A basic format uses the minimum number of time elements necessary for the required accuracy.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.3
    pub struct BasicFormatUsesMinimumComponentsForRequiredAccuracy;
    structural_prop!(
        BasicFormatUsesMinimumComponentsForRequiredAccuracy,
        "BasicFormatUsesMinimumComponentsForRequiredAccuracy"
    );

    /// A decimal representation adds a decimal fraction to the lowest-order component of the expression.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.6
    pub struct DecimalRepresentationUsesLowestOrderComponentFraction;
    structural_prop!(
        DecimalRepresentationUsesLowestOrderComponentFraction,
        "DecimalRepresentationUsesLowestOrderComponentFraction"
    );

    /// A reduced-accuracy representation omits lower-order components.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.7
    pub struct ReducedAccuracyRepresentationOmitsLowerOrderComponents;
    structural_prop!(
        ReducedAccuracyRepresentationOmitsLowerOrderComponents,
        "ReducedAccuracyRepresentationOmitsLowerOrderComponents"
    );

    /// An expanded representation uses additional agreement to extend component width.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 2.3.8 and 3.5.
    pub struct ExpandedRepresentationRequiresAdditionalAgreement;
    structural_prop!(
        ExpandedRepresentationRequiresAdditionalAgreement,
        "ExpandedRepresentationRequiresAdditionalAgreement"
    );

    /// Fixed-width components use leading zeros as needed.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.6.
    pub struct FixedWidthComponentsRequireLeadingZeros;
    structural_prop!(
        FixedWidthComponentsRequireLeadingZeros,
        "FixedWidthComponentsRequireLeadingZeros"
    );

    /// Space is forbidden in representations unless the standard explicitly allows it.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.1.
    pub struct SpaceForbiddenUnlessExplicitlyPermitted;
    structural_prop!(
        SpaceForbiddenUnlessExplicitlyPermitted,
        "SpaceForbiddenUnlessExplicitlyPermitted"
    );

    /// Date-time format representations are not used in ITU-T S.1 repertoire environments.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.1.
    pub struct DateTimeFormatRepresentationsForbiddenInTelexRepertoire;
    structural_prop!(
        DateTimeFormatRepresentationsForbiddenInTelexRepertoire,
        "DateTimeFormatRepresentationsForbiddenInTelexRepertoire"
    );

    /// When underlining is unavailable, the underline marker precedes the format character it qualifies.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.1.
    pub struct UnderlineFallbackPrecedesQualifiedFormatCharacter;
    structural_prop!(
        UnderlineFallbackPrecedesQualifiedFormatCharacter,
        "UnderlineFallbackPrecedesQualifiedFormatCharacter"
    );

    /// Date-time format representations use placeholder characters to denote digits and signs.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.2.
    pub struct FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns;
    structural_prop!(
        FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns,
        "FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns"
    );

    /// Underlined digit placeholders denote zero or more digits in the corresponding representation.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.2.
    pub struct UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits;
    structural_prop!(
        UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits,
        "UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits"
    );

    /// Non-placeholder characters in a format representation are copied literally into the resulting representation.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.2.
    pub struct LiteralFormatCharactersCopyIntoRepresentations;
    structural_prop!(
        LiteralFormatCharactersCopyIntoRepresentations,
        "LiteralFormatCharactersCopyIntoRepresentations"
    );

    /// A week-date representation uses the `W` designator before the calendar-week component.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.3.
    pub struct WeekDateUsesWeekDesignator;
    structural_prop!(WeekDateUsesWeekDesignator, "WeekDateUsesWeekDesignator");

    /// Hyphen separators delimit adjacent date components in extended ISO 8601 forms.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.4.
    pub struct HyphenSeparatesDateComponents;
    structural_prop!(
        HyphenSeparatesDateComponents,
        "HyphenSeparatesDateComponents"
    );

    /// Colon separators delimit adjacent clock-time components in extended ISO 8601 forms.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 3.4.4.
    pub struct ColonSeparatesTimeComponents;
    structural_prop!(ColonSeparatesTimeComponents, "ColonSeparatesTimeComponents");

    /// A local-time expression uses the time designator when context does not otherwise disambiguate it.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.2.5.
    pub struct LocalTimeRequiresTimeDesignatorWhenContextAmbiguous;
    structural_prop!(
        LocalTimeRequiresTimeDesignatorWhenContextAmbiguous,
        "LocalTimeRequiresTimeDesignatorWhenContextAmbiguous"
    );

    /// A 24-hour end-of-day representation is allowed only within interval or recurrence contexts.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.3 note 2; ISO 8601-1:2019/Amd 1:2022, 5.3.2.
    pub struct EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence;
    structural_prop!(
        EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence,
        "EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence"
    );

    /// A 24-hour end-of-day representation shall not identify a single time point.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.3 note 3; ISO 8601-1:2019/Amd 1:2022, 5.3.2.
    pub struct EndOfDayTwentyFourHourForbiddenForSingleTimePoint;
    structural_prop!(
        EndOfDayTwentyFourHourForbiddenForSingleTimePoint,
        "EndOfDayTwentyFourHourForbiddenForSingleTimePoint"
    );

    /// UTC-difference minutes may be omitted only when the offset is an integral number of hours.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1.
    pub struct UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets;
    structural_prop!(
        UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
        "UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets"
    );

    /// A UTC-difference sign encodes whether local time is ahead of or behind UTC.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1.
    pub struct UtcDifferenceSignEncodesDirectionRelativeToUtc;
    structural_prop!(
        UtcDifferenceSignEncodesDirectionRelativeToUtc,
        "UtcDifferenceSignEncodesDirectionRelativeToUtc"
    );

    /// A UTC difference is appended to the local-time expression immediately without spaces.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.2.
    pub struct UtcDifferenceAppendedImmediatelyWithoutSpace;
    structural_prop!(
        UtcDifferenceAppendedImmediatelyWithoutSpace,
        "UtcDifferenceAppendedImmediatelyWithoutSpace"
    );

    /// A UTC-difference expression is not a self-standing representation.
    ///
    /// Normative source: ISO/WD 8601-1:2016(E), 4.2.5.1.
    pub struct UtcDifferenceExpressionIsNotSelfStanding;
    structural_prop!(
        UtcDifferenceExpressionIsNotSelfStanding,
        "UtcDifferenceExpressionIsNotSelfStanding"
    );
}

pub use emit_impls::{
    BasicFormatUsesMinimumComponentsForRequiredAccuracy, CalendarDateHasYearMonthDay,
    CalendarDateUsesGregorianCalendar,
    CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts,
    CalendarDayIsIntervalOfSingleCalendarDateAdvance, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CalendarMonthIsNamedIntervalWithinCalendarYear,
    CalendarWeekIsSevenDayIntervalBeginningOnMonday, CalendarWeekNumberUsesFirstThursdayRule,
    CalendarWeekStartsOnMonday, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CalendarYearIsIntervalOfSuccessiveCalendarMonths,
    CalendarYearThrough1582RequiresMutualAgreement, CentennialYearDivisibleByOneHundred,
    CenturyOrdinalInRangeZeroToNinetyNine, ColonSeparatesTimeComponents,
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    CombinedDateTimePermitsCalendarDateComponent, CombinedDateTimePermitsOrdinalDateComponent,
    CombinedDateTimePermitsWeekDateComponent,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, CommonYearHasThreeHundredSixtyFiveCalendarDays,
    CompleteRepresentationCarriesAllRequiredComponents, DateIdentifiesPositionWithinCalendar,
    DateTimeFormatRepresentationDescribesRepresentationFamily,
    DateTimeFormatRepresentationsForbiddenInTelexRepertoire,
    DateTimeRepresentationIdentifiesPointIntervalOrRecurrence,
    DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays, DayEqualsTwentyFourHours,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    DecimalRepresentationUsesLowestOrderComponentFraction,
    DurationEqualsDifferenceBetweenIntervalEndpoints,
    EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence,
    EndOfDayTwentyFourHourForbiddenForSingleTimePoint,
    ExpandedRepresentationRequiresAdditionalAgreement, FixedWidthComponentsRequireLeadingZeros,
    FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns,
    FractionAppliesToLowestOrderComponent, FractionUsesDecimalSign,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException, HourEqualsSixtyMinutes,
    HourInRangeZeroToTwentyFour, HyphenSeparatesDateComponents, InstantIsPointOnTimeAxis,
    LeapDayOccursOnlyInLeapYear, LeapSecondOccursOnlyAtUtcBoundary,
    LeapYearHasThreeHundredSixtySixCalendarDays, LiteralFormatCharactersCopyIntoRepresentations,
    LocalTimeHasHourMinuteSecond, LocalTimeRequiresTimeDesignatorWhenContextAmbiguous,
    LocalTimeScaleMayBeStandardOrNonUtcBased, LocalTimeUsesLocallyApplicableTimeScale,
    MinuteEqualsSixtySeconds, MinuteInRangeZeroToFiftyNine,
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    MonthDurationMayRequireAgreedEndingCalendarDay,
    MonthMayBeConsideredThirtyCalendarDaysInCertainApplications,
    NominalDayDurationMayDifferFromExactElapsedTime, NominalDurationDependsOnCalendarContext,
    NominalMonthDurationDependsOnCalendarContext,
    NominalWeekDurationIsDistinctFromExactElapsedTime, NominalYearDurationDependsOnCalendarContext,
    OrdinalDateHasYearAndDayOfYear, OrdinalDayInRangeOneToThreeHundredSixtySix,
    OrdinalDayOfYearUsesThreeDigits, ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
    ReducedAccuracyRepresentationOmitsLowerOrderComponents,
    ReducedAccuracyWeekDateOmitsWeekdayComponent, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    SecondInRangeZeroToSixty, SecondIsBaseUnitForExpressingDuration, SecondIsSiBaseUnitOfTime,
    SpaceForbiddenUnlessExplicitlyPermitted, StandardTimeDerivedFromUtcByLocalShift,
    StandardTimeOfDayUsesStandardTimeScale, TimeAxisOrdersTimePointsByTemporalPosition,
    TimeIsMarkOnSpecifiedTimeScale, TimeOfDayOccursWithinCalendarDay,
    TimeScaleAssociatesTimePointsWithOrderedMeasure, TimeShiftIsConstantDurationBetweenTimeScales,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UnderlineFallbackPrecedesQualifiedFormatCharacter,
    UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits, UtcDesignatorIsUppercaseZ,
    UtcDifferenceAppendedImmediatelyWithoutSpace, UtcDifferenceExpressionIsNotSelfStanding,
    UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    UtcDifferenceSignEncodesDirectionRelativeToUtc, UtcIsReferenceTimeScale,
    UtcOfDayIdentifiesTimeWithinUtcCalendarDay, UtcOfDayUsesTrailingZuluDesignatorImmediately,
    UtcOffsetCarriesSignHourAndOptionalMinute, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine, WeekDateHasWeekYearWeekAndWeekday,
    WeekDateUsesWeekDesignator, WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    YearDurationMayRequireAgreedEndingCalendarDate,
};
