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
///     /// Uses server-side elicitation to interactively elicit a value from the
///     /// requesting client via MCP.
///     ///
///     /// Automatically registered as an MCP tool via `#[rmcp::tool]`.
///     #[elicitation::rmcp::tool]
///     pub async fn elicit_checked(
///         peer: elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleServer>,
///     ) -> Result<Self, elicitation::ElicitError> {
///         use elicitation::{Elicitation, ElicitServer};
///         let server = ElicitServer::new(peer);
///         Self::elicit(&server).await
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
            /// Uses server-side elicitation to interactively elicit a value from the
            /// requesting client via MCP.
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
            /// # Implementation
            ///
            /// Creates an `ElicitServer` wrapper and delegates to the `Elicitation` trait.
            /// This provides server-side elicitation with the same style system and
            /// validation logic as client-side elicitation.
            #[elicitation::rmcp::tool]
            pub async fn elicit_checked(
                peer: elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleServer>,
            ) -> Result<Self, elicitation::ElicitError> {
                use elicitation::{ElicitServer, Elicitation};

                // Create server wrapper
                let server = ElicitServer::new(peer);

                // Delegate to trait implementation
                Self::elicit(&server).await
            }
        }

        // Submit to inventory for automatic tool discovery
        elicitation::inventory::submit! {
            elicitation::ElicitToolDescriptor::new(#type_name_str, module_path!())
        }
    }
}
