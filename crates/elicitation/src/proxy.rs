//! [`ElicitProxy`] — bridge between third-party types and their serializable wrappers.
//!
//! `#[reflect_trait]` factory closures must serialize method inputs and return values
//! as JSON.  When a trait method's parameter or return type is a third-party type that
//! does not implement `Serialize + Deserialize + JsonSchema`, implement this trait to
//! declare the serializable proxy that stands in its place.
//!
//! # Implementing
//!
//! **For your own types** — derive the identity impl:
//!
//! ```rust,ignore
//! #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, ElicitProxy)]
//! pub struct MyType { /* ... */ }
//! ```
//!
//! **For third-party types** — write the impl manually:
//!
//! ```rust,ignore
//! impl ElicitProxy for clap::Command {
//!     type Proxy = elicit_clap::Command;
//!     fn into_proxy(self) -> elicit_clap::Command { self.into() }
//!     fn from_proxy(proxy: elicit_clap::Command) -> Self { (*proxy.0).clone() }
//! }
//! ```
//!
//! # How `#[reflect_trait]` uses this
//!
//! The macro generates code that:
//! - Uses `<ParamType as ElicitProxy>::Proxy` as the field type in param structs
//!   (so agents send the serializable form)
//! - Calls `from_proxy(p.field)` before passing to the real method
//! - Calls `into_proxy(result)` on return values before serializing

use schemars::JsonSchema;
use serde::{Serialize, de::DeserializeOwned};

/// Bridge a type to its serializable elicit proxy.
///
/// Implement this trait to make a type usable as a method parameter or return
/// type in `#[reflect_trait]` factories.
///
/// The `Proxy` associated type must be fully serializable (`Serialize +
/// DeserializeOwned + JsonSchema`). For types that are already serializable,
/// use `#[derive(ElicitProxy)]` to generate the trivial identity implementation.
pub trait ElicitProxy: Sized {
    /// The serializable proxy type.
    type Proxy: Serialize + DeserializeOwned + JsonSchema;

    /// Convert this value into its serializable proxy.
    fn into_proxy(self) -> Self::Proxy;

    /// Recover the original type from a deserialized proxy.
    fn from_proxy(proxy: Self::Proxy) -> Self;
}

// ── Stdlib identity impls ─────────────────────────────────────────────────────

macro_rules! identity_proxy {
    ($($t:ty),+ $(,)?) => {
        $(
            impl ElicitProxy for $t {
                type Proxy = $t;
                fn into_proxy(self) -> $t { self }
                fn from_proxy(proxy: $t) -> $t { proxy }
            }
        )+
    };
}

identity_proxy!(
    bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64, char, String,
);

impl<T: ElicitProxy> ElicitProxy for Option<T>
where
    Option<T::Proxy>: Serialize + DeserializeOwned + JsonSchema,
{
    type Proxy = Option<T::Proxy>;

    fn into_proxy(self) -> Option<T::Proxy> {
        self.map(T::into_proxy)
    }

    fn from_proxy(proxy: Option<T::Proxy>) -> Self {
        proxy.map(T::from_proxy)
    }
}

impl<T: ElicitProxy> ElicitProxy for Vec<T>
where
    Vec<T::Proxy>: Serialize + DeserializeOwned + JsonSchema,
{
    type Proxy = Vec<T::Proxy>;

    fn into_proxy(self) -> Vec<T::Proxy> {
        self.into_iter().map(T::into_proxy).collect()
    }

    fn from_proxy(proxy: Vec<T::Proxy>) -> Self {
        proxy.into_iter().map(T::from_proxy).collect()
    }
}

impl<T: ElicitProxy, E: ElicitProxy> ElicitProxy for Result<T, E>
where
    Result<T::Proxy, E::Proxy>: Serialize + DeserializeOwned + JsonSchema,
{
    type Proxy = Result<T::Proxy, E::Proxy>;

    fn into_proxy(self) -> Result<T::Proxy, E::Proxy> {
        self.map(T::into_proxy).map_err(E::into_proxy)
    }

    fn from_proxy(proxy: Result<T::Proxy, E::Proxy>) -> Self {
        proxy.map(T::from_proxy).map_err(E::from_proxy)
    }
}
