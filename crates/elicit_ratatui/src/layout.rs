//! [`ratatui::layout::Layout`] shadow type.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(ratatui::layout::Layout, as Layout);
elicit_newtype_traits!(Layout, ratatui::layout::Layout, []);

impl serde::Serialize for Layout {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        s.serialize_map(Some(0))?.end()
    }
}

impl<'de> serde::Deserialize<'de> for Layout {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct LayoutJson {
            #[serde(default)]
            direction: Option<String>,
            #[serde(default)]
            constraints: Vec<crate::Constraint>,
        }
        let LayoutJson {
            direction,
            constraints,
        } = LayoutJson::deserialize(d)?;
        let ratatui_constraints: Vec<ratatui::layout::Constraint> =
            constraints.into_iter().map(|c| (*c.0).clone()).collect();
        let layout = match direction.as_deref() {
            Some("Horizontal") | Some("horizontal") => {
                ratatui::layout::Layout::horizontal(ratatui_constraints)
            }
            _ => ratatui::layout::Layout::vertical(ratatui_constraints),
        };
        Ok(Layout(std::sync::Arc::new(layout)))
    }
}

#[reflect_methods]
impl Layout {
    /// Set the layout direction.
    #[tracing::instrument(skip(self))]
    pub fn direction(&self, direction: crate::Direction) -> Layout {
        Layout(std::sync::Arc::new(
            (*self.0).clone().direction(direction.into_inner()),
        ))
    }

    /// Set horizontal margin only.
    #[tracing::instrument(skip(self))]
    pub fn horizontal_margin(&self, margin: u16) -> Layout {
        Layout(std::sync::Arc::new(
            (*self.0).clone().horizontal_margin(margin),
        ))
    }

    /// Set vertical margin only.
    #[tracing::instrument(skip(self))]
    pub fn vertical_margin(&self, margin: u16) -> Layout {
        Layout(std::sync::Arc::new(
            (*self.0).clone().vertical_margin(margin),
        ))
    }

    /// Split the area and return the resulting `Rect`s.
    #[tracing::instrument(skip(self, area))]
    pub fn split(&self, area: crate::Rect) -> Vec<crate::Rect> {
        self.0
            .clone()
            .split(*area.0)
            .iter()
            .map(|r| crate::Rect(std::sync::Arc::new(*r)))
            .collect()
    }
}

mod emit_impls {
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;
    impl ToCodeLiteral for super::Layout {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_ratatui::Layout::from(::ratatui::layout::Layout::default()) }
        }
    }
}

impl elicitation::ElicitComplete for Layout {}
