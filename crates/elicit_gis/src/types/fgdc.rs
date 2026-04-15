//! FGDC CSDGM descriptor and input types.
//!
//! These types are the construction receipts returned by the FGDC factory
//! traits and the input bags accepted by those traits.  Each descriptor carries
//! the data used during construction; all validity claims are expressed
//! separately via [`Established<P>`] proof tokens.
//!
//! Source: FGDC Content Standard for Digital Geospatial Metadata (FGDC-STD-001-1998).
//!
//! [`Established<P>`]: elicitation::Established

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Enums ─────────────────────────────────────────────────────────────────────

/// Horizontal coordinate reference system type — exclusive choice.
///
/// Source: FGDC CSDGM §4 — Spatial_Reference_Information:
/// Horizontal_Coordinate_System_Definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FgdcHorizCrsKind {
    /// Geographic coordinate system.
    ///
    /// Exercises: FgdcHorizCrsTypeIsGeographicPlanarOrLocal.
    Geographic,

    /// Planar coordinate system (map projection or local planar).
    ///
    /// Exercises: FgdcHorizCrsTypeIsGeographicPlanarOrLocal.
    Planar,

    /// Local coordinate system.
    ///
    /// Exercises: FgdcHorizCrsTypeIsGeographicPlanarOrLocal.
    Local,
}

/// PVECT info kind — either SDTS or VPF (exclusive choice).
///
/// Source: FGDC CSDGM §3 — Spatial_Data_Organization_Information:
/// Point_and_Vector_Object_Information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FgdcPvectKind {
    /// SDTS (Spatial Data Transfer Standard) object type.
    ///
    /// Exercises: FgdcPvectInfoIsSdtsOrVpfExclusive.
    Sdts,

    /// VPF (Vector Product Format) object type.
    ///
    /// Exercises: FgdcPvectInfoIsSdtsOrVpfExclusive.
    Vpf,
}

/// Time period type — exclusive choice of single date, multiple dates, or range.
///
/// Source: FGDC CSDGM §9 — Time_Period_Information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FgdcTimePeriodKind {
    /// A single calendar date or date/time.
    ///
    /// Exercises: FgdcTimePeriodTypeExclusive.
    Single(String),

    /// Multiple non-contiguous calendar dates.
    ///
    /// Exercises: FgdcTimePeriodTypeExclusive.
    Multiple(Vec<String>),

    /// A continuous range with explicit begin and end.
    ///
    /// Exercises: FgdcTimePeriodTypeExclusive, FgdcRangeEndingDateAfterBeginning.
    Range {
        /// Beginning calendar date (YYYYMMDD or domain token).
        begin: String,

        /// Ending calendar date — must be after or equal to `begin`.
        end: String,
    },
}

/// Attribute domain kind — exclusive choice.
///
/// Source: FGDC CSDGM §5 — Entity_and_Attribute_Information: Attribute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FgdcDomainKind {
    /// Closed list of acceptable values.
    ///
    /// Exercises: FgdcAttributeDomainTypeExclusive,
    /// FgdcEnumeratedDomainHasAtLeastOneValue.
    Enumerated(Vec<FgdcEnumeratedValue>),

    /// Numeric range with minimum and maximum.
    ///
    /// Exercises: FgdcAttributeDomainTypeExclusive,
    /// FgdcRangeDomainMinimumLeMaximum.
    Range(FgdcRangeDomainInfo),

    /// Reference to an external codeset.
    ///
    /// Exercises: FgdcAttributeDomainTypeExclusive,
    /// FgdcCodesetNamePresent, FgdcCodesetSourcePresent.
    Codeset {
        /// Name of the codeset.
        name: String,
        /// Source authority for the codeset.
        source: String,
    },

    /// Domain cannot be represented using the other options.
    ///
    /// Exercises: FgdcAttributeDomainTypeExclusive.
    Unrepresented(String),
}

// ── Small element types ───────────────────────────────────────────────────────

/// A theme, place, stratum, or temporal keyword group.
///
/// Used as input to identification factory methods and as a field in
/// [`FgdcIdentificationDescriptor`].
///
/// Source: FGDC CSDGM §1.6 — Theme_Keyword (and analogous keyword sections).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcKeywordGroup {
    /// At least one non-empty keyword (validated inside the factory).
    ///
    /// Exercises: FgdcThemeHasAtLeastOneKeyword (or Place/Stratum/Temporal variant).
    pub keywords: Vec<String>,

    /// Formal thesaurus from which the keywords are drawn.
    ///
    /// Exercises: FgdcThemeHasKeywordThesaurus (or variant).
    pub thesaurus: String,
}

/// A single process step within Lineage.
///
/// Used as input to [`FgdcDataQualityFactory`] and as a field in
/// [`FgdcDataQualityDescriptor`].
///
/// Source: FGDC CSDGM §2.5 — Process_Step.
///
/// [`FgdcDataQualityFactory`]: crate::FgdcDataQualityFactory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcProcessStepInfo {
    /// Free-text process description (mandatory, non-empty).
    ///
    /// Exercises: FgdcProcessStepDescriptionPresent.
    pub description: String,

    /// Date the process was completed (optional, YYYYMMDD or token).
    ///
    /// Exercises: FgdcProcessStepDateFgdcFormat (when present).
    pub date: Option<String>,
}

/// A data source within Lineage.
///
/// Used as input to [`FgdcDataQualityFactory`] and as a field in
/// [`FgdcDataQualityDescriptor`].
///
/// Source: FGDC CSDGM §2.5 — Source_Information.
///
/// [`FgdcDataQualityFactory`]: crate::FgdcDataQualityFactory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcSourceInfo {
    /// Unique abbreviated citation for the source (mandatory).
    ///
    /// Exercises: FgdcSourceCitationAbbreviationPresent.
    pub citation_abbreviation: String,

    /// Explanation of the contribution of the source (optional).
    ///
    /// Exercises: FgdcSourceContributionPresent (when used).
    pub contribution: Option<String>,

    /// Storage medium of the source (mandatory).
    ///
    /// Exercises: FgdcSourceMediaTypePresent.
    pub media_type: String,
}

/// A single enumerated domain value.
///
/// Source: FGDC CSDGM §5.1 — Enumerated_Domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcEnumeratedValue {
    /// The enumerated attribute value (mandatory).
    ///
    /// Exercises: FgdcEnumeratedDomainValuePresent.
    pub value: String,

    /// Definition of the attribute value (mandatory).
    ///
    /// Exercises: FgdcEnumeratedDomainValueDefinitionPresent.
    pub definition: String,

    /// Source of the definition (optional).
    ///
    /// Exercises: FgdcEnumeratedDomainValueDefinitionSourcePresent (when present).
    pub definition_source: Option<String>,
}

/// Range domain minimum and maximum values.
///
/// Source: FGDC CSDGM §5.1 — Range_Domain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcRangeDomainInfo {
    /// Least possible value (mandatory).
    ///
    /// Exercises: FgdcRangeDomainMinimumPresent.
    pub minimum: String,

    /// Greatest possible value — must be ≥ minimum (mandatory).
    ///
    /// Exercises: FgdcRangeDomainMaximumPresent, FgdcRangeDomainMinimumLeMaximum.
    pub maximum: String,

    /// Unit of measure for the range (optional).
    pub units: Option<String>,
}

/// A single attribute within an Entity_Type.
///
/// Used as input to [`FgdcEntityAttrFactory`] and embedded in
/// [`FgdcEntityAttrDescriptor`].
///
/// Source: FGDC CSDGM §5 — Attribute.
///
/// [`FgdcEntityAttrFactory`]: crate::FgdcEntityAttrFactory
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcAttributeDescriptor {
    /// Name of the attribute (mandatory).
    ///
    /// Exercises: FgdcAttributeLabelPresent.
    pub label: String,

    /// Explanation of the meaning of the attribute (mandatory).
    ///
    /// Exercises: FgdcAttributeDefinitionPresent.
    pub definition: String,

    /// Reference to the source of the definition (mandatory).
    ///
    /// Exercises: FgdcAttributeDefinitionSourcePresent.
    pub definition_source: String,

    /// Domain of the attribute — exactly one kind (mandatory).
    ///
    /// Exercises: FgdcAttributeHasAtLeastOneDomain,
    /// FgdcAttributeDomainTypeExclusive.
    pub domain: FgdcDomainKind,

    /// Smallest difference between values (optional, must be > 0 if present).
    ///
    /// Exercises: FgdcAttributeMeasurementResolutionPositive (when present).
    pub measurement_resolution: Option<f64>,

    /// Unit of measurement for the resolution (optional).
    pub measurement_units: Option<String>,
}

// ── Factory output descriptors ────────────────────────────────────────────────

/// Descriptor for an FGDC §8 Citation_Information.
///
/// Produced by [`FgdcCitationFactory::build_citation`].
///
/// Source: FGDC CSDGM §8 — Citation_Information.
///
/// [`FgdcCitationFactory::build_citation`]: crate::FgdcCitationFactory::build_citation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcCitationDescriptor {
    /// One or more names of the responsible parties (mandatory, non-empty).
    ///
    /// Exercises: FgdcCitationHasAtLeastOneOriginator, FgdcCitationOriginatorNonEmpty.
    pub originators: Vec<String>,

    /// Date of publication (mandatory, YYYYMMDD | "Unknown" | "Unpublished material").
    ///
    /// Exercises: FgdcCitationPublicationDatePresent,
    /// FgdcCitationPublicationDateIsYyyymmddOrToken.
    pub pub_date: String,

    /// Title of the dataset (mandatory, non-empty).
    ///
    /// Exercises: FgdcCitationTitlePresent, FgdcCitationTitleNonEmpty.
    pub title: String,

    /// Place of publication (optional).
    pub pub_place: Option<String>,

    /// Name of the publisher (optional).
    pub publisher: Option<String>,

    /// Additional citation details (optional).
    pub other_citation_details: Option<String>,
}

/// Descriptor for an FGDC §9 Time_Period_Information.
///
/// Produced by [`FgdcTimePeriodFactory::build_time_period`].
///
/// Source: FGDC CSDGM §9 — Time_Period_Information.
///
/// [`FgdcTimePeriodFactory::build_time_period`]: crate::FgdcTimePeriodFactory::build_time_period
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcTimePeriodDescriptor {
    /// Exclusive time period kind.
    ///
    /// Exercises: FgdcTimePeriodTypeExclusive.
    pub kind: FgdcTimePeriodKind,
}

/// Descriptor for an FGDC §10 Contact_Information.
///
/// Produced by [`FgdcContactFactory::build_contact`].
///
/// Source: FGDC CSDGM §10 — Contact_Information.
///
/// [`FgdcContactFactory::build_contact`]: crate::FgdcContactFactory::build_contact
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcContactDescriptor {
    /// Primary contact person name (mandatory when `organization` is `None`).
    ///
    /// Exercises: FgdcContactHasPersonOrOrganizationPrimary (when present).
    pub person: Option<String>,

    /// Primary contact organization name (mandatory when `person` is `None`).
    ///
    /// Exercises: FgdcContactHasPersonOrOrganizationPrimary (when present).
    pub organization: Option<String>,

    /// Position name of the individual (optional).
    pub position: Option<String>,

    /// Type of address (e.g. `"mailing"`, `"physical"`, `"mailing and physical"`).
    ///
    /// Exercises: FgdcContactAddressTypeCodeValid.
    pub address_type: String,

    /// Street address (mandatory within each Contact_Address).
    pub address: String,

    /// City (mandatory).
    ///
    /// Exercises: FgdcContactAddressCityPresent.
    pub city: String,

    /// State or province (mandatory).
    ///
    /// Exercises: FgdcContactAddressStatePresent.
    pub state: String,

    /// Postal code (mandatory).
    ///
    /// Exercises: FgdcContactAddressPostalCodePresent.
    pub postal_code: String,

    /// Country (optional).
    pub country: Option<String>,

    /// Primary voice telephone number (mandatory).
    ///
    /// Exercises: FgdcContactHasAtLeastOneVoiceTelephone.
    pub phone: String,

    /// Facsimile telephone number (optional).
    pub fax: Option<String>,

    /// Electronic mail address (optional).
    pub email: Option<String>,
}

/// Descriptor for an FGDC §1 Identification_Information.
///
/// Produced by [`FgdcIdentificationFactory::build_identification`].
///
/// Source: FGDC CSDGM §1 — Identification_Information.
///
/// [`FgdcIdentificationFactory::build_identification`]:
///     crate::FgdcIdentificationFactory::build_identification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcIdentificationDescriptor {
    /// Citation for the dataset (mandatory).
    pub citation: FgdcCitationDescriptor,

    /// Brief narrative summary of the dataset (mandatory, non-empty).
    ///
    /// Exercises: FgdcDescriptionAbstractPresent.
    pub abstract_text: String,

    /// Summary of the purpose of the dataset (mandatory, non-empty).
    ///
    /// Exercises: FgdcDescriptionPurposePresent.
    pub purpose: String,

    /// Time period of content (mandatory).
    pub time_of_content: FgdcTimePeriodDescriptor,

    /// Status progress code (mandatory).
    ///
    /// Exercises: FgdcStatusProgressCodeValid.
    pub status_progress: String,

    /// Maintenance and update frequency code (mandatory).
    ///
    /// Exercises: FgdcStatusUpdateFrequencyCodeValid.
    pub update_frequency: String,

    /// Western-most longitude in decimal degrees (mandatory).
    ///
    /// Exercises: FgdcBoundingWestCoordInRange.
    pub west: f64,

    /// Eastern-most longitude in decimal degrees (mandatory).
    ///
    /// Exercises: FgdcBoundingEastCoordInRange.
    pub east: f64,

    /// Southern-most latitude in decimal degrees (mandatory).
    ///
    /// Exercises: FgdcBoundingSouthCoordInRange.
    pub south: f64,

    /// Northern-most latitude in decimal degrees (mandatory).
    ///
    /// Exercises: FgdcBoundingNorthCoordInRange, FgdcBoundingNorthGeqSouth.
    pub north: f64,

    /// Theme keyword groups — at least one required (mandatory).
    ///
    /// Exercises: FgdcKeywordsHasAtLeastOneTheme.
    pub theme_keywords: Vec<FgdcKeywordGroup>,

    /// Place keyword groups (optional).
    pub place_keywords: Vec<FgdcKeywordGroup>,

    /// Stratum keyword groups (optional).
    pub stratum_keywords: Vec<FgdcKeywordGroup>,

    /// Temporal keyword groups (optional).
    pub temporal_keywords: Vec<FgdcKeywordGroup>,

    /// Constraints on access to the dataset (mandatory).
    ///
    /// Exercises: FgdcAccessConstraintsPresent.
    pub access_constraints: String,

    /// Constraints on use of the dataset (mandatory).
    ///
    /// Exercises: FgdcUseConstraintsPresent.
    pub use_constraints: String,

    /// Percent of the applicable area covered by clouds (optional).
    ///
    /// Exercises: FgdcCloudCoverZeroToHundred (when present).
    pub cloud_cover: Option<String>,

    /// File name of a graphic depicting the dataset (optional).
    ///
    /// Exercises: FgdcBrowseGraphicFileNamePresent (when present).
    pub browse_file_name: Option<String>,

    /// Description of the browse graphic (optional, required when browse present).
    ///
    /// Exercises: FgdcBrowseGraphicFileDescriptionPresent (when present).
    pub browse_file_description: Option<String>,

    /// File type of the browse graphic (optional, required when browse present).
    ///
    /// Exercises: FgdcBrowseGraphicFileTypePresent (when present).
    pub browse_file_type: Option<String>,

    /// Security classification code (optional).
    ///
    /// Exercises: FgdcSecurityClassificationCodeValid (when present).
    pub security_classification: Option<String>,

    /// Name of the classification system (required when classification present).
    ///
    /// Exercises: FgdcSecurityClassificationSystemPresent (when present).
    pub security_classification_system: Option<String>,

    /// Handling description (required when classification present).
    ///
    /// Exercises: FgdcSecurityHandlingDescriptionPresent (when present).
    pub security_handling_description: Option<String>,
}

/// Descriptor for an FGDC §2 Data_Quality_Information.
///
/// Produced by [`FgdcDataQualityFactory::build_data_quality`].
///
/// Source: FGDC CSDGM §2 — Data_Quality_Information.
///
/// [`FgdcDataQualityFactory::build_data_quality`]:
///     crate::FgdcDataQualityFactory::build_data_quality
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcDataQualityDescriptor {
    /// Explanation of how consistent the relationships among the data elements
    /// are (mandatory).
    ///
    /// Exercises: FgdcDataQualityLogicalConsistencyPresent.
    pub logical_consistency: String,

    /// Assessment of the completeness of the data coverage (mandatory).
    ///
    /// Exercises: FgdcDataQualityCompletenessPresent.
    pub completeness: String,

    /// General statement about the lineage of the dataset (optional).
    pub lineage_statement: Option<String>,

    /// Process steps in the lineage — at least one required.
    ///
    /// Exercises: FgdcLineageHasAtLeastOneProcessStep.
    pub process_steps: Vec<FgdcProcessStepInfo>,

    /// Sources cited in the lineage (optional).
    pub sources: Vec<FgdcSourceInfo>,
}

/// Descriptor for an FGDC §3 Spatial_Data_Organization_Information.
///
/// Produced by [`FgdcSpatialOrgFactory::build_spatial_org`].
///
/// Source: FGDC CSDGM §3 — Spatial_Data_Organization_Information.
///
/// [`FgdcSpatialOrgFactory::build_spatial_org`]:
///     crate::FgdcSpatialOrgFactory::build_spatial_org
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcSpatialOrgDescriptor {
    /// Direct spatial reference method — "Point", "Vector", or "Raster".
    ///
    /// Exercises: FgdcDirectSpatialRefMethodCodeValid.
    pub direct_ref_method: String,

    /// SDTS or VPF choice when direct_ref_method is "Vector" (mandatory for vector).
    ///
    /// Exercises: FgdcPvectInfoIsSdtsOrVpfExclusive.
    pub pvect_kind: Option<FgdcPvectKind>,

    /// SDTS spatial object type (present when pvect_kind is Sdts).
    ///
    /// Exercises: FgdcSdtsObjectTypeCodeValid.
    pub sdts_type: Option<String>,

    /// SDTS object count (present when pvect_kind is Sdts, must be > 0).
    ///
    /// Exercises: FgdcSdtsObjectCountPositive.
    pub sdts_object_count: Option<u64>,

    /// VPF spatial object type (present when pvect_kind is Vpf).
    ///
    /// Exercises: FgdcVpfObjectTypeCodeValid.
    pub vpf_type: Option<String>,

    /// VPF topology level (present when pvect_kind is Vpf, 0–3).
    ///
    /// Exercises: FgdcVpfTopologyLevelZeroToThree.
    pub vpf_topology_level: Option<u8>,

    /// Raster object type (present when direct_ref_method is "Raster").
    ///
    /// Exercises: FgdcRasterObjectTypeCodeValid.
    pub raster_type: Option<String>,

    /// Raster row count (present when raster, must be > 0).
    ///
    /// Exercises: FgdcRasterRowCountPositive.
    pub raster_rows: Option<u64>,

    /// Raster column count (present when raster, must be > 0).
    ///
    /// Exercises: FgdcRasterColumnCountPositive.
    pub raster_cols: Option<u64>,

    /// Raster vertical count (present when raster has Z, must be > 0).
    ///
    /// Exercises: FgdcRasterVerticalCountPositive.
    pub raster_vertical: Option<u64>,
}

/// Descriptor for an FGDC §4 Spatial_Reference_Information.
///
/// Produced by [`FgdcSpatialRefFactory::build_spatial_ref`].
///
/// Source: FGDC CSDGM §4 — Spatial_Reference_Information.
///
/// [`FgdcSpatialRefFactory::build_spatial_ref`]:
///     crate::FgdcSpatialRefFactory::build_spatial_ref
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcSpatialRefDescriptor {
    /// Exclusive horizontal CRS type.
    ///
    /// Exercises: FgdcHorizCrsTypeIsGeographicPlanarOrLocal.
    pub horiz_crs_kind: FgdcHorizCrsKind,

    /// Geographic latitude resolution in decimal degrees (present when Geographic).
    ///
    /// Exercises: FgdcGeographicLatResolutionPositive.
    pub geographic_lat_res: Option<f64>,

    /// Geographic longitude resolution in decimal degrees (present when Geographic).
    ///
    /// Exercises: FgdcGeographicLonResolutionPositive.
    pub geographic_lon_res: Option<f64>,

    /// Geographic coordinate units code (present when Geographic).
    ///
    /// Exercises: FgdcGeographicCoordUnitsCodeValid.
    pub geographic_coord_units: Option<String>,

    /// Map projection name (present when Planar/map-projection).
    ///
    /// Exercises: FgdcMapProjNamePresent.
    pub map_proj_name: Option<String>,

    /// Geodetic reference ellipsoid name (optional).
    ///
    /// Exercises: FgdcGeodeticModelEllipsoidNamePresent.
    pub geodetic_ellipsoid_name: Option<String>,

    /// Semi-major axis of the geodetic ellipsoid in meters (optional, > 0).
    ///
    /// Exercises: FgdcGeodeticModelSemiMajorAxisPositive.
    pub geodetic_semi_major_axis: Option<f64>,

    /// Denominator of the inverse flattening ratio (optional, > 0).
    ///
    /// Exercises: FgdcGeodeticModelFlatteningRatioDenominatorPositive.
    pub geodetic_inv_flattening: Option<f64>,

    /// Altitude datum name (optional).
    ///
    /// Exercises: FgdcAltitudeDatumNamePresent.
    pub altitude_datum_name: Option<String>,

    /// Altitude measurement resolution (optional, ≥ 1.0).
    ///
    /// Exercises: FgdcAltitudeResolutionAtLeastOne.
    pub altitude_res: Option<f64>,

    /// Altitude distance units (optional).
    ///
    /// Exercises: FgdcAltitudeDistanceUnitsPresent.
    pub altitude_distance_units: Option<String>,

    /// Altitude encoding method (optional).
    ///
    /// Exercises: FgdcAltitudeEncodingMethodPresent.
    pub altitude_encoding_method: Option<String>,
}

/// Descriptor for an FGDC §5 Entity_Type with its attributes.
///
/// Produced by [`FgdcEntityAttrFactory::build_entity`].
///
/// Source: FGDC CSDGM §5 — Entity_and_Attribute_Information.
///
/// [`FgdcEntityAttrFactory::build_entity`]: crate::FgdcEntityAttrFactory::build_entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcEntityAttrDescriptor {
    /// Name of the entity type (mandatory).
    ///
    /// Exercises: FgdcEntityTypeLabelPresent.
    pub entity_label: String,

    /// Definition of the entity type (mandatory).
    ///
    /// Exercises: FgdcEntityTypeDefinitionPresent.
    pub entity_definition: String,

    /// Source of the entity type definition (mandatory).
    ///
    /// Exercises: FgdcEntityTypeDefinitionSourcePresent.
    pub entity_definition_source: String,

    /// Attributes of the entity type (may be empty for overview-only sections).
    pub attributes: Vec<FgdcAttributeDescriptor>,
}

/// Descriptor for an FGDC §6 Distribution_Information.
///
/// Produced by [`FgdcDistributionFactory::build_distribution`].
///
/// Source: FGDC CSDGM §6 — Distribution_Information.
///
/// [`FgdcDistributionFactory::build_distribution`]:
///     crate::FgdcDistributionFactory::build_distribution
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcDistributionDescriptor {
    /// Name of the distributor individual or organization (mandatory).
    ///
    /// Exercises: FgdcDistributionDistributorPresent.
    pub distributor_name: String,

    /// Statement about liability for dataset use (mandatory).
    ///
    /// Exercises: FgdcDistributionLiabilityPresent.
    pub liability: String,

    /// Digital format name (optional; exclusive with non-digital).
    ///
    /// Exercises: FgdcDigitalFormatNamePresent,
    /// FgdcStandardOrderHasFormNondigitalOrDigital.
    pub format_name: Option<String>,

    /// Transfer size in megabytes (optional, > 0 when present).
    ///
    /// Exercises: FgdcTransferSizePositive.
    pub transfer_size: Option<f64>,

    /// Fees for obtaining the dataset (mandatory).
    ///
    /// Exercises: FgdcStandardOrderFeesPresent.
    pub fees: String,

    /// Network address for online access (optional).
    pub network_resource: Option<String>,

    /// Offline storage medium code (optional).
    ///
    /// Exercises: FgdcOfflineMediaCodeValid.
    pub offline_media: Option<String>,

    /// Recording density (optional, > 0 when present).
    ///
    /// Exercises: FgdcRecordingDensityPositive.
    pub recording_density: Option<f64>,

    /// Recording format code (optional).
    ///
    /// Exercises: FgdcRecordingFormatCodeValid.
    pub recording_format: Option<String>,
}

/// Descriptor for an FGDC §7 Metadata_Reference_Information.
///
/// Produced by [`FgdcMetadataRefFactory::build_metadata_ref`].
///
/// Source: FGDC CSDGM §7 — Metadata_Reference_Information.
///
/// [`FgdcMetadataRefFactory::build_metadata_ref`]:
///     crate::FgdcMetadataRefFactory::build_metadata_ref
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcMetadataRefDescriptor {
    /// Date the metadata were created or last updated (mandatory, YYYYMMDD).
    ///
    /// Exercises: FgdcMetadataDatePresent, FgdcCalendarDateIsYyyymmddOrToken.
    pub date: String,

    /// Name of the metadata contact person or organization (mandatory).
    ///
    /// Exercises: FgdcMetadataContactPresent.
    pub contact_name: String,

    /// Name of the metadata standard used (mandatory).
    ///
    /// Exercises: FgdcMetadataStandardNamePresent.
    pub standard_name: String,

    /// Version of the metadata standard used (mandatory).
    ///
    /// Exercises: FgdcMetadataStandardVersionPresent.
    pub standard_version: String,

    /// Date of the last review of the metadata (optional, must be after `date`).
    ///
    /// Exercises: FgdcMetadataReviewDateAfterMetadataDate.
    pub review_date: Option<String>,

    /// Date of the next scheduled review (optional, must be after `review_date`).
    ///
    /// Exercises: FgdcMetadataFutureReviewDateAfterReviewDate.
    pub future_review_date: Option<String>,

    /// Metadata security classification code (optional).
    ///
    /// Exercises: FgdcMetadataSecurityClassificationCodeValid.
    pub security_classification: Option<String>,

    /// Calendar/time convention code (optional).
    ///
    /// Exercises: FgdcMetadataTimeConventionCodeValid.
    pub time_convention: Option<String>,
}

/// Descriptor for a complete FGDC §0 Metadata root record.
///
/// Produced by [`FgdcRecordFactory::build_record`].
///
/// Source: FGDC CSDGM §0 — Metadata.
///
/// [`FgdcRecordFactory::build_record`]: crate::FgdcRecordFactory::build_record
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FgdcRecordDescriptor {
    /// Identification section (mandatory).
    pub identification: FgdcIdentificationDescriptor,

    /// Data quality section (optional).
    pub data_quality: Option<FgdcDataQualityDescriptor>,

    /// Spatial data organization section (optional).
    pub spatial_org: Option<FgdcSpatialOrgDescriptor>,

    /// Spatial reference section (optional).
    pub spatial_ref: Option<FgdcSpatialRefDescriptor>,

    /// Entity and attribute sections (optional, repeatable).
    pub entity_attr: Vec<FgdcEntityAttrDescriptor>,

    /// Distribution sections (optional, repeatable).
    pub distribution: Vec<FgdcDistributionDescriptor>,

    /// Metadata reference section (mandatory).
    pub metadata_ref: FgdcMetadataRefDescriptor,
}
