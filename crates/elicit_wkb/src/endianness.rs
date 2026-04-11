//! `Endianness` shadow wrapper.

/// Serializable mirror of [`wkb::Endianness`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum Endianness {
    /// Big-endian byte order.
    BigEndian,
    /// Little-endian byte order.
    LittleEndian,
}

impl From<wkb::Endianness> for Endianness {
    fn from(value: wkb::Endianness) -> Self {
        match value {
            wkb::Endianness::BigEndian => Self::BigEndian,
            wkb::Endianness::LittleEndian => Self::LittleEndian,
        }
    }
}

impl From<Endianness> for wkb::Endianness {
    fn from(value: Endianness) -> Self {
        match value {
            Endianness::BigEndian => Self::BigEndian,
            Endianness::LittleEndian => Self::LittleEndian,
        }
    }
}

impl From<elicitation::WkbEndianness> for Endianness {
    fn from(value: elicitation::WkbEndianness) -> Self {
        match value {
            elicitation::WkbEndianness::BigEndian => Self::BigEndian,
            elicitation::WkbEndianness::LittleEndian => Self::LittleEndian,
        }
    }
}

impl From<Endianness> for elicitation::WkbEndianness {
    fn from(value: Endianness) -> Self {
        match value {
            Endianness::BigEndian => Self::BigEndian,
            Endianness::LittleEndian => Self::LittleEndian,
        }
    }
}
