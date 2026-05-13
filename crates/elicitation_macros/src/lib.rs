//! Helper macros and utilities for Kani formal verification.
//!
//! Provides bounded-length string/vec helpers and the `kani_arbitrary!` macro
//! for writing `kani::Arbitrary` impls for types that contain `String` or
//! `Vec<T>` fields (which Kani 0.67 does not support natively).
//!
//! # Usage
//!
//! ```rust,ignore
//! use elicitation_macros::{bounded_string, bounded_option_string, bounded_vec, kani_arbitrary};
//! ```

#![forbid(unsafe_code)]

/// Generate a bounded-length `String` for Kani verification.
///
/// Produces a `String` from a `[u8; N]` array of arbitrary bytes.
/// Valid for soundness because Kani tracks all possible byte values.
///
/// # Example
///
/// ```rust,ignore
/// #[cfg(kani)]
/// impl kani::Arbitrary for MyType {
///     fn any() -> Self {
///         Self { name: elicitation_macros::bounded_string::<16>() }
///     }
/// }
/// ```
#[cfg(kani)]
pub fn bounded_string<const N: usize>() -> String {
    let bytes: [u8; N] = kani::any();
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Generate an optional bounded-length `String` for Kani verification.
///
/// Returns `None` or `Some(bounded_string::<N>())` with equal probability.
#[cfg(kani)]
pub fn bounded_option_string<const N: usize>() -> Option<String> {
    if kani::any() {
        Some(bounded_string::<N>())
    } else {
        None
    }
}

/// Generate a bounded-length `Vec<T>` for Kani verification.
///
/// Produces a `Vec` with 0..N elements where each element is `kani::any()`.
#[cfg(kani)]
pub fn bounded_vec<T: kani::Arbitrary, const N: usize>() -> Vec<T> {
    let len: usize = kani::any_where(|&x| x <= N);
    (0..len).map(|_| kani::any()).collect()
}

/// Generate a `kani::Arbitrary` impl for a type with `String` or `Vec` fields.
///
/// The macro body is the `fn any() -> Self { ... }` body. Use `bounded_string`,
/// `bounded_option_string`, and `bounded_vec` helpers inside the body.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_macros::kani_arbitrary;
///
/// kani_arbitrary!(MyType {
///     name: bounded_string::<32>(),
///     tag: bounded_option_string::<16>(),
///     items: bounded_vec::<u32, 8>(),
///     count: kani::any(),
/// });
/// ```
#[macro_export]
macro_rules! kani_arbitrary {
    ($ty:ident { $($field:ident : $expr:expr),* $(,)? }) => {
        #[cfg(kani)]
        impl kani::Arbitrary for $ty {
            fn any() -> Self {
                Self {
                    $($field: $expr,)*
                }
            }
        }
    };
    // Tuple struct variant
    ($ty:ident ( $($expr:expr),* $(,)? )) => {
        #[cfg(kani)]
        impl kani::Arbitrary for $ty {
            fn any() -> Self {
                Self ( $($expr,)* )
            }
        }
    };
}
