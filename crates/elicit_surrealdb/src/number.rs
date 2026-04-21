//! [`surrealdb_types::Number`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(surrealdb_types::Number, as Number, forward_serde);
elicit_newtype_traits!(Number, surrealdb_types::Number, [eq, display]);

#[reflect_methods]
impl Number {
    /// `true` if this number is NaN.
    #[tracing::instrument(skip(self))]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }

    /// `true` if this number holds a 64-bit integer.
    #[tracing::instrument(skip(self))]
    pub fn is_int(&self) -> bool {
        self.0.is_int()
    }

    /// `true` if this number holds a 64-bit float.
    #[tracing::instrument(skip(self))]
    pub fn is_float(&self) -> bool {
        self.0.is_float()
    }

    /// `true` if this number holds a decimal.
    #[tracing::instrument(skip(self))]
    pub fn is_decimal(&self) -> bool {
        self.0.is_decimal()
    }

    /// Convert to `i64`, or `None` if conversion is not possible.
    #[tracing::instrument(skip(self))]
    pub fn to_int(&self) -> Option<i64> {
        self.0.to_int()
    }

    /// Convert to `f64`, or `None` if conversion is not possible.
    #[tracing::instrument(skip(self))]
    pub fn to_f64(&self) -> Option<f64> {
        self.0.to_f64()
    }
}

impl elicitation::ElicitComplete for Number {}

mod emit_impls {
    use super::Number;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Number {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Number is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Number>(#json)
                    .expect("valid Number JSON")
                    .into()
            }
        }
    }
}
