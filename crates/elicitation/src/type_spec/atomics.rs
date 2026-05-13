//! [`ElicitSpec`](crate::ElicitSpec) and [`ElicitComplete`](crate::ElicitComplete)
//! implementations for [`std::sync::atomic`] types.
//!
//! Each atomic integer wraps its corresponding primitive, so the spec describes
//! the same value bounds plus the atomic-specific construction: `Atomic*::new(val)`.
//! Atomics are thread-safe by design — agents can use them wherever `Send + Sync`
//! interior mutability is required.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

use crate::{
    ElicitComplete, ElicitPromptTree, ElicitSpec, PromptTree, SpecCategoryBuilder,
    SpecEntryBuilder, TypeSpec, TypeSpecBuilder, TypeSpecInventoryKey,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn atomic_construction_category(type_name: &str) -> crate::SpecCategory {
    SpecCategoryBuilder::default()
        .name("construction".to_string())
        .entries(vec![
            SpecEntryBuilder::default()
                .label("new".to_string())
                .description(format!(
                    "Construct with {type_name}::new(val) where val is the underlying primitive."
                ))
                .expression(Some(format!("{type_name}::new(val)")))
                .build()
                .expect("valid new entry"),
            SpecEntryBuilder::default()
                .label("load".to_string())
                .description(
                    "Read the current value with .load(Ordering::SeqCst). \
                     SeqCst is the safest ordering; use weaker orderings only when you \
                     understand the memory model implications."
                        .to_string(),
                )
                .expression(Some(
                    "value.load(std::sync::atomic::Ordering::SeqCst)".to_string(),
                ))
                .build()
                .expect("valid load entry"),
        ])
        .build()
        .expect("valid construction category")
}

fn atomic_thread_safety_category() -> crate::SpecCategory {
    SpecCategoryBuilder::default()
        .name("thread_safety".to_string())
        .entries(vec![
            SpecEntryBuilder::default()
                .label("send_sync".to_string())
                .description(
                    "All atomic types implement Send + Sync, making them safe to share \
                     across threads without a Mutex."
                        .to_string(),
                )
                .build()
                .expect("valid send_sync entry"),
            SpecEntryBuilder::default()
                .label("not_clone".to_string())
                .description(
                    "Atomic types do not implement Clone. To copy the value, load it \
                     and construct a new atomic: Atomic*::new(val.load(Ordering::SeqCst))."
                        .to_string(),
                )
                .build()
                .expect("valid not_clone entry"),
        ])
        .build()
        .expect("valid thread_safety category")
}

// ── Macros ────────────────────────────────────────────────────────────────────

macro_rules! impl_atomic_bool_spec {
    ($atomic:ty, $name:literal) => {
        impl ElicitSpec for $atomic {
            fn type_spec() -> TypeSpec {
                let values = SpecCategoryBuilder::default()
                    .name("values".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("true".to_string())
                            .description(
                                "Logical true. Accepted inputs: \"true\", \"yes\", \"1\", \"y\"."
                                    .to_string(),
                            )
                            .build()
                            .expect("valid entry"),
                        SpecEntryBuilder::default()
                            .label("false".to_string())
                            .description(
                                "Logical false. Accepted inputs: \"false\", \"no\", \"0\", \"n\"."
                                    .to_string(),
                            )
                            .build()
                            .expect("valid entry"),
                    ])
                    .build()
                    .expect("valid values");

                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary(concat!(
                        "A thread-safe atomic boolean (wraps bool). \
                         Construct with ",
                        $name,
                        "::new(true/false)."
                    ).to_string())
                    .categories(vec![
                        values,
                        atomic_construction_category($name),
                        atomic_thread_safety_category(),
                    ])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$atomic as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$atomic>
        ));

        impl ElicitPromptTree for $atomic {
            fn prompt_tree() -> PromptTree {
                PromptTree::Affirm {
                    prompt: "AtomicBool (true/false)".to_string(),
                    type_name: $name.to_string(),
                }
            }
        }

        impl ElicitComplete for $atomic {}
    };
}

macro_rules! impl_atomic_integer_spec {
    (
        type    = $atomic:ty,
        name    = $name:literal,
        summary = $summary:literal,
        min     = $min:literal,
        max     = $max:literal $(,)?
    ) => {
        impl ElicitSpec for $atomic {
            fn type_spec() -> TypeSpec {
                let bounds = SpecCategoryBuilder::default()
                    .name("bounds".to_string())
                    .entries(vec![
                        SpecEntryBuilder::default()
                            .label("min".to_string())
                            .description(concat!("Minimum value: ", $min).to_string())
                            .build()
                            .expect("valid min entry"),
                        SpecEntryBuilder::default()
                            .label("max".to_string())
                            .description(concat!("Maximum value: ", $max).to_string())
                            .build()
                            .expect("valid max entry"),
                    ])
                    .build()
                    .expect("valid bounds category");

                TypeSpecBuilder::default()
                    .type_name($name.to_string())
                    .summary($summary.to_string())
                    .categories(vec![
                        bounds,
                        atomic_construction_category($name),
                        atomic_thread_safety_category(),
                    ])
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        inventory::submit!(TypeSpecInventoryKey::new(
            $name,
            <$atomic as ElicitSpec>::type_spec,
            std::any::TypeId::of::<$atomic>
        ));

        impl ElicitPromptTree for $atomic {
            fn prompt_tree() -> PromptTree {
                PromptTree::Leaf {
                    prompt: concat!($name, " (integer)").to_string(),
                    type_name: $name.to_string(),
                }
            }
        }

        impl ElicitComplete for $atomic {}
    };
}

impl_atomic_bool_spec!(AtomicBool, "std::sync::atomic::AtomicBool");

// ── Signed atomic integers ────────────────────────────────────────────────────

impl_atomic_integer_spec!(
    type    = AtomicI8,
    name    = "std::sync::atomic::AtomicI8",
    summary = "Thread-safe atomic 8-bit signed integer (wraps i8). Range: -128 to 127.",
    min     = "-128",
    max     = "127",
);

impl_atomic_integer_spec!(
    type    = AtomicI16,
    name    = "std::sync::atomic::AtomicI16",
    summary = "Thread-safe atomic 16-bit signed integer (wraps i16). Range: -32768 to 32767.",
    min     = "-32768",
    max     = "32767",
);

impl_atomic_integer_spec!(
    type    = AtomicI32,
    name    = "std::sync::atomic::AtomicI32",
    summary = "Thread-safe atomic 32-bit signed integer (wraps i32). Range: -2147483648 to 2147483647.",
    min     = "-2147483648",
    max     = "2147483647",
);

impl_atomic_integer_spec!(
    type    = AtomicI64,
    name    = "std::sync::atomic::AtomicI64",
    summary = "Thread-safe atomic 64-bit signed integer (wraps i64). Range: i64::MIN to i64::MAX.",
    min     = "-9223372036854775808",
    max     = "9223372036854775807",
);

impl_atomic_integer_spec!(
    type    = AtomicIsize,
    name    = "std::sync::atomic::AtomicIsize",
    summary = "Thread-safe atomic pointer-sized signed integer (wraps isize, platform-dependent size).",
    min     = "isize::MIN (platform-dependent)",
    max     = "isize::MAX (platform-dependent)",
);

// ── Unsigned atomic integers ──────────────────────────────────────────────────

impl_atomic_integer_spec!(
    type    = AtomicU8,
    name    = "std::sync::atomic::AtomicU8",
    summary = "Thread-safe atomic 8-bit unsigned integer (wraps u8). Range: 0 to 255.",
    min     = "0",
    max     = "255",
);

impl_atomic_integer_spec!(
    type    = AtomicU16,
    name    = "std::sync::atomic::AtomicU16",
    summary = "Thread-safe atomic 16-bit unsigned integer (wraps u16). Range: 0 to 65535.",
    min     = "0",
    max     = "65535",
);

impl_atomic_integer_spec!(
    type    = AtomicU32,
    name    = "std::sync::atomic::AtomicU32",
    summary = "Thread-safe atomic 32-bit unsigned integer (wraps u32). Range: 0 to 4294967295.",
    min     = "0",
    max     = "4294967295",
);

impl_atomic_integer_spec!(
    type    = AtomicU64,
    name    = "std::sync::atomic::AtomicU64",
    summary = "Thread-safe atomic 64-bit unsigned integer (wraps u64). Range: 0 to u64::MAX.",
    min     = "0",
    max     = "18446744073709551615",
);

impl_atomic_integer_spec!(
    type    = AtomicUsize,
    name    = "std::sync::atomic::AtomicUsize",
    summary = "Thread-safe atomic pointer-sized unsigned integer (wraps usize, platform-dependent size).",
    min     = "0",
    max     = "usize::MAX (platform-dependent)",
);
