use proc_macro::{Delimiter, Span, TokenStream as TS1, TokenTree};
use proc_macro2::TokenStream;
use syn::{
    Attribute, Expr, GenericArgument, Generics, Ident, Lifetime, Path, PathArguments, QSelf,
    ReturnType, Type,
};

/// The body of a logical function or a spec.
#[derive(Debug)]
pub(crate) enum LogicBody {
    Some(String),
    /// The function does not have a body. For example, if it is a trait function.
    None,
    /// The function has a body, but it is ignored because the function is `opaque`
    Opaque,
}

/// Generates a piece of documentation corresponding to the spec.
pub(crate) fn document_spec(spec_name: &str, spec_body: LogicBody) -> TokenStream {
    let spec_color = match spec_name {
        "requires" => "Tomato",
        "ensures" => "DodgerBlue",
        "terminates" | "ghost" | "logic" | "law" => "Violet",
        _ if spec_name.starts_with("logic(") => "Violet",
        _ => "Gray",
    };
    let styled_spec_name = format!(
        "<span style=\"color:{spec_color}; white-space:nowrap;\"><samp>{spec_name}</samp></span>"
    );
    let mut spec = match spec_body {
        LogicBody::Some(s) if !s.is_empty() => s,
        _ => {
            let spec = if matches!(spec_body, LogicBody::Opaque) {
                format!(
                    "{styled_spec_name} <span class=\"tooltip\" style=\"color:Red; white-space:nowrap;\" data-title=\"this function is opaque\"><sup>&#9888;</sup></span>"
                )
            } else {
                styled_spec_name
            };
            return quote::quote! {
                #[cfg_attr(not(doctest), doc = "")]
                #[cfg_attr(not(doctest), doc = #spec)]
                #[cfg_attr(not(doctest), doc = "")]
            };
        }
    };

    if spec.len() > 80 - spec_name.len() || spec.contains('\n') {
        spec = spec.replace('\n', "\n> ");
        spec = format!("> ```pearlite\n> {spec}\n> ```");
        quote::quote! {
            #[cfg_attr(not(doctest), doc = "")]
            #[cfg_attr(not(doctest), doc = #styled_spec_name)]
            #[cfg_attr(not(doctest), doc = #spec)]
        }
    } else {
        let spec = format!("```pearlite\n{spec}\n```");
        quote::quote! {
            #[cfg_attr(not(doctest), doc = "<div class=\"container\" style=\"display:flex; align-items:center; gap:5px; clip-path:inset(0.5em 0% 1.1em 0%);\"> <p>")]
            #[cfg_attr(not(doctest), doc = #styled_spec_name)]
            #[cfg_attr(not(doctest), doc = "   </p> <p>")]
            #[cfg_attr(not(doctest), doc = "")]
            #[cfg_attr(not(doctest), doc = #spec)]
            #[cfg_attr(not(doctest), doc = "")]
            #[cfg_attr(not(doctest), doc = "</p> </div>")]
            #[cfg_attr(not(doctest), doc = "")]
        }
    }
}

impl LogicBody {
    // `term_prefix_len`: length of the prefix before the term (`"#[requires("`, `"#[ensures("`)
    // Set it to 0 if you don't know.
    pub(crate) fn term(term_prefix_len: usize, term: TS1) -> LogicBody {
        // We want to find the indentation of terms in the following contexts:
        // ```
        //     #[requires(match x {
        //         p => e,
        //     })]
        // ```
        // and
        // ```
        //     #[requires(
        //         match x {
        //             p => e,
        //         }
        //     )]
        // ```
        // There is no way to access the span of the prefix (`#[requires(` or other) before the term,
        // so we guess based on whether the starting column is less indented than the rest of the term.
        let term = skip_braces(term);
        let span = match stream_span(term) {
            None => return LogicBody::Some("/* Macro-generated */".into()),
            Some(span) if weird_span(span) => {
                return LogicBody::Some("/* Macro-generated */".into());
            }
            Some(span) => span,
        };
        let Some(body) = span.source_text() else {
            return LogicBody::Some("/* Macro-generated */".into());
        };
        let first_indent = span.column() - 1;
        let mut leading_whitespace = usize::MAX;
        for line in body.lines().skip(1) {
            leading_whitespace =
                std::cmp::min(leading_whitespace, line.len() - line.trim_start().len());
        }
        leading_whitespace = if first_indent <= leading_whitespace {
            first_indent
        } else {
            // If the first token is more to the right than the rest of the body,
            // we assume that this is because it is on the same line as the prefix (`#[requires(`...)
            std::cmp::min(first_indent.saturating_sub(term_prefix_len), leading_whitespace)
        };
        let mut res = String::new();
        let mut lines = body.lines();
        if let Some(first_line) = lines.next() {
            res.push_str(first_line);
        }
        for line in lines {
            res.push('\n');
            res.push_str(&line[leading_whitespace..]);
        }
        if res.is_empty() { LogicBody::None } else { LogicBody::Some(res) }
    }
}

/// Heuristic to detect nonsense spans arising from joining tokens originating from different macros.
fn weird_span(s: Span) -> bool {
    // Allow short spans (sometimes they look okay)
    if s.end().line() <= s.line() + 5 {
        return false;
    }
    !local_span(s)
}

/// `true` if the span is contained in the call site
fn local_span(s: Span) -> bool {
    let local = Span::call_site();
    local.local_file() == s.local_file()
        && lt_line_column(local.start(), s.start())
        && lt_line_column(s.end(), local.end())
}

fn lt_line_column(s1: Span, s2: Span) -> bool {
    (s1.line(), s1.column()) <= (s2.line(), s2.column())
}

/// Skip outer braces `{}`, parentheses `()`, and `pearlite! {}`
fn skip_braces(mut stream: TS1) -> TS1 {
    loop {
        let tokens = stream.clone().into_iter().collect::<Vec<_>>();
        if let [TokenTree::Group(group)] = &tokens[..]
            && matches!(group.delimiter(), Delimiter::Parenthesis | Delimiter::Brace)
        {
            // Skip `{}` and `()`
            stream = group.stream()
        } else if let [TokenTree::Ident(ident), TokenTree::Punct(punct), TokenTree::Group(group)] =
            &tokens[..]
            && ident.to_string() == "pearlite"
            && punct.as_char() == '!'
        {
            // Skip `pearlite! {}`
            stream = group.stream()
        } else {
            return stream;
        }
    }
}

fn stream_span(stream: TS1) -> Option<Span> {
    let mut span = None;
    for t in stream {
        match span {
            None => span = Some(t.span()),
            Some(s) => span = s.join(t.span()),
        }
    }
    span
}

pub(crate) fn is_opaque(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        let path = attr.path();

        if path.is_ident("opaque")
            || (path.segments.len() == 3
                && path
                    .segments
                    .iter()
                    .zip(["creusot", "decl", "opaque"])
                    .all(|(s1, s2)| s1.ident == s2))
        {
            return true;
        }
    }
    false
}

/// Create an item name from a type or a trait.
#[derive(Clone, Debug)]
pub(crate) struct DocItemName(pub(crate) String);

impl DocItemName {
    pub(crate) fn add_ident(&mut self, i: &Ident) {
        self.0.push('_');
        self.0.push_str(&i.to_string());
    }

    pub(crate) fn add_type(&mut self, ty: &Type) {
        match ty {
            Type::Array(type_array) => {
                self.0.push_str("__array");
                self.add_type(&type_array.elem);
            }
            Type::BareFn(type_bare_fn) => {
                self.0.push_str("__fn");
                for input in &type_bare_fn.inputs {
                    self.add_type(&input.ty);
                }
                self.add_return_type(&type_bare_fn.output);
            }
            Type::Group(type_group) => self.add_type(&type_group.elem),
            Type::ImplTrait(type_impl_trait) => {
                self.0.push_str("__impl");
                for bound in &type_impl_trait.bounds {
                    self.add_type_param_bound(bound);
                }
            }
            Type::Infer(_) => unreachable!(),
            Type::Macro(_) => self.0.push_str("__macro"),
            Type::Never(_) => {
                self.0.push_str("__never");
            }
            Type::Paren(type_paren) => self.add_type(&type_paren.elem),
            Type::Path(type_path) => {
                self.add_path(&type_path.path);
                self.add_qself(&type_path.qself)
            }
            Type::Ptr(type_ptr) => {
                if type_ptr.mutability.is_some() {
                    self.0.push_str("__ptrmut");
                } else {
                    self.0.push_str("__ptrconst");
                }
                self.add_type(&type_ptr.elem);
            }
            Type::Reference(type_reference) => {
                if type_reference.mutability.is_some() {
                    self.0.push_str("__refmut");
                } else {
                    self.0.push_str("__ref");
                }
                self.add_type(&type_reference.elem);
            }
            Type::Slice(type_slice) => {
                self.0.push_str("__slice");
                self.add_type(&type_slice.elem);
            }
            Type::TraitObject(bounds) => {
                self.0.push_str("__dyn");
                for b in &bounds.bounds {
                    self.add_type_param_bound(b);
                }
            }
            Type::Tuple(type_tuple) => {
                self.0.push_str(&format!("__tuple{}", type_tuple.elems.len()));
                for ty in &type_tuple.elems {
                    self.add_type(ty);
                }
            }
            Type::Verbatim(tokens) => self.0.push_str(&format!("__verbatim{tokens}")),
            // Fill this if new types appear
            _ => {}
        }
    }

    pub(crate) fn add_generics(&mut self, generics: &Generics) {
        for param in &generics.params {
            self.add_generic_param(param);
        }
    }

    pub(crate) fn add_generic_param(&mut self, param: &syn::GenericParam) {
        match param {
            syn::GenericParam::Lifetime(lifetime_param) => {
                self.add_lifetime(&lifetime_param.lifetime);
                if !lifetime_param.bounds.is_empty() {
                    self.0.push_str("__outlives");
                    for lifetime in &lifetime_param.bounds {
                        self.add_lifetime(lifetime);
                    }
                }
            }
            syn::GenericParam::Type(type_param) => {
                self.add_ident(&type_param.ident);
                // if !type_param.bounds.is_empty() {
                //     self.0.push_str("__implements");
                //     for bound in &type_param.bounds {
                //         self.add_type_param_bound(bound);
                //     }
                // }
            }
            syn::GenericParam::Const(const_param) => {
                self.0.push_str("__const");
                self.0.push_str(&const_param.ident.to_string());
            }
        }
    }

    fn add_type_param_bound(&mut self, bound: &syn::TypeParamBound) {
        match bound {
            syn::TypeParamBound::Trait(trait_bound) => self.add_path(&trait_bound.path),
            syn::TypeParamBound::Lifetime(lifetime) => self.add_lifetime(lifetime),
            syn::TypeParamBound::Verbatim(tokens) => {
                self.0.push_str(&format!("__verbatim{tokens}"))
            }
            // Fill this if new types of bounds appear
            _ => {}
        }
    }

    fn add_return_type(&mut self, return_ty: &ReturnType) {
        self.0.push_str("__output");
        match &return_ty {
            ReturnType::Default => self.0.push_str("__unit"),
            ReturnType::Type(_, ty) => self.add_type(ty),
        }
    }

    pub(crate) fn add_path(&mut self, path: &Path) {
        for segment in &path.segments {
            self.add_ident(&segment.ident);
            match &segment.arguments {
                PathArguments::None => {}
                PathArguments::AngleBracketed(generic_args) => {
                    for arg in &generic_args.args {
                        match arg {
                            GenericArgument::Lifetime(lifetime) => self.add_lifetime(lifetime),
                            GenericArgument::Type(ty) => self.add_type(ty),
                            GenericArgument::Const(c) => self.add_expr(c),
                            GenericArgument::AssocType(assoc_ty) => {
                                self.add_ident(&assoc_ty.ident);
                                self.add_type(&assoc_ty.ty);
                            }
                            // If we ever need to disambiguate this, uncomment
                            // those two.
                            // GenericArgument::AssocConst(_) => todo!(),
                            // GenericArgument::Constraint(_) => todo!(),
                            _ => {}
                        }
                    }
                }
                PathArguments::Parenthesized(generic_args) => {
                    self.0.push_str("__lpar");
                    for arg in &generic_args.inputs {
                        self.add_type(arg);
                    }
                    self.0.push_str("__rpar");
                    self.add_return_type(&generic_args.output)
                }
            }
        }
    }

    pub(crate) fn add_qself(&mut self, qself: &Option<QSelf>) {
        if let Some(qself) = qself {
            self.add_type(&qself.ty);
        }
    }

    // If there ever is a need to disambiguate on lifetime parameters, uncomment this
    fn add_lifetime(&mut self, _lifetime: &Lifetime) {
        // self.0.push_str("__lifetime");
        // self.0.push_str(&lifetime.ident.to_string());
    }

    fn add_expr(&mut self, e: &Expr) {
        match e {
            Expr::Path(expr_path) => {
                self.add_qself(&expr_path.qself);
                self.add_path(&expr_path.path);
            }
            // Do nothing in most cases: if a complicated expr appears, we
            // probably don't want to actually see it in the generated name.
            _ => {}
        }
    }
}
