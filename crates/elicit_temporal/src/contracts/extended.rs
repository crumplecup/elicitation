//! ISO 8601-2 extended temporal propositions.
//!
//! Normative source:
//! - ISO 8601-2:2019, *Date and time - Representations for information interchange - Part 2: Extensions*
//! Informative cross-check:
//! - Library of Congress EDTF profile, public cross-check for Level 1/Level 2
//!   headings such as `Letter-prefixed calendar year`,
//!   `Negative calendar year`, `Exponential year`, `Significant digits`,
//!   `Seasons`, `Qualification of a date (complete)`, `Qualification`,
//!   `Unspecified digit(s) from the right`, `Unspecified Digit`,
//!   `Extended Interval`, and `Set representation`
//!
//! Citation fidelity:
//! - `Normative source:` lines name the governing ISO 8601-2 concept.
//! - `Informative cross-check:` lines point at the concrete public EDTF heading
//!   that mirrors that concept.
//! - Exact ISO clause numbers still await the licensed-text extraction pass;
//!   public EDTF headings are alignment evidence, not substitutes for the ISO
//!   text.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601-2 extension contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601-2 extension contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — ISO 8601-2 extension contract */ }
                }
            }
        };
    }

    /// A temporal expression explicitly declares uncertainty qualification.
    ///
    /// Normative source: ISO 8601-2:2019 — uncertain temporal expressions
    /// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete)
    pub struct UncertaintyQualificationDeclared;
    structural_prop!(
        UncertaintyQualificationDeclared,
        "UncertaintyQualificationDeclared"
    );

    /// A temporal expression explicitly declares approximation qualification.
    ///
    /// Normative source: ISO 8601-2:2019 — approximate temporal expressions
    /// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete)
    pub struct ApproximationQualificationDeclared;
    structural_prop!(
        ApproximationQualificationDeclared,
        "ApproximationQualificationDeclared"
    );

    /// Uncertainty and approximation may be combined on the same temporal expression.
    ///
    /// Normative source: ISO 8601-2:2019 — combined uncertain and approximate qualification
    /// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete)
    pub struct UncertaintyAndApproximationMayBeCombined;
    structural_prop!(
        UncertaintyAndApproximationMayBeCombined,
        "UncertaintyAndApproximationMayBeCombined"
    );

    /// A qualification may apply either to a whole temporal expression or to a component.
    ///
    /// Normative source: ISO 8601-2:2019 — qualification scope
    /// Informative cross-check: public LOC EDTF Level 2 — Qualification
    pub struct QualificationScopeDeclared;
    structural_prop!(QualificationScopeDeclared, "QualificationScopeDeclared");

    /// A group qualification marker appears immediately to the right of the component it qualifies from.
    ///
    /// Normative source: ISO 8601-2:2019 — group qualification
    /// Informative cross-check: public LOC EDTF Level 2 — Qualification
    pub struct GroupQualificationUsesImmediateRightPlacement;
    structural_prop!(
        GroupQualificationUsesImmediateRightPlacement,
        "GroupQualificationUsesImmediateRightPlacement"
    );

    /// A right-placed group qualification applies to the marked component and all components to its left.
    ///
    /// Normative source: ISO 8601-2:2019 — group qualification
    /// Informative cross-check: public LOC EDTF Level 2 — Qualification
    pub struct GroupQualificationAppliesToMarkedAndMoreSignificantComponents;
    structural_prop!(
        GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
        "GroupQualificationAppliesToMarkedAndMoreSignificantComponents"
    );

    /// An individual-component qualification marker appears immediately to the left of the component it qualifies.
    ///
    /// Normative source: ISO 8601-2:2019 — qualification of individual component
    /// Informative cross-check: public LOC EDTF Level 2 — Qualification
    pub struct ComponentQualificationUsesImmediateLeftPlacement;
    structural_prop!(
        ComponentQualificationUsesImmediateLeftPlacement,
        "ComponentQualificationUsesImmediateLeftPlacement"
    );

    /// A left-placed individual-component qualification applies only to the immediately following component.
    ///
    /// Normative source: ISO 8601-2:2019 — qualification of individual component
    /// Informative cross-check: public LOC EDTF Level 2 — Qualification
    pub struct ComponentQualificationAppliesOnlyToMarkedComponent;
    structural_prop!(
        ComponentQualificationAppliesOnlyToMarkedComponent,
        "ComponentQualificationAppliesOnlyToMarkedComponent"
    );

    /// An interval explicitly declares an open boundary.
    ///
    /// Normative source: ISO 8601-2:2019 — open interval boundaries
    /// Informative cross-check: public LOC EDTF Level 1 — Extended Interval
    pub struct OpenIntervalBoundaryDeclared;
    structural_prop!(OpenIntervalBoundaryDeclared, "OpenIntervalBoundaryDeclared");

    /// An interval explicitly declares an unknown boundary.
    ///
    /// Normative source: ISO 8601-2:2019 — unknown interval boundaries
    /// Informative cross-check: public LOC EDTF Level 1 — Extended Interval
    pub struct UnknownIntervalBoundaryDeclared;
    structural_prop!(
        UnknownIntervalBoundaryDeclared,
        "UnknownIntervalBoundaryDeclared"
    );

    /// A letter-prefixed calendar year uses a leading uppercase `Y` designator.
    ///
    /// Normative source: ISO 8601-2:2019 — letter-prefixed calendar year
    /// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year
    pub struct LetterPrefixedCalendarYearUsesLeadingYDesignator;
    structural_prop!(
        LetterPrefixedCalendarYearUsesLeadingYDesignator,
        "LetterPrefixedCalendarYearUsesLeadingYDesignator"
    );

    /// A letter-prefixed calendar year is used only when the absolute year magnitude exceeds four digits.
    ///
    /// Normative source: ISO 8601-2:2019 — letter-prefixed calendar year
    /// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year
    pub struct LetterPrefixedCalendarYearMagnitudeExceedsFourDigits;
    structural_prop!(
        LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
        "LetterPrefixedCalendarYearMagnitudeExceedsFourDigits"
    );

    /// A negative calendar year uses an explicit leading minus sign.
    ///
    /// Normative source: ISO 8601-2:2019 — negative calendar year
    /// Informative cross-check: public LOC EDTF Level 1 — Negative calendar year
    pub struct NegativeCalendarYearUsesLeadingMinusSign;
    structural_prop!(
        NegativeCalendarYearUsesLeadingMinusSign,
        "NegativeCalendarYearUsesLeadingMinusSign"
    );

    /// A value before year one uses a trailing uppercase `B` suffix.
    ///
    /// Normative source: ISO 8601-2:2019 — before-year-one suffix for calendar year, decade, and century
    /// Informative cross-check: CalConnect CC 18011:2018 — suffix designator to represent years before year one
    pub struct BeforeYearOneValueUsesTrailingBSuffix;
    structural_prop!(
        BeforeYearOneValueUsesTrailingBSuffix,
        "BeforeYearOneValueUsesTrailingBSuffix"
    );

    /// An exponential year uses `E` power-of-ten notation after the signed significand.
    ///
    /// Normative source: ISO 8601-2:2019 — exponential year
    /// Informative cross-check: public LOC EDTF Level 2 — Exponential year
    pub struct ExponentialYearUsesPowerOfTenNotation;
    structural_prop!(
        ExponentialYearUsesPowerOfTenNotation,
        "ExponentialYearUsesPowerOfTenNotation"
    );

    /// An exponential year uses a positive integer exponent.
    ///
    /// Normative source: ISO 8601-2:2019 — exponential year
    /// Informative cross-check: public LOC EDTF Level 2 — Exponential year
    pub struct ExponentialYearExponentIsPositiveInteger;
    structural_prop!(
        ExponentialYearExponentIsPositiveInteger,
        "ExponentialYearExponentIsPositiveInteger"
    );

    /// A significant-digit year uses a trailing uppercase `S` suffix.
    ///
    /// Normative source: ISO 8601-2:2019 — significant digits
    /// Informative cross-check: public LOC EDTF Level 2 — Significant digits
    pub struct SignificantDigitYearUsesTrailingSSuffix;
    structural_prop!(
        SignificantDigitYearUsesTrailingSSuffix,
        "SignificantDigitYearUsesTrailingSSuffix"
    );

    /// A significant-digit year declares a positive significant-digit count.
    ///
    /// Normative source: ISO 8601-2:2019 — significant digits
    /// Informative cross-check: public LOC EDTF Level 2 — Significant digits
    pub struct SignificantDigitYearCountIsPositiveInteger;
    structural_prop!(
        SignificantDigitYearCountIsPositiveInteger,
        "SignificantDigitYearCountIsPositiveInteger"
    );

    /// A seasonal temporal expression uses a year-and-season form.
    ///
    /// Normative source: ISO 8601-2:2019 — seasons and seasonal temporal expressions
    /// Informative cross-check: public LOC EDTF Level 1 — Seasons
    pub struct SeasonalExpressionUsesYearAndSeasonForm;
    structural_prop!(
        SeasonalExpressionUsesYearAndSeasonForm,
        "SeasonalExpressionUsesYearAndSeasonForm"
    );

    /// A seasonal expression places its season code in the month slot of a year-month form.
    ///
    /// Normative source: ISO 8601-2:2019 — seasons and seasonal temporal expressions
    /// Informative cross-check: public LOC EDTF Level 1 — Seasons
    ///
    /// Informative profile note: the Library of Congress EDTF Level 1/2 profile
    /// exposes this as a year-month-shaped string with seasonal codes occupying
    /// the month position.
    pub struct SeasonalExpressionUsesSeasonCodeInMonthSlot;
    structural_prop!(
        SeasonalExpressionUsesSeasonCodeInMonthSlot,
        "SeasonalExpressionUsesSeasonCodeInMonthSlot"
    );

    /// A season code declares named seasonal semantics such as spring or winter.
    ///
    /// Normative source: ISO 8601-2:2019 — named seasonal temporal expressions
    /// Informative cross-check: public LOC EDTF Level 1 — Seasons; Level 2 — Sub-year groupings
    pub struct SeasonCodeDeclaresNamedSeason;
    structural_prop!(
        SeasonCodeDeclaresNamedSeason,
        "SeasonCodeDeclaresNamedSeason"
    );

    /// A season code declares whether the named season is location-independent or hemisphere-qualified.
    ///
    /// Normative source: ISO 8601-2:2019 — named seasonal temporal expressions
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    ///
    /// Informative profile note: the Library of Congress EDTF Level 2 profile
    /// distinguishes location-independent seasons from northern- and
    /// southern-hemisphere seasonal codes.
    pub struct SeasonCodeDeclaresSeasonScope;
    structural_prop!(
        SeasonCodeDeclaresSeasonScope,
        "SeasonCodeDeclaresSeasonScope"
    );

    /// A Level 2 sub-year grouping uses a year-and-grouping form.
    ///
    /// Normative source: ISO 8601-2:2019 — sub-year groupings
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    pub struct SubYearGroupingExpressionUsesYearAndGroupingForm;
    structural_prop!(
        SubYearGroupingExpressionUsesYearAndGroupingForm,
        "SubYearGroupingExpressionUsesYearAndGroupingForm"
    );

    /// A Level 2 sub-year grouping places its grouping code in the month slot of a year-month-shaped form.
    ///
    /// Normative source: ISO 8601-2:2019 — sub-year groupings
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    pub struct SubYearGroupingExpressionUsesGroupingCodeInMonthSlot;
    structural_prop!(
        SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
        "SubYearGroupingExpressionUsesGroupingCodeInMonthSlot"
    );

    /// A sub-year grouping code may declare quarter semantics.
    ///
    /// Normative source: ISO 8601-2:2019 — sub-year groupings
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    pub struct SubYearGroupingCodeDeclaresQuarter;
    structural_prop!(
        SubYearGroupingCodeDeclaresQuarter,
        "SubYearGroupingCodeDeclaresQuarter"
    );

    /// A sub-year grouping code may declare quadrimester semantics.
    ///
    /// Normative source: ISO 8601-2:2019 — sub-year groupings
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    pub struct SubYearGroupingCodeDeclaresQuadrimester;
    structural_prop!(
        SubYearGroupingCodeDeclaresQuadrimester,
        "SubYearGroupingCodeDeclaresQuadrimester"
    );

    /// A sub-year grouping code may declare semestral semantics.
    ///
    /// Normative source: ISO 8601-2:2019 — sub-year groupings
    /// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
    pub struct SubYearGroupingCodeDeclaresSemestral;
    structural_prop!(
        SubYearGroupingCodeDeclaresSemestral,
        "SubYearGroupingCodeDeclaresSemestral"
    );

    /// An unspecified digit uses the uppercase `X` placeholder.
    ///
    /// Normative source: ISO 8601-2:2019 — unspecified digits and unspecified components
    /// Informative cross-check: public LOC EDTF Level 1 — Unspecified digit(s) from the right;
    /// Level 2 — Unspecified Digit
    ///
    /// Informative profile note: the public Library of Congress EDTF profile
    /// records the transition from lowercase `u` to uppercase `X`.
    pub struct UnspecifiedDigitUsesUppercaseXPlaceholder;
    structural_prop!(
        UnspecifiedDigitUsesUppercaseXPlaceholder,
        "UnspecifiedDigitUsesUppercaseXPlaceholder"
    );

    /// An `X` placeholder declares that the corresponding digit or component value is unspecified.
    ///
    /// Normative source: ISO 8601-2:2019 — unspecified digits and unspecified components
    /// Informative cross-check: public LOC EDTF Level 1 — Unspecified digit(s) from the right;
    /// Level 2 — Unspecified Digit
    pub struct UnspecifiedDigitsDeclareUnknownValue;
    structural_prop!(
        UnspecifiedDigitsDeclareUnknownValue,
        "UnspecifiedDigitsDeclareUnknownValue"
    );

    /// Level 1 unspecified digits occupy one or more rightmost positions.
    ///
    /// Normative source: ISO 8601-2:2019 — unspecified digits and unspecified components
    /// Informative cross-check: public LOC EDTF Level 1 — Unspecified digit(s) from the right
    ///
    /// Informative profile note: the public Library of Congress EDTF Level 1
    /// profile describes rightmost-digit masking and whole-component
    /// placeholders such as `2004-XX` and `1985-04-XX`.
    pub struct LevelOneUnspecifiedDigitsOccupyRightmostPositions;
    structural_prop!(
        LevelOneUnspecifiedDigitsOccupyRightmostPositions,
        "LevelOneUnspecifiedDigitsOccupyRightmostPositions"
    );

    /// Level 2 unspecified digits may appear anywhere within a component.
    ///
    /// Normative source: ISO 8601-2:2019 — unspecified digits and unspecified components
    /// Informative cross-check: public LOC EDTF Level 2 — Unspecified Digit
    ///
    /// Informative profile note: the public Library of Congress EDTF Level 2
    /// profile extends `X` placement beyond the Level 1 rightmost-only rule.
    pub struct LevelTwoUnspecifiedDigitsMayAppearWithinComponent;
    structural_prop!(
        LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
        "LevelTwoUnspecifiedDigitsMayAppearWithinComponent"
    );

    /// A temporal set representation explicitly separates member expressions.
    ///
    /// Normative source: ISO 8601-2:2019 — temporal sets
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetMemberSeparatorDeclared;
    structural_prop!(
        TemporalSetMemberSeparatorDeclared,
        "TemporalSetMemberSeparatorDeclared"
    );

    /// A temporal set carries multiple member expressions.
    ///
    /// Normative source: ISO 8601-2:2019 — temporal sets
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetCarriesMultipleMembers;
    structural_prop!(
        TemporalSetCarriesMultipleMembers,
        "TemporalSetCarriesMultipleMembers"
    );

    /// A temporal set declares one-of semantics rather than a single fixed value.
    ///
    /// Normative source: ISO 8601-2:2019 — set and choice semantics
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetDeclaresAlternativeSemantics;
    structural_prop!(
        TemporalSetDeclaresAlternativeSemantics,
        "TemporalSetDeclaresAlternativeSemantics"
    );

    /// A single-choice temporal set uses square brackets.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalChoiceSetUsesSquareBrackets;
    structural_prop!(
        TemporalChoiceSetUsesSquareBrackets,
        "TemporalChoiceSetUsesSquareBrackets"
    );

    /// An inclusive temporal set uses curly braces.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalInclusiveSetUsesCurlyBraces;
    structural_prop!(
        TemporalInclusiveSetUsesCurlyBraces,
        "TemporalInclusiveSetUsesCurlyBraces"
    );

    /// An inclusive temporal set declares all-members semantics.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetDeclaresInclusiveMemberSemantics;
    structural_prop!(
        TemporalSetDeclaresInclusiveMemberSemantics,
        "TemporalSetDeclaresInclusiveMemberSemantics"
    );

    /// A temporal set expression forbids internal whitespace.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetForbidsInternalWhitespace;
    structural_prop!(
        TemporalSetForbidsInternalWhitespace,
        "TemporalSetForbidsInternalWhitespace"
    );

    /// A temporal set range uses `..` to denote the inclusive values between its bounds.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetRangeUsesInclusiveDoubleDotSemantics;
    structural_prop!(
        TemporalSetRangeUsesInclusiveDoubleDotSemantics,
        "TemporalSetRangeUsesInclusiveDoubleDotSemantics"
    );

    /// A leading or trailing `..` denotes an open-ended temporal set boundary.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetOpenRangeUsesBoundaryDoubleDot;
    structural_prop!(
        TemporalSetOpenRangeUsesBoundaryDoubleDot,
        "TemporalSetOpenRangeUsesBoundaryDoubleDot"
    );

    /// Elements adjacent to a `..` range share the same precision as the values denoted by that range.
    ///
    /// Normative source: ISO 8601-2:2019 — set representation
    /// Informative cross-check: public LOC EDTF Level 2 — Set representation
    pub struct TemporalSetRangeNeighborhoodSharesPrecision;
    structural_prop!(
        TemporalSetRangeNeighborhoodSharesPrecision,
        "TemporalSetRangeNeighborhoodSharesPrecision"
    );
}

pub use emit_impls::{
    ApproximationQualificationDeclared, BeforeYearOneValueUsesTrailingBSuffix,
    ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement,
    LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, NegativeCalendarYearUsesLeadingMinusSign,
    OpenIntervalBoundaryDeclared, QualificationScopeDeclared, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SignificantDigitYearCountIsPositiveInteger,
    SignificantDigitYearUsesTrailingSSuffix, SubYearGroupingCodeDeclaresQuadrimester,
    SubYearGroupingCodeDeclaresQuarter, SubYearGroupingCodeDeclaresSemestral,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, TemporalChoiceSetUsesSquareBrackets,
    TemporalInclusiveSetUsesCurlyBraces, TemporalSetCarriesMultipleMembers,
    TemporalSetDeclaresAlternativeSemantics, TemporalSetDeclaresInclusiveMemberSemantics,
    TemporalSetForbidsInternalWhitespace, TemporalSetMemberSeparatorDeclared,
    TemporalSetOpenRangeUsesBoundaryDoubleDot, TemporalSetRangeNeighborhoodSharesPrecision,
    TemporalSetRangeUsesInclusiveDoubleDotSemantics, UncertaintyAndApproximationMayBeCombined,
    UncertaintyQualificationDeclared, UnknownIntervalBoundaryDeclared,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};
