//! `BevyComponent` and `BevyResource` derive macros.
//!
//! Generated impls route through `::elicit_bevy::__bevy::ecs::…` so consuming
//! crates only need `elicit_bevy` as a dep — not `bevy` or `bevy_ecs` directly.
//!
//! In bevy 0.19, `Resource: Component`, so `#[derive(BevyResource)]` emits both
//! the `Component` impl (with `STORAGE_TYPE` and `type Mutability`) and the
//! `Resource` impl.  `#[derive(BevyComponent)]` emits only the `Component` impl.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub(crate) fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::elicit_bevy::__bevy::ecs::component::Component
            for #name #ty_generics #where_clause
        {
            const STORAGE_TYPE: ::elicit_bevy::__bevy::ecs::component::StorageType =
                ::elicit_bevy::__bevy::ecs::component::StorageType::Table;
            type Mutability = ::elicit_bevy::__bevy::ecs::component::Mutable;
        }
    }
    .into()
}

pub(crate) fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics ::elicit_bevy::__bevy::ecs::component::Component
            for #name #ty_generics #where_clause
        {
            const STORAGE_TYPE: ::elicit_bevy::__bevy::ecs::component::StorageType =
                ::elicit_bevy::__bevy::ecs::component::StorageType::Table;
            type Mutability = ::elicit_bevy::__bevy::ecs::component::Mutable;
        }
        impl #impl_generics ::elicit_bevy::__bevy::ecs::resource::Resource
            for #name #ty_generics #where_clause {}
    }
    .into()
}
