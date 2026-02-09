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

    /// and(A, B) - Both constraints must hold
    And(Box<Contract>, Box<Contract>),

    /// or(A, B) - Either constraint holds
    Or(Box<Contract>, Box<Contract>),
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
                        let func_name = path.path.get_ident().ok_or_else(|| {
                            syn::Error::new_spanned(func.as_ref(), "Expected function name")
                        })?;

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
                            "and" => {
                                if args.len() != 2 {
                                    return Err(syn::Error::new_spanned(
                                        &args,
                                        "and requires exactly 2 arguments: and(contract1, contract2)",
                                    ));
                                }

                                let left = parse_contract_expr(&args[0])?;
                                let right = parse_contract_expr(&args[1])?;

                                Ok(Some(Contract::And(Box::new(left), Box::new(right))))
                            }
                            "or" => {
                                if args.len() != 2 {
                                    return Err(syn::Error::new_spanned(
                                        &args,
                                        "or requires exactly 2 arguments: or(contract1, contract2)",
                                    ));
                                }

                                let left = parse_contract_expr(&args[0])?;
                                let right = parse_contract_expr(&args[1])?;

                                Ok(Some(Contract::Or(Box::new(left), Box::new(right))))
                            }
                            _ => Err(syn::Error::new_spanned(
                                func.as_ref(),
                                "Unknown rand function",
                            )),
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
                _ => Err(syn::Error::new_spanned(
                    expr,
                    "Expected function call or identifier",
                )),
            }
        }
        _ => Ok(None),
    }
}

/// Extract literal from expression.
fn extract_literal(expr: &Expr) -> Result<Lit> {
    match expr {
        Expr::Lit(lit_expr) => Ok(lit_expr.lit.clone()),
        Expr::Unary(unary) if matches!(unary.op, syn::UnOp::Neg(_)) => {
            // Handle negative literals like -100
            match &*unary.expr {
                Expr::Lit(lit_expr) => {
                    match &lit_expr.lit {
                        Lit::Int(lit_int) => {
                            // Create a negative version
                            let value = lit_int
                                .base10_parse::<i64>()
                                .map_err(|e| syn::Error::new_spanned(lit_int, e))?;
                            let neg_value = -value;
                            let neg_lit =
                                Lit::Int(syn::LitInt::new(&neg_value.to_string(), lit_int.span()));
                            Ok(neg_lit)
                        }
                        _ => Err(syn::Error::new_spanned(expr, "Expected integer literal")),
                    }
                }
                _ => Err(syn::Error::new_spanned(
                    expr,
                    "Expected literal after negation",
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(expr, "Expected literal value")),
    }
}

/// Parse contract from expression (for nested contracts).
fn parse_contract_expr(expr: &Expr) -> Result<Contract> {
    match expr {
        // Function call: bounded(1, 100), and(positive, even)
        Expr::Call(ExprCall { func, args, .. }) => {
            if let Expr::Path(path) = func.as_ref() {
                let func_name = path.path.get_ident().ok_or_else(|| {
                    syn::Error::new_spanned(func.as_ref(), "Expected function name")
                })?;

                match func_name.to_string().as_str() {
                    "bounded" => {
                        if args.len() != 2 {
                            return Err(syn::Error::new_spanned(
                                args,
                                "bounded requires exactly 2 arguments",
                            ));
                        }
                        let low = extract_literal(&args[0])?;
                        let high = extract_literal(&args[1])?;
                        Ok(Contract::Bounded { low, high })
                    }
                    "and" => {
                        if args.len() != 2 {
                            return Err(syn::Error::new_spanned(
                                args,
                                "and requires exactly 2 arguments",
                            ));
                        }
                        let left = parse_contract_expr(&args[0])?;
                        let right = parse_contract_expr(&args[1])?;
                        Ok(Contract::And(Box::new(left), Box::new(right)))
                    }
                    "or" => {
                        if args.len() != 2 {
                            return Err(syn::Error::new_spanned(
                                args,
                                "or requires exactly 2 arguments",
                            ));
                        }
                        let left = parse_contract_expr(&args[0])?;
                        let right = parse_contract_expr(&args[1])?;
                        Ok(Contract::Or(Box::new(left), Box::new(right)))
                    }
                    _ => Err(syn::Error::new_spanned(
                        func.as_ref(),
                        "Unknown contract function",
                    )),
                }
            } else {
                Err(syn::Error::new_spanned(func.as_ref(), "Expected path"))
            }
        }
        // Simple identifier: positive, nonzero, even, odd
        Expr::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                match ident.to_string().as_str() {
                    "positive" => Ok(Contract::Positive),
                    "nonzero" => Ok(Contract::NonZero),
                    "even" => Ok(Contract::Even),
                    "odd" => Ok(Contract::Odd),
                    _ => Err(syn::Error::new_spanned(ident, "Unknown contract")),
                }
            } else {
                Err(syn::Error::new_spanned(path, "Expected identifier"))
            }
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Expected function call or identifier",
        )),
    }
}
