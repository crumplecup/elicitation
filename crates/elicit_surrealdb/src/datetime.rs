//! [`surrealdb_types::Datetime`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};

elicit_newtype!(surrealdb_types::Datetime, as Datetime, forward_serde);
elicit_newtype_traits!(Datetime, surrealdb_types::Datetime, [eq, display, from_str]);

impl elicitation::ElicitComplete for Datetime {}

mod emit_impls {
    use super::Datetime;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Datetime {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Datetime is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Datetime>(#json)
                    .expect("valid Datetime JSON")
                    .into()
            }
        }
    }
}
