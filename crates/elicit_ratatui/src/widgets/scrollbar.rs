//! [`ratatui::widgets::Scrollbar`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Scrollbar<'static>, as Scrollbar);
elicit_newtype_traits!(Scrollbar, ratatui::widgets::Scrollbar<'static>, []);

#[reflect_methods]
impl Scrollbar {
    /// Set the track symbol.
    #[tracing::instrument(skip(self))]
    pub fn track_symbol(&self, symbol: Option<String>) -> Scrollbar {
        let s: Option<&'static str> = symbol.map(|s| -> &'static str { Box::leak(s.into_boxed_str()) });
        Scrollbar(std::sync::Arc::new((*self.0).clone().track_symbol(s)))
    }

    /// Set the thumb symbol.
    #[tracing::instrument(skip(self))]
    pub fn thumb_symbol(&self, symbol: String) -> Scrollbar {
        let s: &'static str = Box::leak(symbol.into_boxed_str());
        Scrollbar(std::sync::Arc::new((*self.0).clone().thumb_symbol(s)))
    }

    /// Set the begin-arrow symbol.
    #[tracing::instrument(skip(self))]
    pub fn begin_symbol(&self, symbol: Option<String>) -> Scrollbar {
        let s: Option<&'static str> = symbol.map(|s| -> &'static str { Box::leak(s.into_boxed_str()) });
        Scrollbar(std::sync::Arc::new((*self.0).clone().begin_symbol(s)))
    }

    /// Set the end-arrow symbol.
    #[tracing::instrument(skip(self))]
    pub fn end_symbol(&self, symbol: Option<String>) -> Scrollbar {
        let s: Option<&'static str> = symbol.map(|s| -> &'static str { Box::leak(s.into_boxed_str()) });
        Scrollbar(std::sync::Arc::new((*self.0).clone().end_symbol(s)))
    }

    /// Overall scrollbar style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Scrollbar {
        Scrollbar(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Thumb style.
    #[tracing::instrument(skip(self))]
    pub fn thumb_style(&self, style: crate::Style) -> Scrollbar {
        Scrollbar(std::sync::Arc::new((*self.0).clone().thumb_style(*style.0)))
    }

    /// Track style.
    #[tracing::instrument(skip(self))]
    pub fn track_style(&self, style: crate::Style) -> Scrollbar {
        Scrollbar(std::sync::Arc::new((*self.0).clone().track_style(*style.0)))
    }
}
impl serde::Serialize for Scrollbar {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Scrollbar {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct ScrollbarJson {
            #[serde(default)]
            orientation: Option<crate::ScrollbarOrientation>,
        }
        let ScrollbarJson { orientation } = ScrollbarJson::deserialize(d)?;
        let o = orientation
            .map(|o| o.into_inner())
            .unwrap_or(ratatui::widgets::ScrollbarOrientation::VerticalRight);
        Ok(Scrollbar(std::sync::Arc::new(ratatui::widgets::Scrollbar::new(o))))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Scrollbar {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! {
                ::elicit_ratatui::Scrollbar::new(
                    ::elicit_ratatui::ScrollbarOrientation::VerticalRight
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Scrollbar {}
