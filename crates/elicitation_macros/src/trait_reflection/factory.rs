//! Factory struct generation for `#[reflect_trait]`.
//!
//! Generates the `AnyToolFactory` impl that:
//! 1. Downcasts the slot to recover the vtable for `T`
//! 2. Builds one `DynamicToolDescriptor` per method
//! 3. Submits itself to inventory via `ToolFactoryRegistration`
//!
//! # VTable map
//!
//! Each generated factory has a private static `RwLock<HashMap<TypeId, Box<VTable>>>`
//! named after the factory (e.g. `INSERTABLE_FACTORY_VTABLES`).  The map is keyed
//! by `TypeId` so the factory can serve any number of registered types.
//!
//! Priming (monomorphization) happens inside `prime::<T>()`, which is called
//! from `DynamicToolRegistry::register_type::<T>()` after the user lists the
//! factory.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Path;

use super::{
    naming::{
        camel_to_snake, factory_struct_name, last_segment_str, param_struct_name,
        vtable_struct_name,
    },
    params::MethodInfo,
};

/// Generate the factory struct, `AnyToolFactory` impl, vtable map, and inventory submission.
pub fn factory_tokens(
    trait_path: &Path,
    trait_path_str: &str,
    factory_description: &str,
    methods: &[MethodInfo],
    vis: &syn::Visibility,
) -> TokenStream {
    let factory_name = factory_struct_name(trait_path_str);
    let vtable_name = vtable_struct_name(trait_path_str);
    let trait_name_str = trait_path_str;
    let method_name_strs: Vec<String> = methods.iter().map(|m| m.name.to_string()).collect();

    // Static map name: INSERTABLE_FACTORY_VTABLES
    let last = last_segment_str(trait_path_str);
    let map_ident = proc_macro2::Ident::new(
        &format!("{}_FACTORY_VTABLES", camel_to_snake(last).to_uppercase()),
        Span::call_site(),
    );

    // DynamicToolDescriptor builders — one per method
    let descriptor_builders: Vec<TokenStream> = methods
        .iter()
        .map(|m| {
            let method_str = m.name.to_string();
            let field = &m.name;
            let param_struct = param_struct_name(&method_str);
            quote! {
                {
                    let tool_name = format!("{prefix}__{method}", method = #method_str);
                    let schema_value = ::serde_json::to_value(
                        ::schemars::schema_for!(#param_struct)
                    ).unwrap_or(::serde_json::Value::Object(Default::default()));
                    let handler = vtable.#field.clone();
                    ::elicitation::DynamicToolDescriptor {
                        name: tool_name,
                        description: format!("Call `{}` on {}", #method_str, slot.type_name()),
                        schema: schema_value,
                        handler,
                    }
                }
            }
        })
        .collect();

    quote! {
        // ── Static vtable map for this factory ──────────────────────────────
        //
        // VTable contains only Arc<dyn Fn + Send + Sync> fields, so
        // Box<VTable>: Send + Sync via automatic trait derivation — no unsafe needed.
        static #map_ident: ::std::sync::LazyLock<
            ::std::sync::RwLock<
                ::std::collections::HashMap<
                    ::std::any::TypeId,
                    ::std::boxed::Box<#vtable_name>,
                >
            >
        > = ::std::sync::LazyLock::new(Default::default);

        // ── Factory struct ───────────────────────────────────────────────────

        #[doc = #factory_description]
        #vis struct #factory_name;

        impl #factory_name {
            /// Prime this factory for concrete type `T`.
            ///
            /// Called by `DynamicToolRegistry::register_type::<T>()`.
            /// Monomorphization of vtable closures happens here — `T` is concrete.
            pub fn prime<T>()
            where
                T: #trait_path
                    + ::serde::Serialize
                    + ::serde::de::DeserializeOwned
                    + ::schemars::JsonSchema
                    + ::elicitation::Elicitation
                    + Send + Sync + 'static,
            {
                let type_id = ::std::any::TypeId::of::<T>();
                let mut map = #map_ident.write().expect("vtable map lock poisoned");
                map.entry(type_id).or_insert_with(|| {
                    Box::new(#vtable_name::for_type::<T>())
                });
            }
        }

        impl ::elicitation::AnyToolFactory for #factory_name {
            fn trait_name(&self) -> &'static str {
                #trait_name_str
            }

            fn factory_description(&self) -> &'static str {
                #factory_description
            }

            fn method_names(&self) -> &'static [&'static str] {
                &[#(#method_name_strs,)*]
            }

            fn instantiate(
                &self,
                slot: &dyn ::elicitation::AnyToolSlot,
            ) -> ::std::result::Result<
                ::std::vec::Vec<::elicitation::DynamicToolDescriptor>,
                ::rmcp::ErrorData,
            > {
                let type_id = slot.slot_type_id();
                let map = #map_ident.read().expect("vtable map lock poisoned");
                let vtable = map.get(&type_id).ok_or_else(|| {
                    ::rmcp::ErrorData::invalid_params(
                        format!(
                            "`{}` has not been primed for type `{}`. \
                             Call register_type::<T>(prefix) at startup.",
                            #trait_name_str,
                            slot.type_name(),
                        ),
                        None,
                    )
                })?;
                let prefix = slot.prefix().to_string();
                let descriptors = vec![#(#descriptor_builders,)*];
                Ok(descriptors)
            }
        }

        // ── Inventory submission ─────────────────────────────────────────────

        ::inventory::submit!(::elicitation::ToolFactoryRegistration {
            trait_name: #trait_name_str,
            factory: &#factory_name,
        });
    }
}
