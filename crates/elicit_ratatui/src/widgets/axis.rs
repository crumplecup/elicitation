//! [`ratatui::widgets::Axis`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::widgets::Axis<'static>, as Axis);
elicit_newtype_traits!(Axis, ratatui::widgets::Axis<'static>, []);

#[reflect_methods]
impl Axis {
    /// Set the axis title.
    #[tracing::instrument(skip(self))]
    pub fn title(&self, title: String) -> Axis {
        Axis(std::sync::Arc::new((*self.0).clone().title(title)))
    }

    /// Set the axis style.
    #[tracing::instrument(skip(self))]
    pub fn style(&self, style: crate::Style) -> Axis {
        Axis(std::sync::Arc::new((*self.0).clone().style(*style.0)))
    }

    /// Set the value bounds `[min, max]`.
    #[tracing::instrument(skip(self))]
    pub fn bounds(&self, min: f64, max: f64) -> Axis {
        Axis(std::sync::Arc::new((*self.0).clone().bounds([min, max])))
    }

    /// Set axis tick labels.
    #[tracing::instrument(skip(self, labels))]
    pub fn labels(&self, labels: Vec<String>) -> Axis {
        let spans: Vec<ratatui::text::Span<'static>> =
            labels.into_iter().map(ratatui::text::Span::raw).collect();
        Axis(std::sync::Arc::new((*self.0).clone().labels(spans)))
    }

    /// Set axis labels alignment.
    #[tracing::instrument(skip(self))]
    pub fn labels_alignment(&self, align: crate::Alignment) -> Axis {
        Axis(std::sync::Arc::new((*self.0).clone().labels_alignment(align.into_inner())))
    }
}
impl serde::Serialize for Axis {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}
impl<'de> serde::Deserialize<'de> for Axis {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        serde::de::IgnoredAny::deserialize(d)?;
        Ok(Axis(std::sync::Arc::new(ratatui::widgets::Axis::default())))
    }
}
mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Axis {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Axis::from(::ratatui::widgets::Axis::default()) }
        }
    }
}
impl elicitation::ElicitComplete for Axis {}
