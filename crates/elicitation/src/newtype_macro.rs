//! Declarative macros for generating newtype wrappers.
//!
//! This module provides the `elicit_newtype!` and `elicit_newtypes!` macros
//! for creating transparent newtype wrappers around third-party types.

/// Generates a transparent newtype wrapper around a third-party type.
///
/// This macro creates a newtype that satisfies the orphan rule while providing
/// transparent access to the wrapped type through `Deref` and `DerefMut`.
///
/// # Usage with Custom Name (Required)
///
/// Due to macro limitations, you must specify the wrapper name explicitly.
/// The syntax is: `elicit_newtype!(path::to::Type, as WrapperName);`
///
/// **Note:** The first argument must be a valid type path. Use concrete types like
/// `std::path::PathBuf`, not type aliases like `std::path::Path`.
///
/// ```ignore
/// use elicitation::elicit_newtype;
///
/// // Standard library types (best practice: use :: prefix for clarity)
/// elicit_newtype!(::std::path::PathBuf, as PathBuf);
/// elicit_newtype!(::std::collections::HashMap<String, i32>, as IntMap);
///
/// // Third-party crates (companion crate pattern)
/// elicit_newtype!(reqwest::Client, as Client);
///
/// // Generates:
/// // #[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut, derive_more::From)]
/// // pub struct Client(pub reqwest::Client);
/// //
/// // impl From<Client> for reqwest::Client {
/// //     fn from(wrapper: Client) -> Self { wrapper.0 }
/// // }
/// ```
///
/// # Generated Code
///
/// The macro generates:
/// - Newtype struct with `derive_more` traits (Deref, DerefMut, From, AsRef)
/// - Manual `From` impl for unwrapping direction
/// - Debug and Clone derives
/// - All fields are public for maximum transparency
///
/// # Companion Crate Pattern
///
/// This macro is designed for creating companion crates like `elicit_reqwest`:
///
/// ```ignore
/// // In elicit_reqwest/src/lib.rs
/// elicit_newtype!(reqwest::Client, as Client);
/// elicit_newtype!(reqwest::Request, as Request);
/// elicit_newtype!(reqwest::Response, as Response);
///
/// // Users import familiar names:
/// use elicit_reqwest::Client;  // Same name as original!
/// ```
#[macro_export]
macro_rules! elicit_newtype {
    // Syntax: elicit_newtype!(path::to::Type, as WrapperName);
    // Example: elicit_newtype!(::std::path::PathBuf, as PathBuf);
    ($inner_path:path, as $wrapper_name:ident) => {
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
            ::derive_more::Deref,
            ::derive_more::DerefMut,
            ::derive_more::From,
            ::derive_more::AsRef,
        )]
        pub struct $wrapper_name(pub $inner_path);

        // Manual From impl for unwrapping direction
        impl ::std::convert::From<$wrapper_name> for $inner_path {
            fn from(wrapper: $wrapper_name) -> Self {
                wrapper.0
            }
        }
    };
}

/// Generates multiple newtype wrappers in bulk.
///
/// This is a convenience macro for generating multiple newtypes at once,
/// typically used in companion crates.
///
/// # Syntax
///
/// Items are separated by **semicolons**, with an optional trailing semicolon.
/// Each item uses the same syntax as `elicit_newtype!`: `path::to::Type, as Name`
///
/// # Example
///
/// ```ignore
/// use elicitation::elicit_newtypes;
///
/// // Semicolon-separated items (trailing semicolon optional)
/// elicit_newtypes! {
///     ::std::path::PathBuf, as PathBuf;
///     ::std::collections::HashMap<String, i32>, as IntMap;
///     reqwest::Client, as Client;
///     reqwest::Request, as Request;
///     reqwest::Response, as Response
/// }
///
/// // Generates:
/// // pub struct PathBuf(pub ::std::path::PathBuf);
/// // pub struct IntMap(pub ::std::collections::HashMap<String, i32>);
/// // pub struct Client(pub reqwest::Client);
/// // pub struct Request(pub reqwest::Request);
/// // pub struct Response(pub reqwest::Response);
/// // (each with derive_more traits)
/// ```
#[macro_export]
macro_rules! elicit_newtypes {
    // Empty case
    () => {};

    // Single type
    ($inner_path:path, as $wrapper_name:ident $(;)?) => {
        $crate::elicit_newtype!($inner_path, as $wrapper_name);
    };

    // Multiple types (semicolon-separated)
    ($inner_path:path, as $wrapper_name:ident; $($rest:tt)*) => {
        $crate::elicit_newtype!($inner_path, as $wrapper_name);
        $crate::elicit_newtypes!($($rest)*);
    };
}
