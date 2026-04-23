//! [`surrealdb_types::Regex`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Regex, as Regex, forward_serde);
elicit_newtype_traits!(Regex, surrealdb_types::Regex, [eq]);

/// Unwrap the Arc back to an owned `surrealdb_types::Regex`.
impl From<Regex> for surrealdb_types::Regex {
    fn from(val: Regex) -> Self {
        std::sync::Arc::try_unwrap(val.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl Regex {
    /// Returns the regex pattern string (without surrounding `/` delimiters).
    #[tracing::instrument(skip(self))]
    pub fn pattern(&self) -> String {
        self.0.regex().as_str().to_string()
    }

    /// Returns `true` if the pattern matches the given string.
    #[tracing::instrument(skip(self))]
    pub fn is_match(&self, text: String) -> bool {
        self.0.regex().is_match(&text)
    }

    /// Returns the SurrealQL literal representation, e.g. `/pattern/`.
    #[tracing::instrument(skip(self))]
    pub fn to_surreal_literal(&self) -> String {
        format!("/{}/", self.0.regex().as_str())
    }
}

impl elicitation::ElicitComplete for Regex {}

mod emit_impls {
    use super::Regex;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Regex {
        fn to_code_literal(&self) -> TokenStream {
            let pattern = self.0.regex().as_str().to_string();
            quote::quote! {
                ::surrealdb_types::Regex::from(
                    ::regex::Regex::new(#pattern).expect("valid regex pattern")
                )
                .into()
            }
        }
    }
}
