//! Error wrapper for reqwest errors.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Error.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::Error, as Error);

/// Serialize as `{"message": "..."}` using the `Display` representation.
impl serde::Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Error", 1)?;
        s.serialize_field("message", &self.0.to_string())?;
        s.end()
    }
}

/// `reqwest::Error` is opaque and cannot be constructed from user code.
///
/// Deserialization always fails with an explanatory message — the trait
/// bound is satisfied while being honest about the limitation.
impl<'de> serde::Deserialize<'de> for Error {
    fn deserialize<D: serde::Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "reqwest::Error cannot be reconstructed from JSON",
        ))
    }
}

impl elicitation::ElicitComplete for Error {}

mod emit_impls {
    use super::Error;
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for Error {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { { unimplemented!("reqwest::Error cannot be reconstructed as a code literal") } }
        }
    }
}
