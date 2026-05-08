//! VSM source scanner using `syn`.
//!
//! Walks a directory tree of Rust source files, locating
//! `#[derive(VerifiedStateMachine)]` structs and their companion
//! `#[derive(Prop)]` structs.  Returns `Vec<VsmDescriptor>` describing
//! every state machine found.
//!
//! No crate compilation is required — the scanner reads source text only.

use std::path::{Path, PathBuf};
use syn::{Attribute, Expr, ExprLit, File, FnArg, Item, Lit, Meta, MetaList, MetaNameValue};
use walkdir::WalkDir;

// ─── Public types ────────────────────────────────────────────────────────────

/// Invariant companion metadata extracted from `#[derive(Prop)] #[prop(...)]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropDescriptor {
    /// Struct name, e.g. `ArchiveConnectionConsistent`.
    pub name: String,
    /// `kani_invariant_fn = "..."` value.
    pub kani_fn: Option<String>,
    /// `verus_invariant_fn = "..."` value.
    pub verus_fn: Option<String>,
    /// `creusot_invariant_fn = "..."` value.
    pub creusot_fn: Option<String>,
    /// `verus_inv_body = "..."` value — verbatim body for the Verus `open spec fn`.
    pub verus_inv_body: Option<String>,
}

/// Classification of a single transition function parameter for harness generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgKind {
    /// The VSM state enum — first non-special parameter.
    State,
    /// `Established<T>` — proof credential; `inner` is the type name of `T`.
    Proof { inner: String },
    /// `String` — initialized to `String::new()` in harnesses.
    StringArg,
    /// `Vec<_>` — initialized to `Vec::new()` in harnesses.
    VecArg,
    /// `Option<_>` — initialized to `None` in harnesses.
    OptionArg,
    /// Any other payload type — uses `<T as KaniCompose>::kani_depth0()`.
    Other,
}

/// A single typed parameter extracted from a transition function signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgDescriptor {
    /// Parameter name (or `_name`), e.g. `_state`, `profile_name`.
    pub name: String,
    /// Type as a source-faithful string, e.g. `ArchiveNavState`, `String`.
    pub ty: String,
    /// Classified role of this argument.
    pub kind: ArgKind,
}

/// A transition function's name and classified parameter list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionFn {
    /// Free function name, e.g. `load_nav`.
    pub name: String,
    /// All parameters of the function, including state and proof.
    pub args: Vec<ArgDescriptor>,
}

impl TransitionFn {
    /// Returns the state parameter (first `ArgKind::State` arg), if present.
    pub fn state_arg(&self) -> Option<&ArgDescriptor> {
        self.args.iter().find(|a| a.kind == ArgKind::State)
    }

    /// Returns the proof parameter (first `ArgKind::Proof` arg), if present.
    pub fn proof_arg(&self) -> Option<&ArgDescriptor> {
        self.args
            .iter()
            .find(|a| matches!(a.kind, ArgKind::Proof { .. }))
    }

    /// Returns only the extra (non-state, non-proof) parameters.
    pub fn extra_args(&self) -> impl Iterator<Item = &ArgDescriptor> {
        self.args
            .iter()
            .filter(|a| !matches!(a.kind, ArgKind::State | ArgKind::Proof { .. }))
    }
}

/// Describes a single `#[derive(VerifiedStateMachine)]` struct and its
/// associated companion types, extracted purely from source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VsmDescriptor {
    /// Machine struct name, e.g. `ArchiveConnectionMachine`.
    pub machine: String,
    /// Transition function names from `#[vsm(transitions = [...])]`.
    pub transitions: Vec<String>,
    /// Full transition function signatures found in the same file, if available.
    pub transition_fns: Vec<TransitionFn>,
    /// Invariant companion, if found in the same or adjacent source file.
    pub invariant: Option<PropDescriptor>,
    /// Source file the machine was found in.
    pub source_file: PathBuf,
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Scan `root` recursively for Rust source files containing VSMs.
///
/// Returns one `VsmDescriptor` per `#[derive(VerifiedStateMachine)]` struct
/// found.  Files that cannot be parsed are silently skipped.
#[tracing::instrument(skip(root), fields(root = %root.as_ref().display()))]
pub fn scan_vsms(root: impl AsRef<Path>) -> Vec<VsmDescriptor> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs")
        })
    {
        let path = entry.path();
        tracing::debug!(file = %path.display(), "scanning");

        let src = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(err) => {
                tracing::warn!(file = %path.display(), error = %err, "read failed");
                continue;
            }
        };

        let syntax: File = match syn::parse_str(&src) {
            Ok(f) => f,
            Err(err) => {
                tracing::debug!(file = %path.display(), error = %err, "parse failed, skipping");
                continue;
            }
        };

        let descs = extract_vsms_from_file(&syntax, path);
        tracing::debug!(file = %path.display(), found = descs.len(), "machines found");
        results.extend(descs);
    }

    tracing::info!(total = results.len(), "scan complete");
    results
}

// ─── Internal helpers ────────────────────────────────────────────────────────

/// Extract all `VsmDescriptor`s from a single parsed `syn::File`.
pub fn extract_vsms_from_file(file: &File, source_path: &Path) -> Vec<VsmDescriptor> {
    // First pass: collect all Prop descriptors (companion structs) in this file.
    let props: Vec<PropDescriptor> = file
        .items
        .iter()
        .filter_map(|item| {
            if let Item::Struct(s) = item {
                extract_prop_descriptor(s)
            } else {
                None
            }
        })
        .collect();

    // Second pass: collect all VSM machine structs.
    file.items
        .iter()
        .filter_map(|item| {
            if let Item::Struct(s) = item {
                extract_vsm_descriptor(s, &props, source_path, file)
            } else {
                None
            }
        })
        .collect()
}

/// If `s` derives `VerifiedStateMachine` and has `#[vsm(transitions = [...])]`,
/// return a `VsmDescriptor`.
fn extract_vsm_descriptor(
    s: &syn::ItemStruct,
    props: &[PropDescriptor],
    source_path: &Path,
    file: &File,
) -> Option<VsmDescriptor> {
    if !has_derive(s, "VerifiedStateMachine") {
        return None;
    }

    let machine = s.ident.to_string();
    let transitions = extract_vsm_transitions(s)?;

    // Convention: `XxxMachine` companion invariant is named `XxxConsistent`.
    let consistent_name = machine.replace("Machine", "Consistent");
    let invariant = props.iter().find(|p| p.name == consistent_name).cloned();

    // Scan the same file for the transition function signatures.
    let transition_fns = scan_transition_fns(file, &transitions);

    tracing::debug!(
        machine = %machine,
        transitions = transitions.len(),
        transition_fns = transition_fns.len(),
        has_invariant = invariant.is_some(),
        "VSM found"
    );

    Some(VsmDescriptor {
        machine,
        transitions,
        transition_fns,
        invariant,
        source_file: source_path.to_path_buf(),
    })
}

/// Extract `PropDescriptor` from `#[derive(Prop)] #[prop(...)]` struct.
pub fn extract_prop_descriptor(s: &syn::ItemStruct) -> Option<PropDescriptor> {
    if !has_derive(s, "Prop") {
        return None;
    }

    let name = s.ident.to_string();
    let mut kani_fn = None;
    let mut verus_fn = None;
    let mut creusot_fn = None;
    let mut verus_inv_body = None;

    for attr in &s.attrs {
        if attr.path().is_ident("prop") {
            parse_prop_attr(
                attr,
                &mut kani_fn,
                &mut verus_fn,
                &mut creusot_fn,
                &mut verus_inv_body,
            );
        }
    }

    Some(PropDescriptor {
        name,
        kani_fn,
        verus_fn,
        creusot_fn,
        verus_inv_body,
    })
}

/// Scan `file` for free functions whose names are in `names`, returning
/// `TransitionFn` descriptors with classified argument lists.
fn scan_transition_fns(file: &File, names: &[String]) -> Vec<TransitionFn> {
    let name_set: std::collections::HashSet<&str> = names.iter().map(|s| s.as_str()).collect();
    let mut result: Vec<TransitionFn> = Vec::new();

    for item in &file.items {
        if let Item::Fn(f) = item {
            let fn_name = f.sig.ident.to_string();
            if !name_set.contains(fn_name.as_str()) {
                continue;
            }
            let args = classify_fn_args(&f.sig.inputs);
            result.push(TransitionFn {
                name: fn_name,
                args,
            });
        }
    }

    // Preserve the declaration order from `names`.
    result.sort_by_key(|tf| {
        names
            .iter()
            .position(|n| n == &tf.name)
            .unwrap_or(usize::MAX)
    });
    result
}

/// Classify the inputs of a function signature into `ArgDescriptor`s.
fn classify_fn_args(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
) -> Vec<ArgDescriptor> {
    let mut state_seen = false;
    inputs
        .iter()
        .filter_map(|arg| {
            let pat_type = match arg {
                FnArg::Typed(pt) => pt,
                FnArg::Receiver(_) => return None,
            };
            let name = pat_to_string(&pat_type.pat);
            let ty = ty_to_string(&pat_type.ty);
            let kind = classify_ty(&pat_type.ty, &mut state_seen);
            Some(ArgDescriptor { name, ty, kind })
        })
        .collect()
}

/// Produce a string representation of a `syn::Pat`.
fn pat_to_string(pat: &syn::Pat) -> String {
    match pat {
        syn::Pat::Ident(pi) => pi.ident.to_string(),
        other => quote::quote!(#other).to_string(),
    }
}

/// Produce a source-faithful string representation of a `syn::Type`.
fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string().replace(" ", "")
}

/// Classify a `syn::Type` into `ArgKind`.
fn classify_ty(ty: &syn::Type, state_seen: &mut bool) -> ArgKind {
    let inner = leading_ident(ty);
    match inner.as_deref() {
        Some("Established") => ArgKind::Proof {
            inner: extract_single_generic_arg(ty).unwrap_or_default(),
        },
        Some("String") => ArgKind::StringArg,
        Some("Vec") => ArgKind::VecArg,
        Some("Option") => ArgKind::OptionArg,
        _ => {
            if !*state_seen {
                *state_seen = true;
                ArgKind::State
            } else {
                ArgKind::Other
            }
        }
    }
}

/// Return the outermost type-path ident string for a `syn::Type`, if present.
fn leading_ident(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(tp) = ty {
        tp.path.segments.last().map(|seg| seg.ident.to_string())
    } else {
        None
    }
}

/// Extract the single angle-bracket generic argument from a type like `Established<T>`.
fn extract_single_generic_arg(ty: &syn::Type) -> Option<String> {
    if let syn::Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
            if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                return Some(ty_to_string(inner));
            }
        }
    }
    None
}

/// Parse `#[prop(kani_invariant_fn = "...", verus_invariant_fn = "...", verus_inv_body = "...", ...)]`.
fn parse_prop_attr(
    attr: &Attribute,
    kani_fn: &mut Option<String>,
    verus_fn: &mut Option<String>,
    creusot_fn: &mut Option<String>,
    verus_inv_body: &mut Option<String>,
) {
    let list: MetaList = match attr.meta.clone() {
        Meta::List(l) => l,
        _ => return,
    };

    // Parse as a comma-separated list of `name = "value"` pairs.
    let nested = match list
        .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
    {
        Ok(n) => n,
        Err(_) => return,
    };

    for meta in nested {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            let key = path.get_ident().map(|i| i.to_string()).unwrap_or_default();
            let val = string_lit_value(&value);
            match key.as_str() {
                "kani_invariant_fn" => *kani_fn = val,
                "verus_invariant_fn" => *verus_fn = val,
                "creusot_invariant_fn" => *creusot_fn = val,
                "verus_inv_body" => *verus_inv_body = val,
                _ => {}
            }
        }
    }
}

/// Extract the transition list from `#[vsm(transitions = [a, b, c])]`.
fn extract_vsm_transitions(s: &syn::ItemStruct) -> Option<Vec<String>> {
    for attr in &s.attrs {
        if attr.path().is_ident("vsm") {
            if let Some(transitions) = parse_vsm_transitions_attr(attr) {
                return Some(transitions);
            }
        }
    }
    None
}

/// Parse `#[vsm(transitions = [a, b, c])]` → `vec!["a", "b", "c"]`.
fn parse_vsm_transitions_attr(attr: &Attribute) -> Option<Vec<String>> {
    let list: MetaList = match attr.meta.clone() {
        Meta::List(l) => l,
        _ => return None,
    };

    let nested = list
        .parse_args_with(syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated)
        .ok()?;

    for meta in &nested {
        if let Meta::NameValue(MetaNameValue { path, value, .. }) = meta {
            if path.is_ident("transitions") {
                return extract_ident_array(value);
            }
        }
    }
    None
}

/// Extract `[a, b, c]` from an `Expr::Array` of path expressions.
fn extract_ident_array(expr: &Expr) -> Option<Vec<String>> {
    let arr = match expr {
        Expr::Array(a) => a,
        _ => return None,
    };

    let names = arr
        .elems
        .iter()
        .filter_map(|e| {
            if let Expr::Path(ep) = e {
                ep.path.get_ident().map(|id| id.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if names.is_empty() { None } else { Some(names) }
}

/// Return `true` if `s` has `#[derive(..., Name, ...)]`.
pub fn has_derive(s: &syn::ItemStruct, name: &str) -> bool {
    s.attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }
        // Parse the derive list and check for the target ident.
        matches!(
            attr.meta.clone(),
            Meta::List(list) if list.tokens.to_string().split(',')
                .any(|token| token.trim() == name)
        )
    })
}

/// Extract a string literal value from an `Expr`.
fn string_lit_value(expr: &Expr) -> Option<String> {
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = expr
    {
        Some(s.value())
    } else {
        None
    }
}
