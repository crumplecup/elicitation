//! Main derive macro dispatcher.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_macro_input};

/// Expand the #[derive(Elicit)] macro.
///
/// Dispatches to enum or struct implementation based on the input type.
/// Generates the Elicitation trait impl AND MCP tool function.
///
/// **Important:** Users must also add `#[derive(schemars::JsonSchema)]` for MCP tool compatibility.
/// This is required because the generated `elicit_checked()` function returns `Self`, which
/// must implement JsonSchema for rmcp's `#[tool]` attribute to work.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Generate Elicitation trait impl
    let elicit_impl = match &input.data {
        Data::Enum(_) => crate::enum_impl::expand_enum(input.clone()),
        Data::Struct(_) => crate::struct_impl::expand_struct(input.clone()),
        Data::Union(_) => {
            let error =
                syn::Error::new_spanned(&input.ident, "Elicit cannot be derived for unions");
            return error.to_compile_error().into();
        }
    };

    // Generate MCP tool function, wrapped in an allow(unexpected_cfgs) mod so that
    // #[cfg(not(creusot))] inside tool_gen does not leak warnings into the downstream crate.
    let tool_impl = crate::tool_gen::generate_tool_function(&input);
    let tool_mod_name = quote::format_ident!("_elicit_tool_{}", &input.ident);
    let elicit_tokens: proc_macro2::TokenStream = elicit_impl.into();
    let combined = quote! {
        #elicit_tokens
        #[allow(unexpected_cfgs, non_snake_case)]
        mod #tool_mod_name {
            use super::*;
            #tool_impl
        }
    };

    TokenStream::from(combined)
}
