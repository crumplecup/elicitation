//! MCP tool generation for #[derive(Elicit)].

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

/// Generate an MCP tool function for a type with #[derive(Elicit)].
///
/// Generates:
/// ```ignore
/// #[rmcp::tool]
/// pub async fn elicit_type_name(
///     client: &rmcp::service::Peer<rmcp::service::RoleClient>,
/// ) -> Result<TypeName, elicitation::ElicitError> {
///     use elicitation::{Elicitation, ElicitClient};
///     TypeName::elicit(&ElicitClient::new(client)).await
/// }
/// ```
pub fn generate_tool_function(input: &DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let fn_name = format_ident!("elicit_{}", to_snake_case(&type_name.to_string()));

    quote! {
        /// Auto-generated MCP tool function for eliciting [`#type_name`].
        ///
        /// This function uses the derived `Elicitation` impl to
        /// interactively elicit a value from the user via MCP.
        ///
        /// # Usage
        ///
        /// Call this directly from an MCP server handler, or add `#[rmcp::tool]`
        /// attribute for automatic registration with tool routers.
        pub async fn #fn_name(
            client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
        ) -> Result<#type_name, elicitation::ElicitError> {
            use elicitation::{Elicitation, ElicitClient};
            #type_name::elicit(&ElicitClient::new(client)).await
        }
    }
}

/// Convert PascalCase or camelCase to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 5);
    let mut is_first = true;

    for ch in s.chars() {
        if ch.is_uppercase() {
            if !is_first {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
        is_first = false;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Config"), "config");
        assert_eq!(to_snake_case("TomlAct"), "toml_act");
        assert_eq!(to_snake_case("TomlActInput"), "toml_act_input");
        assert_eq!(to_snake_case("HTTPConfig"), "h_t_t_p_config");
        assert_eq!(to_snake_case("MyType"), "my_type");
    }
}
