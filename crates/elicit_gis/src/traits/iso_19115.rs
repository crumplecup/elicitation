//! ISO 19115 metadata construction, contact, date, and quality traits.
//!
//! Three-role taxonomy applied to ISO 19115 Metadata:
//!
//! ## Role 1 — Factory / builder traits
//!
//! Each factory accepts raw data (and, for composite types, `Established<P>`
//! proof tokens from already-validated sub-objects) and returns either an
//! error or a descriptor together with an `Established<P>` postcondition token
//! certifying that the constructed object satisfies the relevant validity
//! constraints.  The factory method signatures encode the validity dependency
//! graph of ISO 19115 in the type system.
//!
//! | Trait | Builds | Postcondition |
//! |---|---|---|
//! | [`Iso19115CitationFactory`] | CI_Citation | [`CiCitationValid`] |
//! | [`Iso19115CitationFactory`] | CI_Responsibility | [`CiResponsibilityValid`] |
//! | [`Iso19115ExtentFactory`] | EX_GeographicBoundingBox | [`ExGeographicBoundingBoxValid`] |
//! | [`Iso19115ExtentFactory`] | EX_Extent | [`ExExtentValid`] |
//! | [`Iso19115LineageFactory`] | LI_Lineage | [`LiLineageValid`] |
//! | [`Iso19115RecordFactory`] | MD_Identification | [`MdIdentificationValid`] |
//! | [`Iso19115RecordFactory`] | MD_Metadata | [`MdMetadataValid`] |
//!
//! ## Role 2 — Orthogonal concern traits
//!
//! These traits operate independently of structural validity.  A CI_Responsibility
//! can report its party name and role even if the associated metadata record is
//! incomplete; a DQ_DataQuality element can report its lineage statement even if
//! the quality report has no results yet.
//!
//! | Trait | Reports |
//! |---|---|
//! | [`Iso19115ContactMeta`] | Party name, role, phone, email, URL |
//! | [`Iso19115DateMeta`] | All dates, creation, publication, revision |
//! | [`Iso19115QualityMeta`] | Quality scope, report count, lineage statement |
//!
//! ## Role 3 — Abstraction supertrait
//!
//! [`Iso19115CitationFactory`], [`Iso19115ExtentFactory`],
//! [`Iso19115LineageFactory`], [`Iso19115RecordFactory`],
//! [`Iso19115ContactMeta`], [`Iso19115DateMeta`], and [`Iso19115QualityMeta`]
//! are composed into [`crate::GisBackend`], which hides the interplay of CI,
//! EX, DQ, and LI sections behind a single coherent interface.
//!
//! Source: ISO 19115-1:2014 — Metadata.
//!
//! [`CiCitationValid`]: crate::CiCitationValid
//! [`CiResponsibilityValid`]: crate::CiResponsibilityValid
//! [`ExGeographicBoundingBoxValid`]: crate::ExGeographicBoundingBoxValid
//! [`ExExtentValid`]: crate::ExExtentValid
//! [`LiLineageValid`]: crate::LiLineageValid
//! [`MdIdentificationValid`]: crate::MdIdentificationValid
//! [`MdMetadataValid`]: crate::MdMetadataValid

use elicitation::Established;

use crate::{
    CiCitationIsbnFormatValid, CiCitationIssnFormatValid, CiCitationValid, CiResponsibilityValid,
    CitationDescriptor, DataQualityDescriptor, DqElementResultMandatory, ExExtentValid,
    ExGeographicBoundingBoxValid, ExtentDescriptor, GeographicBboxDescriptor, GisResult,
    IdentificationDescriptor, Iso19115Date, LiLineageValid, LineageDescriptor, LineageProcessStep,
    MdIdentificationValid, MdMetadataValid, MetadataDescriptor, ResponsibilityDescriptor,
};

// ── Role 1: Factory / builder traits ─────────────────────────────────────────

/// Build and validate ISO 19115 CI_Citation and CI_Responsibility objects.
///
/// Both classes belong to the ISO 19115 Citation Information (CI) package.
///
/// # CI_Citation
///
/// A valid CI_Citation requires a non-empty title and at least one CI_Date.
/// [`build_citation`] validates `CiCitationTitleMandatory`,
/// `CiCitationTitleNonEmpty`, and `CiCitationDateMandatory`, then emits
/// `Established<CiCitationValid>` as a proof token.
///
/// # CI_Responsibility
///
/// A valid CI_Responsibility requires a non-null party and a role.
/// [`build_responsibility`] validates `CiResponsibilityPartyMandatory`,
/// `CiResponsibilityPartyNameNonNull`, and `CiResponsibilityRoleMandatory`,
/// then emits `Established<CiResponsibilityValid>`.
///
/// # Optional format checks
///
/// [`validate_isbn`] and [`validate_issn`] are separate factory methods because
/// ISBN/ISSN format validity is an independent claim that does not compose into
/// `CiCitationValid` — a citation may be structurally valid without those fields.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Citation, CI_Responsibility.
///
/// [`build_citation`]: Iso19115CitationFactory::build_citation
/// [`build_responsibility`]: Iso19115CitationFactory::build_responsibility
/// [`validate_isbn`]: Iso19115CitationFactory::validate_isbn
/// [`validate_issn`]: Iso19115CitationFactory::validate_issn
pub trait Iso19115CitationFactory: Send + Sync {
    /// Build a CI_Citation from a mandatory non-empty title and at least one
    /// CI_Date.
    ///
    /// Validates: CiCitationTitleMandatory, CiCitationTitleNonEmpty,
    /// CiCitationDateMandatory → CiCitationValid.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Citation validity.
    fn build_citation(
        &self,
        title: String,
        dates: Vec<Iso19115Date>,
    ) -> GisResult<(CitationDescriptor, Established<CiCitationValid>)>;

    /// Build a CI_Responsibility from a party name and a role code.
    ///
    /// Validates: CiResponsibilityPartyMandatory,
    /// CiResponsibilityPartyNameNonNull, CiResponsibilityRoleMandatory →
    /// CiResponsibilityValid.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Responsibility validity.
    fn build_responsibility(
        &self,
        party_name: String,
        role: String,
    ) -> GisResult<(ResponsibilityDescriptor, Established<CiResponsibilityValid>)>;

    /// Validate the ISBN field of a citation descriptor.
    ///
    /// Separate from [`build_citation`] because ISBN validity is an independent
    /// claim: a citation may be structurally valid (CiCitationValid) without
    /// carrying an ISBN, and an ISBN, if present, must conform to the
    /// ISBN-13/ISBN-10 check-digit algorithm.
    ///
    /// Requires the descriptor's `isbn` field to be `Some(_)`.
    ///
    /// Validates: CiCitationIsbnFormatValid.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Citation.ISBN.
    ///
    /// [`build_citation`]: Iso19115CitationFactory::build_citation
    fn validate_isbn(
        &self,
        citation: &CitationDescriptor,
    ) -> GisResult<Established<CiCitationIsbnFormatValid>>;

    /// Validate the ISSN field of a citation descriptor.
    ///
    /// Separate from [`build_citation`] because ISSN validity is an independent
    /// claim: a citation may be structurally valid (CiCitationValid) without
    /// carrying an ISSN, and an ISSN, if present, must match the ISSN-8
    /// format.
    ///
    /// Requires the descriptor's `issn` field to be `Some(_)`.
    ///
    /// Validates: CiCitationIssnFormatValid.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Citation.ISSN.
    ///
    /// [`build_citation`]: Iso19115CitationFactory::build_citation
    fn validate_issn(
        &self,
        citation: &CitationDescriptor,
    ) -> GisResult<Established<CiCitationIssnFormatValid>>;
}

/// Build and validate ISO 19115 EX_* extent objects.
///
/// The EX package provides geographic (bounding box), vertical, and temporal
/// extent representations used throughout ISO 19115 identification,
/// maintenance, and CRS scope records.
///
/// # Dependency graph
///
/// `ExExtentValid` requires at least one sub-element; the corresponding
/// sub-element tokens are passed as preconditions.  Only
/// `ExGeographicBoundingBoxValid` tokens are required at the method boundary
/// because bbox is the most common extent type; vertical and temporal extents
/// are carried in the raw `ExtentDescriptor` without additional tokens.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.16–§6.17 — EX_Extent, EX_GeographicBoundingBox.
pub trait Iso19115ExtentFactory: Send + Sync {
    /// Build an EX_GeographicBoundingBox from the four longitude/latitude
    /// bounds.
    ///
    /// Validates: ExBboxWestBoundIsFinite, ExBboxEastBoundIsFinite,
    /// ExBboxSouthBoundIsFinite, ExBboxNorthBoundIsFinite,
    /// ExBboxLongitudeRange, ExBboxLatitudeRange, ExBboxSouthLeNorth,
    /// ExBboxWestLeEastOrAntimeridian → ExGeographicBoundingBoxValid.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox.
    fn build_geographic_bbox(
        &self,
        west: f64,
        east: f64,
        south: f64,
        north: f64,
    ) -> GisResult<(
        GeographicBboxDescriptor,
        Established<ExGeographicBoundingBoxValid>,
    )>;

    /// Build an EX_Extent from validated geographic bounding boxes and
    /// optional raw vertical and temporal extents.
    ///
    /// Preconditions: `Established<ExGeographicBoundingBoxValid>` for each
    /// geographic sub-element.
    ///
    /// At least one of `geographic`, `vertical`, or `temporal` must be
    /// non-empty.
    ///
    /// Validates: ExExtentAtLeastOneElementRequired → ExExtentValid.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent.
    fn build_extent(
        &self,
        description: Option<String>,
        geographic: Vec<Established<ExGeographicBoundingBoxValid>>,
        vertical: Vec<(f64, f64, Option<String>)>,
        temporal: Vec<(Option<String>, Option<String>)>,
    ) -> GisResult<(ExtentDescriptor, Established<ExExtentValid>)>;
}

/// Build and validate ISO 19115 LI_Lineage records.
///
/// A valid LI_Lineage requires at least one of: a statement, a process step,
/// or a source.  Each process step has a mandatory non-empty description.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.28 — LI_Lineage.
pub trait Iso19115LineageFactory: Send + Sync {
    /// Build an LI_Lineage from an optional statement, zero or more process
    /// steps, and zero or more source descriptions.
    ///
    /// Validates each process step's description for presence and
    /// non-emptiness (LiProcessStepDescriptionMandatory,
    /// LiProcessStepDescriptionNonEmpty).
    ///
    /// Validates: LiLineageAtLeastOneProvided,
    /// LiLineageStatementConditional → LiLineageValid.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Lineage.
    fn build_lineage(
        &self,
        statement: Option<String>,
        process_steps: Vec<LineageProcessStep>,
        source_descriptions: Vec<String>,
    ) -> GisResult<(LineageDescriptor, Established<LiLineageValid>)>;

    /// Validate that a single process-step description is present and
    /// non-empty.
    ///
    /// Useful in FV harnesses that need to check a step in isolation before
    /// composing it into a full lineage record.
    ///
    /// Validates: LiProcessStepDescriptionMandatory,
    /// LiProcessStepDescriptionNonEmpty.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_ProcessStep.description.
    fn validate_process_step_description(
        &self,
        step: &LineageProcessStep,
    ) -> GisResult<Established<crate::LiProcessStepDescriptionNonEmpty>>;
}

/// Build and validate ISO 19115 MD_Identification and MD_Metadata records.
///
/// # MD_Identification
///
/// [`build_identification`] requires a valid citation (`Established<CiCitationValid>`)
/// and a non-empty abstract.  It emits `Established<MdIdentificationValid>`.
///
/// # MD_Metadata
///
/// [`build_metadata`] requires a valid responsible contact
/// (`Established<CiResponsibilityValid>`), at least one date, and a valid
/// identification block (`Established<MdIdentificationValid>`).  It emits
/// `Established<MdMetadataValid>`.
///
/// Passing `Established<CiResponsibilityValid>` as a precondition to
/// `build_metadata` encodes the dependency
/// `MdMetadataContactMandatory + MdMetadataContactPartyNameNonNull →
/// MdMetadataValid` in the type system: no client can call `build_metadata`
/// without having already proved the contact is well-formed.
///
/// # Object safety
///
/// All method signatures use only concrete types.  The trait is
/// `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.2 — MD_Metadata; §6.12 — MD_Identification.
///
/// [`build_identification`]: Iso19115RecordFactory::build_identification
/// [`build_metadata`]: Iso19115RecordFactory::build_metadata
pub trait Iso19115RecordFactory: Send + Sync {
    /// Build an MD_Identification record from a valid citation, a non-empty
    /// abstract, and optional extents.
    ///
    /// Preconditions: `Established<CiCitationValid>` for the mandatory
    /// citation.
    ///
    /// For each optional extent, `Established<ExExtentValid>` is required.
    ///
    /// Validates: MdIdentificationCitationMandatory,
    /// MdIdentificationAbstractMandatory, MdIdentificationAbstractNonEmpty →
    /// MdIdentificationValid.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_Identification.
    fn build_identification(
        &self,
        citation: Established<CiCitationValid>,
        abstract_text: String,
        purpose: Option<String>,
        language: Option<String>,
        extents: Vec<Established<ExExtentValid>>,
    ) -> GisResult<(IdentificationDescriptor, Established<MdIdentificationValid>)>;

    /// Build the MD_Metadata root record from a valid contact, at least one
    /// date, and a valid identification block.
    ///
    /// Preconditions:
    /// - `Established<CiResponsibilityValid>` — the mandatory responsible
    ///   contact.
    /// - `date_info` — at least one CI_Date entry (MdMetadataDateInfoMandatory).
    /// - `Established<MdIdentificationValid>` — the mandatory identification
    ///   block.
    ///
    /// Validates: MdMetadataContactMandatory, MdMetadataContactPartyNameNonNull,
    /// MdMetadataDateInfoMandatory, MdMetadataIdentificationInfoMandatory →
    /// MdMetadataValid.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata.
    fn build_metadata(
        &self,
        contact: Established<CiResponsibilityValid>,
        date_info: Vec<Iso19115Date>,
        identification: Established<MdIdentificationValid>,
        hierarchy_level: Option<String>,
        file_identifier: Option<String>,
        language: Option<String>,
    ) -> GisResult<(MetadataDescriptor, Established<MdMetadataValid>)>;
}

// ── Role 2: Orthogonal concern traits ─────────────────────────────────────────

/// Query contact and responsibility metadata from any ISO 19115 object.
///
/// This trait is an **orthogonal concern**: it reports identity and contact
/// facts about a CI_Responsibility or CI_Party regardless of whether the
/// enclosing metadata record is structurally complete.  Even a partially
/// constructed metadata record has a contact party name and a role code.
///
/// The trait is `dyn`-compatible: all methods return `&str`, `Option<&str>`,
/// or `bool` — concrete types with no generics.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Responsibility, CI_Party.
pub trait Iso19115ContactMeta: Send + Sync {
    /// Returns the name of the responsible individual or organisation, or
    /// `None` if not yet set.
    ///
    /// Exercises: CiResponsibilityPartyNameNonNull (when `Some`),
    /// CiOrganisationNameOptional.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Party.name.
    fn party_name(&self) -> Option<&str>;

    /// Returns the CI_RoleCode for this responsibility.
    ///
    /// Exercises: CiResponsibilityRoleMandatory.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Responsibility.role.
    fn role(&self) -> &str;

    /// Returns the primary phone number, or `None`.
    ///
    /// Exercises: CiContactPhoneOptional.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Contact.phone.
    fn contact_phone(&self) -> Option<&str>;

    /// Returns the primary email address, or `None`.
    ///
    /// Exercises: CiAddressEmailOptional.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Address.electronicMailAddress.
    fn contact_email(&self) -> Option<&str>;

    /// Returns the primary online resource URL, or `None`.
    ///
    /// Exercises: CiOnlineResourceLinkageMandatory (when `Some`),
    /// CiContactOnlineResourceOptional.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_OnlineResource.linkage.
    fn contact_url(&self) -> Option<&str>;
}

/// Query date and temporal metadata from any ISO 19115 object.
///
/// This trait is an **orthogonal concern**: it reports date information from
/// CI_Date entries attached to any metadata class (CI_Citation, MD_Metadata,
/// etc.) regardless of the structural completeness of the enclosing record.
/// Even a citation that is missing a title still has whatever dates have been
/// attached to it.
///
/// The trait is `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.11 — CI_Date.
pub trait Iso19115DateMeta: Send + Sync {
    /// Returns all CI_Date entries attached to this object.
    ///
    /// Exercises: CiDateValueMandatory, CiDateTypeCodeMandatory.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_Date.
    fn all_dates(&self) -> &[Iso19115Date];

    /// Returns the value of the first date with `date_type == "creation"`, or
    /// `None`.
    ///
    /// Exercises: CiDateTypeCreation.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_DateTypeCode.creation.
    fn creation_date(&self) -> Option<&str>;

    /// Returns the value of the first date with `date_type == "publication"`,
    /// or `None`.
    ///
    /// Exercises: CiDateTypePublication.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_DateTypeCode.publication.
    fn publication_date(&self) -> Option<&str>;

    /// Returns the value of the first date with `date_type == "revision"`, or
    /// `None`.
    ///
    /// Exercises: CiDateTypeRevision.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — CI_DateTypeCode.revision.
    fn revision_date(&self) -> Option<&str>;

    /// Returns the value of the most recent date across all entries, or `None`
    /// if no dates are present.
    ///
    /// "Most recent" is determined lexicographically over ISO 8601 strings,
    /// which is correct when all dates use the same format.
    ///
    /// Exercises: Iso19115DateIso8601Format (implicitly — comparison is only
    /// meaningful on conforming date strings).
    fn most_recent_date(&self) -> Option<&str>;
}

/// Query quality and lineage metadata from any ISO 19115 object.
///
/// This trait is an **orthogonal concern**: it reports data quality and
/// lineage facts regardless of whether the DQ_DataQuality element or
/// LI_Lineage record is structurally complete.  Even a quality element with no
/// results attached has a scope and can report whether any results are present.
///
/// The trait is `dyn`-compatible.
///
/// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality, DQ_Element; §6.28 —
/// LI_Lineage.
pub trait Iso19115QualityMeta: Send + Sync {
    /// Returns the MD_Scope description for this quality evaluation, or `None`.
    ///
    /// Exercises: DqDataQualityScopeMandatory.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality.scope.
    fn quality_scope(&self) -> Option<&str>;

    /// Returns the count of DQ_Element reports attached to this element.
    ///
    /// Exercises: DqDataQualityReportOptional.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality.report.
    fn quality_report_count(&self) -> usize;

    /// Returns the lineage statement from the associated LI_Lineage, if any.
    ///
    /// Exercises: LiLineageStatementConditional.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Lineage.statement.
    fn lineage_statement(&self) -> Option<&str>;

    /// Returns `true` when every DQ_Element in the quality report has at least
    /// one DQ_Result attached.
    ///
    /// A `false` return indicates that one or more elements violate
    /// `DqElementResultMandatory`.  This method provides a runtime check for
    /// the structural invariant; the corresponding FV proof is expressed via
    /// the `DqElementResultMandatory` contract.
    ///
    /// Exercises: DqElementResultMandatory.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element.result (1..*).
    fn all_elements_have_results(&self) -> bool;

    /// Given a validated `DqElementResultMandatory` proof token, confirm that
    /// the quality descriptor satisfies the result-presence invariant.
    ///
    /// This method bridges the orthogonal reporting concern with the contract
    /// layer: it takes a runtime-established proof and returns a token that
    /// FV harnesses can use to compose further quality validity claims.
    ///
    /// Exercises: DqElementResultMandatory.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element.result (1..*).
    fn prove_elements_have_results(
        &self,
        descriptor: &DataQualityDescriptor,
    ) -> GisResult<Established<DqElementResultMandatory>>;
}
