//! Main derive macro dispatcher.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

/// Expand the #[derive(Elicit)] macro.
///
/// Dispatches to enum or struct implementation based on the input type.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Enum(_) => crate::enum_impl::expand_enum(input),
        Data::Struct(_) => crate::struct_impl::expand_struct(input),
        Data::Union(_) => {
            let error = syn::Error::new_spanned(input.ident, "Elicit cannot be derived for unions");
            error.to_compile_error().into()
        }
    }
}
