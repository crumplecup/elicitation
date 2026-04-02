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
    (bool: $atomic:ty) => {
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
                <bool as Elicitation>::kani_proof()
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                <bool as Elicitation>::verus_proof()
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                <bool as Elicitation>::creusot_proof()
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

    (int: $atomic:ty => $prim:ty) => {
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
                <$prim as Elicitation>::kani_proof()
            }

            fn verus_proof() -> proc_macro2::TokenStream {
                <$prim as Elicitation>::verus_proof()
            }

            fn creusot_proof() -> proc_macro2::TokenStream {
                <$prim as Elicitation>::creusot_proof()
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

impl_atomic_elicitation!(bool: AtomicBool);

impl_atomic_elicitation!(int: AtomicI8 => i8);
impl_atomic_elicitation!(int: AtomicI16 => i16);
impl_atomic_elicitation!(int: AtomicI32 => i32);
impl_atomic_elicitation!(int: AtomicI64 => i64);
impl_atomic_elicitation!(int: AtomicIsize => isize);

impl_atomic_elicitation!(int: AtomicU8 => u8);
impl_atomic_elicitation!(int: AtomicU16 => u16);
impl_atomic_elicitation!(int: AtomicU32 => u32);
impl_atomic_elicitation!(int: AtomicU64 => u64);
impl_atomic_elicitation!(int: AtomicUsize => usize);
