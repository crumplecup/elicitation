//! Mesh type wrappers.
//!
//! Covers [`PrimitiveTopology`] and [`Indices`].
//!
//! The full [`bevy::mesh::Mesh`] type manages vertex buffers and GPU resources.
//! It is not directly wrapped here — use a `MeshBuilder` factory pattern instead.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── PrimitiveTopology ─────────────────────────────────────────────────────────

elicit_newtype!(bevy::mesh::PrimitiveTopology, as PrimitiveTopology);
elicit_newtype_traits!(PrimitiveTopology, bevy::mesh::PrimitiveTopology, [eq]);

impl From<PrimitiveTopology> for bevy::mesh::PrimitiveTopology {
    fn from(v: PrimitiveTopology) -> Self {
        *v.0
    }
}

impl serde::Serialize for PrimitiveTopology {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PrimitiveTopology {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "PointList" => bevy::mesh::PrimitiveTopology::PointList,
            "LineList" => bevy::mesh::PrimitiveTopology::LineList,
            "LineStrip" => bevy::mesh::PrimitiveTopology::LineStrip,
            "TriangleList" => bevy::mesh::PrimitiveTopology::TriangleList,
            "TriangleStrip" => bevy::mesh::PrimitiveTopology::TriangleStrip,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "PointList",
                        "LineList",
                        "LineStrip",
                        "TriangleList",
                        "TriangleStrip",
                    ],
                ));
            }
        };
        Ok(PrimitiveTopology(Arc::new(inner)))
    }
}

#[reflect_methods]
impl PrimitiveTopology {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::mesh::PrimitiveTopology::PointList => "PointList",
            bevy::mesh::PrimitiveTopology::LineList => "LineList",
            bevy::mesh::PrimitiveTopology::LineStrip => "LineStrip",
            bevy::mesh::PrimitiveTopology::TriangleList => "TriangleList",
            bevy::mesh::PrimitiveTopology::TriangleStrip => "TriangleStrip",
        }
    }

    /// Returns `true` if this is [`PrimitiveTopology::TriangleList`].
    #[tracing::instrument(skip(self))]
    pub fn is_triangle_list(&self) -> bool {
        matches!(*self.0, bevy::mesh::PrimitiveTopology::TriangleList)
    }

    /// Returns `true` if this is [`PrimitiveTopology::LineList`].
    #[tracing::instrument(skip(self))]
    pub fn is_line_list(&self) -> bool {
        matches!(*self.0, bevy::mesh::PrimitiveTopology::LineList)
    }

    /// Returns `true` if this is a strip topology (line strip or triangle strip).
    #[tracing::instrument(skip(self))]
    pub fn is_strip(&self) -> bool {
        matches!(
            *self.0,
            bevy::mesh::PrimitiveTopology::LineStrip | bevy::mesh::PrimitiveTopology::TriangleStrip
        )
    }
}

mod emit_impls_topology {
    use super::PrimitiveTopology;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PrimitiveTopology {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::PrimitiveTopology::from(
                    ::bevy::mesh::PrimitiveTopology::#variant
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for PrimitiveTopology {}

// ── Indices ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::mesh::Indices, as Indices);
elicit_newtype_traits!(Indices, bevy::mesh::Indices, []);

impl From<Indices> for bevy::mesh::Indices {
    fn from(v: Indices) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for Indices {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        match &*self.0 {
            bevy::mesh::Indices::U16(_) => {
                map.serialize_entry("type", "U16")?;
            }
            bevy::mesh::Indices::U32(_) => {
                map.serialize_entry("type", "U32")?;
            }
        }
        map.serialize_entry("len", &self.len())?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Indices {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Indices;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "type": "U16" | "U32""#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Indices, A::Error> {
                let mut ty: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => ty = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let t = ty.ok_or_else(|| de::Error::missing_field("type"))?;
                let inner = match t.as_str() {
                    "U16" => bevy::mesh::Indices::U16(vec![]),
                    "U32" => bevy::mesh::Indices::U32(vec![]),
                    other => {
                        return Err(de::Error::unknown_variant(other, &["U16", "U32"]));
                    }
                };
                Ok(Indices(Arc::new(inner)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Indices {
    /// Returns the number of indices.
    #[tracing::instrument(skip(self))]
    pub fn len(&self) -> usize {
        match &*self.0 {
            bevy::mesh::Indices::U16(v) => v.len(),
            bevy::mesh::Indices::U32(v) => v.len(),
        }
    }

    /// Returns `true` if there are no indices.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if this stores 16-bit indices.
    #[tracing::instrument(skip(self))]
    pub fn is_u16(&self) -> bool {
        matches!(&*self.0, bevy::mesh::Indices::U16(_))
    }

    /// Returns `true` if this stores 32-bit indices.
    #[tracing::instrument(skip(self))]
    pub fn is_u32(&self) -> bool {
        matches!(&*self.0, bevy::mesh::Indices::U32(_))
    }
}

mod emit_impls_indices {
    use super::Indices;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Indices {
        fn to_code_literal(&self) -> TokenStream {
            if self.is_u16() {
                quote::quote! {
                    ::elicit_bevy::Indices::from(::bevy::mesh::Indices::U16(vec![]))
                }
            } else {
                quote::quote! {
                    ::elicit_bevy::Indices::from(::bevy::mesh::Indices::U32(vec![]))
                }
            }
        }
    }
}

impl elicitation::ElicitComplete for Indices {}
