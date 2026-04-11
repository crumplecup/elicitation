//! `PlanarConfiguration` — TIFF planar configuration wrapper.

/// Serializable shadow of [`tiff::tags::PlanarConfiguration`].
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
pub enum PlanarConfiguration {
    /// All samples for a pixel are stored together.
    Chunky,
    /// Samples are stored in separate planes.
    Planar,
}

impl From<tiff::tags::PlanarConfiguration> for PlanarConfiguration {
    fn from(value: tiff::tags::PlanarConfiguration) -> Self {
        match value {
            tiff::tags::PlanarConfiguration::Chunky => Self::Chunky,
            tiff::tags::PlanarConfiguration::Planar => Self::Planar,
            _ => panic!("unsupported future PlanarConfiguration variant"),
        }
    }
}

impl From<PlanarConfiguration> for tiff::tags::PlanarConfiguration {
    fn from(value: PlanarConfiguration) -> Self {
        match value {
            PlanarConfiguration::Chunky => Self::Chunky,
            PlanarConfiguration::Planar => Self::Planar,
        }
    }
}
