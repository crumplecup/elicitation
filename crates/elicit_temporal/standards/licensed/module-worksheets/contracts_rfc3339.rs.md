# Citation Worksheet: `src/contracts/rfc3339.rs`

This worksheet stages exhaustive coverage review for the RFC 3339 contract
surface in `src/contracts/rfc3339.rs`.

## Source set

- `../../public/rfc3339.txt`
- `../../public/rfc9557.txt`

## RFC contract map

| Contract symbol | Current topic label | Exact section | Source artifact | Updated in code |
| --- | --- | --- | --- | --- |
| `Rfc3339UsesFourDigitYear` | `four-digit year` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339UsesExtendedCalendarDate` | `full-date production` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339UsesFullTime` | `full-time production` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339RequiresUtcRelationship` | `explicit relationship to UTC` | `RFC 3339 §4.4` | `../../public/rfc3339.txt lines 262-297` | `yes` |
| `Rfc3339UnqualifiedLocalTimeForbidden` | `unqualified local time forbidden` | `RFC 3339 §4.4` | `../../public/rfc3339.txt lines 262-297` | `yes` |
| `Rfc3339FractionUsesDotSeparator` | `fractional seconds separator` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339OffsetIsUtcOrNumeric` | `UTC relationship encoded as Z or numeric offset` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339UnknownLocalOffsetUsesZuluDesignator` | `unknown local-offset convention` | `RFC 3339 §4.3 as updated by RFC 9557 §2.2` | `../../public/rfc3339.txt lines 254-261; ../../public/rfc9557.txt lines 302-319` | `yes` |
| `Rfc3339LocalOffsetNotUnknown` | `timestamp does not use unknown local-offset convention` | `RFC 3339 §4.3 as updated by RFC 9557 §2.2` | `../../public/rfc3339.txt lines 254-261; ../../public/rfc9557.txt lines 302-319` | `yes` |
| `Rfc3339PositiveZeroOffsetDeclaresPreferredUtcReferencePoint` | `+00:00 preferred-reference semantics` | `RFC 3339 §4.3 as updated by RFC 9557 §2.2` | `../../public/rfc3339.txt lines 254-261; ../../public/rfc9557.txt lines 302-319` | `yes` |
| `Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding` | `lexical ordering precondition for UTC-relationship encoding` | `RFC 3339 §5.1` | `../../public/rfc3339.txt lines 299-308` | `yes` |
| `Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits` | `lexical ordering precondition for fractional-second width` | `RFC 3339 §5.1` | `../../public/rfc3339.txt lines 299-308` | `yes` |
| `Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory` | `profile simplicity through mandatory structure` | `RFC 3339 §5.5; §5.6` | `../../public/rfc3339.txt lines 367-396; 399-452` | `yes` |
| `Rfc3339FractionalSecondsAreOnlyRarelyUsedOption` | `fractional seconds as only rarely used option` | `RFC 3339 §5.3` | `../../public/rfc3339.txt lines 343-355` | `yes` |
| `Rfc3339GeneratorsShouldUseUppercaseTAndZ` | `generator guidance for uppercase T and Z` | `RFC 3339 §5.6` | `../../public/rfc3339.txt lines 399-452` | `yes` |
| `Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator` | `space separator readability option` | `RFC 3339 §5.6 note` | `../../public/rfc3339.txt lines 440-444` | `yes` |
| `Rfc3339LeapSecondGenerationRequiresPriorAnnouncement` | `leap-second generation restriction` | `RFC 3339 §5.7` | `../../public/rfc3339.txt lines 455-509` | `yes` |

## RFC 3339 Section Coverage Audit

| Section | Coverage status | Current surface anchors | Follow-up |
| --- | --- | --- | --- |
| `4.1 Coordinated Universal Time (UTC)` | `covered elsewhere` | `UtcIsReferenceTimeScale`, `UtcOfDayIdentifiesTimeWithinUtcCalendarDay`, `OffsetDateTimeIdentifiesSingleInstant` | Keep UTC reference-scale semantics in ISO and instant layers, not the profile-only module. |
| `4.2 Local Offsets` | `covered elsewhere` | `UtcOffsetCarriesSignHourAndOptionalMinute`, `UtcDifferenceSignEncodesDirectionRelativeToUtc`, `Rfc3339OffsetIsUtcOrNumeric` | Keep offset arithmetic and range law in the ISO surface; keep profile selection law here. |
| `4.3 Unknown Local Offset Convention` | `covered` | `Rfc3339UnknownLocalOffsetUsesZuluDesignator`, `Rfc3339LocalOffsetNotUnknown`, `Rfc3339PositiveZeroOffsetDeclaresPreferredUtcReferencePoint` | The worksheet now reflects RFC 3339 Section `4.3` as updated by RFC 9557 Section `2.2`. |
| `4.4 Unqualified Local Time` | `covered` | `Rfc3339RequiresUtcRelationship`, `Rfc3339UnqualifiedLocalTimeForbidden` | No additional profile proposition is currently required. |
| `5.1 Ordering` | `covered` | `Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding`, `Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits` | No additional profile proposition is currently required. |
| `5.2 Human Readability` | `intentionally omitted` | parser and formatter policy surface | Guidance is operational and UX-oriented, not a stable proof token. |
| `5.3 Rarely Used Options` | `covered` | `Rfc3339FractionalSecondsAreOnlyRarelyUsedOption` | No additional profile proposition is currently required. |
| `5.4 Redundant Information` | `covered indirectly` | `Rfc3339UsesExtendedCalendarDate`, `Rfc3339UsesFullTime` | The grammar excludes redundant weekday material, but we have no first-class redundancy token. |
| `5.5 Simplicity` | `covered` | `Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory` | No additional profile proposition is currently required. |
| `5.6 Internet Date/Time Format` | `covered` | the full `Rfc3339*` grammar family above; `Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator` | The space-separator note is represented explicitly, while parser and formatter seams remain strict to the core `T`-separated profile. |
| `5.7 Restrictions` | `covered elsewhere` | `Rfc3339LeapSecondGenerationRequiresPriorAnnouncement`, ISO range and leap-second laws | Month/day range and leap-second bounds remain intentionally shared with the ISO contract surface. |

## Notes

- This file now cites the preserved public RFC text directly.
- RFC 3339 Section `4.3` is tracked here as updated by RFC 9557 Section `2.2`,
  not as an isolated 2002-era rule.
- The Section `5.6` space-separator note is represented as an application-level
  standards proposition, but `parse_rfc3339_timestamp` and
  `format_rfc3339_timestamp` intentionally remain strict to the ABNF profile.

## Remaining Coverage Checklist

- [x] Stage the public RFC 3339 source text locally for stable worksheet references.
- [x] Audit RFC 3339 section-by-section and mark which substantive sections are intentionally represented elsewhere in `elicit_temporal`.
- [x] Reconcile the RFC 9557 update to RFC 3339 Section `4.3` with the existing unknown-local-offset contracts.
- [x] Decide whether the optional space-separated note in Section `5.6` belongs in parser and formatter trait seams.
