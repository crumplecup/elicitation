//! [`ratatui::text::Line`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::text::Line<'static>, as Line);
elicit_newtype_traits!(Line, ratatui::text::Line<'static>, []);

impl serde::Serialize for Line {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(1))?;
        let contents: Vec<String> =
            self.0.spans.iter().map(|s| s.content.to_string()).collect();
        map.serialize_entry("spans", &contents)?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Line {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct LineJson {
            #[serde(default)]
            spans: Vec<String>,
        }
        let LineJson { spans } = LineJson::deserialize(d)?;
        let ratatui_spans: Vec<ratatui::text::Span<'static>> =
            spans.into_iter().map(ratatui::text::Span::raw).collect();
        Ok(Line(std::sync::Arc::new(ratatui::text::Line::from(ratatui_spans))))
    }
}

#[reflect_methods]
impl Line {
    /// Display width of this line in columns.
    #[tracing::instrument(skip(self))]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    /// Set alignment.
    #[tracing::instrument(skip(self))]
    pub fn alignment(&self, align: crate::Alignment) -> Line {
        Line(std::sync::Arc::new((*self.0).clone().alignment(align.into_inner())))
    }

    /// Apply a style patch to all spans.
    #[tracing::instrument(skip(self))]
    pub fn patch_style(&self, style: crate::Style) -> Line {
        Line(std::sync::Arc::new((*self.0).clone().patch_style(*style.0)))
    }
}

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for super::Line {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Line::from(#inner) }
        }
    }
}

impl elicitation::ElicitComplete for Line {}
