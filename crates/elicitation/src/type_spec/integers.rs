//! [`ElicitSpec`](crate::ElicitSpec) implementations for integer primitive types.
//!
//! Each integer type gets a [`TypeSpec`] describing its bounds so agents
//! can query valid ranges on demand rather than having them injected into
//! every prompt.

use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};
use anodized::spec;

// ── Macro ────────────────────────────────────────────────────────────────────

/// Generate `ElicitSpec` + inventory submission for a primitive integer type.
macro_rules! impl_integer_spec {
    (
        type   = $primitive:ty,
        name   = $name:literal,
        summary = $summary:literal,
        min    = $min:literal,
        max    = $max:literal $(,)?
    ) => {
        impl ElicitSpec for $primitive {
            fn type_spec() -> TypeSpec {
                let min_entry = SpecEntryBuilder::default()
                    .label("min".to_string())
                    .description(concat!("Minimum value: ", $min).to_string())
                    .expression(Some(
                        concat!(stringify!($primitive), "::MIN == ", $min).to_string(),
                    ))
                    .build()
                    .expect("valid min entry");

                let max_entry = SpecEntryBuilder::default()
                    .label("max".to_string())
                    .description(concat!("Maximum value: ", $max).to_string())
                    .expression(Some(
                        concat!(stringify!($primitive), "::MAX == ", $max).to_string(),
                    ))
                    .build()
                    .expect("valid max entry");

                let bounds = SpecCategoryBuilder::default()
                    .name("bounds".to_string())
                    .entries(vec![min_entry, max_entry])
                    .build()
                    .expect("valid bounds category");

                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![bounds])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$primitive as ElicitSpec>::type_spec
        ));
    };
}

// ── Signed integers ──────────────────────────────────────────────────────────

impl_integer_spec!(
    type    = i8,
    name    = "i8",
    summary = "An 8-bit signed integer.",
    min     = "-128",
    max     = "127",
);

impl_integer_spec!(
    type    = i16,
    name    = "i16",
    summary = "A 16-bit signed integer.",
    min     = "-32768",
    max     = "32767",
);

impl_integer_spec!(
    type    = i32,
    name    = "i32",
    summary = "A 32-bit signed integer.",
    min     = "-2147483648",
    max     = "2147483647",
);

impl_integer_spec!(
    type    = i64,
    name    = "i64",
    summary = "A 64-bit signed integer.",
    min     = "-9223372036854775808",
    max     = "9223372036854775807",
);

impl_integer_spec!(
    type    = i128,
    name    = "i128",
    summary = "A 128-bit signed integer.",
    min     = "-170141183460469231731687303715884105728",
    max     = "170141183460469231731687303715884105727",
);

impl_integer_spec!(
    type    = isize,
    name    = "isize",
    summary = "A pointer-sized signed integer (platform-dependent: 32 or 64 bits).",
    min     = "isize::MIN (platform-dependent)",
    max     = "isize::MAX (platform-dependent)",
);

// ── Unsigned integers ────────────────────────────────────────────────────────

impl_integer_spec!(
    type    = u8,
    name    = "u8",
    summary = "An 8-bit unsigned integer.",
    min     = "0",
    max     = "255",
);

impl_integer_spec!(
    type    = u16,
    name    = "u16",
    summary = "A 16-bit unsigned integer.",
    min     = "0",
    max     = "65535",
);

impl_integer_spec!(
    type    = u32,
    name    = "u32",
    summary = "A 32-bit unsigned integer.",
    min     = "0",
    max     = "4294967295",
);

impl_integer_spec!(
    type    = u64,
    name    = "u64",
    summary = "A 64-bit unsigned integer.",
    min     = "0",
    max     = "18446744073709551615",
);

impl_integer_spec!(
    type    = u128,
    name    = "u128",
    summary = "A 128-bit unsigned integer.",
    min     = "0",
    max     = "340282366920938463463374607431768211455",
);

impl_integer_spec!(
    type    = usize,
    name    = "usize",
    summary = "A pointer-sized unsigned integer (platform-dependent: 32 or 64 bits).",
    min     = "0",
    max     = "usize::MAX (platform-dependent)",
);

// ── Spec-annotated helper (demonstrates anodized integration) ────────────────

/// Returns true if `value` fits within the i8 range.
///
/// Used as a canonical example of anodized `#[spec]` alongside a `TypeSpec`.
#[spec(
    requires: [
        value >= i8::MIN as i32,
        value <= i8::MAX as i32,
    ],
    ensures: [
        *output == (value as i8),
    ],
)]
pub fn i32_as_i8_checked(value: i32) -> i8 {
    value as i8
}
