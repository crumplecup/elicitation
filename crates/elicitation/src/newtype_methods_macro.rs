//! Declarative macro for generating newtype wrappers with method delegation and MCP tools.
//!
//! This module provides the `elicit_newtype_methods!` macro that combines:
//! 1. Newtype wrapper creation
//! 2. Method delegation to inner type
//! 3. Parameter struct generation
//! 4. MCP tool wrapper generation
//!
//! # Example
//!
//! ```ignore
//! elicit_newtype_methods! {
//!     Client => reqwest::Client,
//!     fn get(url: &str) -> Result<Response, Error>;
//!     async fn post(url: &str, body: Vec<u8>) -> Result<Response, Error>;
//! }
//! ```
//!
//! # Limitations
//!
//! **Generic methods are not supported** in the declarative macro due to parsing
//! limitations. For generic method support, use the `#[reflect_methods]` proc macro
//! from `elicitation_derive` which has full AST access via `syn`.

/// Generates both newtype and method wrappers with MCP tools.
///
/// This macro combines newtype creation with automatic method delegation
/// and MCP tool generation, eliminating boilerplate.
///
/// # Syntax
///
/// ```ignore
/// elicit_newtype_methods! {
///     WrapperName => inner::path::Type,
///     fn method_name(param: Type) -> ReturnType;
///     async fn async_method(param: Type) -> ReturnType;
/// }
/// ```
///
/// # Example
///
/// ```ignore
/// use elicitation::elicit_newtype_methods;
///
/// // Wrap reqwest::Client with methods
/// elicit_newtype_methods! {
///     Client => reqwest::Client,
///     fn get(url: &str) -> Result<Response, Error>;
///     async fn post(url: &str, body: Vec<u8>) -> Result<Response, Error>;
/// }
/// ```
///
/// This generates:
/// 1. Newtype: `pub struct Client(pub reqwest::Client)`
/// 2. Delegating methods in `impl Client { fn get(...) { self.0.get(...) } }`
/// 3. Parameter structs: `GetParams { url: String }`, `PostParams { url: String, body: Vec<u8> }`
/// 4. MCP tool wrappers: `get_tool(params: Parameters<GetParams>) -> Result<Json<Response>, ErrorData>`
#[macro_export]
macro_rules! elicit_newtype_methods {
    // Entry point: WrapperName => InnerType, method list
    (
        $wrapper_name:ident => $inner_path:path,
        $($methods:tt)*
    ) => {
        // Generate the newtype
        $crate::elicit_newtype!($inner_path, as $wrapper_name);

        // Parse and generate methods
        $crate::__elicit_methods_impl! {
            $wrapper_name,
            $($methods)*
        }
    };
}

/// Internal macro for parsing method signatures.
/// Users should use `elicit_newtype_methods!` instead.
#[macro_export]
#[doc(hidden)]
macro_rules! __elicit_methods_impl {
    // Base case: no methods
    ($wrapper_name:ident,) => {};

    // Parse each method - pass entire signature to classifier
    (
        $wrapper_name:ident,
        $($method_tokens:tt)*
    ) => {
        $crate::__classify_method! {
            $wrapper_name,
            $($method_tokens)*
        }
    };
}

/// Internal macro for classifying and routing method types.
#[macro_export]
#[doc(hidden)]
macro_rules! __classify_method {
    // Synchronous method
    (
        $wrapper_name:ident,
        fn $method:ident ( $($param_name:ident : $param_ty:ty),* $(,)? ) -> $ret:ty ; $($rest:tt)*
    ) => {
        $crate::__elicit_method_generate! {
            $wrapper_name,
            fn $method($($param_name: $param_ty),*) -> $ret
        }

        $crate::__elicit_methods_impl! {
            $wrapper_name,
            $($rest)*
        }
    };

    // Async method
    (
        $wrapper_name:ident,
        async fn $method:ident ( $($param_name:ident : $param_ty:ty),* $(,)? ) -> $ret:ty ; $($rest:tt)*
    ) => {
        $crate::__elicit_method_generate! {
            $wrapper_name,
            async fn $method($($param_name: $param_ty),*) -> $ret
        }

        $crate::__elicit_methods_impl! {
            $wrapper_name,
            $($rest)*
        }
    };
}

/// Internal macro for generating a single method's code.
#[macro_export]
#[doc(hidden)]
macro_rules! __elicit_method_generate {
    // Synchronous method
    (
        $wrapper_name:ident,
        fn $method:ident($($param_name:ident: $param_ty:ty),*) -> $ret:ty
    ) => {
        $crate::paste::paste! {
            // Generate parameter struct if there are parameters
            $crate::__elicit_param_struct! {
                [<$method:camel Params>],
                $($param_name: $param_ty),*
            }

            // Generate delegating method
            impl $wrapper_name {
                #[doc = concat!("Delegates to inner `", stringify!($method), "` method.")]
                pub fn $method(&self, $($param_name: $param_ty),*) -> $ret {
                    self.0.$method($($param_name),*)
                }
            }

            // Generate MCP tool wrapper
            $crate::__elicit_tool_wrapper! {
                $wrapper_name,
                fn $method($($param_name: $param_ty),*) -> $ret
            }
        }
    };

    // Async method
    (
        $wrapper_name:ident,
        async fn $method:ident($($param_name:ident: $param_ty:ty),*) -> $ret:ty
    ) => {
        $crate::paste::paste! {
            // Generate parameter struct if there are parameters
            $crate::__elicit_param_struct! {
                [<$method:camel Params>],
                $($param_name: $param_ty),*
            }

            // Generate async delegating method
            impl $wrapper_name {
                #[doc = concat!("Delegates to inner async `", stringify!($method), "` method.")]
                pub async fn $method(&self, $($param_name: $param_ty),*) -> $ret {
                    self.0.$method($($param_name),*).await
                }
            }

            // Generate async MCP tool wrapper
            $crate::__elicit_tool_wrapper! {
                $wrapper_name,
                async fn $method($($param_name: $param_ty),*) -> $ret
            }
        }
    };
}

/// Internal macro for generating parameter structs.
#[macro_export]
#[doc(hidden)]
macro_rules! __elicit_param_struct {
    // No parameters - don't generate a struct
    ($struct_name:ident,) => {};

    // With parameters - generate struct with Elicit and JsonSchema derives
    ($struct_name:ident, $($param_name:ident: $param_ty:ty),+) => {
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
            $crate::Elicit,
            ::schemars::JsonSchema,
        )]
        pub struct $struct_name {
            $(pub $param_name: $crate::__convert_param_type!($param_ty)),+
        }
    };
}

/// Internal macro for converting parameter types.
/// &str -> String, &[T] -> Vec<T>, etc.
#[macro_export]
#[doc(hidden)]
macro_rules! __convert_param_type {
    // &str -> String
    (&str) => { String };
    // &[T] -> Vec<T>
    (&[$inner:ty]) => { Vec<$inner> };
    // &T -> T (will require Clone at runtime)
    (&$inner:ty) => { $inner };
    // Owned types pass through
    ($ty:ty) => { $ty };
}

/// Internal macro for generating MCP tool wrappers.
#[macro_export]
#[doc(hidden)]
macro_rules! __elicit_tool_wrapper {
    // Synchronous method with no parameters
    (
        $wrapper_name:ident,
        fn $method:ident() -> $ret:ty
    ) => {
        $crate::paste::paste! {
            impl $wrapper_name {
                #[doc = concat!("`", stringify!($method), "` MCP tool wrapper.")]
                // Note: #[tool] attribute must be added manually or via proc macro
                pub fn [<$method _tool>](&self) -> ::std::result::Result<
                    $crate::rmcp::handler::server::wrapper::Json<$ret>,
                    $crate::rmcp::ErrorData
                > {
                    let result = self.$method();
                    Ok($crate::rmcp::handler::server::wrapper::Json(result))
                }
            }
        }
    };

    // Synchronous method with parameters
    (
        $wrapper_name:ident,
        fn $method:ident($($param_name:ident: $param_ty:ty),+) -> $ret:ty
    ) => {
        $crate::paste::paste! {
            impl $wrapper_name {
                #[doc = concat!("`", stringify!($method), "` MCP tool wrapper.")]
                pub fn [<$method _tool>](
                    &self,
                    params: $crate::rmcp::handler::server::wrapper::Parameters<[<$method:camel Params>]>,
                ) -> ::std::result::Result<
                    $crate::rmcp::handler::server::wrapper::Json<$ret>,
                    $crate::rmcp::ErrorData
                > {
                    let result = self.$method($($crate::__convert_param_access!(params, $param_name, $param_ty)),+);
                    Ok($crate::rmcp::handler::server::wrapper::Json(result))
                }
            }
        }
    };

    // Async method with no parameters
    (
        $wrapper_name:ident,
        async fn $method:ident() -> $ret:ty
    ) => {
        $crate::paste::paste! {
            impl $wrapper_name {
                #[doc = concat!("`", stringify!($method), "` MCP tool wrapper (async).")]
                pub async fn [<$method _tool>](&self) -> ::std::result::Result<
                    $crate::rmcp::handler::server::wrapper::Json<$ret>,
                    $crate::rmcp::ErrorData
                > {
                    let result = self.$method().await;
                    Ok($crate::rmcp::handler::server::wrapper::Json(result))
                }
            }
        }
    };

    // Async method with parameters
    (
        $wrapper_name:ident,
        async fn $method:ident($($param_name:ident: $param_ty:ty),+) -> $ret:ty
    ) => {
        $crate::paste::paste! {
            impl $wrapper_name {
                #[doc = concat!("`", stringify!($method), "` MCP tool wrapper (async).")]
                pub async fn [<$method _tool>](
                    &self,
                    params: $crate::rmcp::handler::server::wrapper::Parameters<[<$method:camel Params>]>,
                ) -> ::std::result::Result<
                    $crate::rmcp::handler::server::wrapper::Json<$ret>,
                    $crate::rmcp::ErrorData
                > {
                    let result = self.$method($($crate::__convert_param_access!(params, $param_name, $param_ty)),+).await;
                    Ok($crate::rmcp::handler::server::wrapper::Json(result))
                }
            }
        }
    };
}

/// Internal macro for accessing parameters with conversion.
/// params.0.field_name with appropriate conversion (e.g., .as_str() for &str)
#[macro_export]
#[doc(hidden)]
macro_rules! __convert_param_access {
    // &str: call .as_str() on the String
    ($params:ident, $field:ident, &str) => {
        $params.0.$field.as_str()
    };
    // &[T]: take reference to Vec<T>
    ($params:ident, $field:ident, &[$inner:ty]) => {
        &$params.0.$field
    };
    // &T: take reference
    ($params:ident, $field:ident, &$inner:ty) => {
        &$params.0.$field
    };
    // Owned types: direct access
    ($params:ident, $field:ident, $ty:ty) => {
        $params.0.$field
    };
}
