//! Method discovery for extracting public methods from types.
//!
//! This module provides functionality to discover public methods on wrapped types
//! through compile-time introspection.

use syn::{FnArg, ImplItemFn, ItemImpl, ReturnType, Signature, Type, Visibility};

/// Information about a discovered method.
#[derive(Debug, Clone)]
pub struct MethodInfo {
    /// Method name
    pub name: String,
    /// Method signature
    pub signature: Signature,
    /// Method visibility
    pub visibility: Visibility,
    /// Whether the method takes &self
    pub has_self: bool,
    /// Whether the method takes &mut self
    pub has_mut_self: bool,
    /// Method parameters (excluding self)
    pub params: Vec<FnArg>,
    /// Return type
    pub return_type: ReturnType,
}

/// Discovers public methods from an impl block.
///
/// This extracts all public method signatures from the newtype's inner type.
/// Currently this is a placeholder - actual discovery will require:
/// 1. Extracting the inner type from the newtype
/// 2. Querying the compiler for available methods (challenging at proc macro time)
/// 3. Or requiring users to manually list methods to wrap
///
/// For now, we'll implement a simpler approach: the user provides an empty
/// impl block and we generate wrappers for all methods they explicitly add.
pub fn discover_methods(_impl_block: &ItemImpl) -> Vec<MethodInfo> {
    // TODO: Implement actual method discovery
    // This is challenging because:
    // 1. We need to know what methods exist on reqwest::Client at proc macro time
    // 2. Proc macros run before type checking, so we can't query the type system
    // 3. We can't easily inspect methods on external types
    //
    // Possible approaches:
    // A) Require explicit method signatures in impl block (user lists what to wrap)
    // B) Use a configuration file/macro with method names
    // C) Generate at runtime using reflection (not available in Rust)
    //
    // For Milestone 2, we'll use approach A: users add method signatures to wrap

    vec![]
}

/// Extracts the inner type from a newtype wrapper.
///
/// Given `impl TypeName`, determines what inner type is being wrapped.
/// This requires the newtype to follow the standard pattern:
/// `pub struct TypeName(pub InnerType);`
fn extract_inner_type(_impl_block: &ItemImpl) -> Option<Type> {
    // TODO: Parse the newtype struct to find the inner type
    // This requires reading the struct definition, which isn't available
    // in the impl block alone
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_methods_placeholder() {
        // Placeholder test - will implement when discovery is ready
        let methods: Vec<MethodInfo> = vec![];
        assert_eq!(methods.len(), 0);
    }
}
