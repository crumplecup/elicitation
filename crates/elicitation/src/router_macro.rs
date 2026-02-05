//! Macro for aggregating elicit tools into an rmcp ToolRouter.
//!
//! This module provides the `elicit_router!` macro which creates an aggregator
//! impl block that registers all `elicit_checked` methods from specified types
//! with rmcp's tool system using `#[tool_router]`.

/// Creates an aggregator struct that registers elicit_checked tools from multiple types.
///
/// # Example
///
/// ```ignore
/// elicit_router! {
///     pub ElicitRouter: Config, User, Settings
/// }
/// ```
///
/// This generates:
/// - A struct `ElicitRouter`
/// - An impl block with `#[tool_router]` attribute
/// - Proxy methods with `#[tool]` for each type's elicit_checked
///
/// The proxy methods forward calls to the original `Type::elicit_checked(peer)`.
#[macro_export]
macro_rules! elicit_router {
    ($vis:vis $name:ident : $($ty:ty),+ $(,)?) => {
        $vis struct $name;

        #[$crate::rmcp::tool_router]
        impl $name {
            $(
                $crate::paste::paste! {
                    #[doc = concat!("Elicit `", stringify!($ty), "` via MCP.")]
                    #[$crate::rmcp::tool]
                    async fn [<elicit_ $ty:snake>] (
                        peer: $crate::rmcp::service::Peer<$crate::rmcp::service::RoleServer>,
                    ) -> ::std::result::Result<$ty, $crate::ElicitError> {
                        <$ty>::elicit_checked(peer).await
                    }
                }
            )+
        }
    };
}
