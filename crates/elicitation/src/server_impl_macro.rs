//! Macro for generating elicit_checked methods on external types.
//!
//! This macro reduces duplication when adding server-side integration
//! to types that have manual `Elicitation` implementations.

/// Generate server-side `Elicit` trait implementation for a type.
///
/// This macro creates a trait implementation with the standard `elicit_checked` pattern,
/// matching what `#[derive(Elicit)]` generates for user types.
///
/// # Example
///
/// ```ignore
/// server_elicit_impl!(url::Url);
/// server_elicit_impl!(uuid::Uuid);
/// ```
///
/// Expands to:
/// ```ignore
/// #[async_trait::async_trait]
/// impl Elicit for url::Url {
///     async fn elicit_checked(
///         peer: Peer<RoleServer>,
///     ) -> Result<Self, ElicitError> {
///         use crate::{ElicitServer, Elicitation};
///         let server = ElicitServer::new(peer);
///         Self::elicit(&server).await
///     }
/// }
/// ```
#[macro_export]
macro_rules! server_elicit_impl {
    ($type:ty) => {
        #[::async_trait::async_trait]
        impl $crate::Elicit for $type {
            async fn elicit_checked(
                peer: $crate::rmcp::service::Peer<$crate::rmcp::service::RoleServer>,
            ) -> $crate::ElicitResult<Self> {
                use $crate::{ElicitServer, Elicitation};

                let server = ElicitServer::new(peer);
                Self::elicit(&server).await
            }
        }
    };
}
