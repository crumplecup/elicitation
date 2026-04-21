//! [`surrealdb_types::Duration`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Duration, as Duration, forward_serde);
elicit_newtype_traits!(Duration, surrealdb_types::Duration, [eq, display, from_str]);

#[reflect_methods]
impl Duration {
    /// Total nanoseconds.
    #[tracing::instrument(skip(self))]
    pub fn nanos(&self) -> u128 {
        self.0.nanos()
    }

    /// Total microseconds.
    #[tracing::instrument(skip(self))]
    pub fn micros(&self) -> u128 {
        self.0.micros()
    }

    /// Total milliseconds.
    #[tracing::instrument(skip(self))]
    pub fn millis(&self) -> u128 {
        self.0.millis()
    }

    /// Whole seconds component.
    #[tracing::instrument(skip(self))]
    pub fn secs(&self) -> u64 {
        self.0.secs()
    }

    /// Whole minutes component.
    #[tracing::instrument(skip(self))]
    pub fn mins(&self) -> u64 {
        self.0.mins()
    }

    /// Whole hours component.
    #[tracing::instrument(skip(self))]
    pub fn hours(&self) -> u64 {
        self.0.hours()
    }

    /// Whole days component.
    #[tracing::instrument(skip(self))]
    pub fn days(&self) -> u64 {
        self.0.days()
    }

    /// Whole weeks component.
    #[tracing::instrument(skip(self))]
    pub fn weeks(&self) -> u64 {
        self.0.weeks()
    }

    /// Whole years component.
    #[tracing::instrument(skip(self))]
    pub fn years(&self) -> u64 {
        self.0.years()
    }
}

impl elicitation::ElicitComplete for Duration {}

mod emit_impls {
    use super::Duration;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Duration {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Duration is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Duration>(#json)
                    .expect("valid Duration JSON")
                    .into()
            }
        }
    }
}
