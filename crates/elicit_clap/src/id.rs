//! [`clap::Id`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Id, as Id);
elicit_newtype_traits!(Id, clap::Id, [cmp, display]);

/// Serialize [`Id`] as its string representation.
impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

/// Unwrap the Arc back to an owned `clap::Id`.
///
/// Used by `#[reflect_trait]` factories when `clap::Id` appears as
/// a method parameter.
impl From<Id> for clap::Id {
    fn from(val: Id) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Id {
    /// Returns the identifier as a string slice.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
}
