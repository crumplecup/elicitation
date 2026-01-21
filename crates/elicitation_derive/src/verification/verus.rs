//! Verus verifier backend.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{Field, Ident, Type, TypePath};

/// Generate Verus verification for a struct.
pub fn generate_verus_verification(
    struct_name: &Ident,
    fields: &[&Field],
) -> TokenStream {
    let constructor = generate_constructor(struct_name, fields);
    let harness = generate_harness(struct_name, fields);
    
    quote! {
        #[cfg(feature = "verify-verus")]
        #constructor
        
        #[cfg(feature = "verify-verus")]
        #harness
    }
}

/// Generate Verus verification for an enum.
pub fn generate_verus_enum_verification(
    enum_name: &Ident,
    variants: &[&syn::Variant],
) -> TokenStream {
    // Generate one verification function per variant
    let harnesses: Vec<_> = variants
        .iter()
        .map(|variant| generate_variant_harness(enum_name, variant))
        .collect();
    
    quote! {
        #[cfg(feature = "verify-verus")]
        const _: () = {
            verus! {
                #(#harnesses)*
            }
        };
    }
}

/// Extract the type path from a field type (best effort).
fn extract_type_path(ty: &Type) -> Option<&TypePath> {
    match ty {
        Type::Path(type_path) => Some(type_path),
        _ => None,
    }
}

fn generate_constructor(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor_name = format_ident!("__make_{}", struct_name);
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    
    // Verus uses exec fn with requires/ensures inside verus! macro
    quote! {
        verus! {
            exec fn #constructor_name(#(#field_names: #field_types),*) -> #struct_name
                requires true
                ensures true
            {
                #struct_name { #(#field_names),* }
            }
        }
    }
}

fn generate_harness(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let harness_name = format_ident!("__verify_{}", struct_name);
    let constructor_name = format_ident!("__make_{}", struct_name);
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    
    // Verus proof functions use proof fn inside verus! macro
    let field_inits: Vec<_> = field_names.iter().zip(&field_types).map(|(name, ty)| {
        quote! {
            let #name: #ty;
        }
    }).collect();
    
    quote! {
        #[cfg(verus)]
        verus! {
            proof fn #harness_name() {
                #(#field_inits)*
                let _result = #constructor_name(#(#field_names),*);
            }
        }
    }
}

/// Generate a proof function for a single enum variant.
fn generate_variant_harness(enum_name: &Ident, variant: &syn::Variant) -> TokenStream {
    let variant_name = &variant.ident;
    let harness_name = format_ident!("__verify_{}_{}", enum_name, variant_name);
    
    match &variant.fields {
        syn::Fields::Unit => {
            quote! {
                #[cfg(verus)]
                proof fn #harness_name() {
                    let _value = #enum_name::#variant_name;
                }
            }
        }
        
        syn::Fields::Unnamed(fields) => {
            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
            let field_names: Vec<_> = (0..field_types.len())
                .map(|i| format_ident!("field_{}", i))
                .collect();
            
            let field_inits: Vec<_> = field_names.iter().zip(&field_types).map(|(name, ty)| {
                quote! { let #name: #ty; }
            }).collect();
            
            quote! {
                #[cfg(verus)]
                proof fn #harness_name() {
                    #(#field_inits)*
                    let _value = #enum_name::#variant_name(#(#field_names),*);
                }
            }
        }
        
        syn::Fields::Named(fields) => {
            let field_names: Vec<_> = fields.named.iter()
                .filter_map(|f| f.ident.as_ref())
                .collect();
            let field_types: Vec<_> = fields.named.iter()
                .map(|f| &f.ty)
                .collect();
            
            let field_inits: Vec<_> = field_names.iter().zip(&field_types).map(|(name, ty)| {
                quote! { let #name: #ty; }
            }).collect();
            
            quote! {
                #[cfg(verus)]
                proof fn #harness_name() {
                    #(#field_inits)*
                    let _value = #enum_name::#variant_name { #(#field_names),* };
                }
            }
        }
    }
}
