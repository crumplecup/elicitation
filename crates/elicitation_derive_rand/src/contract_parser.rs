//! Contract attribute parsing.
//!
//! Extracts and parses `#[rand(...)]` attributes to determine
//! appropriate sampling strategies.
//!
//! Syntax: `#[rand(bounded(1, 100))]`, `#[rand(positive)]`, etc.

use syn::{Attribute, Expr, ExprCall, Lit, Meta, Result};

/// Parsed contract information.
#[derive(Debug, Clone)]
pub enum Contract {
    /// bounded(L, H) - Uniform distribution in range
    Bounded { low: Lit, high: Lit },
    
    /// positive - Positive values only
    Positive,
    
    /// nonzero - Non-zero values
    NonZero,
    
    /// even - Even values only
    Even,
    
    /// odd - Odd values only
    Odd,
}

/// Parse contract from attributes.
pub fn parse_contract(attrs: &[Attribute]) -> Result<Option<Contract>> {
    for attr in attrs {
        if attr.path().is_ident("rand") {
            return parse_rand_meta(&attr.meta);
        }
    }
    Ok(None)
}

/// Parse rand attribute meta.
fn parse_rand_meta(meta: &Meta) -> Result<Option<Contract>> {
    match meta {
        Meta::List(list) => {
            // #[rand(bounded(1, 100))]
            let tokens = &list.tokens;
            let expr: Expr = syn::parse2(tokens.clone())?;
            
            match expr {
                // Function call: bounded(1, 100)
                Expr::Call(ExprCall { func, args, .. }) => {
                    if let Expr::Path(path) = func.as_ref() {
                        let func_name = path.path.get_ident()
                            .ok_or_else(|| syn::Error::new_spanned(func.as_ref(), "Expected function name"))?;
                        
                        match func_name.to_string().as_str() {
                            "bounded" => {
                                if args.len() != 2 {
                                    return Err(syn::Error::new_spanned(
                                        &args,
                                        "bounded requires exactly 2 arguments: bounded(low, high)",
                                    ));
                                }
                                
                                let low = extract_literal(&args[0])?;
                                let high = extract_literal(&args[1])?;
                                
                                Ok(Some(Contract::Bounded { low, high }))
                            }
                            _ => Err(syn::Error::new_spanned(func.as_ref(), "Unknown rand function")),
                        }
                    } else {
                        Err(syn::Error::new_spanned(func.as_ref(), "Expected path"))
                    }
                }
                // Simple identifier: positive, nonzero, even, odd
                Expr::Path(path) => {
                    if let Some(ident) = path.path.get_ident() {
                        match ident.to_string().as_str() {
                            "positive" => Ok(Some(Contract::Positive)),
                            "nonzero" => Ok(Some(Contract::NonZero)),
                            "even" => Ok(Some(Contract::Even)),
                            "odd" => Ok(Some(Contract::Odd)),
                            _ => Err(syn::Error::new_spanned(ident, "Unknown contract")),
                        }
                    } else {
                        Err(syn::Error::new_spanned(path, "Expected identifier"))
                    }
                }
                _ => Err(syn::Error::new_spanned(expr, "Expected function call or identifier")),
            }
        }
        _ => Ok(None),
    }
}

/// Extract literal from expression.
fn extract_literal(expr: &Expr) -> Result<Lit> {
    match expr {
        Expr::Lit(lit_expr) => Ok(lit_expr.lit.clone()),
        _ => Err(syn::Error::new_spanned(expr, "Expected literal value")),
    }
}
