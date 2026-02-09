//! Generator implementation code generation.
//!
//! Maps parsed contracts to appropriate generator implementations.

use crate::contract_parser::{parse_contract, Contract};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Lit, Result};

/// Expand the derive macro.
pub fn expand_derive_rand(input: &DeriveInput) -> Result<TokenStream> {
    // Parse contract attribute
    let contract = parse_contract(&input.attrs)?;
    
    match &input.data {
        Data::Struct(data_struct) => {
            generate_struct_impl(input, data_struct, contract)
        }
        Data::Enum(data_enum) => {
            generate_enum_impl(input, data_enum, contract)
        }
        Data::Union(_) => {
            Err(syn::Error::new_spanned(
                input,
                "Union types are not supported",
            ))
        }
    }
}

/// Generate implementation for struct.
fn generate_struct_impl(
    input: &DeriveInput,
    data_struct: &syn::DataStruct,
    contract: Option<Contract>,
) -> Result<TokenStream> {
    let name = &input.ident;
    
    match &data_struct.fields {
        // Newtype: struct D6(u32)
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let inner_type = &fields.unnamed[0].ty;
            generate_newtype_impl(name, inner_type, contract)
        }
        // Unit struct: struct Marker;
        Fields::Unit => {
            generate_unit_struct_impl(name)
        }
        // Named fields: struct Config { port: u16, timeout: u32 }
        Fields::Named(fields) => {
            generate_named_struct_impl(name, fields, contract)
        }
        _ => Err(syn::Error::new_spanned(
            input,
            "Only newtype, unit, and named field structs are supported",
        )),
    }
}

/// Generate implementation for newtype struct.
fn generate_newtype_impl(
    name: &syn::Ident,
    inner_type: &syn::Type,
    contract: Option<Contract>,
) -> Result<TokenStream> {
    let generator_expr = match contract {
        Some(Contract::Bounded { low, high }) => {
            generate_bounded_impl(inner_type, &low, &high)?
        }
        Some(Contract::Positive) => {
            generate_positive_impl(inner_type)?
        }
        Some(Contract::NonZero) => {
            generate_nonzero_impl(inner_type)?
        }
        Some(Contract::Even) => {
            generate_even_impl(inner_type)?
        }
        Some(Contract::Odd) => {
            generate_odd_impl(inner_type)?
        }
        Some(Contract::And(left, right)) => {
            generate_and_impl(inner_type, &left, &right)?
        }
        Some(Contract::Or(left, right)) => {
            generate_or_impl(inner_type, &left, &right)?
        }
        None => {
            // No contract - use standard RandomGenerator
            quote! {
                ::elicitation_rand::generators::RandomGenerator::<#inner_type>::with_seed(seed)
            }
        }
    };
    
    Ok(quote! {
        impl #name {
            /// Create a random generator for this type with the given seed.
            ///
            /// The generator respects the type's contract constraints.
            pub fn random_generator(seed: u64) -> impl ::elicitation_rand::Generator<Target = Self> {
                ::elicitation_rand::generators::MapGenerator::new(
                    #generator_expr,
                    |value| #name(value)
                )
            }
        }
        
        // Note: Rand trait implementation would require concrete associated type
        // Users can call random_generator() directly for now
    })
}

/// Generate bounded implementation.
fn generate_bounded_impl(
    inner_type: &syn::Type,
    low: &Lit,
    high: &Lit,
) -> Result<TokenStream> {
    Ok(quote! {
        ::elicitation_rand::distributions::UniformGenerator::<#inner_type>::with_seed(
            seed,
            #low,
            #high
        )
    })
}

/// Generate positive implementation.
fn generate_positive_impl(inner_type: &syn::Type) -> Result<TokenStream> {
    // For now, use bounded starting at 1
    // TODO: Handle signed vs unsigned, float types
    Ok(quote! {
        ::elicitation_rand::distributions::UniformGenerator::<#inner_type>::with_seed(
            seed,
            1,
            #inner_type::MAX
        )
    })
}

/// Generate non-zero implementation.
fn generate_nonzero_impl(inner_type: &syn::Type) -> Result<TokenStream> {
    // Similar to positive - start at 1
    Ok(quote! {
        ::elicitation_rand::distributions::UniformGenerator::<#inner_type>::with_seed(
            seed,
            1,
            #inner_type::MAX
        )
    })
}

/// Generate even implementation.
fn generate_even_impl(inner_type: &syn::Type) -> Result<TokenStream> {
    // Generate value, then transform to even
    Ok(quote! {
        ::elicitation_rand::generators::TransformGenerator::new(
            ::elicitation_rand::generators::RandomGenerator::<#inner_type>::with_seed(seed),
            |value: #inner_type| if value % 2 == 0 { value } else { value.wrapping_add(1) }
        )
    })
}

/// Generate odd implementation.
fn generate_odd_impl(inner_type: &syn::Type) -> Result<TokenStream> {
    // Generate value, then transform to odd
    Ok(quote! {
        ::elicitation_rand::generators::TransformGenerator::new(
            ::elicitation_rand::generators::RandomGenerator::<#inner_type>::with_seed(seed),
            |value: #inner_type| if value % 2 != 0 { value } else { value.wrapping_add(1) }
        )
    })
}

/// Generate And implementation - apply both constraints sequentially.
fn generate_and_impl(
    inner_type: &syn::Type,
    left: &Contract,
    right: &Contract,
) -> Result<TokenStream> {
    // Special case: bounded + even/odd
    if let (Contract::Bounded { low, high }, Contract::Even) = (left, right) {
        // Adjust bounds to only generate even values
        return Ok(quote! {
            ::elicitation_rand::distributions::BoundedEvenGenerator::new(seed, #low, #high)
        });
    }
    if let (Contract::Bounded { low, high }, Contract::Odd) = (left, right) {
        // Adjust bounds to only generate odd values
        return Ok(quote! {
            ::elicitation_rand::distributions::BoundedOddGenerator::new(seed, #low, #high)
        });
    }
    
    // General case: generate left first, then apply right transformation
    let left_gen = generate_contract_impl(inner_type, left)?;
    let right_transform = generate_contract_transform(inner_type, right)?;
    
    Ok(quote! {
        ::elicitation_rand::generators::TransformGenerator::new(
            #left_gen,
            #right_transform
        )
    })
}

/// Generate Or implementation - randomly choose one constraint path.
fn generate_or_impl(
    inner_type: &syn::Type,
    left: &Contract,
    right: &Contract,
) -> Result<TokenStream> {
    let left_gen = generate_contract_impl(inner_type, left)?;
    let right_gen = generate_contract_impl(inner_type, right)?;
    
    // Use a coin flip to decide which constraint to use
    Ok(quote! {
        ::elicitation_rand::generators::ChoiceGenerator::new(
            seed,
            #left_gen,
            #right_gen
        )
    })
}

/// Generate implementation for a contract (used in composition).
fn generate_contract_impl(inner_type: &syn::Type, contract: &Contract) -> Result<TokenStream> {
    match contract {
        Contract::Bounded { low, high } => generate_bounded_impl(inner_type, low, high),
        Contract::Positive => generate_positive_impl(inner_type),
        Contract::NonZero => generate_nonzero_impl(inner_type),
        Contract::Even => generate_even_impl(inner_type),
        Contract::Odd => generate_odd_impl(inner_type),
        Contract::And(left, right) => generate_and_impl(inner_type, left, right),
        Contract::Or(left, right) => generate_or_impl(inner_type, left, right),
    }
}

/// Generate a transformation function for a contract (used in And composition).
fn generate_contract_transform(inner_type: &syn::Type, contract: &Contract) -> Result<TokenStream> {
    match contract {
        Contract::Even => {
            Ok(quote! {
                |value: #inner_type| if value % 2 == 0 { value } else { value.saturating_sub(1) }
            })
        }
        Contract::Odd => {
            Ok(quote! {
                |value: #inner_type| if value % 2 != 0 { value } else { value.saturating_sub(1) }
            })
        }
        Contract::Positive => {
            // For signed types, take absolute value and ensure > 0
            Ok(quote! {
                |value: #inner_type| value.abs().max(1)
            })
        }
        _ => {
            // For complex contracts, just regenerate
            // This is less efficient but simpler
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "Complex contract not supported in And composition yet",
            ))
        }
    }
}

/// Generate implementation for struct with named fields.
fn generate_named_struct_impl(
    name: &syn::Ident,
    fields: &syn::FieldsNamed,
    _struct_contract: Option<Contract>,
) -> Result<TokenStream> {
    // Generate constructor for each field
    let field_inits: Vec<TokenStream> = fields
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            
            // Parse contract from field attributes
            let field_contract = parse_contract(&field.attrs)?;
            
            // Generate the generator expression for this field
            let generator_expr = match field_contract {
                Some(Contract::Bounded { low, high }) => {
                    generate_bounded_impl(field_type, &low, &high)?
                }
                Some(Contract::Positive) => {
                    generate_positive_impl(field_type)?
                }
                Some(Contract::NonZero) => {
                    generate_nonzero_impl(field_type)?
                }
                Some(Contract::Even) => {
                    generate_even_impl(field_type)?
                }
                Some(Contract::Odd) => {
                    generate_odd_impl(field_type)?
                }
                Some(Contract::And(left, right)) => {
                    generate_and_impl(field_type, &left, &right)?
                }
                Some(Contract::Or(left, right)) => {
                    generate_or_impl(field_type, &left, &right)?
                }
                None => {
                    // No contract - delegate to Rand implementation
                    quote! {
                        <#field_type as ::elicitation_rand::Rand>::rand_generator(seed)
                    }
                }
            };
            
            Ok(quote! {
                #field_name: {
                    let generator = #generator_expr;
                    generator.generate()
                }
            })
        })
        .collect::<Result<Vec<_>>>()?;
    
    Ok(quote! {
        impl #name {
            /// Create a random generator with the given seed.
            pub fn random_generator(seed: u64) -> impl ::elicitation::Generator<Target = Self> {
                ::elicitation_rand::generators::MapGenerator::new(
                    ::elicitation_rand::generators::RandomGenerator::<u64>::with_seed(seed),
                    move |seed: u64| {
                        Self {
                            #(#field_inits),*
                        }
                    }
                )
            }
        }

        impl ::elicitation::Generator for #name {
            type Target = Self;

            fn generate(&self) -> Self {
                // Use a fixed seed for deterministic generation
                Self::random_generator(42).generate()
            }
        }
    })
}

/// Generate implementation for enum.
fn generate_enum_impl(
    input: &DeriveInput,
    data_enum: &syn::DataEnum,
    _enum_contract: Option<Contract>,
) -> Result<TokenStream> {
    let name = &input.ident;
    let variant_count = data_enum.variants.len();
    
    if variant_count == 0 {
        return Err(syn::Error::new_spanned(input, "Cannot derive Rand for empty enum"));
    }
    
    // Generate match arms for each variant
    let match_arms: Vec<TokenStream> = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(idx, variant)| {
            let variant_name = &variant.ident;
            
            match &variant.fields {
                // Unit variant: Status::Active
                Fields::Unit => {
                    Ok(quote! {
                        #idx => #name::#variant_name,
                    })
                }
                // Tuple variant: Result::Success(u32)
                Fields::Unnamed(fields) => {
                    let field_gens: Vec<TokenStream> = fields
                        .unnamed
                        .iter()
                        .map(|field| {
                            let field_type = &field.ty;
                            let field_contract = parse_contract(&field.attrs)?;
                            
                            let generator_expr = generate_field_generator(field_type, field_contract)?;
                            
                            Ok(quote! {
                                {
                                    let generator = #generator_expr;
                                    generator.generate()
                                }
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    
                    Ok(quote! {
                        #idx => #name::#variant_name(#(#field_gens),*),
                    })
                }
                // Struct variant: Event::Click { x: i32, y: i32 }
                Fields::Named(fields) => {
                    let field_inits: Vec<TokenStream> = fields
                        .named
                        .iter()
                        .map(|field| {
                            let field_name = field.ident.as_ref().unwrap();
                            let field_type = &field.ty;
                            let field_contract = parse_contract(&field.attrs)?;
                            
                            let generator_expr = generate_field_generator(field_type, field_contract)?;
                            
                            Ok(quote! {
                                #field_name: {
                                    let generator = #generator_expr;
                                    generator.generate()
                                }
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    
                    Ok(quote! {
                        #idx => #name::#variant_name { #(#field_inits),* },
                    })
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;
    
    Ok(quote! {
        impl #name {
            /// Create a random generator with the given seed.
            pub fn random_generator(seed: u64) -> impl ::elicitation::Generator<Target = Self> {
                ::elicitation_rand::generators::MapGenerator::new(
                    ::elicitation_rand::generators::RandomGenerator::<u64>::with_seed(seed),
                    move |seed: u64| {
                        // Select variant based on seed
                        let variant_gen = ::elicitation_rand::distributions::UniformGenerator::with_seed(seed, 0usize, #variant_count);
                        let variant_idx = variant_gen.generate();
                        
                        match variant_idx {
                            #(#match_arms)*
                            _ => unreachable!("Invalid variant index"),
                        }
                    }
                )
            }
        }

        impl ::elicitation::Generator for #name {
            type Target = Self;

            fn generate(&self) -> Self {
                // Use a fixed seed for deterministic generation
                Self::random_generator(42).generate()
            }
        }
    })
}

/// Generate a generator expression for a field (helper for enum variants).
fn generate_field_generator(
    field_type: &syn::Type,
    field_contract: Option<Contract>,
) -> Result<TokenStream> {
    match field_contract {
        Some(Contract::Bounded { low, high }) => {
            generate_bounded_impl(field_type, &low, &high)
        }
        Some(Contract::Positive) => {
            generate_positive_impl(field_type)
        }
        Some(Contract::NonZero) => {
            generate_nonzero_impl(field_type)
        }
        Some(Contract::Even) => {
            generate_even_impl(field_type)
        }
        Some(Contract::Odd) => {
            generate_odd_impl(field_type)
        }
        Some(Contract::And(left, right)) => {
            generate_and_impl(field_type, &left, &right)
        }
        Some(Contract::Or(left, right)) => {
            generate_or_impl(field_type, &left, &right)
        }
        None => {
            // No contract - delegate to the type's Rand implementation
            Ok(quote! {
                <#field_type as ::elicitation_rand::Rand>::rand_generator(seed)
            })
        }
    }
}

/// Generate implementation for unit struct (zero-sized type).
fn generate_unit_struct_impl(name: &syn::Ident) -> Result<TokenStream> {
    // Unit structs are zero-sized - just return the value
    Ok(quote! {
        impl #name {
            /// Create a random generator (trivial for unit structs).
            pub fn random_generator(_seed: u64) -> impl ::elicitation::Generator<Target = Self> {
                ::elicitation_rand::generators::ConstantGenerator::new(#name)
            }
        }

        impl ::elicitation::Generator for #name {
            type Target = Self;

            fn generate(&self) -> Self {
                #name
            }
        }
    })
}
