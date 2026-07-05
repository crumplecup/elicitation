# Citation Worksheet: `src/contracts/extended.rs`

This worksheet tracks exact ISO 8601-2 citations for the extended temporal
contracts in `src/contracts/extended.rs`.

## Source set

- `../iso-8601-2-2019.*`
- `../../public/iso-wd-8601-2-2016.txt`
- `../../public/loc-edtf-2019.html`

## ISO contract map

| Contract symbol | Current topic label | Exact clause | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `UncertaintyQualificationDeclared` | `uncertain temporal expressions` | `4.5; 8.2.1; 8.5` | `iso-8601-2-2019.sample.txt` lines 132, 275-345; `loc-edtf-2019.html` lines 141-149 | `yes` |
| `ApproximationQualificationDeclared` | `approximate temporal expressions` | `4.5; 8.2.1; 8.5` | `iso-8601-2-2019.sample.txt` lines 132, 275-345; `loc-edtf-2019.html` lines 141-149 | `yes` |
| `UncertaintyAndApproximationMayBeCombined` | `combined uncertain and approximate qualification` | `4.5; 8.2.1; 8.5` | `iso-8601-2-2019.sample.txt` lines 132, 275-345; `loc-edtf-2019.html` lines 141-149 | `yes` |
| `QualificationScopeDeclared` | `qualification scope` | `4.5; 8.2.2; 8.2.3` | `iso-8601-2-2019.sample.txt` lines 132, 279-283; `loc-edtf-2019.html` lines 307-321 | `yes` |
| `GroupQualificationUsesImmediateRightPlacement` | `group qualification` | `4.5; 8.2.2; 8.4.4` | `iso-8601-2-2019.sample.txt` lines 132, 279-283, 340; `loc-edtf-2019.html` lines 307-316 | `yes` |
| `GroupQualificationAppliesToMarkedAndMoreSignificantComponents` | `group qualification` | `4.5; 8.2.2; 8.4.4` | `iso-8601-2-2019.sample.txt` lines 132, 279-283, 340; `loc-edtf-2019.html` lines 307-316 | `yes` |
| `ComponentQualificationUsesImmediateLeftPlacement` | `qualification of individual component` | `4.5; 8.2.3; 8.4.5` | `iso-8601-2-2019.sample.txt` lines 132, 281-283, 342; `loc-edtf-2019.html` lines 318-321 | `yes` |
| `ComponentQualificationAppliesOnlyToMarkedComponent` | `qualification of individual component` | `4.5; 8.2.3; 8.4.5` | `iso-8601-2-2019.sample.txt` lines 132, 281-283, 342; `loc-edtf-2019.html` lines 318-321 | `yes` |
| `BeforeOrAfterQualificationIsLevelTwoOnly` | `before or after` | `4.4.1; 4.4.2` | `../../public/iso-wd-8601-2-2016.txt` lines 407-422 | `yes` |
| `BeforeOrOnDateUsesLeadingDoubleDotQualifier` | `before or on` | `4.4.2` | `../../public/iso-wd-8601-2-2016.txt` lines 414-422 | `yes` |
| `OnOrAfterDateUsesTrailingDoubleDotQualifier` | `on or after` | `4.4.2` | `../../public/iso-wd-8601-2-2016.txt` lines 414-422 | `yes` |
| `EnhancedIntervalLevelOnePermitsTerminalBoundaryQualification` | `enhanced interval Level 1 terminal boundary qualification` | `4.5.1` | `../../public/iso-wd-8601-2-2016.txt` lines 446-447 | `yes` |
| `EnhancedIntervalLevelTwoPermitsInternalBoundaryQualification` | `enhanced interval Level 2 internal boundary qualification` | `4.5.2` | `../../public/iso-wd-8601-2-2016.txt` lines 474-475 | `yes` |
| `EnhancedIntervalLevelTwoPermitsInternalBoundaryUnspecifiedDigits` | `enhanced interval Level 2 internal boundary unspecified digits` | `4.5.2` | `../../public/iso-wd-8601-2-2016.txt` lines 474-475 | `yes` |
| `EnhancedIntervalLevelTwoPermitsBeforeOrAfterBoundaryQualification` | `enhanced interval Level 2 before-or/or-after boundary qualification` | `4.5.2` | `../../public/iso-wd-8601-2-2016.txt` lines 474-476 | `yes` |
| `OpenIntervalBoundaryDeclared` | `open interval boundaries` | `10.2` | `iso-8601-2-2019.sample.txt` line 360; `loc-edtf-2019.html` lines 166-176 | `yes` |
| `UnknownIntervalBoundaryDeclared` | `unknown interval boundaries` | `10.2` | `iso-8601-2-2019.sample.txt` line 360; `loc-edtf-2019.html` lines 166-176 | `yes` |
| `LetterPrefixedCalendarYearUsesLeadingYDesignator` | `letter-prefixed calendar year` | `4.7.2` | `iso-8601-2-2019.sample.txt` lines 146, 961; `loc-edtf-2019.html` lines 202-208 | `yes` |
| `LetterPrefixedCalendarYearMagnitudeExceedsFourDigits` | `letter-prefixed calendar year` | `4.7.2` | `iso-8601-2-2019.sample.txt` lines 146, 961; `iso-wd-8601-1-2016.txt` lines 1034-1036; `loc-edtf-2019.html` lines 202-208 | `yes` |
| `NegativeCalendarYearUsesLeadingMinusSign` | `negative calendar year` | `4.4.1` | `iso-8601-2-2019.sample.txt` lines 126, 974, 1013-1017; `loc-edtf-2019.html` lines 210-216 | `yes` |
| `BeforeYearOneValueUsesTrailingBSuffix` | `before-year-one suffix for calendar year, decade, and century` | `3.2.5; 4.4.1` | `iso-8601-2-2019.sample.txt` lines 1034-1035, 126, 974 | `yes` |
| `ExponentialYearUsesPowerOfTenNotation` | `exponential year` | `4.4.2; 4.7.3` | `iso-8601-2-2019.sample.txt` lines 128, 148, 972, 1036-1038; `loc-edtf-2019.html` lines 218-225 | `yes` |
| `ExponentialYearExponentIsPositiveInteger` | `exponential year` | `3.2.4; 4.4.2; 4.7.3` | `iso-8601-2-2019.sample.txt` lines 1013-1017, 128, 148, 972, 1036-1038; `loc-edtf-2019.html` lines 218-225 | `yes` |
| `SignificantDigitYearUsesTrailingSSuffix` | `significant digits` | `3.2.5; 4.4.3` | `iso-8601-2-2019.sample.txt` lines 130, 965, 1039; `loc-edtf-2019.html` lines 228-237 | `yes` |
| `SignificantDigitYearCountIsPositiveInteger` | `significant digits` | `3.2.4; 4.4.3` | `iso-8601-2-2019.sample.txt` lines 1013-1014, 130, 965, 1039; `loc-edtf-2019.html` lines 228-237 | `yes` |
| `SeasonalExpressionUsesYearAndSeasonForm` | `seasons and seasonal temporal expressions` | `4.8.1; 4.8.3` | `iso-8601-2-2019.sample.txt` lines 154-158, 864-867; `loc-edtf-2019.html` lines 239-259 | `yes` |
| `SeasonalExpressionUsesSeasonCodeInMonthSlot` | `seasons and seasonal temporal expressions` | `4.8.1; 4.8.3` | `iso-8601-2-2019.sample.txt` lines 154-158, 864-867; `loc-edtf-2019.html` lines 239-259 | `yes` |
| `SeasonCodeDeclaresNamedSeason` | `named seasonal temporal expressions` | `4.8.1` | `iso-8601-2-2019.sample.txt` lines 154-155, 873-875; `loc-edtf-2019.html` lines 239-253 | `yes` |
| `SeasonCodeDeclaresSeasonScope` | `named seasonal temporal expressions` | `4.8.1` | `iso-8601-2-2019.sample.txt` lines 154-155, 873-875; `loc-edtf-2019.html` lines 239-253 | `yes` |
| `SubYearGroupingExpressionUsesYearAndGroupingForm` | `sub-year groupings` | `4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt` lines 156-158, 864-867; `loc-edtf-2019.html` lines 239-259 | `yes` |
| `SubYearGroupingExpressionUsesGroupingCodeInMonthSlot` | `sub-year groupings` | `4.8.2; 4.8.3` | `iso-8601-2-2019.sample.txt` lines 156-158, 864-867; `loc-edtf-2019.html` lines 239-259 | `yes` |
| `SubYearGroupingCodeDeclaresQuarter` | `sub-year groupings` | `4.8.1; 4.8.2` | `iso-8601-2-2019.sample.txt` lines 154-158, 873-875; `loc-edtf-2019.html` lines 254-259 | `yes` |
| `SubYearGroupingCodeDeclaresQuadrimester` | `sub-year groupings` | `4.8.1; 4.8.2` | `iso-8601-2-2019.sample.txt` lines 154-158, 873-875; `loc-edtf-2019.html` lines 254-259 | `yes` |
| `SubYearGroupingCodeDeclaresSemestral` | `sub-year groupings` | `4.8.1; 4.8.2` | `iso-8601-2-2019.sample.txt` lines 154-158, 873-875; `loc-edtf-2019.html` lines 254-259 | `yes` |
| `UnspecifiedDigitUsesUppercaseXPlaceholder` | `unspecified digits and unspecified components` | `4.6.1; 4.6.2; 4.6.3; 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt` lines 136-140, 351-357; `loc-edtf-2019.html` lines 152-161, 322-333 | `yes` |
| `UnspecifiedDigitsDeclareUnknownValue` | `unspecified digits and unspecified components` | `4.6.1; 4.6.2; 4.6.3; 9.2.1; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt` lines 136-140, 351-357; `loc-edtf-2019.html` lines 152-161, 322-333 | `yes` |
| `LevelOneUnspecifiedDigitsOccupyRightmostPositions` | `unspecified digits and unspecified components` | `4.6.2; 9.2.1; 9.3` | `iso-8601-2-2019.sample.txt` lines 138, 351-357; `loc-edtf-2019.html` lines 152-161 | `yes` |
| `LevelTwoUnspecifiedDigitsMayAppearWithinComponent` | `unspecified digits and unspecified components` | `4.6.3; 9.2.2; 9.3` | `iso-8601-2-2019.sample.txt` lines 140, 351-357; `loc-edtf-2019.html` lines 322-333 | `yes` |
| `TemporalSetMemberSeparatorDeclared` | `temporal sets` | `6.1; 6.4` | `iso-8601-2-2019.sample.txt` lines 202-208; `loc-edtf-2019.html` lines 266-272 | `yes` |
| `TemporalSetCarriesMultipleMembers` | `temporal sets` | `6.1` | `iso-8601-2-2019.sample.txt` lines 202-203; `loc-edtf-2019.html` lines 266-270 | `yes` |
| `TemporalSetDeclaresAlternativeSemantics` | `set and choice semantics` | `6.2` | `iso-8601-2-2019.sample.txt` lines 204-205; `loc-edtf-2019.html` lines 266-279 | `yes` |
| `TemporalChoiceSetUsesSquareBrackets` | `set representation` | `6.2` | `iso-8601-2-2019.sample.txt` lines 204-205; `loc-edtf-2019.html` lines 266-279 | `yes` |
| `TemporalInclusiveSetUsesCurlyBraces` | `set representation` | `6.1; 6.4` | `iso-8601-2-2019.sample.txt` lines 202-208; `loc-edtf-2019.html` lines 267-270, 295-304 | `yes` |
| `TemporalSetDeclaresInclusiveMemberSemantics` | `set representation` | `6.1; 6.4` | `iso-8601-2-2019.sample.txt` lines 202-208; `loc-edtf-2019.html` lines 267-270, 295-304 | `yes` |
| `TemporalSetForbidsInternalWhitespace` | `set representation` | `6.4` | `iso-8601-2-2019.sample.txt` lines 208-209; `loc-edtf-2019.html` lines 270-271 | `yes` |
| `TemporalSetRangeUsesInclusiveDoubleDotSemantics` | `set representation` | `6.3; 6.4` | `iso-8601-2-2019.sample.txt` lines 206-209; `loc-edtf-2019.html` lines 271-279 | `yes` |
| `TemporalSetOpenRangeUsesBoundaryDoubleDot` | `set representation` | `6.3` | `iso-8601-2-2019.sample.txt` lines 206-207; `loc-edtf-2019.html` lines 272-279 | `yes` |
| `TemporalSetRangeNeighborhoodSharesPrecision` | `set representation` | `6.4; 6.5` | `iso-8601-2-2019.sample.txt` lines 208-210; `loc-edtf-2019.html` lines 272-273 | `yes` |

## ISO WD 8601-2 Clause Coverage Audit

| Clause concept | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `4.2 Uncertain and/or approximate date` | `covered` | `UncertaintyQualificationDeclared`, `ApproximationQualificationDeclared`, `UncertaintyAndApproximationMayBeCombined` | Qualification presence and composition are explicit. |
| `4.3 Unspecified` | `covered` | `UnspecifiedDigitUsesUppercaseXPlaceholder`, `UnspecifiedDigitsDeclareUnknownValue`, `LevelOneUnspecifiedDigitsOccupyRightmostPositions`, `LevelTwoUnspecifiedDigitsMayAppearWithinComponent` | Level split and placement rules are explicit. |
| `4.4 Before or after` | `covered` | `BeforeOrAfterQualificationIsLevelTwoOnly`, `BeforeOrOnDateUsesLeadingDoubleDotQualifier`, `OnOrAfterDateUsesTrailingDoubleDotQualifier` | Single-date before-or-on and on-or-after semantics are now first-class. |
| `4.5 Enhanced time interval` | `covered` | `EnhancedIntervalLevelOnePermitsTerminalBoundaryQualification`, `EnhancedIntervalLevelTwoPermitsInternalBoundaryQualification`, `EnhancedIntervalLevelTwoPermitsInternalBoundaryUnspecifiedDigits`, `EnhancedIntervalLevelTwoPermitsBeforeOrAfterBoundaryQualification`, `OpenIntervalBoundaryDeclared`, `UnknownIntervalBoundaryDeclared` | Boundary-state and Level 1/Level 2 qualification semantics are explicit; interval-form distinctions remain tracked in `contracts_interval.rs.md`. |
| `4.6 Year exceeding four digits` | `covered` | `LetterPrefixedCalendarYearUsesLeadingYDesignator`, `LetterPrefixedCalendarYearMagnitudeExceedsFourDigits`, `ExponentialYearUsesPowerOfTenNotation`, `ExponentialYearExponentIsPositiveInteger` | Integer and exponential year extensions are explicit. |
| `4.7 Significant digits` | `covered` | `SignificantDigitYearUsesTrailingSSuffix`, `SignificantDigitYearCountIsPositiveInteger` | Significant-digit suffix semantics are explicit. |
| `4.8 Divisions of a year` | `covered` | `SeasonalExpressionUsesYearAndSeasonForm`, `SeasonCodeDeclaresNamedSeason`, `SeasonCodeDeclaresSeasonScope`, `SubYearGrouping*` | Named seasons and registered grouping families are explicit. |
| `4.9 One of a set` | `covered` | `TemporalSetDeclaresAlternativeSemantics`, `TemporalChoiceSetUsesSquareBrackets`, `TemporalSetRangeUsesInclusiveDoubleDotSemantics`, `TemporalSetOpenRangeUsesBoundaryDoubleDot` | Choice-set semantics and interval-like range notation are explicit. |
| `4.10 Multiple dates` | `covered` | `TemporalInclusiveSetUsesCurlyBraces`, `TemporalSetDeclaresInclusiveMemberSemantics`, `TemporalSetCarriesMultipleMembers` | Inclusive-set semantics are explicit. |
| `4.11 Decade` | `covered elsewhere` | `DecadeOrdinalInRangeZeroToNineHundredNinetyNine`, `DecadeEvidence` | Decade core coverage lives in `iso_8601.rs.md` and `contracts_proof_composition.rs.md`. |

## Checklist

- [x] Decide whether enhanced-interval level partitioning itself needs a
  first-class proposition beyond the specific `4.4` and `10.2` laws.
