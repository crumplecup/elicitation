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
///     #[elicitation::rmcp::tool]
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
            /// from the peer via MCP.
            ///
            /// Automatically registered as an MCP tool via `#[rmcp::tool]`.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// // In a tool handler with peer: Peer<RoleServer>
            /// let config = Config::elicit_checked(peer).await?;
            /// ```
            ///
            /// # Implementation Status
            ///
            /// Currently returns a stub error. Full implementation requires:
            /// 1. Prompt generation from type metadata
            /// 2. Multi-turn interaction support (for complex types)
            /// 3. Validation and parsing logic
            /// 4. Style system integration
            ///
            /// See Phase 2 in rmcp_integration_plan.md for details.
            #[elicitation::rmcp::tool]
            pub async fn elicit_checked(
                peer: elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleServer>,
            ) -> Result<Self, elicitation::ElicitError> {
                let _ = peer;  // TODO: use peer.create_message() for elicitation
                let kind = elicitation::ElicitErrorKind::ParseError(
                    format!("Server-side elicitation not yet implemented for {}", #type_name_str)
                );
                Err(elicitation::ElicitError::new(kind))
            }
        }

        // Submit to inventory for automatic tool discovery
        elicitation::inventory::submit! {
            elicitation::ElicitToolDescriptor::new(#type_name_str, module_path!())
        }
    }
}
