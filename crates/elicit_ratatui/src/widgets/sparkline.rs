//! [`ratatui::widgets::Sparkline`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Sparkline<'static>, as Sparkline);
elicit_newtype_traits!(Sparkline, ratatui::widgets::Sparkline<'static>, []);

#[reflect_methods]
impl Sparkline {
    /// Wrap in a block container.
    #[tracing::instrument(skip(self, block))]
    pub fn block(&self, block: crate::Block) -> Sparkline {
        Sparkline(std::sync::Arc::new(
            (*self.0).clone().block((*block.0).clone()),
        ))
    }

    /// Set the data values.
    #[tracing::instrument(skip(self, data))]
    pub fn data(&self, data: Vec<u64>) -> Sparkline {
        let data: &'static [u64] = Box::leak(data.into_boxed_slice());
        Sparkline(std::sync::Arc::new((*self.0).clone().data(data)))
    }

    /// Set the maximum value (auto-scales if not set).
    #[tracing::instrument(skip(self))]
    pub fn max(&self, max: u64) -> Sparkline {
        Sparkline(std::sync::Arc::new((*self.0).clone().max(max)))
    }

    /// Set the render direction ("LeftToRight" or "RightToLeft").
    #[tracing::instrument(skip(self))]
    pub fn direction(&self, direction: String) -> Sparkline {
        let rd = if direction.eq_ignore_ascii_case("righttoleft") {
            ratatui::widgets::RenderDirection::RightToLeft
        } else {
            ratatui::widgets::RenderDirection::LeftToRight
        };
        Sparkline(std::sync::Arc::new((*self.0).clone().direction(rd)))
    }

    /// Set the sparkline style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Sparkline {
        Sparkline(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }
}
impl serde::Serialize for Sparkline {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Sparkline {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(Sparkline(std::sync::Arc::new(
            ratatui::widgets::Sparkline::default(),
        )))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Sparkline {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Sparkline::from(::ratatui::widgets::Sparkline::default()) }
        }
    }
}
impl elicitation::ElicitComplete for Sparkline {}
