//! [`clap::ArgAction`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::ArgAction, as ArgAction);
elicit_newtype_traits!(ArgAction, clap::ArgAction, []);

#[reflect_methods]
impl ArgAction {
    /// Returns `true` if this action stores a value (Set or Append).
    #[tracing::instrument(skip(self))]
    pub fn takes_value(&self) -> bool {
        matches!(*self.0, clap::ArgAction::Set | clap::ArgAction::Append)
    }

    /// Returns `true` if this is a flag action (SetTrue, SetFalse, Count).
    #[tracing::instrument(skip(self))]
    pub fn is_flag(&self) -> bool {
        matches!(
            *self.0,
            clap::ArgAction::SetTrue | clap::ArgAction::SetFalse | clap::ArgAction::Count
        )
    }

    /// Returns the name of this variant as a string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match &*self.0 {
            clap::ArgAction::Set => "Set",
            clap::ArgAction::Append => "Append",
            clap::ArgAction::SetTrue => "SetTrue",
            clap::ArgAction::SetFalse => "SetFalse",
            clap::ArgAction::Count => "Count",
            clap::ArgAction::Help => "Help",
            clap::ArgAction::HelpShort => "HelpShort",
            clap::ArgAction::Version => "Version",
            _ => "Unknown",
        }
    }
}
