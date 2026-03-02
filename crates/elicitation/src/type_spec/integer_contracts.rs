//! [`ElicitSpec`](crate::ElicitSpec) implementations for integer contract types.
//!
//! These are the constrained verification wrappers from `verification::types` —
//! e.g., `I8Positive` (value > 0), `I16NonZero` (value != 0), etc.
//!
//! Note: `I8Range<MIN, MAX>` and similar const-generic Range types are intentionally
//! omitted here; their constraints are parameterized and cannot be described with a
//! single static `TypeSpec`. Agents should query the base integer type instead.

use crate::verification::types::{
    I8NonNegative, I8NonZero, I8Positive, I16NonNegative, I16NonZero, I16Positive, I32NonNegative,
    I32NonZero, I32Positive, I64NonNegative, I64NonZero, I64Positive, I128NonNegative, I128NonZero,
    I128Positive, IsizeNonNegative, IsizeNonZero, IsizePositive, U8NonZero, U8Positive, U16NonZero,
    U16Positive, U32NonZero, U32Positive, U64NonZero, U64Positive, U128NonZero, U128Positive,
    UsizeNonZero, UsizePositive,
};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── Macro ────────────────────────────────────────────────────────────────────

/// Generate `ElicitSpec` + inventory submission for an integer contract type.
macro_rules! impl_integer_contract_spec {
    (
        type     = $ty:ty,
        name     = $name:literal,
        base     = $base:literal,
        summary  = $summary:literal,
        requires = [$(($label:literal, $desc:literal, $expr:literal)),+ $(,)?] $(,)?
    ) => {
        impl ElicitSpec for $ty {
            fn type_spec() -> TypeSpec {
                let requires = SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![
                        $(
                            SpecEntryBuilder::default()
                                .label($label.to_string())
                                .description($desc.to_string())
                                .expression(Some($expr.to_string()))
                                .build()
                                .expect("valid entry"),
                        )+
                    ])
                    .build()
                    .expect("valid requires");

                let related = SpecCategoryBuilder::default()
                    .name("related".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("base_type".to_string())
                            .description(concat!("Wraps `", $base, "`. Use describe_type(\"", $base, "\") for full range information.").to_string())
                            .build()
                            .expect("valid entry"),
                    ])
                    .build()
                    .expect("valid related");

                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![requires, related])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new($name, <$ty as ElicitSpec>::type_spec));
    };
}

// ── Signed i8 ────────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = I8Positive,
    name    = "I8Positive",
    base    = "i8",
    summary = "A positive 8-bit signed integer (value > 0, range 1..=127).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I8NonNegative,
    name    = "I8NonNegative",
    base    = "i8",
    summary = "A non-negative 8-bit signed integer (value >= 0, range 0..=127).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I8NonZero,
    name    = "I8NonZero",
    base    = "i8",
    summary = "A non-zero 8-bit signed integer (value != 0, range -128..=-1 or 1..=127).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Signed i16 ───────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = I16Positive,
    name    = "I16Positive",
    base    = "i16",
    summary = "A positive 16-bit signed integer (value > 0, range 1..=32767).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I16NonNegative,
    name    = "I16NonNegative",
    base    = "i16",
    summary = "A non-negative 16-bit signed integer (value >= 0, range 0..=32767).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I16NonZero,
    name    = "I16NonZero",
    base    = "i16",
    summary = "A non-zero 16-bit signed integer (value != 0, range -32768..=-1 or 1..=32767).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Unsigned u8 ──────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = U8NonZero,
    name    = "U8NonZero",
    base    = "u8",
    summary = "A non-zero 8-bit unsigned integer (value != 0, range 1..=255).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = U8Positive,
    name    = "U8Positive",
    base    = "u8",
    summary = "A positive 8-bit unsigned integer (value > 0, range 1..=255). Equivalent to U8NonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

// ── Unsigned u16 ─────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = U16NonZero,
    name    = "U16NonZero",
    base    = "u16",
    summary = "A non-zero 16-bit unsigned integer (value != 0, range 1..=65535).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = U16Positive,
    name    = "U16Positive",
    base    = "u16",
    summary = "A positive 16-bit unsigned integer (value > 0, range 1..=65535). Equivalent to U16NonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

// ── Signed i32 ───────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = I32Positive,
    name    = "I32Positive",
    base    = "i32",
    summary = "A positive 32-bit signed integer (value > 0, range 1..=2147483647).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I32NonNegative,
    name    = "I32NonNegative",
    base    = "i32",
    summary = "A non-negative 32-bit signed integer (value >= 0, range 0..=2147483647).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I32NonZero,
    name    = "I32NonZero",
    base    = "i32",
    summary = "A non-zero 32-bit signed integer (value != 0, range -2147483648..=-1 or 1..=2147483647).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Signed i64 ───────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = I64Positive,
    name    = "I64Positive",
    base    = "i64",
    summary = "A positive 64-bit signed integer (value > 0, range 1..=9223372036854775807).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I64NonNegative,
    name    = "I64NonNegative",
    base    = "i64",
    summary = "A non-negative 64-bit signed integer (value >= 0, range 0..=9223372036854775807).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I64NonZero,
    name    = "I64NonZero",
    base    = "i64",
    summary = "A non-zero 64-bit signed integer (value != 0).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Signed i128 ──────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = I128Positive,
    name    = "I128Positive",
    base    = "i128",
    summary = "A positive 128-bit signed integer (value > 0).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I128NonNegative,
    name    = "I128NonNegative",
    base    = "i128",
    summary = "A non-negative 128-bit signed integer (value >= 0).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = I128NonZero,
    name    = "I128NonZero",
    base    = "i128",
    summary = "A non-zero 128-bit signed integer (value != 0).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Signed isize ─────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = IsizePositive,
    name    = "IsizePositive",
    base    = "isize",
    summary = "A positive pointer-sized signed integer (value > 0).",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

impl_integer_contract_spec!(
    type    = IsizeNonNegative,
    name    = "IsizeNonNegative",
    base    = "isize",
    summary = "A non-negative pointer-sized signed integer (value >= 0).",
    requires = [
        ("non_negative", "Value must be zero or greater.", "value >= 0"),
    ],
);

impl_integer_contract_spec!(
    type    = IsizeNonZero,
    name    = "IsizeNonZero",
    base    = "isize",
    summary = "A non-zero pointer-sized signed integer (value != 0).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

// ── Unsigned u32 ─────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = U32NonZero,
    name    = "U32NonZero",
    base    = "u32",
    summary = "A non-zero 32-bit unsigned integer (value != 0, range 1..=4294967295).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = U32Positive,
    name    = "U32Positive",
    base    = "u32",
    summary = "A positive 32-bit unsigned integer (value > 0, range 1..=4294967295). Equivalent to U32NonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

// ── Unsigned u64 ─────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = U64NonZero,
    name    = "U64NonZero",
    base    = "u64",
    summary = "A non-zero 64-bit unsigned integer (value != 0, range 1..=18446744073709551615).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = U64Positive,
    name    = "U64Positive",
    base    = "u64",
    summary = "A positive 64-bit unsigned integer (value > 0). Equivalent to U64NonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

// ── Unsigned u128 ────────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = U128NonZero,
    name    = "U128NonZero",
    base    = "u128",
    summary = "A non-zero 128-bit unsigned integer (value != 0).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = U128Positive,
    name    = "U128Positive",
    base    = "u128",
    summary = "A positive 128-bit unsigned integer (value > 0). Equivalent to U128NonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);

// ── Unsigned usize ───────────────────────────────────────────────────────────

impl_integer_contract_spec!(
    type    = UsizeNonZero,
    name    = "UsizeNonZero",
    base    = "usize",
    summary = "A non-zero pointer-sized unsigned integer (value != 0).",
    requires = [
        ("non_zero", "Value must not be zero.", "value != 0"),
    ],
);

impl_integer_contract_spec!(
    type    = UsizePositive,
    name    = "UsizePositive",
    base    = "usize",
    summary = "A positive pointer-sized unsigned integer (value > 0). Equivalent to UsizeNonZero for unsigned types.",
    requires = [
        ("positive", "Value must be strictly greater than zero.", "value > 0"),
    ],
);
