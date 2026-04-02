//! [`Elicitation`] implementations for [`std::sync::atomic`] types.
//!
//! All eleven non-pointer atomic types are supported:
//! [`AtomicBool`], [`AtomicI8`], [`AtomicI16`], [`AtomicI32`], [`AtomicI64`],
//! [`AtomicIsize`], [`AtomicU8`], [`AtomicU16`], [`AtomicU32`], [`AtomicU64`],
//! and [`AtomicUsize`].
//!
//! Each atomic wraps its corresponding primitive: eliciting an atomic value
//! elicits the underlying primitive and wraps it with `Atomic*::new(val)`.
//!
//! The proof methods generate harnesses that verify the core atomic invariant:
//! `Atomic*::new(val).load(SeqCst) == val`. Verus and Creusot proofs are
//! `#[trusted]` since those tools cannot verify unsafe hardware atomic ops.
//!
//! [`ToCodeLiteral`](crate::emit_code::ToCodeLiteral) is implemented in
//! `emit_code.rs` alongside the other primitive impls.

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};

macro_rules! impl_atomic_elicitation {
    (bool: $atomic:ty, $atomic_path:literal) => {
        impl Prompt for $atomic {
            fn prompt() -> Option<&'static str> {
                Some(concat!(
                    "Please enter a boolean value for ",
                    stringify!($atomic),
                    " (true or false):"
                ))
            }
        }

        impl Elicitation for $atomic {
            type Style = <bool as Elicitation>::Style;

            #[tracing::instrument(skip(communicator), fields(type_name = stringify!($atomic)))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!("Eliciting ", stringify!($atomic), " via bool"));
                let val = bool::elicit(communicator).await?;
                Ok(<$atomic>::new(val))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_atomic($atomic_path, "bool")
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_atomic($atomic_path, "bool")
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_atomic($atomic_path, "bool")
            }
        }

        impl ElicitIntrospect for $atomic {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Affirm
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($atomic),
                    description: <Self as Prompt>::prompt(),
                    details: PatternDetails::Affirm,
                }
            }
        }
    };

    (int: $atomic:ty, $atomic_path:literal => $prim:ty) => {
        impl Prompt for $atomic {
            fn prompt() -> Option<&'static str> {
                Some(concat!(
                    "Please enter a ",
                    stringify!($prim),
                    " value for ",
                    stringify!($atomic),
                    ":"
                ))
            }
        }

        impl Elicitation for $atomic {
            type Style = <$prim as Elicitation>::Style;

            #[tracing::instrument(skip(communicator), fields(type_name = stringify!($atomic)))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                tracing::debug!(concat!(
                    "Eliciting ",
                    stringify!($atomic),
                    " via ",
                    stringify!($prim)
                ));
                let val = <$prim as Elicitation>::elicit(communicator).await?;
                Ok(<$atomic>::new(val))
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_atomic($atomic_path, stringify!($prim))
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_atomic($atomic_path, stringify!($prim))
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_atomic($atomic_path, stringify!($prim))
            }
        }

        impl ElicitIntrospect for $atomic {
            fn pattern() -> ElicitationPattern {
                ElicitationPattern::Primitive
            }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($atomic),
                    description: <Self as Prompt>::prompt(),
                    details: PatternDetails::Primitive,
                }
            }
        }
    };
}

impl_atomic_elicitation!(bool: AtomicBool, "::std::sync::atomic::AtomicBool");

impl_atomic_elicitation!(int: AtomicI8,    "::std::sync::atomic::AtomicI8"    => i8);
impl_atomic_elicitation!(int: AtomicI16,   "::std::sync::atomic::AtomicI16"   => i16);
impl_atomic_elicitation!(int: AtomicI32,   "::std::sync::atomic::AtomicI32"   => i32);
impl_atomic_elicitation!(int: AtomicI64,   "::std::sync::atomic::AtomicI64"   => i64);
impl_atomic_elicitation!(int: AtomicIsize, "::std::sync::atomic::AtomicIsize" => isize);

impl_atomic_elicitation!(int: AtomicU8,    "::std::sync::atomic::AtomicU8"    => u8);
impl_atomic_elicitation!(int: AtomicU16,   "::std::sync::atomic::AtomicU16"   => u16);
impl_atomic_elicitation!(int: AtomicU32,   "::std::sync::atomic::AtomicU32"   => u32);
impl_atomic_elicitation!(int: AtomicU64,   "::std::sync::atomic::AtomicU64"   => u64);
impl_atomic_elicitation!(int: AtomicUsize, "::std::sync::atomic::AtomicUsize" => usize);
