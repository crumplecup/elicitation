//! Method discovery for extracting public methods from types.
//!
//! This module provides functionality to discover public methods on wrapped types
//! through compile-time introspection.

use syn::{FnArg, Generics, ImplItemFn, ItemImpl, ReturnType, Signature, Visibility};

/// Information about a discovered method.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// Method name
    pub name: String,
    /// Method signature
    pub signature: Signature,
    /// Method parameters (excluding self)
    pub params: Vec<FnArg>,
    /// Return type
    pub return_type: ReturnType,
    /// Generic parameters (type parameters, lifetimes, const params)
    pub generics: Generics,
    /// Whether this method consumes self (true) or borrows &self (false)
    pub is_consuming: bool,
}

impl MethodInfo {
    /// Returns true if this method has generic type parameters.
    pub fn is_generic(&self) -> bool {
        !self.generics.params.is_empty()
    }

    /// Returns true if this method consumes self instead of borrowing &self.
    pub fn is_consuming(&self) -> bool {
        self.is_consuming
    }
}

/// Discovers public methods from an impl block.
///
/// This extracts all public method signatures that the user has explicitly
/// added to the impl block. Users write the methods they want to wrap,
/// and we generate parameter structs and MCP tool wrappers for them.
///
/// # Approach
///
/// Users add explicit method signatures to the impl block:
/// ```ignore
/// #[reflect_methods]
/// impl Client {
///     pub async fn get(&self, url: &str) -> Result<Response, Error> {
///         self.0.get(url).await
///     }
/// }
/// ```
///
/// We extract these methods and generate wrappers for each one.
pub fn discover_methods(impl_block: &ItemImpl) -> Vec<MethodInfo> {
    impl_block
        .items
        .iter()
        .filter_map(|item| {
            if let syn::ImplItem::Fn(method) = item {
                // Only process public methods
                if matches!(method.vis, Visibility::Public(_)) {
                    extract_method_info(method)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

/// Extracts method information from a method definition.
fn extract_method_info(method: &ImplItemFn) -> Option<MethodInfo> {
    let name = method.sig.ident.to_string();
    let signature = method.sig.clone();

    // Detect if method consumes self or borrows &self
    let is_consuming = signature.inputs.iter().any(|arg| {
        if let FnArg::Receiver(receiver) = arg {
            // If there's no reference, it's consuming (self)
            // If there's a reference, it's borrowing (&self)
            receiver.reference.is_none()
        } else {
            false
        }
    });

    // Extract non-self parameters
    let params: Vec<FnArg> = signature
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(_) => Some(arg.clone()),
        })
        .collect();

    let return_type = signature.output.clone();
    let generics = signature.generics.clone();

    Some(MethodInfo {
        name,
        signature,
        params,
        return_type,
        generics,
        is_consuming,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_methods_from_impl_block() {
        // Parse a sample impl block
        let impl_block: ItemImpl = syn::parse_quote! {
            impl Client {
                pub fn get(&self, url: &str) -> Result<Response, Error> {
                    self.0.get(url)
                }

                pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<Response, Error> {
                    self.0.post(url, body).await
                }

                // Private method - should be filtered out
                fn internal_helper(&self) -> bool {
                    true
                }
            }
        };

        let methods = discover_methods(&impl_block);

        // Should discover 2 public methods, ignoring the private one
        assert_eq!(methods.len(), 2);

        // Check first method
        assert_eq!(methods[0].name, "get");
        assert_eq!(methods[0].params.len(), 1); // url parameter

        // Check second method
        assert_eq!(methods[1].name, "post");
        assert_eq!(methods[1].params.len(), 2); // url and body parameters
    }

    #[test]
    fn test_discover_no_methods() {
        let impl_block: ItemImpl = syn::parse_quote! {
            impl Client {
                // No methods
            }
        };

        let methods = discover_methods(&impl_block);
        assert_eq!(methods.len(), 0);
    }

    #[test]
    fn test_discover_generic_methods() {
        let impl_block: ItemImpl = syn::parse_quote! {
            impl StringList {
                pub fn contains<T>(&self, item: &T) -> bool
                where
                    T: Elicitation + JsonSchema + PartialEq,
                {
                    self.0.contains(item)
                }

                pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<V>
                where
                    K: Elicitation + JsonSchema + Hash + Eq,
                    V: Elicitation + JsonSchema,
                {
                    self.0.insert(key, value)
                }
            }
        };

        let methods = discover_methods(&impl_block);

        // Should discover 2 generic methods
        assert_eq!(methods.len(), 2);

        // Check first method - has 1 type parameter
        assert_eq!(methods[0].name, "contains");
        assert_eq!(methods[0].params.len(), 1); // item parameter
        assert!(methods[0].is_generic());
        assert_eq!(methods[0].generics.params.len(), 1); // T

        // Check second method - has 2 type parameters
        assert_eq!(methods[1].name, "insert");
        assert_eq!(methods[1].params.len(), 2); // key and value parameters
        assert!(methods[1].is_generic());
        assert_eq!(methods[1].generics.params.len(), 2); // K, V
    }

    #[test]
    fn test_discover_mixed_generic_non_generic() {
        let impl_block: ItemImpl = syn::parse_quote! {
            impl Client {
                pub fn get(&self, url: &str) -> Result<Response, Error> {
                    self.0.get(url)
                }

                pub fn fetch<T>(&self, url: &str) -> Result<T, Error>
                where
                    T: Elicitation + JsonSchema,
                {
                    self.0.fetch(url)
                }
            }
        };

        let methods = discover_methods(&impl_block);

        assert_eq!(methods.len(), 2);

        // First method is non-generic
        assert_eq!(methods[0].name, "get");
        assert!(!methods[0].is_generic());
        assert_eq!(methods[0].generics.params.len(), 0);

        // Second method is generic
        assert_eq!(methods[1].name, "fetch");
        assert!(methods[1].is_generic());
        assert_eq!(methods[1].generics.params.len(), 1);
    }
}
