//! [`clap::Command`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(clap::Command, as Command);
elicit_newtype_traits!(Command, clap::Command, []);

/// Unwrap the Arc back to an owned `clap::Command`.
///
/// Used by `#[reflect_trait]` factories when `clap::Command` appears as
/// a method parameter and must be reconstructed from an agent-supplied
/// `Command` JSON value.
impl From<Command> for clap::Command {
    fn from(val: Command) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for Command {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let cmd = &*self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("name", cmd.get_name())?;
        if let Some(about) = cmd.get_about() {
            map.serialize_entry("about", &about.to_string())?;
        }
        if let Some(version) = cmd.get_version() {
            map.serialize_entry("version", version)?;
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Command {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct CommandVisitor;
        impl<'de> Visitor<'de> for CommandVisitor {
            type Value = Command;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"a command object with at least a "name" field"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Command, A::Error> {
                let mut name: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "name" {
                        name = Some(map.next_value()?);
                    } else {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                Ok(Command(std::sync::Arc::new(clap::Command::new(name))))
            }
        }
        deserializer.deserialize_map(CommandVisitor)
    }
}

#[reflect_methods]
impl Command {
    /// Returns the command's name.
    #[tracing::instrument(skip(self))]
    pub fn get_name(&self) -> String {
        self.0.get_name().to_string()
    }

    /// Returns the short about string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_about(&self) -> Option<String> {
        self.0.get_about().map(|s| s.to_string())
    }

    /// Returns the version string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_version(&self) -> Option<String> {
        self.0.get_version().map(str::to_string)
    }

    /// Returns the author string, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_author(&self) -> Option<String> {
        self.0.get_author().map(str::to_string)
    }

    /// Returns the display name (used in help), if set.
    #[tracing::instrument(skip(self))]
    pub fn get_display_name(&self) -> Option<String> {
        self.0.get_display_name().map(str::to_string)
    }

    /// Returns the binary name, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_bin_name(&self) -> Option<String> {
        self.0.get_bin_name().map(str::to_string)
    }

    /// Returns the display order for this command.
    #[tracing::instrument(skip(self))]
    pub fn get_display_order(&self) -> usize {
        self.0.get_display_order()
    }
}

// ── ElicitComplete + ToCodeLiteral ───────────────────────────────────────────

mod emit_impls {
    use super::Command;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Command {
        fn to_code_literal(&self) -> TokenStream {
            let name = self.0.get_name().to_string();
            quote::quote! {
                ::elicit_clap::Command::from(::clap::Command::new(#name))
            }
        }
    }
}

impl elicitation::ElicitComplete for Command {}
