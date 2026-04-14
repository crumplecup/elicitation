# OGC 18-010r7 WKT-CRS Contract Implementation Notes

**Standard:** OGC 18-010r7 — Geographic information — Well-Known Text representation of Coordinate
Reference Systems (WKT2-2019)

**Official URL:** <https://docs.ogc.org/is/18-010r7/18-010r7.html>

---

## Pattern correction notice

The correct prop pattern for this standard (from `crates/elicit_db/src/contracts/iso_sql.rs`):

```rust
mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Brief description of the proposition.
    ///
    /// Source: OGC 18-010r7 §X.Y — <section title>
    pub struct PropName;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream { quote! { /* structural */ } }
                fn verus_proof() -> TokenStream { quote! { /* structural */ } }
                fn creusot_proof() -> TokenStream { quote! { /* structural */ } }
            }
        };
    }
    structural_prop!(PropName, "PropName");
}
pub use emit_impls::PropName;
```

Do **NOT** use `#[derive(Prop)]` or `#[spec_reference(...)]` — both are fabricated and do not exist.

---

## OGC 18-010r7 Contract Checklist

---

### §1 — General Syntax Rules

**Reference:** OGC 18-010r7 §6 — Requirements class "WKT string"

A WKT string is a structured text representation of a coordinate reference system (CRS) or
coordinate operation. All WKT objects share a common syntactic envelope described in §6 of the
standard. The grammar is defined using EBNF and every conformant implementation must honour the
rules below.

**Required syntax properties:**

- [ ] Keywords are case-insensitive ASCII alpha identifiers
- [ ] Delimiters are either `[`/`]` (square brackets) or `(`/`)` (parentheses) — not mixed
- [ ] Decimal separator is always `.` — locale commas are illegal
- [ ] String literals are enclosed in double-quotes `"`
- [ ] Embedded double-quotes are escaped as `""` (not `\"`)
- [ ] Commas separate elements within a WKT component
- [ ] Optional sub-keywords may be omitted but ordering must follow the grammar
- [ ] Insignificant whitespace (spaces, tabs, newlines) is allowed between tokens

```rust
/// WKT keywords are case-insensitive ASCII identifiers.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktKeywordCaseInsensitive;
structural_prop!(WktKeywordCaseInsensitive, "WktKeywordCaseInsensitive");

/// Left/right delimiters may be square brackets `[` `]`.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktSquareBracketDelimiters;
structural_prop!(WktSquareBracketDelimiters, "WktSquareBracketDelimiters");

/// Left/right delimiters may be parentheses `(` `)`.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktParenthesisDelimiters;
structural_prop!(WktParenthesisDelimiters, "WktParenthesisDelimiters");

/// Delimiter style must be consistent throughout a single WKT string; mixing is forbidden.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktDelimiterConsistency;
structural_prop!(WktDelimiterConsistency, "WktDelimiterConsistency");

/// Decimal separator in numeric literals must be `.` (period); locale commas are illegal.
///
/// Source: OGC 18-010r7 §6.3.2 — Unsigned integer and floating-point numbers
pub struct WktDecimalSeparatorPeriod;
structural_prop!(WktDecimalSeparatorPeriod, "WktDecimalSeparatorPeriod");

/// Integer numeric literals are allowed wherever a number is required.
///
/// Source: OGC 18-010r7 §6.3.2 — Unsigned integer and floating-point numbers
pub struct WktIntegerNumbers;
structural_prop!(WktIntegerNumbers, "WktIntegerNumbers");

/// Floating-point numeric literals (with decimal point or exponent) are allowed.
///
/// Source: OGC 18-010r7 §6.3.2 — Unsigned integer and floating-point numbers
pub struct WktFloatingPointNumbers;
structural_prop!(WktFloatingPointNumbers, "WktFloatingPointNumbers");

/// String literals are enclosed in double-quotes `"`.
///
/// Source: OGC 18-010r7 §6.3.1 — Quoted strings
pub struct WktDoubleQuoteStrings;
structural_prop!(WktDoubleQuoteStrings, "WktDoubleQuoteStrings");

/// An embedded double-quote within a string literal is escaped as `""`, not `\"`.
///
/// Source: OGC 18-010r7 §6.3.1 — Quoted strings
pub struct WktDoubleQuoteEscape;
structural_prop!(WktDoubleQuoteEscape, "WktDoubleQuoteEscape");

/// Commas are the element separator between sub-keywords within any WKT component.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktCommaElementSeparator;
structural_prop!(WktCommaElementSeparator, "WktCommaElementSeparator");

/// Whitespace (spaces, tabs, newlines) between tokens is ignored and has no semantic effect.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktOptionalWhitespace;
structural_prop!(WktOptionalWhitespace, "WktOptionalWhitespace");

/// Keywords consist of ASCII alphabetic characters only (no digits, hyphens, or underscores).
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktKeywordAsciiAlpha;
structural_prop!(WktKeywordAsciiAlpha, "WktKeywordAsciiAlpha");

/// Optional sub-keywords may be omitted, but when present their ordering must follow the grammar.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktOptionalElementOrdering;
structural_prop!(WktOptionalElementOrdering, "WktOptionalElementOrdering");

/// Re-serialising a parsed WKT string produces a WKT string equivalent to the original.
///
/// Source: OGC 18-010r7 §6 — Requirements class "WKT string"
pub struct WktRoundTripPreservation;
structural_prop!(WktRoundTripPreservation, "WktRoundTripPreservation");
```

---

### §5.2 — UNIT sub-keyword

**Reference:** OGC 18-010r7 §7.5.2 — Unit of measure

The `UNIT` keyword (and its type-specific aliases) identifies the unit of measure for a CRS axis,
a conversion parameter, or a prime meridian longitude. A conversion factor relative to the SI base
unit is mandatory. The type-specific keywords (`LENGTHUNIT`, `ANGLEUNIT`, etc.) are preferred over
the generic `UNIT` when the unit type is known.

**Constraints:**

- `name` — non-empty quoted string
- `conversionFactor` — positive real number (not zero, not negative)
- For length: factor relative to metres (1.0 = metre, 0.3048 = foot, etc.)
- For angle: factor relative to radians (approx. 0.017453292519943278 for degrees)
- For time: factor relative to seconds
- For scale: factor relative to unity (dimensionless ratio 1)
- `ID` sub-keyword is optional

**Example:**
`LENGTHUNIT["metre", 1.0, ID["EPSG", 9001]]`
`ANGLEUNIT["degree", 0.017453292519943278, ID["EPSG", 9102]]`

```rust
/// UNIT keyword introduces a unit-of-measure sub-object in a WKT component.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitKeywordPresent;
structural_prop!(UnitKeywordPresent, "UnitKeywordPresent");

/// Unit name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitNameNonEmpty;
structural_prop!(UnitNameNonEmpty, "UnitNameNonEmpty");

/// Unit conversion factor is a positive real number (strictly greater than zero).
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitConversionFactorPositive;
structural_prop!(UnitConversionFactorPositive, "UnitConversionFactorPositive");

/// Length unit conversion factor is relative to metres (SI base unit for length).
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitLengthFactorToMetres;
structural_prop!(UnitLengthFactorToMetres, "UnitLengthFactorToMetres");

/// Angle unit conversion factor is relative to radians (SI base unit for plane angle).
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitAngleFactorToRadians;
structural_prop!(UnitAngleFactorToRadians, "UnitAngleFactorToRadians");

/// Time unit conversion factor is relative to seconds (SI base unit for time).
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitTimeFactorToSeconds;
structural_prop!(UnitTimeFactorToSeconds, "UnitTimeFactorToSeconds");

/// Scale unit conversion factor is relative to unity (dimensionless — ratio 1).
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitScaleFactorToUnity;
structural_prop!(UnitScaleFactorToUnity, "UnitScaleFactorToUnity");

/// LENGTHUNIT keyword is the preferred type-specific alias for length units.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct LengthUnitKeyword;
structural_prop!(LengthUnitKeyword, "LengthUnitKeyword");

/// ANGLEUNIT keyword is the preferred type-specific alias for angular units.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct AngleUnitKeyword;
structural_prop!(AngleUnitKeyword, "AngleUnitKeyword");

/// SCALEUNIT keyword is the preferred type-specific alias for scale (dimensionless) units.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct ScaleUnitKeyword;
structural_prop!(ScaleUnitKeyword, "ScaleUnitKeyword");

/// TIMEUNIT keyword is the preferred type-specific alias for temporal units.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct TimeUnitKeyword;
structural_prop!(TimeUnitKeyword, "TimeUnitKeyword");

/// PARAMETRICUNIT keyword is the preferred type-specific alias for parametric units.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct ParametricUnitKeyword;
structural_prop!(ParametricUnitKeyword, "ParametricUnitKeyword");

/// ID sub-keyword within UNIT is optional; if absent the unit is identified by name only.
///
/// Source: OGC 18-010r7 §7.5.2 — Unit of measure
pub struct UnitIdOptional;
structural_prop!(UnitIdOptional, "UnitIdOptional");
```

---

### §5.3 — AXIS sub-keyword

**Reference:** OGC 18-010r7 §7.5.3 — Coordinate system axis

An `AXIS` sub-keyword appears once per axis within a `CS` block. The name includes an abbreviation
in parentheses. The direction is a mandatory enumerated code. The optional `ORDER` keyword gives
the 1-based integer ordering of this axis within the coordinate system. The optional `UNIT` within
an `AXIS` overrides the CRS-level unit for that axis alone.

**Constraints:**

- `name` — non-empty quoted string with the abbreviation inside parentheses
- `direction` — one of the defined enumeration codes (see below)
- `ORDER` — optional positive integer (1-based axis index)
- `UNIT` within AXIS — optional per-axis unit override

**Example:**
`AXIS["Latitude (lat)", north, ORDER[1], ANGLEUNIT["degree", 0.017453292519943278]]`

```rust
/// AXIS keyword introduces a coordinate system axis sub-object.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisKeywordPresent;
structural_prop!(AxisKeywordPresent, "AxisKeywordPresent");

/// Axis name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisNameNonEmpty;
structural_prop!(AxisNameNonEmpty, "AxisNameNonEmpty");

/// Axis name contains the axis abbreviation in parentheses, e.g., "Latitude (lat)".
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisAbbreviationInParentheses;
structural_prop!(AxisAbbreviationInParentheses, "AxisAbbreviationInParentheses");

/// Axis direction must be one of the enumerated direction code strings defined by the standard.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisDirectionEnumerated;
structural_prop!(AxisDirectionEnumerated, "AxisDirectionEnumerated");

/// ORDER sub-keyword within AXIS is optional.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisOrderOptional;
structural_prop!(AxisOrderOptional, "AxisOrderOptional");

/// ORDER value within AXIS is a positive integer giving the 1-based axis sequence index.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisOrderPositiveInteger;
structural_prop!(AxisOrderPositiveInteger, "AxisOrderPositiveInteger");

/// UNIT within AXIS is optional; when present it overrides the CRS-level unit for that axis.
///
/// Source: OGC 18-010r7 §7.5.3 — Coordinate system axis
pub struct AxisUnitOptional;
structural_prop!(AxisUnitOptional, "AxisUnitOptional");
```

#### §5.3 — Axis Direction Codes (Table 2)

Each of the following strings is a valid value for the axis direction element. Case-insensitive
matching applies per the general syntax rule that direction codes are enumerated tokens.

```rust
/// Axis direction code `north` — positive axis direction toward geographic or magnetic north.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionNorth;
structural_prop!(AxisDirectionNorth, "AxisDirectionNorth");

/// Axis direction code `south` — positive axis direction toward geographic or magnetic south.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionSouth;
structural_prop!(AxisDirectionSouth, "AxisDirectionSouth");

/// Axis direction code `east` — positive axis direction toward geographic or magnetic east.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionEast;
structural_prop!(AxisDirectionEast, "AxisDirectionEast");

/// Axis direction code `west` — positive axis direction toward geographic or magnetic west.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionWest;
structural_prop!(AxisDirectionWest, "AxisDirectionWest");

/// Axis direction code `up` — positive axis direction is vertically upward.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionUp;
structural_prop!(AxisDirectionUp, "AxisDirectionUp");

/// Axis direction code `down` — positive axis direction is vertically downward.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionDown;
structural_prop!(AxisDirectionDown, "AxisDirectionDown");

/// Axis direction code `northNorthEast` — between north and north-east (approx 22.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionNorthNorthEast;
structural_prop!(AxisDirectionNorthNorthEast, "AxisDirectionNorthNorthEast");

/// Axis direction code `northEast` — north-east (45 degrees from north).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionNorthEast;
structural_prop!(AxisDirectionNorthEast, "AxisDirectionNorthEast");

/// Axis direction code `eastNorthEast` — between east and north-east (approx 67.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionEastNorthEast;
structural_prop!(AxisDirectionEastNorthEast, "AxisDirectionEastNorthEast");

/// Axis direction code `eastSouthEast` — between east and south-east (approx 112.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionEastSouthEast;
structural_prop!(AxisDirectionEastSouthEast, "AxisDirectionEastSouthEast");

/// Axis direction code `southEast` — south-east (135 degrees from north).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionSouthEast;
structural_prop!(AxisDirectionSouthEast, "AxisDirectionSouthEast");

/// Axis direction code `southSouthEast` — between south and south-east (approx 157.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionSouthSouthEast;
structural_prop!(AxisDirectionSouthSouthEast, "AxisDirectionSouthSouthEast");

/// Axis direction code `southSouthWest` — between south and south-west (approx 202.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionSouthSouthWest;
structural_prop!(AxisDirectionSouthSouthWest, "AxisDirectionSouthSouthWest");

/// Axis direction code `southWest` — south-west (225 degrees from north).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionSouthWest;
structural_prop!(AxisDirectionSouthWest, "AxisDirectionSouthWest");

/// Axis direction code `westSouthWest` — between west and south-west (approx 247.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionWestSouthWest;
structural_prop!(AxisDirectionWestSouthWest, "AxisDirectionWestSouthWest");

/// Axis direction code `westNorthWest` — between west and north-west (approx 292.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionWestNorthWest;
structural_prop!(AxisDirectionWestNorthWest, "AxisDirectionWestNorthWest");

/// Axis direction code `northWest` — north-west (315 degrees from north).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionNorthWest;
structural_prop!(AxisDirectionNorthWest, "AxisDirectionNorthWest");

/// Axis direction code `northNorthWest` — between north and north-west (approx 337.5 deg).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionNorthNorthWest;
structural_prop!(AxisDirectionNorthNorthWest, "AxisDirectionNorthNorthWest");

/// Axis direction code `geocentricX` — toward intersection of equator and prime meridian.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionGeocentricX;
structural_prop!(AxisDirectionGeocentricX, "AxisDirectionGeocentricX");

/// Axis direction code `geocentricY` — toward intersection of equator and 90-degree east meridian.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionGeocentricY;
structural_prop!(AxisDirectionGeocentricY, "AxisDirectionGeocentricY");

/// Axis direction code `geocentricZ` — toward north pole along the Earth rotation axis.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionGeocentricZ;
structural_prop!(AxisDirectionGeocentricZ, "AxisDirectionGeocentricZ");

/// Axis direction code `columnPositive` — positive column index direction (image or display CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionColumnPositive;
structural_prop!(AxisDirectionColumnPositive, "AxisDirectionColumnPositive");

/// Axis direction code `columnNegative` — negative column index direction (image or display CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionColumnNegative;
structural_prop!(AxisDirectionColumnNegative, "AxisDirectionColumnNegative");

/// Axis direction code `rowPositive` — positive row index direction (image or display CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionRowPositive;
structural_prop!(AxisDirectionRowPositive, "AxisDirectionRowPositive");

/// Axis direction code `rowNegative` — negative row index direction (image or display CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionRowNegative;
structural_prop!(AxisDirectionRowNegative, "AxisDirectionRowNegative");

/// Axis direction code `displayRight` — toward right of display screen.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionDisplayRight;
structural_prop!(AxisDirectionDisplayRight, "AxisDirectionDisplayRight");

/// Axis direction code `displayLeft` — toward left of display screen.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionDisplayLeft;
structural_prop!(AxisDirectionDisplayLeft, "AxisDirectionDisplayLeft");

/// Axis direction code `displayUp` — toward top of display screen.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionDisplayUp;
structural_prop!(AxisDirectionDisplayUp, "AxisDirectionDisplayUp");

/// Axis direction code `displayDown` — toward bottom of display screen.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionDisplayDown;
structural_prop!(AxisDirectionDisplayDown, "AxisDirectionDisplayDown");

/// Axis direction code `future` — increasing time coordinate value (temporal CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionFuture;
structural_prop!(AxisDirectionFuture, "AxisDirectionFuture");

/// Axis direction code `past` — decreasing time coordinate value (temporal CRS).
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionPast;
structural_prop!(AxisDirectionPast, "AxisDirectionPast");

/// Axis direction code `towards` — positive axis moves toward a reference point.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionTowards;
structural_prop!(AxisDirectionTowards, "AxisDirectionTowards");

/// Axis direction code `awayFrom` — positive axis moves away from a reference point.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionAwayFrom;
structural_prop!(AxisDirectionAwayFrom, "AxisDirectionAwayFrom");

/// Axis direction code `clockwise` — positive rotation is clockwise.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionClockwise;
structural_prop!(AxisDirectionClockwise, "AxisDirectionClockwise");

/// Axis direction code `counterClockwise` — positive rotation is counter-clockwise.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionCounterClockwise;
structural_prop!(AxisDirectionCounterClockwise, "AxisDirectionCounterClockwise");

/// Axis direction code `unspecified` — direction is not constrained or not known.
///
/// Source: OGC 18-010r7 §7.5.3 Table 2 — Axis direction codes
pub struct AxisDirectionUnspecified;
structural_prop!(AxisDirectionUnspecified, "AxisDirectionUnspecified");
```

---

### §5.4 — REMARK sub-keyword

**Reference:** OGC 18-010r7 §7.5.4 — Remark

The `REMARK` keyword is universally optional and may appear as the last element in any WKT object.
Its single argument is a quoted string (possibly empty) containing free-form human-readable notes.
Because it is always last, a parser can use its presence or absence as a sentinel without affecting
the parsing of preceding mandatory elements.

**Value constraints:**

- Text is a quoted string — may contain any Unicode characters
- Text may be an empty string `""`
- Must be the final sub-keyword in the enclosing WKT object

```rust
/// REMARK keyword is optional in any WKT object and carries a free-text annotation.
///
/// Source: OGC 18-010r7 §7.5.4 — Remark
pub struct RemarkKeywordOptional;
structural_prop!(RemarkKeywordOptional, "RemarkKeywordOptional");

/// REMARK argument is a quoted string (the text content may be empty).
///
/// Source: OGC 18-010r7 §7.5.4 — Remark
pub struct RemarkTextQuotedString;
structural_prop!(RemarkTextQuotedString, "RemarkTextQuotedString");

/// REMARK, when present, must be the last element in its parent WKT component.
///
/// Source: OGC 18-010r7 §7.5.4 — Remark
pub struct RemarkPositionLast;
structural_prop!(RemarkPositionLast, "RemarkPositionLast");

/// REMARK text content may be an empty string.
///
/// Source: OGC 18-010r7 §7.5.4 — Remark
pub struct RemarkTextMayBeEmpty;
structural_prop!(RemarkTextMayBeEmpty, "RemarkTextMayBeEmpty");
```

---

### §6.2 — GEODCRS / GEOGCRS (Geodetic / Geographic CRS)

**Reference:** OGC 18-010r7 §8.2 — Geodetic CRS

A geodetic CRS (`GEODCRS`) or geographic CRS (`GEOGCRS`) is the foundational CRS type. The two
keywords are aliases; `GEOGCRS` is preferred for geographic (lat/lon) systems. The WKT structure
requires a geodetic datum, an optional prime meridian, and a coordinate system.

**Minimal example (2D geographic, EPSG 4326):**

```
GEOGCRS["WGS 84",
  DATUM["World Geodetic System 1984",
    ELLIPSOID["WGS 84", 6378137, 298.257223563,
      LENGTHUNIT["metre", 1]]],
  CS[ellipsoidal, 2],
  AXIS["Latitude (lat)", north, ORDER[1]],
  AXIS["Longitude (lon)", east, ORDER[2]],
  ANGLEUNIT["degree", 0.017453292519943278],
  ID["EPSG", 4326]]
```

**Constraints:**

- First element: non-empty quoted name
- `DATUM` or `GEODETICDATUM` sub-keyword mandatory
- `ELLIPSOID` within `DATUM` mandatory
- `PRIMEM` optional (default is Greenwich, longitude 0)
- `CS` sub-keyword with type and axis count mandatory
- Axis count must be 2 (lat/lon) or 3 (lat/lon/ellipsoidal height)

```rust
/// GEODCRS or GEOGCRS keyword introduces a geodetic or geographic CRS object.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsKeyword;
structural_prop!(GeodcrsKeyword, "GeodcrsKeyword");

/// GEOGCRS is the preferred keyword alias when the CRS is geographic (lat/lon).
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeogcrsKeywordAlias;
structural_prop!(GeogcrsKeywordAlias, "GeogcrsKeywordAlias");

/// First element of a GEODCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsNamePresent;
structural_prop!(GeodcrsNamePresent, "GeodcrsNamePresent");

/// DATUM (or GEODETICDATUM) sub-keyword is mandatory within GEODCRS.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsDatumPresent;
structural_prop!(GeodcrsDatumPresent, "GeodcrsDatumPresent");

/// ELLIPSOID sub-keyword is mandatory within the geodetic DATUM.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsEllipsoidPresent;
structural_prop!(GeodcrsEllipsoidPresent, "GeodcrsEllipsoidPresent");

/// PRIMEM is optional within GEODCRS; when absent Greenwich (longitude 0) is implied.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsPrimeMeridianOptional;
structural_prop!(GeodcrsPrimeMeridianOptional, "GeodcrsPrimeMeridianOptional");

/// CS sub-keyword with type code and axis count is mandatory in GEODCRS.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsCsPresent;
structural_prop!(GeodcrsCsPresent, "GeodcrsCsPresent");

/// Axis count in a geodetic CRS must be 2 (lat/lon) or 3 (lat/lon/height).
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsAxisCountMatchesCsType;
structural_prop!(GeodcrsAxisCountMatchesCsType, "GeodcrsAxisCountMatchesCsType");

/// ID sub-keyword at the GEODCRS level is optional.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsIdOptional;
structural_prop!(GeodcrsIdOptional, "GeodcrsIdOptional");

/// REMARK sub-keyword at the GEODCRS level is optional.
///
/// Source: OGC 18-010r7 §8.2 — Geodetic CRS
pub struct GeodcrsRemarkOptional;
structural_prop!(GeodcrsRemarkOptional, "GeodcrsRemarkOptional");
```

#### §6.2.2 — DATUM sub-keyword (Geodetic Reference Frame)

**Reference:** OGC 18-010r7 §8.2.2 — Geodetic reference frame

The `DATUM` (also `GEODETICDATUM`) sub-keyword defines the geodetic reference frame. Its mandatory
content is an `ELLIPSOID`. The optional `ANCHOR` sub-keyword carries a prose description of the
datum's realisation epoch or reference station. The legacy `TOWGS84` element is deprecated in
WKT2-2019 but must be accepted for backward compatibility with WKT1 datasets.

**Value constraints:**

- Name: non-empty quoted string (e.g., "World Geodetic System 1984")
- `ELLIPSOID` mandatory
- `ANCHOR` — optional free-text realisation description
- `TOWGS84` — deprecated, 3 or 7 numeric parameters

```rust
/// Geodetic datum name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumNameNonEmpty;
structural_prop!(DatumNameNonEmpty, "DatumNameNonEmpty");

/// ELLIPSOID sub-keyword is mandatory within the geodetic DATUM.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumEllipsoidMandatory;
structural_prop!(DatumEllipsoidMandatory, "DatumEllipsoidMandatory");

/// ANCHOR sub-keyword within DATUM is optional; it carries a prose realisation note.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumAnchorOptional;
structural_prop!(DatumAnchorOptional, "DatumAnchorOptional");

/// TOWGS84 is deprecated in WKT2-2019 but must be accepted as a parseable legacy element.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumToWgs84Deprecated;
structural_prop!(DatumToWgs84Deprecated, "DatumToWgs84Deprecated");

/// TOWGS84, when present, contains either 3 (translation) or 7 (Helmert) numeric parameters.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumToWgs84SevenParams;
structural_prop!(DatumToWgs84SevenParams, "DatumToWgs84SevenParams");

/// GEODETICDATUM is an accepted alias for the DATUM keyword within GEODCRS.
///
/// Source: OGC 18-010r7 §8.2.2 — Geodetic reference frame
pub struct DatumGeodeticDatumAlias;
structural_prop!(DatumGeodeticDatumAlias, "DatumGeodeticDatumAlias");
```

#### §6.2.3 — ELLIPSOID sub-keyword

**Reference:** OGC 18-010r7 §8.2.3 — Ellipsoid

The `ELLIPSOID` keyword encodes the reference ellipsoid used by a geodetic datum. The semi-major
axis `a` is in metres unless a `LENGTHUNIT` overrides it. Inverse flattening `rf` equal to zero
means the ellipsoid degenerates to a sphere. Typical values: WGS 84 has a = 6378137.0 m,
rf = 298.257223563.

**Value constraints:**

- Name: non-empty quoted string
- `a` (semi-major axis): positive real number, default unit metres
- `rf` (inverse flattening): non-negative real number (0 = sphere)
- `LENGTHUNIT`: optional; when absent the semi-major axis is in metres
- `ID`: optional

```rust
/// ELLIPSOID keyword introduces an ellipsoid sub-object within DATUM.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidKeywordPresent;
structural_prop!(EllipsoidKeywordPresent, "EllipsoidKeywordPresent");

/// Ellipsoid name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidNameNonEmpty;
structural_prop!(EllipsoidNameNonEmpty, "EllipsoidNameNonEmpty");

/// Semi-major axis `a` is a positive real number; default unit is metres.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidSemiMajorAxisPositive;
structural_prop!(EllipsoidSemiMajorAxisPositive, "EllipsoidSemiMajorAxisPositive");

/// Inverse flattening `rf` is a non-negative real number.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidInverseFlatteningNonNegative;
structural_prop!(EllipsoidInverseFlatteningNonNegative, "EllipsoidInverseFlatteningNonNegative");

/// When `rf` equals 0 the ellipsoid is a sphere (no flattening).
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidSphericalWhenRfZero;
structural_prop!(EllipsoidSphericalWhenRfZero, "EllipsoidSphericalWhenRfZero");

/// LENGTHUNIT within ELLIPSOID is optional; when absent the semi-major axis is in metres.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidLengthUnitOptional;
structural_prop!(EllipsoidLengthUnitOptional, "EllipsoidLengthUnitOptional");

/// ID sub-keyword within ELLIPSOID is optional.
///
/// Source: OGC 18-010r7 §8.2.3 — Ellipsoid
pub struct EllipsoidIdOptional;
structural_prop!(EllipsoidIdOptional, "EllipsoidIdOptional");
```

#### §6.2.4 — PRIMEM sub-keyword (Prime Meridian)

**Reference:** OGC 18-010r7 §8.2.4 — Prime meridian

The `PRIMEM` keyword defines the prime meridian of a geodetic CRS. Its longitude is expressed in
the angular unit of the enclosing CRS. The conventional default when `PRIMEM` is absent is
Greenwich (0 degrees). The EPSG registry code for Greenwich is 8901.

**Value constraints:**

- Name: non-empty quoted string (typically "Greenwich")
- Longitude: real number in the open-closed interval (-180, 180]
- Longitude unit: parent CRS angular unit unless overridden by `ANGLEUNIT`
- `ANGLEUNIT`: optional sub-keyword
- `ID`: optional sub-keyword

```rust
/// PRIMEM keyword introduces a prime meridian sub-object within GEODCRS.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemKeywordPresent;
structural_prop!(PrimemKeywordPresent, "PrimemKeywordPresent");

/// Prime meridian name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemNameNonEmpty;
structural_prop!(PrimemNameNonEmpty, "PrimemNameNonEmpty");

/// Prime meridian longitude is a real number in the interval (-180, 180].
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemLongitudeRange;
structural_prop!(PrimemLongitudeRange, "PrimemLongitudeRange");

/// Prime meridian longitude value is expressed in the angular unit of the parent CRS.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemLongitudeInParentAngularUnit;
structural_prop!(PrimemLongitudeInParentAngularUnit, "PrimemLongitudeInParentAngularUnit");

/// ANGLEUNIT within PRIMEM is optional; when absent the parent CRS angular unit applies.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemAngleUnitOptional;
structural_prop!(PrimemAngleUnitOptional, "PrimemAngleUnitOptional");

/// ID sub-keyword within PRIMEM is optional.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemIdOptional;
structural_prop!(PrimemIdOptional, "PrimemIdOptional");

/// When PRIMEM is absent, Greenwich (longitude = 0) is the implicit default.
///
/// Source: OGC 18-010r7 §8.2.4 — Prime meridian
pub struct PrimemGreenwichDefault;
structural_prop!(PrimemGreenwichDefault, "PrimemGreenwichDefault");
```

---

### §6.3 — PROJCRS (Projected CRS)

**Reference:** OGC 18-010r7 §8.3 — Projected CRS

A `PROJCRS` encodes a 2D (or 3D) projected CRS derived from a base geodetic CRS by applying a
named map projection. The `BASEGEODCRS` carries the source geodetic CRS. The `CONVERSION` block
names the projection method and lists its parameters. Each parameter has a name, a numeric value,
and a unit.

**Minimal example (UTM zone 32N, EPSG 32632):**

```
PROJCRS["WGS 84 / UTM zone 32N",
  BASEGEODCRS["WGS 84",
    DATUM["World Geodetic System 1984",
      ELLIPSOID["WGS 84", 6378137, 298.257223563]],
    CS[ellipsoidal, 2],
    AXIS["Latitude (lat)", north],
    AXIS["Longitude (lon)", east],
    ANGLEUNIT["degree", 0.017453292519943278]],
  CONVERSION["UTM zone 32N",
    METHOD["Transverse Mercator", ID["EPSG", 9807]],
    PARAMETER["Latitude of natural origin", 0,
      ANGLEUNIT["degree", 0.017453292519943278]],
    PARAMETER["Longitude of natural origin", 9,
      ANGLEUNIT["degree", 0.017453292519943278]],
    PARAMETER["Scale factor at natural origin", 0.9996,
      SCALEUNIT["unity", 1]],
    PARAMETER["False easting", 500000, LENGTHUNIT["metre", 1]],
    PARAMETER["False northing", 0, LENGTHUNIT["metre", 1]]],
  CS[Cartesian, 2],
  AXIS["Easting (E)", east, LENGTHUNIT["metre", 1]],
  AXIS["Northing (N)", north, LENGTHUNIT["metre", 1]],
  ID["EPSG", 32632]]
```

**Constraints:**

- Name: non-empty quoted string
- `BASEGEODCRS` mandatory (full embedded geodetic CRS)
- `CONVERSION` mandatory with at least `METHOD`
- `METHOD` name: non-empty quoted string
- Each `PARAMETER`: name, numeric value, UNIT
- `CS` type: Cartesian
- Standard 2D projected CRS has exactly 2 axes
- Axis unit: linear (length)

```rust
/// PROJCRS keyword introduces a projected CRS object.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsKeyword;
structural_prop!(ProjcrsKeyword, "ProjcrsKeyword");

/// First element of a PROJCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsNamePresent;
structural_prop!(ProjcrsNamePresent, "ProjcrsNamePresent");

/// BASEGEODCRS sub-keyword is mandatory within PROJCRS.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsBaseGeodcrsPresent;
structural_prop!(ProjcrsBaseGeodcrsPresent, "ProjcrsBaseGeodcrsPresent");

/// CONVERSION sub-keyword is mandatory within PROJCRS.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsConversionPresent;
structural_prop!(ProjcrsConversionPresent, "ProjcrsConversionPresent");

/// METHOD sub-keyword is mandatory within CONVERSION in PROJCRS.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsConversionMethodPresent;
structural_prop!(ProjcrsConversionMethodPresent, "ProjcrsConversionMethodPresent");

/// Projection method name within METHOD is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsMethodNameNonEmpty;
structural_prop!(ProjcrsMethodNameNonEmpty, "ProjcrsMethodNameNonEmpty");

/// Each PARAMETER name within CONVERSION is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsParameterNameNonEmpty;
structural_prop!(ProjcrsParameterNameNonEmpty, "ProjcrsParameterNameNonEmpty");

/// Each PARAMETER value within CONVERSION is a numeric literal.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsParameterValueIsNumber;
structural_prop!(ProjcrsParameterValueIsNumber, "ProjcrsParameterValueIsNumber");

/// Each PARAMETER within CONVERSION must carry a UNIT sub-keyword.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsParameterUnitPresent;
structural_prop!(ProjcrsParameterUnitPresent, "ProjcrsParameterUnitPresent");

/// CS type in a standard PROJCRS is Cartesian.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsCartesianCsType;
structural_prop!(ProjcrsCartesianCsType, "ProjcrsCartesianCsType");

/// A standard 2D PROJCRS has exactly 2 coordinate axes.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsAxisCountIsTwo;
structural_prop!(ProjcrsAxisCountIsTwo, "ProjcrsAxisCountIsTwo");

/// Axes in a PROJCRS use a linear (length) unit (typically metres).
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsLinearUnit;
structural_prop!(ProjcrsLinearUnit, "ProjcrsLinearUnit");

/// ID sub-keyword at the PROJCRS level is optional.
///
/// Source: OGC 18-010r7 §8.3 — Projected CRS
pub struct ProjcrsIdOptional;
structural_prop!(ProjcrsIdOptional, "ProjcrsIdOptional");
```

---

### §6.4 — VERTCRS (Vertical CRS)

**Reference:** OGC 18-010r7 §8.4 — Vertical CRS

A `VERTCRS` encodes a 1D vertical CRS (heights or depths). The vertical datum is `VDATUM` (also
`VERTICALDATUM`). The CS type is `vertical` and there is exactly one axis with direction `up`
or `down`. The axis unit is a length unit (metres, feet, etc.).

**Minimal example (NAVD88, EPSG 5703):**

```
VERTCRS["NAVD88",
  VDATUM["North American Vertical Datum 1988"],
  CS[vertical, 1],
  AXIS["Gravity-related height (H)", up, LENGTHUNIT["metre", 1]],
  ID["EPSG", 5703]]
```

```rust
/// VERTCRS keyword introduces a vertical CRS object.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsKeyword;
structural_prop!(VertcrsKeyword, "VertcrsKeyword");

/// First element of a VERTCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsNamePresent;
structural_prop!(VertcrsNamePresent, "VertcrsNamePresent");

/// VDATUM (or VERTICALDATUM) sub-keyword is mandatory within VERTCRS.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsDatumPresent;
structural_prop!(VertcrsDatumPresent, "VertcrsDatumPresent");

/// CS type in a VERTCRS must be `vertical`.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsCsType;
structural_prop!(VertcrsCsType, "VertcrsCsType");

/// A VERTCRS has exactly 1 coordinate axis.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsAxisCount;
structural_prop!(VertcrsAxisCount, "VertcrsAxisCount");

/// The single axis in a VERTCRS must have direction `up` or `down`.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsAxisDirectionUpOrDown;
structural_prop!(VertcrsAxisDirectionUpOrDown, "VertcrsAxisDirectionUpOrDown");

/// The axis unit in a VERTCRS is a length (linear) unit.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsLinearUnit;
structural_prop!(VertcrsLinearUnit, "VertcrsLinearUnit");

/// VERTICALDATUM is an accepted alias for VDATUM within VERTCRS.
///
/// Source: OGC 18-010r7 §8.4 — Vertical CRS
pub struct VertcrsVerticalDatumAlias;
structural_prop!(VertcrsVerticalDatumAlias, "VertcrsVerticalDatumAlias");
```

---

### §6.5 — ENGCRS (Engineering CRS)

**Reference:** OGC 18-010r7 §8.5 — Engineering CRS

An `ENGCRS` (also `ENGINEERINGCRS`) encodes a local engineering CRS used for plant layout,
machinery, image coordinates, or similar applications where coordinates are relative to a local
origin. The datum is `EDATUM`. Any valid CS type is permitted depending on the application.

```rust
/// ENGCRS keyword introduces an engineering CRS object.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsKeyword;
structural_prop!(EngcrsKeyword, "EngcrsKeyword");

/// ENGINEERINGCRS is an accepted long-form alias for the ENGCRS keyword.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngineeringcrsKeywordAlias;
structural_prop!(EngineeringcrsKeywordAlias, "EngineeringcrsKeywordAlias");

/// First element of an ENGCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsNamePresent;
structural_prop!(EngcrsNamePresent, "EngcrsNamePresent");

/// EDATUM sub-keyword is mandatory within ENGCRS.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsDatumPresent;
structural_prop!(EngcrsDatumPresent, "EngcrsDatumPresent");

/// EDATUM name is a non-empty quoted string describing the local engineering datum.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsDatumNameNonEmpty;
structural_prop!(EngcrsDatumNameNonEmpty, "EngcrsDatumNameNonEmpty");

/// Any valid CS type is permitted in an ENGCRS.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsCsType;
structural_prop!(EngcrsCsType, "EngcrsCsType");

/// ID sub-keyword at the ENGCRS level is optional.
///
/// Source: OGC 18-010r7 §8.5 — Engineering CRS
pub struct EngcrsIdOptional;
structural_prop!(EngcrsIdOptional, "EngcrsIdOptional");
```

---

### §6.6 — PARAMETRICCRS (Parametric CRS)

**Reference:** OGC 18-010r7 §8.6 — Parametric CRS

A `PARAMETRICCRS` represents a CRS whose single axis measures a non-spatial physical quantity
such as atmospheric pressure, ocean salinity, or temperature. The datum is `PDATUM`. The CS type
is `parametric` and there is exactly 1 axis.

```rust
/// PARAMETRICCRS keyword introduces a parametric CRS object.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsKeyword;
structural_prop!(ParametriccrsKeyword, "ParametriccrsKeyword");

/// First element of a PARAMETRICCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsNamePresent;
structural_prop!(ParametriccrsNamePresent, "ParametriccrsNamePresent");

/// PDATUM sub-keyword is mandatory within PARAMETRICCRS.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsDatumPresent;
structural_prop!(ParametriccrsDatumPresent, "ParametriccrsDatumPresent");

/// PDATUM name is a non-empty quoted string describing the parametric datum.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsDatumNameNonEmpty;
structural_prop!(ParametriccrsDatumNameNonEmpty, "ParametriccrsDatumNameNonEmpty");

/// CS type in a PARAMETRICCRS must be `parametric`.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsCsType;
structural_prop!(ParametriccrsCsType, "ParametriccrsCsType");

/// A PARAMETRICCRS has exactly 1 coordinate axis.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsAxisCount;
structural_prop!(ParametriccrsAxisCount, "ParametriccrsAxisCount");

/// ID sub-keyword at the PARAMETRICCRS level is optional.
///
/// Source: OGC 18-010r7 §8.6 — Parametric CRS
pub struct ParametriccrsIdOptional;
structural_prop!(ParametriccrsIdOptional, "ParametriccrsIdOptional");
```

---

### §6.7 — TIMECRS (Temporal CRS)

**Reference:** OGC 18-010r7 §8.7 — Temporal CRS

A `TIMECRS` encodes a 1D temporal CRS. The datum is `TDATUM` (also `TIMEDATUM`). The CS type is
`temporal` and there is exactly 1 axis with direction `future` or `past`. The `TIMEORIGIN`
sub-keyword within `TDATUM` provides the epoch of the temporal datum.

**Minimal example (GPS Time):**

```
TIMECRS["GPS Time",
  TDATUM["Time origin", TIMEORIGIN[1980-01-06T00:00:00Z]],
  CS[temporal, 1],
  AXIS["Time (T)", future, TIMEUNIT["second", 1]]]
```

```rust
/// TIMECRS keyword introduces a temporal CRS object.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsKeyword;
structural_prop!(TimecrsKeyword, "TimecrsKeyword");

/// First element of a TIMECRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsNamePresent;
structural_prop!(TimecrsNamePresent, "TimecrsNamePresent");

/// TDATUM (or TIMEDATUM) sub-keyword is mandatory within TIMECRS.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsDatumPresent;
structural_prop!(TimecrsDatumPresent, "TimecrsDatumPresent");

/// CS type in a TIMECRS must be `temporal`.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsCsType;
structural_prop!(TimecrsCsType, "TimecrsCsType");

/// A TIMECRS has exactly 1 coordinate axis.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsAxisCount;
structural_prop!(TimecrsAxisCount, "TimecrsAxisCount");

/// The single axis in a TIMECRS must have direction `future` or `past`.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsAxisDirectionFutureOrPast;
structural_prop!(TimecrsAxisDirectionFutureOrPast, "TimecrsAxisDirectionFutureOrPast");

/// TIMEORIGIN sub-keyword within TDATUM defines the temporal datum origin epoch.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsOriginIsDatetime;
structural_prop!(TimecrsOriginIsDatetime, "TimecrsOriginIsDatetime");

/// TIMEDATUM is an accepted long-form alias for TDATUM within TIMECRS.
///
/// Source: OGC 18-010r7 §8.7 — Temporal CRS
pub struct TimecrsTimedatumAlias;
structural_prop!(TimecrsTimedatumAlias, "TimecrsTimedatumAlias");
```

#### §6.7.2 — TIMEORIGIN sub-keyword

**Reference:** OGC 18-010r7 §8.7.2 — Temporal datum

The `TIMEORIGIN` sub-keyword within `TDATUM` specifies the epoch from which temporal coordinates
are measured. The value must be an ISO 8601-1 extended format datetime string with a mandatory
timezone designator. A date-only string is **NOT** valid — a time component is required.

**ISO 8601-1 extended format:** `YYYY-MM-DDThh:mm:ss[Z | +hh:mm | -hh:mm]`

**Valid examples:**

- `1980-01-06T00:00:00Z` — GPS epoch in UTC
- `2000-01-01T12:00:00Z` — J2000.0 epoch

**Invalid examples:**

- `1980-01-06` — date only (missing time component)
- `1980-01-06T00:00:00` — missing timezone designator

```rust
/// TIMEORIGIN keyword within TDATUM provides the temporal datum origin epoch.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginKeywordPresent;
structural_prop!(TimeoriginKeywordPresent, "TimeoriginKeywordPresent");

/// TIMEORIGIN datetime value must conform to ISO 8601-1 extended format YYYY-MM-DDThh:mm:ss.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginDatetimeIso8601;
structural_prop!(TimeoriginDatetimeIso8601, "TimeoriginDatetimeIso8601");

/// TIMEORIGIN datetime must include a timezone designator (Z or offset); it is mandatory.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginTimezoneRequired;
structural_prop!(TimeoriginTimezoneRequired, "TimeoriginTimezoneRequired");

/// A date-only string (no time component) is NOT valid for TIMEORIGIN.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginDateOnlyInvalid;
structural_prop!(TimeoriginDateOnlyInvalid, "TimeoriginDateOnlyInvalid");

/// The `Z` suffix designates UTC as the timezone for a TIMEORIGIN datetime.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginUtcDesignator;
structural_prop!(TimeoriginUtcDesignator, "TimeoriginUtcDesignator");

/// A signed `+hh:mm` or `-hh:mm` offset designates a UTC-offset timezone for TIMEORIGIN.
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginOffsetDesignator;
structural_prop!(TimeoriginOffsetDesignator, "TimeoriginOffsetDesignator");

/// TIMEORIGIN value is a quoted string (not a bare token).
///
/// Source: OGC 18-010r7 §8.7.2 — Temporal datum
pub struct TimeoriginValueQuoted;
structural_prop!(TimeoriginValueQuoted, "TimeoriginValueQuoted");
```

---

### §6.8 — COMPOUNDCRS (Compound CRS)

**Reference:** OGC 18-010r7 §8.8 — Compound CRS

A `COMPOUNDCRS` combines two or more component CRS objects into a single CRS with an axis count
equal to the sum of the component axis counts. The most common combination is a 2D horizontal CRS
plus a 1D vertical CRS (3D total), but other combinations are valid. Duplicate horizontal CRS
components are not permitted.

**Minimal example (WGS 84 + EGM2008 height, EPSG 9518):**

```
COMPOUNDCRS["WGS 84 + EGM2008 height",
  GEOGCRS["WGS 84", ...],
  VERTCRS["EGM2008 height", ...],
  ID["EPSG", 9518]]
```

**Constraints:**

- Name: non-empty quoted string
- Minimum 2 component CRS sub-objects
- Each component is a complete WKT CRS object
- Total axis count = sum of all component axis counts
- Components must be logically consistent (no duplicate horizontal components)

```rust
/// COMPOUNDCRS keyword introduces a compound CRS object.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsKeyword;
structural_prop!(CompoundcrsKeyword, "CompoundcrsKeyword");

/// First element of a COMPOUNDCRS is a non-empty quoted name string.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsNamePresent;
structural_prop!(CompoundcrsNamePresent, "CompoundcrsNamePresent");

/// A COMPOUNDCRS must contain at least 2 component CRS sub-objects.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsMinTwoComponents;
structural_prop!(CompoundcrsMinTwoComponents, "CompoundcrsMinTwoComponents");

/// Each component of a COMPOUNDCRS is a fully specified CRS WKT object.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsComponentsFullCrs;
structural_prop!(CompoundcrsComponentsFullCrs, "CompoundcrsComponentsFullCrs");

/// Total axis count of a COMPOUNDCRS equals the sum of component CRS axis counts.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsAxisCountSum;
structural_prop!(CompoundcrsAxisCountSum, "CompoundcrsAxisCountSum");

/// A COMPOUNDCRS must not contain two components that both define a horizontal CRS.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsNoDuplicateHorizontal;
structural_prop!(CompoundcrsNoDuplicateHorizontal, "CompoundcrsNoDuplicateHorizontal");

/// ID sub-keyword at the COMPOUNDCRS level is optional.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsIdOptional;
structural_prop!(CompoundcrsIdOptional, "CompoundcrsIdOptional");

/// REMARK sub-keyword at the COMPOUNDCRS level is optional.
///
/// Source: OGC 18-010r7 §8.8 — Compound CRS
pub struct CompoundcrsRemarkOptional;
structural_prop!(CompoundcrsRemarkOptional, "CompoundcrsRemarkOptional");
```

---

### §6.9 — Derived CRS Subtypes

**Reference:** OGC 18-010r7 §8.9 — Derived CRS

A derived CRS is obtained by applying a `DERIVINGCONVERSION` to a base CRS. Each derived CRS
subtype is named after the type of the base CRS it extends. All derived CRS types share the
mandatory `DERIVINGCONVERSION` sub-keyword and a `CS` sub-keyword with an appropriate type.

The base CRS is embedded using a `BASExxxCRS` keyword (e.g., `BASEGEODCRS`, `BASEPROJCRS`).

**Subtypes and their typical uses:**

| Keyword                  | Base CRS type    | Typical use                         |
| ------------------------ | ---------------- | ----------------------------------- |
| `DERIVEDGEODCRS`         | Geographic       | Rotated or tilted geographic grids  |
| `DERIVEDPROJCRS`         | Projected        | Rotated projected grids             |
| `DERIVEDVERTCRS`         | Vertical         | Datum-shifted heights               |
| `DERIVEDENGCRS`          | Engineering      | Tilted engineering frames           |
| `DERIVEDPARAMETRICCRS`   | Parametric       | Derived physical quantity CRS       |
| `DERIVEDTIMECRS`         | Temporal         | Time-shifted temporal CRS           |

```rust
/// DERIVEDGEODCRS keyword introduces a derived geodetic CRS with a geographic base.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedgeodcrsKeyword;
structural_prop!(DerivedgeodcrsKeyword, "DerivedgeodcrsKeyword");

/// The base of a DERIVEDGEODCRS must be a geographic (ellipsoidal) CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedgeodcrsBaseIsGeographic;
structural_prop!(DerivedgeodcrsBaseIsGeographic, "DerivedgeodcrsBaseIsGeographic");

/// DERIVEDPROJCRS keyword introduces a derived CRS whose base is a projected CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedprojcrsKeyword;
structural_prop!(DerivedprojcrsKeyword, "DerivedprojcrsKeyword");

/// The base of a DERIVEDPROJCRS must be a projected CRS (embedded as BASEPROJCRS).
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedprojcrsBaseIsProjected;
structural_prop!(DerivedprojcrsBaseIsProjected, "DerivedprojcrsBaseIsProjected");

/// DERIVEDVERTCRS keyword introduces a derived CRS whose base is a vertical CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedvertcrsKeyword;
structural_prop!(DerivedvertcrsKeyword, "DerivedvertcrsKeyword");

/// The base of a DERIVEDVERTCRS must be a vertical CRS (embedded as BASEVERTCRS).
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedvertcrsBaseIsVertical;
structural_prop!(DerivedvertcrsBaseIsVertical, "DerivedvertcrsBaseIsVertical");

/// DERIVEDENGCRS keyword introduces a derived CRS whose base is an engineering CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedengcrsKeyword;
structural_prop!(DerivedengcrsKeyword, "DerivedengcrsKeyword");

/// The base of a DERIVEDENGCRS must be an engineering CRS (embedded as BASEENGCRS).
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedengcrsBaseIsEngineering;
structural_prop!(DerivedengcrsBaseIsEngineering, "DerivedengcrsBaseIsEngineering");

/// DERIVEDPARAMETRICCRS keyword introduces a derived CRS whose base is a parametric CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedparametriccrsKeyword;
structural_prop!(DerivedparametriccrsKeyword, "DerivedparametriccrsKeyword");

/// DERIVEDTIMECRS keyword introduces a derived CRS whose base is a temporal CRS.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedtimecrsKeyword;
structural_prop!(DerivedtimecrsKeyword, "DerivedtimecrsKeyword");

/// DERIVINGCONVERSION sub-keyword is mandatory in every derived CRS type.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedcrsDerivingConversionPresent;
structural_prop!(DerivedcrsDerivingConversionPresent, "DerivedcrsDerivingConversionPresent");

/// CS type in a derived CRS must be appropriate for the derived CRS subtype.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedcrsProperCsType;
structural_prop!(DerivedcrsProperCsType, "DerivedcrsProperCsType");

/// Each derived CRS type embeds its base CRS using the corresponding BASExxxCRS keyword.
///
/// Source: OGC 18-010r7 §8.9 — Derived CRS
pub struct DerivedcrsBaseCrsKeyword;
structural_prop!(DerivedcrsBaseCrsKeyword, "DerivedcrsBaseCrsKeyword");
```

---

### §7 — CS sub-keyword (Coordinate System)

**Reference:** OGC 18-010r7 §7.5.1 — Coordinate system

The `CS` sub-keyword identifies the coordinate system type and the number of axes. It must be
immediately followed by exactly `n` `AXIS` sub-keywords. The type is a case-insensitive string
code from the enumerated set below. The value `n` is a positive integer.

**Syntax:** `CS[type, n]`

**Type-to-axis-count relationships:**

| CS type       | Typical axis count | Typical CRS                  |
| ------------- | ------------------ | -----------------------------|
| ellipsoidal   | 2 or 3             | geographic CRS               |
| Cartesian     | 2 or 3             | projected / geocentric CRS   |
| vertical      | 1                  | vertical CRS                 |
| temporal      | 1                  | temporal CRS                 |
| parametric    | 1                  | parametric CRS               |
| ordinal       | any                | engineering CRS              |
| affine        | 2 or 3             | engineering CRS              |
| polar         | 2                  | engineering CRS              |
| cylindrical   | 3                  | engineering CRS              |
| spherical     | 3                  | geodetic (3D geocentric)     |

```rust
/// CS keyword introduces a coordinate system sub-object with a type code and axis count.
///
/// Source: OGC 18-010r7 §7.5.1 — Coordinate system
pub struct CsKeywordPresent;
structural_prop!(CsKeywordPresent, "CsKeywordPresent");

/// CS type must be one of the enumerated type code strings defined by the standard.
///
/// Source: OGC 18-010r7 §7.5.1 — Coordinate system
pub struct CsTypeEnumerated;
structural_prop!(CsTypeEnumerated, "CsTypeEnumerated");

/// The axis count `n` in CS must equal the number of AXIS sub-keywords that follow it.
///
/// Source: OGC 18-010r7 §7.5.1 — Coordinate system
pub struct CsAxisCountMatchesN;
structural_prop!(CsAxisCountMatchesN, "CsAxisCountMatchesN");

/// CS type `ellipsoidal` — 2D or 3D latitude/longitude(/height) coordinate system.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeEllipsoidal;
structural_prop!(CsTypeEllipsoidal, "CsTypeEllipsoidal");

/// CS type `Cartesian` — 2D or 3D orthogonal rectilinear coordinate system.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeCartesian;
structural_prop!(CsTypeCartesian, "CsTypeCartesian");

/// CS type `vertical` — 1D coordinate system for height or depth.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeVertical;
structural_prop!(CsTypeVertical, "CsTypeVertical");

/// CS type `temporal` — 1D coordinate system for time.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeTemporal;
structural_prop!(CsTypeTemporal, "CsTypeTemporal");

/// CS type `parametric` — 1D coordinate system for a non-spatial physical quantity.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeParametric;
structural_prop!(CsTypeParametric, "CsTypeParametric");

/// CS type `ordinal` — coordinate system with ordered labels but no defined unit.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeOrdinal;
structural_prop!(CsTypeOrdinal, "CsTypeOrdinal");

/// CS type `affine` — coordinate system with a defined origin and potentially oblique axes.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeAffine;
structural_prop!(CsTypeAffine, "CsTypeAffine");

/// CS type `polar` — 2D coordinate system with radial distance and azimuth angle axes.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypePolar;
structural_prop!(CsTypePolar, "CsTypePolar");

/// CS type `cylindrical` — 3D coordinate system with radial distance, angle, and height axes.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeCylindrical;
structural_prop!(CsTypeCylindrical, "CsTypeCylindrical");

/// CS type `spherical` — 3D coordinate system with two angular and one radial axis.
///
/// Source: OGC 18-010r7 §7.5.1 Table 1 — Coordinate system types
pub struct CsTypeSpherical;
structural_prop!(CsTypeSpherical, "CsTypeSpherical");
```

---

### §8 — Conformance Classes

**Reference:** OGC 18-010r7 §A — Conformance classes

OGC 18-010r7 defines 18 conformance classes. An implementation must pass all requirements in
each class it claims to support. Each class has a URI of the form:
`http://www.opengis.net/spec/WKT-CRS/2.0/conf/<class-name>`

Conformance classes build on each other: the basic CRS class is a prerequisite for all CRS-type
classes. An implementation claiming geodetic CRS support implicitly claims basic CRS support.

```rust
/// Conformance class WKT2-2019 CRS — basic WKT string encoding requirements.
///
/// Source: OGC 18-010r7 §A.1 — Conformance class: WKT2-2019 CRS
pub struct ConformanceWkt2BasicCrs;
structural_prop!(ConformanceWkt2BasicCrs, "ConformanceWkt2BasicCrs");

/// Conformance class WKT2-2019 Geodetic CRS — GEODCRS / GEOGCRS encoding.
///
/// Source: OGC 18-010r7 §A.2 — Conformance class: WKT2-2019 Geodetic CRS
pub struct ConformanceWkt2GeodeticCrs;
structural_prop!(ConformanceWkt2GeodeticCrs, "ConformanceWkt2GeodeticCrs");

/// Conformance class WKT2-2019 Projected CRS — PROJCRS encoding.
///
/// Source: OGC 18-010r7 §A.3 — Conformance class: WKT2-2019 Projected CRS
pub struct ConformanceWkt2ProjectedCrs;
structural_prop!(ConformanceWkt2ProjectedCrs, "ConformanceWkt2ProjectedCrs");

/// Conformance class WKT2-2019 Vertical CRS — VERTCRS encoding.
///
/// Source: OGC 18-010r7 §A.4 — Conformance class: WKT2-2019 Vertical CRS
pub struct ConformanceWkt2VerticalCrs;
structural_prop!(ConformanceWkt2VerticalCrs, "ConformanceWkt2VerticalCrs");

/// Conformance class WKT2-2019 Engineering CRS — ENGCRS encoding.
///
/// Source: OGC 18-010r7 §A.5 — Conformance class: WKT2-2019 Engineering CRS
pub struct ConformanceWkt2EngineeringCrs;
structural_prop!(ConformanceWkt2EngineeringCrs, "ConformanceWkt2EngineeringCrs");

/// Conformance class WKT2-2019 Parametric CRS — PARAMETRICCRS encoding.
///
/// Source: OGC 18-010r7 §A.6 — Conformance class: WKT2-2019 Parametric CRS
pub struct ConformanceWkt2ParametricCrs;
structural_prop!(ConformanceWkt2ParametricCrs, "ConformanceWkt2ParametricCrs");

/// Conformance class WKT2-2019 Temporal CRS — TIMECRS encoding.
///
/// Source: OGC 18-010r7 §A.7 — Conformance class: WKT2-2019 Temporal CRS
pub struct ConformanceWkt2TemporalCrs;
structural_prop!(ConformanceWkt2TemporalCrs, "ConformanceWkt2TemporalCrs");

/// Conformance class WKT2-2019 Compound CRS — COMPOUNDCRS encoding.
///
/// Source: OGC 18-010r7 §A.8 — Conformance class: WKT2-2019 Compound CRS
pub struct ConformanceWkt2CompoundCrs;
structural_prop!(ConformanceWkt2CompoundCrs, "ConformanceWkt2CompoundCrs");

/// Conformance class WKT2-2019 Derived CRS — all derived CRS encodings.
///
/// Source: OGC 18-010r7 §A.9 — Conformance class: WKT2-2019 Derived CRS
pub struct ConformanceWkt2DerivedCrs;
structural_prop!(ConformanceWkt2DerivedCrs, "ConformanceWkt2DerivedCrs");

/// Conformance class WKT2-2019 DATUM — geodetic, vertical, and engineering datum encoding.
///
/// Source: OGC 18-010r7 §A.10 — Conformance class: WKT2-2019 DATUM
pub struct ConformanceWkt2Datum;
structural_prop!(ConformanceWkt2Datum, "ConformanceWkt2Datum");

/// Conformance class WKT2-2019 ELLIPSOID — ellipsoid encoding.
///
/// Source: OGC 18-010r7 §A.11 — Conformance class: WKT2-2019 ELLIPSOID
pub struct ConformanceWkt2Ellipsoid;
structural_prop!(ConformanceWkt2Ellipsoid, "ConformanceWkt2Ellipsoid");

/// Conformance class WKT2-2019 PRIMEM — prime meridian encoding.
///
/// Source: OGC 18-010r7 §A.12 — Conformance class: WKT2-2019 PRIMEM
pub struct ConformanceWkt2Primem;
structural_prop!(ConformanceWkt2Primem, "ConformanceWkt2Primem");

/// Conformance class WKT2-2019 CS — coordinate system encoding.
///
/// Source: OGC 18-010r7 §A.13 — Conformance class: WKT2-2019 CS
pub struct ConformanceWkt2CoordinateSystem;
structural_prop!(ConformanceWkt2CoordinateSystem, "ConformanceWkt2CoordinateSystem");

/// Conformance class WKT2-2019 AXIS — coordinate axis encoding.
///
/// Source: OGC 18-010r7 §A.14 — Conformance class: WKT2-2019 AXIS
pub struct ConformanceWkt2Axis;
structural_prop!(ConformanceWkt2Axis, "ConformanceWkt2Axis");

/// Conformance class WKT2-2019 UNIT — unit of measure encoding.
///
/// Source: OGC 18-010r7 §A.15 — Conformance class: WKT2-2019 UNIT
pub struct ConformanceWkt2Unit;
structural_prop!(ConformanceWkt2Unit, "ConformanceWkt2Unit");

/// Conformance class WKT2-2019 CONVERSION — coordinate conversion encoding.
///
/// Source: OGC 18-010r7 §A.16 — Conformance class: WKT2-2019 CONVERSION
pub struct ConformanceWkt2Conversion;
structural_prop!(ConformanceWkt2Conversion, "ConformanceWkt2Conversion");

/// Conformance class WKT2-2019 ID — identifier (ID) sub-keyword encoding.
///
/// Source: OGC 18-010r7 §A.17 — Conformance class: WKT2-2019 ID
pub struct ConformanceWkt2Id;
structural_prop!(ConformanceWkt2Id, "ConformanceWkt2Id");

/// Conformance class WKT2-2019 Backwards compat — round-trip fidelity with WKT1.
///
/// Source: OGC 18-010r7 §A.18 — Conformance class: WKT2-2019 Backwards compat
pub struct ConformanceWkt2BackwardsCompat;
structural_prop!(ConformanceWkt2BackwardsCompat, "ConformanceWkt2BackwardsCompat");
```

---

### §9 — ID sub-keyword (Identifier)

**Reference:** OGC 18-010r7 §7.5.5 — Identifier

The `ID` sub-keyword attaches an authority-scoped identifier to any WKT object. Multiple `ID`
sub-keywords may appear in a single object, one per authority. The optional `version` element
carries a version string or integer. The optional `URI` sub-keyword provides a resolvable
identifier URI.

**Syntax variants:**

```
ID["EPSG", 4326]
ID["EPSG", 4326, "8.9"]
ID["EPSG", 4326, "8.9", URI["urn:ogc:def:crs:EPSG::4326"]]
```

**Constraints:**

- `authority` — non-empty quoted string (e.g., "EPSG", "OGC", "IAU")
- `code` — quoted string or unquoted integer
- `version` — optional: quoted string or number
- `URI` — optional sub-keyword carrying a URI string

```rust
/// ID keyword introduces an authority-scoped identifier sub-object.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdKeywordPresent;
structural_prop!(IdKeywordPresent, "IdKeywordPresent");

/// Authority name within ID is a non-empty quoted string (e.g., "EPSG", "OGC").
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdAuthorityNonEmpty;
structural_prop!(IdAuthorityNonEmpty, "IdAuthorityNonEmpty");

/// Code within ID may be a quoted string or an unquoted integer.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdCodePresentAsStringOrInt;
structural_prop!(IdCodePresentAsStringOrInt, "IdCodePresentAsStringOrInt");

/// Version element within ID is optional; it may be a quoted string or a number.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdVersionOptional;
structural_prop!(IdVersionOptional, "IdVersionOptional");

/// URI sub-keyword within ID is optional; when present it carries a resolvable URI string.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdUriOptional;
structural_prop!(IdUriOptional, "IdUriOptional");

/// Multiple ID sub-keywords are allowed in a single WKT object, one per authority.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdMultipleAllowed;
structural_prop!(IdMultipleAllowed, "IdMultipleAllowed");

/// ID code when given as a quoted string may include non-numeric characters.
///
/// Source: OGC 18-010r7 §7.5.5 — Identifier
pub struct IdCodeStringNonNumericAllowed;
structural_prop!(IdCodeStringNonNumericAllowed, "IdCodeStringNonNumericAllowed");
```

---

### §10 — Backwards Compatibility with WKT1 (ISO 19162:2015)

**Reference:** OGC 18-010r7 §B — Backwards compatibility

WKT2-2019 replaces WKT1 (ISO 19162:2015). Implementations claiming the backwards-compat
conformance class must accept WKT1 strings and should transform them to equivalent WKT2
representations.

**WKT1 vs WKT2 keyword mapping:**

| WKT1 keyword  | WKT2 keyword           | Notes                                      |
| ------------- | ---------------------- | ------------------------------------------ |
| `GEOGCS`      | `GEOGCRS`              | Geographic CRS                             |
| `PROJCS`      | `PROJCRS`              | Projected CRS                              |
| `VERT_CS`     | `VERTCRS`              | Vertical CRS                               |
| `LOCAL_CS`    | `ENGCRS`               | Engineering CRS                            |
| `DATUM`       | `DATUM`                | Same keyword; context differs              |
| `SPHEROID`    | `ELLIPSOID`            | Reference ellipsoid                        |
| `PROJECTION`  | `METHOD` in CONVERSION | Projection method name                     |
| `PARAMETER`   | `PARAMETER`            | Same; now requires explicit UNIT           |
| `AUTHORITY`   | `ID`                   | Authority identifier                       |
| `TOWGS84`     | deprecated             | Helmert transformation to WGS 84           |

```rust
/// WKT1 GEOGCS keyword must be accepted for backwards compatibility.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1GeogcsKeyword;
structural_prop!(Wkt1GeogcsKeyword, "Wkt1GeogcsKeyword");

/// WKT1 PROJCS keyword must be accepted for backwards compatibility.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1ProjcsKeyword;
structural_prop!(Wkt1ProjcsKeyword, "Wkt1ProjcsKeyword");

/// WKT1 VERT_CS keyword must be accepted; maps to VERTCRS in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1VertCsKeyword;
structural_prop!(Wkt1VertCsKeyword, "Wkt1VertCsKeyword");

/// WKT1 LOCAL_CS keyword must be accepted; maps to ENGCRS in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1LocalCsKeyword;
structural_prop!(Wkt1LocalCsKeyword, "Wkt1LocalCsKeyword");

/// WKT1 DATUM keyword is accepted; maps to the geodetic datum role in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1DatumKeyword;
structural_prop!(Wkt1DatumKeyword, "Wkt1DatumKeyword");

/// WKT1 SPHEROID keyword must be accepted; maps to ELLIPSOID in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1SpheroidKeyword;
structural_prop!(Wkt1SpheroidKeyword, "Wkt1SpheroidKeyword");

/// WKT1 PROJECTION keyword must be accepted; maps to METHOD within CONVERSION in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1ProjectionKeyword;
structural_prop!(Wkt1ProjectionKeyword, "Wkt1ProjectionKeyword");

/// In WKT1 a single UNIT applies uniformly to all axes; WKT2 allows per-axis units.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1UnitAppliesToAllAxes;
structural_prop!(Wkt1UnitAppliesToAllAxes, "Wkt1UnitAppliesToAllAxes");

/// WKT1 axis direction strings are uppercase (e.g., NORTH); WKT2 uses lowercase (north).
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1AxisDirectionUppercase;
structural_prop!(Wkt1AxisDirectionUppercase, "Wkt1AxisDirectionUppercase");

/// WKT1 TOWGS84 carries 3-parameter (translation) or 7-parameter (Helmert) transformation.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1ToWgs84SevenParam;
structural_prop!(Wkt1ToWgs84SevenParam, "Wkt1ToWgs84SevenParam");

/// Migration from WKT1 to WKT2 requires keyword remapping (see mapping table above).
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1KeywordRemapping;
structural_prop!(Wkt1KeywordRemapping, "Wkt1KeywordRemapping");

/// WKT1 AUTHORITY sub-keyword corresponds to the ID sub-keyword in WKT2.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1AuthorityToId;
structural_prop!(Wkt1AuthorityToId, "Wkt1AuthorityToId");

/// WKT1 does not have explicit CS or AXIS sub-keywords in all CRS types; they are implied.
///
/// Source: OGC 18-010r7 §B — Backwards compatibility
pub struct Wkt1ImpliedCsStructure;
structural_prop!(Wkt1ImpliedCsStructure, "Wkt1ImpliedCsStructure");
```

---

### §11 — Coordinate Operations in WKT

**Reference:** OGC 18-010r7 §9 — Coordinate operations

Coordinate operations are encoded using `CONVERSION`, `COORDINATEOPERATION`, and related keywords.
A `CONVERSION` names a projection method and lists its `PARAMETER` sub-keywords. A
`COORDINATEOPERATION` carries source and target CRS references and an operation accuracy.

**Key structure:**

```
COORDINATEOPERATION["name",
  SOURCECRS[...full CRS...],
  TARGETCRS[...full CRS...],
  METHOD["name", ID[...]],
  PARAMETER["name", value, UNIT[...]],
  OPERATIONACCURACY[accuracy_in_metres],
  ID[...]]

CONCATENATEDOPERATION["name",
  SOURCECRS[...],
  TARGETCRS[...],
  STEP[COORDINATEOPERATION[...]],
  STEP[COORDINATEOPERATION[...]]]
```

```rust
/// CONVERSION keyword introduces a named coordinate conversion (no datum shift).
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ConversionKeywordPresent;
structural_prop!(ConversionKeywordPresent, "ConversionKeywordPresent");

/// Conversion name is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ConversionNameNonEmpty;
structural_prop!(ConversionNameNonEmpty, "ConversionNameNonEmpty");

/// METHOD keyword within a coordinate operation names the operation method.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct MethodKeywordPresent;
structural_prop!(MethodKeywordPresent, "MethodKeywordPresent");

/// Method name within METHOD is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct MethodNameNonEmpty;
structural_prop!(MethodNameNonEmpty, "MethodNameNonEmpty");

/// PARAMETER keyword provides a named numeric value for a coordinate operation parameter.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterKeywordPresent;
structural_prop!(ParameterKeywordPresent, "ParameterKeywordPresent");

/// Parameter name within PARAMETER is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterNameNonEmpty;
structural_prop!(ParameterNameNonEmpty, "ParameterNameNonEmpty");

/// Parameter value within PARAMETER is a numeric literal.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterValueIsNumber;
structural_prop!(ParameterValueIsNumber, "ParameterValueIsNumber");

/// Each PARAMETER must carry a UNIT sub-keyword describing the unit of its value.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterUnitPresent;
structural_prop!(ParameterUnitPresent, "ParameterUnitPresent");

/// PARAMETERFILE provides a file-based alternative to PARAMETER for grid-based methods.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterFileKeyword;
structural_prop!(ParameterFileKeyword, "ParameterFileKeyword");

/// PARAMETERFILE first element (parameter name) is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterFileNameNonEmpty;
structural_prop!(ParameterFileNameNonEmpty, "ParameterFileNameNonEmpty");

/// PARAMETERFILE second element (file name) is a non-empty quoted string.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct ParameterFileFilenameNonEmpty;
structural_prop!(ParameterFileFilenameNonEmpty, "ParameterFileFilenameNonEmpty");

/// COORDINATEOPERATION keyword introduces a complete coordinate transformation object.
///
/// Source: OGC 18-010r7 §9.3 — Coordinate transformation
pub struct CoordinateOperationKeyword;
structural_prop!(CoordinateOperationKeyword, "CoordinateOperationKeyword");

/// SOURCECRS sub-keyword within COORDINATEOPERATION is mandatory.
///
/// Source: OGC 18-010r7 §9.3 — Coordinate transformation
pub struct CoordinateOperationSourceCrsPresent;
structural_prop!(CoordinateOperationSourceCrsPresent, "CoordinateOperationSourceCrsPresent");

/// TARGETCRS sub-keyword within COORDINATEOPERATION is mandatory.
///
/// Source: OGC 18-010r7 §9.3 — Coordinate transformation
pub struct CoordinateOperationTargetCrsPresent;
structural_prop!(CoordinateOperationTargetCrsPresent, "CoordinateOperationTargetCrsPresent");

/// OPERATIONACCURACY value is a positive real number giving accuracy in metres.
///
/// Source: OGC 18-010r7 §9.3 — Coordinate transformation
pub struct OperationAccuracyPositive;
structural_prop!(OperationAccuracyPositive, "OperationAccuracyPositive");

/// OPERATIONACCURACY sub-keyword within COORDINATEOPERATION is optional.
///
/// Source: OGC 18-010r7 §9.3 — Coordinate transformation
pub struct OperationAccuracyOptional;
structural_prop!(OperationAccuracyOptional, "OperationAccuracyOptional");

/// METHOD ID sub-keyword within a coordinate operation is optional but recommended.
///
/// Source: OGC 18-010r7 §9.2 — Coordinate conversion
pub struct MethodIdOptional;
structural_prop!(MethodIdOptional, "MethodIdOptional");

/// CONCATENATEDOPERATION chains two or more coordinate operations applied sequentially.
///
/// Source: OGC 18-010r7 §9.4 — Concatenated operation
pub struct ConcatenatedOperationKeyword;
structural_prop!(ConcatenatedOperationKeyword, "ConcatenatedOperationKeyword");

/// STEP sub-keyword within CONCATENATEDOPERATION wraps an individual operation component.
///
/// Source: OGC 18-010r7 §9.4 — Concatenated operation
pub struct ConcatenatedOperationStepPresent;
structural_prop!(ConcatenatedOperationStepPresent, "ConcatenatedOperationStepPresent");

/// PASSTHROUGH operation forwards a subset of axes unchanged while transforming others.
///
/// Source: OGC 18-010r7 §9.5 — Pass-through coordinate operation
pub struct PassThroughOperationKeyword;
structural_prop!(PassThroughOperationKeyword, "PassThroughOperationKeyword");
```

---

### Cross-Cutting Validity Props

These props apply to any WKT string regardless of CRS type and enforce the universal syntax rules
from §6 of the standard.

```rust
/// All delimiter pairs in one WKT string must use the same style (all [] or all ()).
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktDelimiterConsistencyCross;
structural_prop!(WktDelimiterConsistencyCross, "WktDelimiterConsistencyCross");

/// Decimal separator is always `.` in numeric literals throughout a WKT string.
///
/// Source: OGC 18-010r7 §6.3.2 — Unsigned integer and floating-point numbers
pub struct WktDecimalSeparatorPeriodCross;
structural_prop!(WktDecimalSeparatorPeriodCross, "WktDecimalSeparatorPeriodCross");

/// Embedded double-quotes in any quoted string are escaped as two consecutive double-quotes.
///
/// Source: OGC 18-010r7 §6.3.1 — Quoted strings
pub struct WktDoubleQuoteEscapeCross;
structural_prop!(WktDoubleQuoteEscapeCross, "WktDoubleQuoteEscapeCross");

/// Keywords are matched case-insensitively throughout a WKT string.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktKeywordCaseInsensitiveCross;
structural_prop!(WktKeywordCaseInsensitiveCross, "WktKeywordCaseInsensitiveCross");

/// Insignificant whitespace between tokens is semantically ignored everywhere.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktOptionalWhitespaceCross;
structural_prop!(WktOptionalWhitespaceCross, "WktOptionalWhitespaceCross");

/// Parsing and re-serialising a WKT string produces a string equivalent to the original.
///
/// Source: OGC 18-010r7 §6 — Requirements class "WKT string"
pub struct WktRoundTripPreservationCross;
structural_prop!(WktRoundTripPreservationCross, "WktRoundTripPreservationCross");

/// Every quoted name in a WKT object is non-empty (zero-length names are not permitted).
///
/// Source: OGC 18-010r7 §6.3.1 — Quoted strings
pub struct WktNameNeverEmpty;
structural_prop!(WktNameNeverEmpty, "WktNameNeverEmpty");

/// Optional sub-keywords that are absent do not affect the validity of the WKT string.
///
/// Source: OGC 18-010r7 §6.2 — WKT string syntax
pub struct WktAbsentOptionalNotInvalid;
structural_prop!(WktAbsentOptionalNotInvalid, "WktAbsentOptionalNotInvalid");
```

---

## Summary — All Prop Names by Section

### §1 General Syntax (14 props)

`WktKeywordCaseInsensitive`, `WktSquareBracketDelimiters`, `WktParenthesisDelimiters`,
`WktDelimiterConsistency`, `WktDecimalSeparatorPeriod`, `WktIntegerNumbers`,
`WktFloatingPointNumbers`, `WktDoubleQuoteStrings`, `WktDoubleQuoteEscape`,
`WktCommaElementSeparator`, `WktOptionalWhitespace`, `WktKeywordAsciiAlpha`,
`WktOptionalElementOrdering`, `WktRoundTripPreservation`

### §5.2 UNIT (13 props)

`UnitKeywordPresent`, `UnitNameNonEmpty`, `UnitConversionFactorPositive`,
`UnitLengthFactorToMetres`, `UnitAngleFactorToRadians`, `UnitTimeFactorToSeconds`,
`UnitScaleFactorToUnity`, `LengthUnitKeyword`, `AngleUnitKeyword`, `ScaleUnitKeyword`,
`TimeUnitKeyword`, `ParametricUnitKeyword`, `UnitIdOptional`

### §5.3 AXIS — keyword-level (7 props)

`AxisKeywordPresent`, `AxisNameNonEmpty`, `AxisAbbreviationInParentheses`,
`AxisDirectionEnumerated`, `AxisOrderOptional`, `AxisOrderPositiveInteger`, `AxisUnitOptional`

### §5.3 AXIS — direction codes (36 props)

`AxisDirectionNorth`, `AxisDirectionSouth`, `AxisDirectionEast`, `AxisDirectionWest`,
`AxisDirectionUp`, `AxisDirectionDown`, `AxisDirectionNorthNorthEast`,
`AxisDirectionNorthEast`, `AxisDirectionEastNorthEast`, `AxisDirectionEastSouthEast`,
`AxisDirectionSouthEast`, `AxisDirectionSouthSouthEast`, `AxisDirectionSouthSouthWest`,
`AxisDirectionSouthWest`, `AxisDirectionWestSouthWest`, `AxisDirectionWestNorthWest`,
`AxisDirectionNorthWest`, `AxisDirectionNorthNorthWest`, `AxisDirectionGeocentricX`,
`AxisDirectionGeocentricY`, `AxisDirectionGeocentricZ`, `AxisDirectionColumnPositive`,
`AxisDirectionColumnNegative`, `AxisDirectionRowPositive`, `AxisDirectionRowNegative`,
`AxisDirectionDisplayRight`, `AxisDirectionDisplayLeft`, `AxisDirectionDisplayUp`,
`AxisDirectionDisplayDown`, `AxisDirectionFuture`, `AxisDirectionPast`,
`AxisDirectionTowards`, `AxisDirectionAwayFrom`, `AxisDirectionClockwise`,
`AxisDirectionCounterClockwise`, `AxisDirectionUnspecified`

### §5.4 REMARK (4 props)

`RemarkKeywordOptional`, `RemarkTextQuotedString`, `RemarkPositionLast`,
`RemarkTextMayBeEmpty`

### §6.2 GEODCRS / GEOGCRS (10 props)

`GeodcrsKeyword`, `GeogcrsKeywordAlias`, `GeodcrsNamePresent`, `GeodcrsDatumPresent`,
`GeodcrsEllipsoidPresent`, `GeodcrsPrimeMeridianOptional`, `GeodcrsCsPresent`,
`GeodcrsAxisCountMatchesCsType`, `GeodcrsIdOptional`, `GeodcrsRemarkOptional`

### §6.2.2 DATUM (6 props)

`DatumNameNonEmpty`, `DatumEllipsoidMandatory`, `DatumAnchorOptional`,
`DatumToWgs84Deprecated`, `DatumToWgs84SevenParams`, `DatumGeodeticDatumAlias`

### §6.2.3 ELLIPSOID (7 props)

`EllipsoidKeywordPresent`, `EllipsoidNameNonEmpty`, `EllipsoidSemiMajorAxisPositive`,
`EllipsoidInverseFlatteningNonNegative`, `EllipsoidSphericalWhenRfZero`,
`EllipsoidLengthUnitOptional`, `EllipsoidIdOptional`

### §6.2.4 PRIMEM (7 props)

`PrimemKeywordPresent`, `PrimemNameNonEmpty`, `PrimemLongitudeRange`,
`PrimemLongitudeInParentAngularUnit`, `PrimemAngleUnitOptional`, `PrimemIdOptional`,
`PrimemGreenwichDefault`

### §6.3 PROJCRS (13 props)

`ProjcrsKeyword`, `ProjcrsNamePresent`, `ProjcrsBaseGeodcrsPresent`,
`ProjcrsConversionPresent`, `ProjcrsConversionMethodPresent`, `ProjcrsMethodNameNonEmpty`,
`ProjcrsParameterNameNonEmpty`, `ProjcrsParameterValueIsNumber`,
`ProjcrsParameterUnitPresent`, `ProjcrsCartesianCsType`, `ProjcrsAxisCountIsTwo`,
`ProjcrsLinearUnit`, `ProjcrsIdOptional`

### §6.4 VERTCRS (8 props)

`VertcrsKeyword`, `VertcrsNamePresent`, `VertcrsDatumPresent`, `VertcrsCsType`,
`VertcrsAxisCount`, `VertcrsAxisDirectionUpOrDown`, `VertcrsLinearUnit`,
`VertcrsVerticalDatumAlias`

### §6.5 ENGCRS (7 props)

`EngcrsKeyword`, `EngineeringcrsKeywordAlias`, `EngcrsNamePresent`, `EngcrsDatumPresent`,
`EngcrsDatumNameNonEmpty`, `EngcrsCsType`, `EngcrsIdOptional`

### §6.6 PARAMETRICCRS (7 props)

`ParametriccrsKeyword`, `ParametriccrsNamePresent`, `ParametriccrsDatumPresent`,
`ParametriccrsDatumNameNonEmpty`, `ParametriccrsCsType`, `ParametriccrsAxisCount`,
`ParametriccrsIdOptional`

### §6.7 TIMECRS (8 props)

`TimecrsKeyword`, `TimecrsNamePresent`, `TimecrsDatumPresent`, `TimecrsCsType`,
`TimecrsAxisCount`, `TimecrsAxisDirectionFutureOrPast`, `TimecrsOriginIsDatetime`,
`TimecrsTimedatumAlias`

### §6.7.2 TIMEORIGIN (7 props)

`TimeoriginKeywordPresent`, `TimeoriginDatetimeIso8601`, `TimeoriginTimezoneRequired`,
`TimeoriginDateOnlyInvalid`, `TimeoriginUtcDesignator`, `TimeoriginOffsetDesignator`,
`TimeoriginValueQuoted`

### §6.8 COMPOUNDCRS (8 props)

`CompoundcrsKeyword`, `CompoundcrsNamePresent`, `CompoundcrsMinTwoComponents`,
`CompoundcrsComponentsFullCrs`, `CompoundcrsAxisCountSum`, `CompoundcrsNoDuplicateHorizontal`,
`CompoundcrsIdOptional`, `CompoundcrsRemarkOptional`

### §6.9 Derived CRS (13 props)

`DerivedgeodcrsKeyword`, `DerivedgeodcrsBaseIsGeographic`, `DerivedprojcrsKeyword`,
`DerivedprojcrsBaseIsProjected`, `DerivedvertcrsKeyword`, `DerivedvertcrsBaseIsVertical`,
`DerivedengcrsKeyword`, `DerivedengcrsBaseIsEngineering`, `DerivedparametriccrsKeyword`,
`DerivedtimecrsKeyword`, `DerivedcrsDerivingConversionPresent`, `DerivedcrsProperCsType`,
`DerivedcrsBaseCrsKeyword`

### §7 CS sub-keyword (13 props)

`CsKeywordPresent`, `CsTypeEnumerated`, `CsAxisCountMatchesN`, `CsTypeEllipsoidal`,
`CsTypeCartesian`, `CsTypeVertical`, `CsTypeTemporal`, `CsTypeParametric`,
`CsTypeOrdinal`, `CsTypeAffine`, `CsTypePolar`, `CsTypeCylindrical`, `CsTypeSpherical`

### §8 Conformance Classes (18 props)

`ConformanceWkt2BasicCrs`, `ConformanceWkt2GeodeticCrs`, `ConformanceWkt2ProjectedCrs`,
`ConformanceWkt2VerticalCrs`, `ConformanceWkt2EngineeringCrs`, `ConformanceWkt2ParametricCrs`,
`ConformanceWkt2TemporalCrs`, `ConformanceWkt2CompoundCrs`, `ConformanceWkt2DerivedCrs`,
`ConformanceWkt2Datum`, `ConformanceWkt2Ellipsoid`, `ConformanceWkt2Primem`,
`ConformanceWkt2CoordinateSystem`, `ConformanceWkt2Axis`, `ConformanceWkt2Unit`,
`ConformanceWkt2Conversion`, `ConformanceWkt2Id`, `ConformanceWkt2BackwardsCompat`

### §9 ID sub-keyword (7 props)

`IdKeywordPresent`, `IdAuthorityNonEmpty`, `IdCodePresentAsStringOrInt`,
`IdVersionOptional`, `IdUriOptional`, `IdMultipleAllowed`, `IdCodeStringNonNumericAllowed`

### §10 Backwards Compatibility with WKT1 (13 props)

`Wkt1GeogcsKeyword`, `Wkt1ProjcsKeyword`, `Wkt1VertCsKeyword`, `Wkt1LocalCsKeyword`,
`Wkt1DatumKeyword`, `Wkt1SpheroidKeyword`, `Wkt1ProjectionKeyword`,
`Wkt1UnitAppliesToAllAxes`, `Wkt1AxisDirectionUppercase`, `Wkt1ToWgs84SevenParam`,
`Wkt1KeywordRemapping`, `Wkt1AuthorityToId`, `Wkt1ImpliedCsStructure`

### §11 Coordinate Operations (20 props)

`ConversionKeywordPresent`, `ConversionNameNonEmpty`, `MethodKeywordPresent`,
`MethodNameNonEmpty`, `ParameterKeywordPresent`, `ParameterNameNonEmpty`,
`ParameterValueIsNumber`, `ParameterUnitPresent`, `ParameterFileKeyword`,
`ParameterFileNameNonEmpty`, `ParameterFileFilenameNonEmpty`, `CoordinateOperationKeyword`,
`CoordinateOperationSourceCrsPresent`, `CoordinateOperationTargetCrsPresent`,
`OperationAccuracyPositive`, `OperationAccuracyOptional`, `MethodIdOptional`,
`ConcatenatedOperationKeyword`, `ConcatenatedOperationStepPresent`,
`PassThroughOperationKeyword`

### Cross-cutting (8 props)

`WktDelimiterConsistencyCross`, `WktDecimalSeparatorPeriodCross`,
`WktDoubleQuoteEscapeCross`, `WktKeywordCaseInsensitiveCross`,
`WktOptionalWhitespaceCross`, `WktRoundTripPreservationCross`,
`WktNameNeverEmpty`, `WktAbsentOptionalNotInvalid`

---

**Total: 230 props across 17 sections.**
