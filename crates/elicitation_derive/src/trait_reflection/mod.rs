//! `#[reflect_trait]` macro expansion.
//!
//! Parses the annotated impl block, extracts method signatures, and generates:
//!
//! 1. Param structs (`Deserialize + JsonSchema`) for each method
//! 2. A vtable struct with `Arc<dyn Fn>` fields + `for_type::<T>()` constructor
//! 3. A factory struct implementing `AnyToolFactory`
//! 4. A static vtable map (`LazyLock<RwLock<HashMap<TypeId, Box<VTable>>>>`)
//! 5. A `prime::<T>()` method on the factory for startup registration
//! 6. An `inventory::submit!` for `ToolFactoryRegistration`
//!
//! # Syntax
//!
//! ```rust,ignore
//! #[reflect_trait(diesel::Insertable)]
//! pub impl InsertableTools {
//!     fn insert(&self, conn: &mut PgConnection) -> QueryResult<usize>;
//!     fn batch_insert(items: Vec<Self>, conn: &mut PgConnection) -> QueryResult<usize>;
//! }
//! ```
//!
//! The struct name (`InsertableTools`) is only used for the impl block syntax;
//! the generated types are named after the trait path.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemImpl, ItemTrait, Path, Token, parse::Parse, parse::ParseStream};

pub mod factory;
pub mod naming;
pub mod params;
pub mod type_map;
pub mod vtable;

use naming::factory_struct_name;
use params::MethodInfo;
use type_map::TypeMap;

// ── Attribute argument parsing ────────────────────────────────────────────────

/// Parsed attribute argument: trait path + optional type_map entries.
///
/// Syntax: `#[reflect_trait(diesel::Insertable)]`
/// or:     `#[reflect_trait(clap::CommandFactory, type_map(clap::Command => crate::Command))]`
struct ReflectTraitAttr {
    trait_path: Path,
    type_map: TypeMap,
}

impl Parse for ReflectTraitAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let trait_path: Path = input.parse()?;
        let mut type_map = TypeMap::default();
        // Optionally parse `, type_map(...)`
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            // Expect the keyword `type_map`
            let kw: syn::Ident = input.parse()?;
            if kw != "type_map" {
                return Err(syn::Error::new_spanned(
                    kw,
                    "#[reflect_trait]: expected `type_map(...)` after trait path",
                ));
            }
            type_map = input.parse::<TypeMap>()?;
        }
        Ok(ReflectTraitAttr {
            trait_path,
            type_map,
        })
    }
}

// ── Main expansion ────────────────────────────────────────────────────────────

/// Expand `#[reflect_trait(their::Trait)]` applied to a trait or impl block.
///
/// # Input syntax — prefer `trait`
///
/// The cleanest input is a marker trait whose body lists the method signatures
/// you want to wrap.  Because Rust allows abstract methods inside `trait` blocks,
/// no stub bodies are needed:
///
/// ```rust,ignore
/// #[reflect_trait(diesel::Insertable)]
/// trait InsertableTools {
///     fn insert(&self, conn: &mut PgConnection) -> QueryResult<usize>;
///     fn batch_insert(items: Vec<Self>, conn: &mut PgConnection) -> QueryResult<usize>;
/// }
/// ```
///
/// An `impl` block is also accepted for backward compatibility, but methods
/// inside impl blocks require stub bodies:
///
/// ```rust,ignore
/// #[reflect_trait(diesel::Insertable)]
/// impl InsertableTools {
///     fn insert(&self, conn: &mut PgConnection) -> QueryResult<usize> {}
/// }
/// ```
pub fn expand(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let ReflectTraitAttr {
        trait_path,
        type_map,
    } = syn::parse2::<ReflectTraitAttr>(attr)?;
    let trait_path_str = path_to_string(&trait_path);

    // Try parsing as ItemTrait first (preferred — allows bodyless methods),
    // then fall back to ItemImpl for backward compatibility.
    let methods: Vec<MethodInfo> = if let Ok(trait_item) = syn::parse2::<ItemTrait>(item.clone()) {
        let vis = &trait_item.vis;
        let _ = vis;
        MethodInfo::from_trait_items(&trait_item.items)?
    } else {
        let impl_block = syn::parse2::<ItemImpl>(item)?;
        let vis = &impl_block.self_ty;
        let _ = vis;
        MethodInfo::from_impl_items(&impl_block.items)?
    };

    if methods.is_empty() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[reflect_trait]: trait/impl block must contain at least one method signature",
        ));
    }

    let gen_vis: syn::Visibility = syn::parse_quote!(pub);

    let factory_description = format!("Tools for types implementing `{trait_path_str}`");

    // Generate each piece
    let param_structs: Vec<TokenStream> = methods
        .iter()
        .map(|m| m.param_struct_tokens(&gen_vis, &type_map))
        .collect();

    let vtable_ts =
        vtable::vtable_tokens(&trait_path, &trait_path_str, &methods, &gen_vis, &type_map);

    let factory_ts = factory::factory_tokens(
        &trait_path,
        &trait_path_str,
        &factory_description,
        &methods,
        &gen_vis,
    );

    let factory_name = factory_struct_name(&trait_path_str);

    // We also need to integrate with DynamicToolRegistry::register_type.
    // The user calls register_type::<T>(prefix) — we need prime::<T>() to
    // be called at the same time.  We generate a trait `ReflectTraitPrime`
    // with a blanket impl that calls prime::<T>() for each factory.
    // Actually: we generate a free function `prime_<snake_name>::<T>()` that
    // the user (or a wrapper macro) can call.
    let prime_fn_name = proc_macro2::Ident::new(
        &format!(
            "prime_{}",
            naming::to_snake_path(&trait_path_str).replace("::", "__")
        ),
        proc_macro2::Span::call_site(),
    );

    Ok(quote! {
        // ── Param structs ──────────────────────────────────────────────────
        #(#param_structs)*

        // ── VTable ─────────────────────────────────────────────────────────
        #vtable_ts

        // ── Factory + static map + inventory submission ────────────────────
        #factory_ts

        // ── Convenience prime function ─────────────────────────────────────
        /// Prime the [`#factory_name`] factory for type `T`.
        ///
        /// Call this at server startup alongside `register_type::<T>(prefix)`.
        pub fn #prime_fn_name<T>()
        where
            T: #trait_path
                + ::serde::Serialize
                + ::serde::de::DeserializeOwned
                + ::schemars::JsonSchema
                + ::elicitation::Elicitation
                + Send + Sync + 'static,
        {
            #factory_name::prime::<T>();
        }
    })
}

/// Convert a `syn::Path` to a `"::"` separated string.
///
/// `diesel::Insertable` → `"diesel::Insertable"`
fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
