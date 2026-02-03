//! Main derive macro dispatcher.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Expand the #[derive(Elicit)] macro.
///
/// Dispatches to enum or struct implementation based on the input type.
/// Generates both the Elicitation trait impl AND an MCP tool function.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Generate Elicitation trait impl
    let elicit_impl = match &input.data {
        Data::Enum(_) => crate::enum_impl::expand_enum(input.clone()),
        Data::Struct(_) => crate::struct_impl::expand_struct(input.clone()),
        Data::Union(_) => {
            let error = syn::Error::new_spanned(&input.ident, "Elicit cannot be derived for unions");
            return error.to_compile_error().into();
        }
    };

    // Generate MCP tool function
    let tool_impl = crate::tool_gen::generate_tool_function(&input);

    // Combine both implementations
    let elicit_tokens: proc_macro2::TokenStream = elicit_impl.into();
    let combined = quote! {
        #elicit_tokens
        #tool_impl
    };

    TokenStream::from(combined)
}
