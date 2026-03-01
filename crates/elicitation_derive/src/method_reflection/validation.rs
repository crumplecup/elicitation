//! Validation for generic method bounds.
//!
//! This module ensures that generic type parameters meet the requirements
//! for MCP tool generation and library self-compatibility.

use syn::{
    GenericParam, Generics, PredicateType, TraitBound, TraitBoundModifier, Type, TypeParam,
    TypeParamBound, WherePredicate,
};

/// Validates that all generic type parameters have required bounds.
///
/// For MCP compatibility and library self-compatibility, all type parameters must have:
/// - `Elicitation` trait bound
/// - `JsonSchema` trait bound
///
/// # Returns
///
/// - `Ok(())` if all type parameters have required bounds
/// - `Err(syn::Error)` with diagnostic message if validation fails
pub fn validate_generic_bounds(generics: &Generics) -> syn::Result<()> {
    // Extract all type parameters
    let type_params: Vec<&TypeParam> = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param)
            } else {
                None
            }
        })
        .collect();

    // Check each type parameter
    for type_param in type_params {
        let param_name = &type_param.ident;

        // Check if this type parameter has required bounds either:
        // 1. Directly on the parameter declaration (T: Bound)
        // 2. In a where clause (where T: Bound)

        let has_elicitation = has_trait_bound(type_param, &generics.where_clause, "Elicitation");
        let has_jsonschema = has_trait_bound(type_param, &generics.where_clause, "JsonSchema");

        if !has_elicitation {
            return Err(syn::Error::new(
                type_param.ident.span(),
                format!(
                    "Generic type parameter `{}` must have `Elicitation` bound.\n\
                     For MCP compatibility and library self-compatibility, all type parameters must implement `Elicitation + JsonSchema`.\n\
                     Example: `pub fn method<{}: Elicitation + JsonSchema>(...)`",
                    param_name, param_name
                ),
            ));
        }

        if !has_jsonschema {
            return Err(syn::Error::new(
                type_param.ident.span(),
                format!(
                    "Generic type parameter `{}` must have `JsonSchema` bound.\n\
                     For MCP compatibility, all type parameters must implement `JsonSchema` for JSON schema generation.\n\
                     Example: `pub fn method<{}: Elicitation + JsonSchema>(...)`",
                    param_name, param_name
                ),
            ));
        }
    }

    Ok(())
}

/// Checks if a type parameter has a specific trait bound.
///
/// Checks both inline bounds (T: Bound) and where clause bounds (where T: Bound).
fn has_trait_bound(
    type_param: &TypeParam,
    where_clause: &Option<syn::WhereClause>,
    trait_name: &str,
) -> bool {
    // Check inline bounds
    let has_inline = type_param.bounds.iter().any(|bound| {
        if let TypeParamBound::Trait(trait_bound) = bound {
            is_trait_path(trait_bound, trait_name)
        } else {
            false
        }
    });

    if has_inline {
        return true;
    }

    // Check where clause
    if let Some(where_clause) = where_clause {
        let param_ident = &type_param.ident;

        for predicate in &where_clause.predicates {
            if let WherePredicate::Type(PredicateType { bounded_ty, bounds, .. }) = predicate {
                // Check if this predicate is for our type parameter
                if let Type::Path(type_path) = bounded_ty {
                    if let Some(segment) = type_path.path.segments.last() {
                        if segment.ident == *param_ident {
                            // This is our type parameter - check its bounds
                            let has_bound = bounds.iter().any(|bound| {
                                if let TypeParamBound::Trait(trait_bound) = bound {
                                    is_trait_path(trait_bound, trait_name)
                                } else {
                                    false
                                }
                            });

                            if has_bound {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

/// Checks if a trait bound matches a trait name.
///
/// Handles both simple paths (Trait) and qualified paths (crate::Trait, ::elicitation::Elicitation).
fn is_trait_path(trait_bound: &TraitBound, expected_name: &str) -> bool {
    // Only check non-maybe bounds (T: ?Sized is a maybe bound)
    if matches!(trait_bound.modifier, TraitBoundModifier::Maybe(_)) {
        return false;
    }

    // Get the last segment of the path
    if let Some(segment) = trait_bound.path.segments.last() {
        segment.ident == expected_name
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Signature;

    /// Helper to extract generics from a function signature
    fn parse_generics(sig: &str) -> Generics {
        let sig: Signature = syn::parse_str(sig).unwrap();
        sig.generics
    }

    #[test]
    fn test_validate_bounds_inline() {
        let generics = parse_generics("fn test<T: Elicitation + JsonSchema>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bounds_where_clause() {
        let generics = parse_generics("fn test<T>() where T: Elicitation + JsonSchema");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bounds_mixed() {
        let generics = parse_generics("fn test<T: Elicitation>() where T: JsonSchema");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bounds_with_additional_traits() {
        let generics = parse_generics("fn test<T>() where T: Elicitation + JsonSchema + PartialEq + Clone");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bounds_multiple_params() {
        let generics = parse_generics(
            "fn test<K, V>() where K: Elicitation + JsonSchema + Hash + Eq, V: Elicitation + JsonSchema"
        );

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_bounds_missing_elicitation() {
        let generics = parse_generics("fn test<T: JsonSchema>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Elicitation"));
    }

    #[test]
    fn test_validate_bounds_missing_jsonschema() {
        let generics = parse_generics("fn test<T: Elicitation>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JsonSchema"));
    }

    #[test]
    fn test_validate_bounds_missing_both() {
        let generics = parse_generics("fn test<T>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_bounds_qualified_paths() {
        let generics = parse_generics("fn test<T: ::elicitation::Elicitation + ::schemars::JsonSchema>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_no_type_params() {
        // Lifetimes and const params should be ignored
        let generics = parse_generics("fn test<'a, const N: usize>()");

        let result = validate_generic_bounds(&generics);
        assert!(result.is_ok()); // No type params, so validation passes
    }
}
