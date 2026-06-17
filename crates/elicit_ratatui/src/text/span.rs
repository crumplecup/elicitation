//! [`ratatui::text::Span`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::text::Span<'static>, as Span);
elicit_newtype_traits!(Span, ratatui::text::Span<'static>, []);

impl serde::Serialize for Span {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(1))?;
        map.serialize_entry("content", self.0.content.as_ref())?;
        map.end()
    }
}
impl<'de> serde::Deserialize<'de> for Span {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct SpanJson { content: String }
        let SpanJson { content } = SpanJson::deserialize(d)?;
        Ok(Span(std::sync::Arc::new(ratatui::text::Span::raw(content))))
    }
}

#[reflect_methods]
impl Span {
    /// The span's text content.
    #[tracing::instrument(skip(self))]
    pub fn content(&self) -> String {
        self.0.content.to_string()
    }

    /// Apply a style patch.
    #[tracing::instrument(skip(self))]
    pub fn patch_style(&self, style: crate::Style) -> Span {
        Span(std::sync::Arc::new((*self.0).clone().patch_style(*style.0)))
    }

    /// Reset to default style.
    #[tracing::instrument(skip(self))]
    pub fn reset_style(&self) -> Span {
        Span(std::sync::Arc::new((*self.0).clone().reset_style()))
    }
}

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for super::Span {
        fn to_code_literal(&self) -> TokenStream {
            let inner = (*self.0).to_code_literal();
            quote::quote! { ::elicit_ratatui::Span::from(#inner) }
        }
    }
}

impl elicitation::ElicitComplete for Span {}
