//! Tuple type implementations using macros.

#![allow(non_snake_case)]

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};

/// Macro to implement Elicitation for tuples up to arity 12.
macro_rules! impl_tuple_elicit {
    // Base case: empty tuple (unit type is already handled by primitives)

    // Single element and up
    ($($T:ident $idx:tt),+) => {
        // Each tuple size gets its own default-only style enum
        paste::paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
            pub enum [<Tuple $( $idx )+ Style>] {
                #[default]
                Default,
            }

            impl Prompt for [<Tuple $( $idx )+ Style>] {
                fn prompt() -> Option<&'static str> {
                    None
                }
            }

            impl Elicitation for [<Tuple $( $idx )+ Style>] {
                type Style = [<Tuple $( $idx )+ Style>];

                async fn elicit(_client: &ElicitClient<'_>) -> ElicitResult<Self> {
                    Ok(Self::Default)
                }
            }
        }

        impl<$($T),+> Prompt for ($($T,)+)
        where
            $($T: Elicitation + Send,)+
        {
            fn prompt() -> Option<&'static str> {
                Some("Eliciting tuple elements:")
            }
        }

        impl<$($T),+> Elicitation for ($($T,)+)
        where
            $($T: Elicitation + Send,)+
        {
            paste::paste! {
                type Style = [<Tuple $( $idx )+ Style>];
            }

            #[tracing::instrument(skip(client), fields(
                tuple_size = count!($($T)+),
                types = concat!($(stringify!($T), ", "),+)
            ))]
            async fn elicit(
                client: &ElicitClient<'_>,
            ) -> ElicitResult<Self> {
                tracing::debug!("Eliciting tuple");

                $(
                    tracing::debug!(index = $idx, type_name = std::any::type_name::<$T>(), "Eliciting tuple element");
                    let $T = $T::elicit(client).await?;
                )+

                tracing::debug!("Tuple complete");
                Ok(($($T,)+))
            }
        }
    };
}

/// Helper macro to count the number of items
macro_rules! count {
    () => (0);
    ($head:tt $($tail:tt)*) => (1 + count!($($tail)*));
}

// Implement for tuples of arity 1 through 12
impl_tuple_elicit!(T0 0);
impl_tuple_elicit!(T0 0, T1 1);
impl_tuple_elicit!(T0 0, T1 1, T2 2);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10);
impl_tuple_elicit!(T0 0, T1 1, T2 2, T3 3, T4 4, T5 5, T6 6, T7 7, T8 8, T9 9, T10 10, T11 11);
