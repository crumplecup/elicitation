//! Macros for integrating elicitation with rmcp tool routers.
//!
//! This module provides two macros for different integration patterns:
//! - `elicit_router!` - Creates a standalone router struct
//! - `elicit_tools!` - Generates tool methods inside an existing impl block

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
                        &self,
                        peer: $crate::rmcp::service::Peer<$crate::rmcp::service::RoleServer>,
                    ) -> ::std::result::Result<$ty, $crate::ElicitError> {
                        <$ty>::elicit_checked(peer).await
                    }
                }
            )+
        }
    };
}

/// Generates elicitation tool methods inside an existing `#[tool_router]` impl block.
///
/// Use this when you want to add elicitation tools to an existing server type
/// without creating a separate router. The generated methods will be part of
/// your server's `ToolRouter<YourServer>`.
///
/// # Example
///
/// ```ignore
/// use elicitation::elicit_tools;
///
/// #[rmcp::tool_router]
/// impl MyServer {
///     // Your existing tool methods...
///     
///     #[tool]
///     async fn my_custom_tool(&self) -> Result<String, rmcp::ErrorData> {
///         Ok("hello".to_string())
///     }
///     
///     // Add elicitation tools
///     elicit_tools! {
///         CacheKeyNewParams,
///         StorageNewParams,
///         MyOtherType,
///     }
/// }
/// ```
///
/// This generates tool methods like:
/// ```ignore
/// #[tool]
/// async fn elicit_cache_key_new_params(
///     &self,
///     peer: Peer<RoleServer>,
/// ) -> Result<CacheKeyNewParams, ElicitError> {
///     CacheKeyNewParams::elicit_checked(peer).await
/// }
/// ```
///
/// # Pattern Comparison
///
/// **Standalone router** (when you only want elicitation):
/// ```ignore
/// elicit_router! {
///     pub ElicitRouter: Type1, Type2, Type3
/// }
/// ```
///
/// **Embedded in existing router** (when you have other tools):
/// ```ignore
/// #[tool_router]
/// impl MyServer {
///     // Other tools...
///     
///     elicit_tools! { Type1, Type2, Type3 }
/// }
/// ```
#[macro_export]
macro_rules! elicit_tools {
    ($($ty:ty),+ $(,)?) => {
        $(
            $crate::paste::paste! {
                #[doc = concat!("Elicit `", stringify!($ty), "` via MCP.")]
                #[$crate::rmcp::tool]
                async fn [<elicit_ $ty:snake>] (
                    &self,
                    peer: $crate::rmcp::service::Peer<$crate::rmcp::service::RoleServer>,
                ) -> ::std::result::Result<$ty, $crate::ElicitError> {
                    <$ty>::elicit_checked(peer).await
                }
            }
        )+
    };
}
