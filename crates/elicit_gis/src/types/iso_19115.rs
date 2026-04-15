//! ISO 19115 metadata descriptor types.
//!
//! These types are the construction receipts returned by the ISO 19115 factory
//! traits and the input bags accepted by those traits.  Each descriptor carries
//! the data used during construction; all validity claims are expressed
//! separately via [`Established<P>`] proof tokens.
//!
//! Source: ISO 19115-1:2014 — Metadata.
//!
//! [`Established<P>`]: elicitation::Established

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Input types ───────────────────────────────────────────────────────────────

/// A single CI_Date entry: a date value and its CI_DateTypeCode.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Iso19115Date {
    /// Date value in ISO 8601 format (e.g. `"2024-03-15"` or
    /// `"2024-03-15T12:00:00Z"`).
    ///
    /// Exercises: Iso19115DateIso8601Format.
    pub value: String,

    /// CI_DateTypeCode value (e.g. `"creation"`, `"publication"`, `"revision"`).
    ///
    /// Exercises: CiDateTypeCodeMandatory.
    pub date_type: String,
}

/// A single process step within LI_Lineage, used as input to
/// [`Iso19115LineageFactory::build_lineage`].
///
/// Source: ISO 19115-1:2014 §6.28 — LI_ProcessStep.
///
/// [`Iso19115LineageFactory::build_lineage`]:
///     crate::Iso19115LineageFactory::build_lineage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LineageProcessStep {
    /// Free-text description of the process step (mandatory, non-empty).
    ///
    /// Exercises: LiProcessStepDescriptionMandatory,
    /// LiProcessStepDescriptionNonEmpty.
    pub description: String,

    /// Date and time of the process step in ISO 8601 format (optional).
    ///
    /// Exercises: LiProcessStepDateTimeOptional.
    pub date_time: Option<String>,
}

// ── Factory output descriptors ────────────────────────────────────────────────

/// Descriptor for an EX_GeographicBoundingBox.
///
/// Produced by [`Iso19115ExtentFactory::build_geographic_bbox`].  All four
/// bounds must be IEEE 754 finite values in the correct ranges with south ≤
/// north and west ≤ east (or spanning the antimeridian); these are expressed
/// via the `ExGeographicBoundingBoxValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox.
///
/// [`Iso19115ExtentFactory::build_geographic_bbox`]:
///     crate::Iso19115ExtentFactory::build_geographic_bbox
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeographicBboxDescriptor {
    /// Western-most longitude in decimal degrees (−180..=180).
    pub west_bound: f64,

    /// Eastern-most longitude in decimal degrees (−180..=180).
    pub east_bound: f64,

    /// Southern-most latitude in decimal degrees (−90..=90).
    pub south_bound: f64,

    /// Northern-most latitude in decimal degrees (−90..=90).
    pub north_bound: f64,

    /// Optional extent type code: `true` = inclusion, `false` = exclusion.
    pub extent_type_code: Option<bool>,
}

/// Descriptor for an EX_VerticalExtent.
///
/// Source: ISO 19115-1:2014 §6.17 — EX_VerticalExtent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct VerticalExtentDescriptor {
    /// Minimum vertical value (in units of the vertical CRS).
    pub minimum: f64,

    /// Maximum vertical value (in units of the vertical CRS).
    pub maximum: f64,

    /// Optional identifier of the vertical CRS.
    pub crs_name: Option<String>,
}

/// Descriptor for an EX_TemporalExtent.
///
/// Source: ISO 19115-1:2014 §6.17 — EX_TemporalExtent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TemporalExtentDescriptor {
    /// Beginning of the temporal extent in ISO 8601 format (optional).
    pub begin: Option<String>,

    /// End of the temporal extent in ISO 8601 format (optional).
    pub end: Option<String>,
}

/// Descriptor for an EX_Extent.
///
/// Produced by [`Iso19115ExtentFactory::build_extent`].  At least one of the
/// three element lists must be non-empty; this constraint is expressed via the
/// `ExExtentValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.16 — EX_Extent.
///
/// [`Iso19115ExtentFactory::build_extent`]:
///     crate::Iso19115ExtentFactory::build_extent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExtentDescriptor {
    /// Optional free-text description of the extent.
    pub description: Option<String>,

    /// Zero or more geographic bounding boxes.
    pub geographic_elements: Vec<GeographicBboxDescriptor>,

    /// Zero or more vertical extents.
    pub vertical_elements: Vec<VerticalExtentDescriptor>,

    /// Zero or more temporal extents.
    pub temporal_elements: Vec<TemporalExtentDescriptor>,
}

/// Descriptor for a CI_Citation.
///
/// Produced by [`Iso19115CitationFactory::build_citation`].  The title is
/// mandatory and non-empty; at least one date is mandatory; these constraints
/// are expressed via the `CiCitationValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Citation.
///
/// [`Iso19115CitationFactory::build_citation`]:
///     crate::Iso19115CitationFactory::build_citation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CitationDescriptor {
    /// Primary title of the cited resource (mandatory, non-empty).
    pub title: String,

    /// One or more reference dates (mandatory).
    pub dates: Vec<Iso19115Date>,

    /// Optional name of the responsible party or publisher.
    pub responsible_party: Option<String>,

    /// Optional edition identifier.
    pub edition: Option<String>,

    /// Optional abstract identifier for the cited resource.
    pub identifier: Option<String>,

    /// Optional ISBN.  Validated separately by
    /// [`Iso19115CitationFactory::validate_isbn`].
    ///
    /// [`Iso19115CitationFactory::validate_isbn`]:
    ///     crate::Iso19115CitationFactory::validate_isbn
    pub isbn: Option<String>,

    /// Optional ISSN.  Validated separately by
    /// [`Iso19115CitationFactory::validate_issn`].
    ///
    /// [`Iso19115CitationFactory::validate_issn`]:
    ///     crate::Iso19115CitationFactory::validate_issn
    pub issn: Option<String>,
}

/// Descriptor for a CI_Responsibility.
///
/// Produced by [`Iso19115CitationFactory::build_responsibility`].  The party
/// name is mandatory and non-null; the role is mandatory; these constraints
/// are expressed via the `CiResponsibilityValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Responsibility.
///
/// [`Iso19115CitationFactory::build_responsibility`]:
///     crate::Iso19115CitationFactory::build_responsibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ResponsibilityDescriptor {
    /// Name of the responsible individual or organisation (mandatory, non-null).
    pub party_name: Option<String>,

    /// CI_RoleCode value (mandatory, e.g. `"author"`, `"custodian"`).
    pub role: String,

    /// Optional telephone number.
    pub contact_phone: Option<String>,

    /// Optional email address.
    pub contact_email: Option<String>,

    /// Optional URL for the online resource.
    pub contact_url: Option<String>,
}

/// Descriptor for an MD_Identification record.
///
/// Produced by [`Iso19115RecordFactory::build_identification`].  The citation
/// and abstract are mandatory; the abstract must be non-empty; these
/// constraints are expressed via the `MdIdentificationValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.12 — MD_Identification.
///
/// [`Iso19115RecordFactory::build_identification`]:
///     crate::Iso19115RecordFactory::build_identification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IdentificationDescriptor {
    /// Title of the cited resource (from the mandatory CI_Citation).
    pub citation_title: String,

    /// Abstract description of the resource (mandatory, non-empty).
    pub abstract_text: String,

    /// Optional purpose of the dataset.
    pub purpose: Option<String>,

    /// Optional language of the resource (ISO 639-2 code).
    pub language: Option<String>,

    /// Optional geographic/temporal/vertical extents.
    pub extents: Vec<ExtentDescriptor>,
}

/// Descriptor for the MD_Metadata root record.
///
/// Produced by [`Iso19115RecordFactory::build_metadata`].  A contact party and
/// at least one dateInfo entry are mandatory; identification info is mandatory;
/// these constraints are expressed via the `MdMetadataValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.2 — MD_Metadata.
///
/// [`Iso19115RecordFactory::build_metadata`]:
///     crate::Iso19115RecordFactory::build_metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MetadataDescriptor {
    /// Name of the contact party responsible for this metadata record.
    pub contact_party: Option<String>,

    /// CI_RoleCode for the responsible contact.
    pub contact_role: String,

    /// One or more date/time stamps for the metadata record (mandatory).
    pub date_info: Vec<Iso19115Date>,

    /// Title from the primary identification citation.
    pub identification_title: String,

    /// Abstract from the primary identification block.
    pub identification_abstract: String,

    /// Optional MD_ScopeCode hierarchy level.
    pub hierarchy_level: Option<String>,

    /// Optional file identifier for this metadata record.
    pub file_identifier: Option<String>,

    /// Optional language code (ISO 639-2).
    pub language: Option<String>,
}

/// Descriptor for LI_Lineage.
///
/// Produced by [`Iso19115LineageFactory::build_lineage`].  At least one of
/// statement, process_steps, or source_descriptions must be non-empty; this
/// constraint is expressed via the `LiLineageValid` proof token.
///
/// Source: ISO 19115-1:2014 §6.28 — LI_Lineage.
///
/// [`Iso19115LineageFactory::build_lineage`]:
///     crate::Iso19115LineageFactory::build_lineage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LineageDescriptor {
    /// Optional general lineage statement.
    pub statement: Option<String>,

    /// Zero or more process steps (each with a mandatory description).
    pub process_steps: Vec<LineageProcessStep>,

    /// Zero or more source descriptions.
    pub source_descriptions: Vec<String>,
}

/// A single DQ_Element report entry.
///
/// Source: ISO 19115-1:2014 §6.29 — DQ_Element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DataQualityReport {
    /// Name of the DQ_Element subtype (e.g. `"DQ_CompletenessOmission"`).
    pub element_type: String,

    /// Number of DQ_Result objects attached (must be ≥ 1).
    pub result_count: usize,
}

/// Descriptor for DQ_DataQuality.
///
/// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DataQualityDescriptor {
    /// Optional MD_Scope description for the quality evaluation.
    pub scope: Option<String>,

    /// Zero or more DQ_Element reports.
    pub reports: Vec<DataQualityReport>,

    /// Optional lineage statement from the associated LI_Lineage.
    pub lineage_statement: Option<String>,
}
