//! Declarative macros for generating newtype wrappers.
//!
//! This module provides the `elicit_newtype!` and `elicit_newtypes!` macros
//! for creating transparent newtype wrappers around third-party types.

/// Generates a transparent newtype wrapper around a third-party type.
///
/// This macro creates a newtype that satisfies the orphan rule while providing
/// transparent access to the wrapped type through `Deref` and `DerefMut`.
///
/// # Implementation Strategy
///
/// The wrapper uses `Arc<T>` internally to ensure `Clone` is always available,
/// regardless of whether the inner type implements `Clone`. This is transparent
/// to users due to `Deref`/`DerefMut`.
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
/// // #[derive(Debug, Clone, derive_more::Deref, derive_more::DerefMut)]
/// // pub struct Client(pub Arc<reqwest::Client>);
/// //
/// // impl From<reqwest::Client> for Client {
/// //     fn from(inner: reqwest::Client) -> Self { Self(Arc::new(inner)) }
/// // }
/// //
/// // impl From<Arc<reqwest::Client>> for Client {
/// //     fn from(arc: Arc<reqwest::Client>) -> Self { Self(arc) }
/// // }
/// //
/// // impl From<Client> for Arc<reqwest::Client> {
/// //     fn from(wrapper: Client) -> Self { wrapper.0 }
/// // }
/// ```
///
/// # Generated Code
///
/// The macro generates:
/// - Newtype struct wrapping `Arc<T>` with `Debug`, `Clone`
/// - `Deref`, `DerefMut`, `AsRef` impls
/// - `From<T>`, `From<Arc<T>>`, `From<Wrapper> for Arc<T>` impls
/// - Conditional trait-forwarding impls (present only when `T` supports the trait):
///   `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, `Display`, `FromStr`
/// - `Copy` is intentionally absent — `Arc<T>` is never `Copy`
///
/// Use [`elicit_newtype_traits!`] after this macro to forward standard comparison,
/// display, and parsing traits for inner types that support them.
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
/// /// Users import familiar names:
/// use elicit_reqwest::Client;  // Same name as original!
/// ```
/// # Variants
///
/// | Syntax | `JsonSchema` | `Serialize`/`Deserialize` |
/// |--------|-------------|--------------------------|
/// | `elicit_newtype!(T, as Name)` | Generic object schema | No |
/// | `elicit_newtype!(T, as Name, serde)` | Delegated to `T` | Yes (`T: Serialize`) |
#[macro_export]
macro_rules! elicit_newtype {
    // Syntax: elicit_newtype!(path::to::Type, as WrapperName);
    // Example: elicit_newtype!(::std::path::PathBuf, as PathBuf);
    ($inner_path:path, as $wrapper_name:ident) => {
        #[doc = concat!("Elicitation-enabled wrapper around `", stringify!($inner_path), "`.")]
        #[doc = ""]
        #[doc = "This newtype uses `Arc` internally to ensure `Clone` is always available,"]
        #[doc = "providing transparent access via `Deref` and `DerefMut`."]
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
        )]
        pub struct $wrapper_name(pub ::std::sync::Arc<$inner_path>);

        impl ::schemars::JsonSchema for $wrapper_name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                stringify!($wrapper_name).into()
            }

            fn json_schema(_gen: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
                ::schemars::json_schema!({
                    "type": "object",
                    "description": concat!(
                        "Elicitation-enabled wrapper around `",
                        stringify!($inner_path),
                        "`"
                    )
                })
            }
        }

        // Manual Deref impl that derefs through Arc to the inner type
        impl ::std::ops::Deref for $wrapper_name {
            type Target = $inner_path;

            fn deref(&self) -> &Self::Target {
                &*self.0
            }
        }

        // Manual DerefMut impl that derefs through Arc to the inner type
        impl ::std::ops::DerefMut for $wrapper_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::std::sync::Arc::get_mut(&mut self.0)
                    .expect("Cannot get mutable reference to Arc with multiple references")
            }
        }

        // AsRef impl for convenience
        impl ::std::convert::AsRef<$inner_path> for $wrapper_name {
            fn as_ref(&self) -> &$inner_path {
                &*self.0
            }
        }

        // From T -> Wrapper (auto-wraps in Arc)
        impl ::std::convert::From<$inner_path> for $wrapper_name {
            fn from(inner: $inner_path) -> Self {
                Self(::std::sync::Arc::new(inner))
            }
        }

        // From Arc<T> -> Wrapper (zero-copy)
        impl ::std::convert::From<::std::sync::Arc<$inner_path>> for $wrapper_name {
            fn from(arc: ::std::sync::Arc<$inner_path>) -> Self {
                Self(arc)
            }
        }

        // From Wrapper -> Arc<T> (extract the Arc)
        impl ::std::convert::From<$wrapper_name> for ::std::sync::Arc<$inner_path> {
            fn from(wrapper: $wrapper_name) -> Self {
                wrapper.0
            }
        }
    };

    // Syntax: elicit_newtype!(path::to::Type, as WrapperName, serde);
    // Like the base form but also derives Serialize + Deserialize (only for types where T: Serialize).
    ($inner_path:path, as $wrapper_name:ident, serde) => {
        #[doc = concat!("Elicitation-enabled wrapper around `", stringify!($inner_path), "`.")]
        #[doc = ""]
        #[doc = "This newtype uses `Arc` internally to ensure `Clone` is always available,"]
        #[doc = "providing transparent access via `Deref` and `DerefMut`."]
        #[doc = "Serialization is delegated transparently to the inner type."]
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
            ::serde::Serialize,
            ::serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $wrapper_name(pub ::std::sync::Arc<$inner_path>);

        impl ::schemars::JsonSchema for $wrapper_name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                stringify!($wrapper_name).into()
            }

            fn json_schema(schema_gen: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
                <$inner_path as ::schemars::JsonSchema>::json_schema(schema_gen)
            }
        }

        impl ::std::ops::Deref for $wrapper_name {
            type Target = $inner_path;

            fn deref(&self) -> &Self::Target {
                &*self.0
            }
        }

        impl ::std::ops::DerefMut for $wrapper_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                ::std::sync::Arc::get_mut(&mut self.0)
                    .expect("Cannot get mutable reference to Arc with multiple references")
            }
        }

        impl ::std::convert::AsRef<$inner_path> for $wrapper_name {
            fn as_ref(&self) -> &$inner_path {
                &*self.0
            }
        }

        impl ::std::convert::From<$inner_path> for $wrapper_name {
            fn from(inner: $inner_path) -> Self {
                Self(::std::sync::Arc::new(inner))
            }
        }

        impl ::std::convert::From<::std::sync::Arc<$inner_path>> for $wrapper_name {
            fn from(arc: ::std::sync::Arc<$inner_path>) -> Self {
                Self(arc)
            }
        }

        impl ::std::convert::From<$wrapper_name> for ::std::sync::Arc<$inner_path> {
            fn from(wrapper: $wrapper_name) -> Self {
                wrapper.0
            }
        }
    };
}

/// Forwards standard library traits from the inner type to an `elicit_newtype!` wrapper.
///
/// Because `elicit_newtype!` uses `Arc<T>` internally, Rust cannot generate
/// conditional `where T: Trait` impls for concrete structs.  This macro lets
/// the crate author explicitly opt in to the traits they know the inner type
/// supports.
///
/// # Flags (one or more, in a bracket list)
///
/// | Flag | Traits generated |
/// |------|-----------------|
/// | `eq` | `PartialEq + Eq` |
/// | `eq_hash` | `PartialEq + Eq + Hash` |
/// | `ord` | `PartialEq + Eq + PartialOrd + Ord` |
/// | `cmp` | `PartialEq + Eq + Hash + PartialOrd + Ord` |
/// | `display` | `Display` |
/// | `from_str` | `FromStr` |
///
/// Higher-flag supersets (`ord`, `cmp`) include all the traits of their subsets.
///
/// # Example
///
/// ```rust,ignore
/// use elicitation::{elicit_newtype, elicit_newtype_traits};
///
/// elicit_newtype!(uuid::Uuid, as Uuid, serde);
/// // uuid::Uuid: PartialEq + Eq + Hash + PartialOrd + Ord + Display + FromStr
/// elicit_newtype_traits!(Uuid, uuid::Uuid, [cmp, display, from_str]);
///
/// elicit_newtype!(serde_json::Value, as JsonValue, serde);
/// // serde_json::Value: PartialEq + Eq, but no Hash/Ord
/// elicit_newtype_traits!(JsonValue, serde_json::Value, [eq]);
/// ```
#[macro_export]
macro_rules! elicit_newtype_traits {
    // Base case: empty list
    ($name:ident, $inner:path, []) => {};

    // Peel one flag and recurse
    ($name:ident, $inner:path, [$flag:ident $(, $rest:ident)*]) => {
        $crate::elicit_newtype_trait_flag!($name, $inner, $flag);
        $crate::elicit_newtype_traits!($name, $inner, [$($rest),*]);
    };
}

/// Generates one group of standard trait impls for a newtype wrapper.
///
/// Called by [`elicit_newtype_traits!`]; not intended for direct use.
#[doc(hidden)]
#[macro_export]
macro_rules! elicit_newtype_trait_flag {
    // ── eq ────────────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, eq) => {
        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                *self.0 == *other.0
            }
        }
        impl ::std::cmp::Eq for $name {}
    };

    // ── eq_hash ───────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, eq_hash) => {
        $crate::elicit_newtype_trait_flag!($name, $inner, eq);
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                (*self.0).hash(state);
            }
        }
    };

    // ── ord ──────────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, ord) => {
        $crate::elicit_newtype_trait_flag!($name, $inner, eq);
        impl ::std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                (*self.0).partial_cmp(&*other.0)
            }
        }
        impl ::std::cmp::Ord for $name {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                (*self.0).cmp(&*other.0)
            }
        }
    };

    // ── cmp ──────────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, cmp) => {
        $crate::elicit_newtype_trait_flag!($name, $inner, ord);
        impl ::std::hash::Hash for $name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                (*self.0).hash(state);
            }
        }
    };

    // ── display ───────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, display) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&*self.0, f)
            }
        }
    };

    // ── from_str ─────────────────────────────────────────────────────────────
    ($name:ident, $inner:path, from_str) => {
        impl ::std::str::FromStr for $name {
            type Err = <$inner as ::std::str::FromStr>::Err;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                s.parse::<$inner>().map(Self::from)
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

    // Single type with serde
    ($inner_path:path, as $wrapper_name:ident, serde $(;)?) => {
        $crate::elicit_newtype!($inner_path, as $wrapper_name, serde);
    };

    // Multiple types with serde flag (serde flag applies per item)
    ($inner_path:path, as $wrapper_name:ident, serde; $($rest:tt)*) => {
        $crate::elicit_newtype!($inner_path, as $wrapper_name, serde);
        $crate::elicit_newtypes!($($rest)*);
    };
}
