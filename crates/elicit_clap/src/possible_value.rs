//! [`clap::builder::PossibleValue`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::builder::PossibleValue, as PossibleValue);
elicit_newtype_traits!(PossibleValue, clap::builder::PossibleValue, [eq]);

/// Unwrap the Arc back to an owned `clap::builder::PossibleValue`.
impl From<PossibleValue> for clap::builder::PossibleValue {
    fn from(val: PossibleValue) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for PossibleValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let pv = &*self.0;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("name", pv.get_name())?;
        if let Some(help) = pv.get_help() {
            map.serialize_entry("help", &help.to_string())?;
        }
        map.end()
    }
}

#[reflect_methods]
impl PossibleValue {
    /// Returns the name of this possible value.
    #[tracing::instrument(skip(self))]
    pub fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    /// Returns `true` if this value is hidden from help output.
    #[tracing::instrument(skip(self))]
    pub fn is_hidden(&self) -> bool {
        self.0.is_hide_set()
    }

    /// Returns `true` if the given string matches this value (case-sensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches(&self, value: String) -> bool {
        self.0.matches(&value, false)
    }

    /// Returns `true` if the given string matches this value (case-insensitive).
    #[tracing::instrument(skip(self))]
    pub fn matches_ignore_case(&self, value: String) -> bool {
        self.0.matches(&value, true)
    }
}
