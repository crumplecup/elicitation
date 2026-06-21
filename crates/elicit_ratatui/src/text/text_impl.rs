//! [`ratatui::text::Text`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::text::Text<'static>, as Text);
elicit_newtype_traits!(Text, ratatui::text::Text<'static>, []);

impl serde::Serialize for Text {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(1))?;
        let lines: Vec<Vec<String>> = self
            .0
            .lines
            .iter()
            .map(|l| l.spans.iter().map(|s| s.content.to_string()).collect())
            .collect();
        map.serialize_entry("lines", &lines)?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Text {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct TextJson {
            #[serde(default)]
            lines: Vec<Vec<String>>,
        }
        let TextJson { lines } = TextJson::deserialize(d)?;
        let ratatui_lines: Vec<ratatui::text::Line<'static>> = lines
            .into_iter()
            .map(|spans| {
                let ratatui_spans: Vec<ratatui::text::Span<'static>> =
                    spans.into_iter().map(ratatui::text::Span::raw).collect();
                ratatui::text::Line::from(ratatui_spans)
            })
            .collect();
        Ok(Text(std::sync::Arc::new(ratatui::text::Text::from(
            ratatui_lines,
        ))))
    }
}

#[reflect_methods]
impl Text {
    /// Display width in columns.
    #[tracing::instrument(skip(self))]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    /// Number of lines.
    #[tracing::instrument(skip(self))]
    pub fn height(&self) -> usize {
        self.0.height()
    }

    /// Set alignment on all lines.
    #[tracing::instrument(skip(self))]
    pub fn alignment(&self, align: crate::Alignment) -> Text {
        Text(std::sync::Arc::new(
            (*self.0).clone().alignment(align.into_inner()),
        ))
    }

    /// Apply a style patch to all spans.
    #[tracing::instrument(skip(self))]
    pub fn patch_style(&self, style: crate::Style) -> Text {
        Text(std::sync::Arc::new((*self.0).clone().patch_style(*style.0)))
    }
}

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for super::Text {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Text::from(#inner) }
        }
    }
}

impl elicitation::ElicitComplete for Text {}
