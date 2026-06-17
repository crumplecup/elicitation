//! [`crate::emit_code::ToCodeLiteral`] impls for bare ratatui value types.
//!
//! These impls allow `Arc<ratatui::T>` to satisfy `ToCodeLiteral` (via the
//! blanket `Arc<T: ToCodeLiteral>` impl), which is required when the
//! `elicit_newtype!` wrappers in shadow crates derive `ToCodeLiteral`.
//!
//! Literals reconstruct the value from its serialized form so the emitted
//! code is round-trip correct.

mod impls {
    use crate::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ratatui::style::Style {
        fn to_code_literal(&self) -> TokenStream {
            let json = crate::serde_json::to_string(self)
                .unwrap_or_else(|_| "{}".to_string());
            quote::quote! {
                ::serde_json::from_str::<::ratatui::style::Style>(#json).unwrap()
            }
        }
    }

    impl ToCodeLiteral for ratatui::layout::Constraint {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                ratatui::layout::Constraint::Percentage(v) => {
                    quote::quote! { ::ratatui::layout::Constraint::Percentage(#v) }
                }
                ratatui::layout::Constraint::Ratio(n, d) => {
                    quote::quote! { ::ratatui::layout::Constraint::Ratio(#n, #d) }
                }
                ratatui::layout::Constraint::Length(v) => {
                    quote::quote! { ::ratatui::layout::Constraint::Length(#v) }
                }
                ratatui::layout::Constraint::Max(v) => {
                    quote::quote! { ::ratatui::layout::Constraint::Max(#v) }
                }
                ratatui::layout::Constraint::Min(v) => {
                    quote::quote! { ::ratatui::layout::Constraint::Min(#v) }
                }
                ratatui::layout::Constraint::Fill(v) => {
                    quote::quote! { ::ratatui::layout::Constraint::Fill(#v) }
                }
            }
        }
    }

    impl ToCodeLiteral for ratatui::layout::Rect {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.x;
            let y = self.y;
            let width = self.width;
            let height = self.height;
            quote::quote! {
                ::ratatui::layout::Rect { x: #x, y: #y, width: #width, height: #height }
            }
        }
    }

    impl ToCodeLiteral for ratatui::style::Modifier {
        fn to_code_literal(&self) -> TokenStream {
            let bits = self.bits();
            quote::quote! {
                ::ratatui::style::Modifier::from_bits_truncate(#bits)
            }
        }
    }

    impl ToCodeLiteral for ratatui::text::Span<'static> {
        fn to_code_literal(&self) -> TokenStream {
            let content = self.content.as_ref();
            quote::quote! {
                ::ratatui::text::Span::raw(#content)
            }
        }
    }

    impl ToCodeLiteral for ratatui::text::Line<'static> {
        fn to_code_literal(&self) -> TokenStream {
            let spans: Vec<TokenStream> = self
                .spans
                .iter()
                .map(|s| {
                    let c = s.content.as_ref();
                    quote::quote! { ::ratatui::text::Span::raw(#c) }
                })
                .collect();
            quote::quote! {
                ::ratatui::text::Line::from(::std::vec![#(#spans),*])
            }
        }
    }

    impl ToCodeLiteral for ratatui::text::Text<'static> {
        fn to_code_literal(&self) -> TokenStream {
            let lines: Vec<TokenStream> = self
                .lines
                .iter()
                .map(|l| {
                    let spans: Vec<TokenStream> = l
                        .spans
                        .iter()
                        .map(|s| {
                            let c = s.content.as_ref();
                            quote::quote! { ::ratatui::text::Span::raw(#c) }
                        })
                        .collect();
                    quote::quote! { ::ratatui::text::Line::from(::std::vec![#(#spans),*]) }
                })
                .collect();
            quote::quote! {
                ::ratatui::text::Text::from(::std::vec![#(#lines),*])
            }
        }
    }

    impl ToCodeLiteral for ratatui::layout::Layout {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::ratatui::layout::Layout::default() }
        }
    }
}
