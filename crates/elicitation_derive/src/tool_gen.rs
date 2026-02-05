//! MCP tool generation for #[derive(Elicit)].

use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generate an MCP tool method for a type with #[derive(Elicit)].
///
/// Generates a method on the type's impl block that provides verified,
/// registered elicitation via the MCP protocol. Follows Rust's `checked_*`
/// idiom for operations that add verification and safety.
///
/// Also submits the type to inventory for automatic tool discovery.
///
/// Generates:
/// ```ignore
/// impl TypeName {
///     /// Checked elicitation via MCP protocol.
///     ///
///     /// This is the verified, registered variant suitable for production use.
///     /// Automatically registered as an MCP tool via `#[rmcp::tool]`.
///     #[cfg_attr(not(test), elicitation::rmcp::tool)]
///     pub async fn elicit_checked(
///         client: std::sync::Arc<elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>>,
///     ) -> Result<Self, elicitation::ElicitError> {
///         use elicitation::{Elicitation, ElicitClient};
///         Self::elicit(&ElicitClient::new(client)).await
///     }
/// }
///
/// // Inventory submission for automatic discovery
/// inventory::submit! {
///     elicitation::ElicitToolDescriptor::new("TypeName", module_path!())
/// }
/// ```
pub fn generate_tool_function(input: &DeriveInput) -> TokenStream {
    let type_name = &input.ident;
    let type_name_str = type_name.to_string();

    quote! {
        impl #type_name {
            /// Checked elicitation via MCP protocol.
            ///
            /// This is the verified, registered variant suitable for production use.
            /// Uses the derived `Elicitation` impl to interactively elicit a value
            /// from the user via MCP.
            ///
            /// Automatically registered as an MCP tool via `#[rmcp::tool]` in non-test builds.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let client = Arc::new(peer.clone());
            /// let config = Config::elicit_checked(client).await?;
            /// ```
            #[cfg_attr(not(test), elicitation::rmcp::tool)]
            pub async fn elicit_checked(
                client: std::sync::Arc<elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>>,
            ) -> Result<Self, elicitation::ElicitError> {
                use elicitation::{Elicitation, ElicitClient};
                Self::elicit(&ElicitClient::new(client)).await
            }
        }

        // Submit to inventory for automatic tool discovery
        elicitation::inventory::submit! {
            elicitation::ElicitToolDescriptor::new(#type_name_str, module_path!())
        }
    }
}
