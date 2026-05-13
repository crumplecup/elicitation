//! FGDC CSDGM construction, validation, reporting, and backend traits.
//!
//! Three-role taxonomy applied to FGDC CSDGM:
//!
//! ## Role 1a — Tier 1 leaf validators (FV targets)
//!
//! Each leaf validator asserts a single, independently-testable CSDGM
//! invariant.  Kani/Creusot harnesses drive these directly; each method
//! returns one `Established<P>` token that can be composed individually.
//!
//! | Trait | Validates |
//! |---|---|
//! | [`FgdcBoundingValidator`] | Bounding coordinate ranges and ordering |
//! | [`FgdcDateValidator`] | FGDC-format dates and relative ordering |
//! | [`FgdcStatusValidator`] | Progress, update-frequency, and time-convention codes |
//! | [`FgdcSpatialParamValidator`] | Direct ref method, SDTS/VPF/raster codes and counts |
//! | [`FgdcMapProjValidator`] | Map projection numeric parameters and zone codes |
//! | [`FgdcVertRefValidator`] | Altitude and depth resolution constraints |
//! | [`FgdcEntityAttrValidator`] | Range-domain ordering and measurement resolution |
//! | [`FgdcDistributionParamValidator`] | Dialup BPS ordering, bits, parity, density codes |
//! | [`FgdcSecurityValidator`] | Classification codes and cloud-cover range |
//! | [`FgdcContactValidator`] | Contact address-type code |
//!
//! ## Role 1b — Tier 2 section factories (compositionality)
//!
//! Each section factory takes `Established<P>` tokens from already-validated
//! sub-sections (the "proof preconditions"), validates any leaf invariants that
//! are internal to its own section, and emits a section-level
//! `Established<SectionValid>` aggregate token together with a descriptor.
//! The factory signature *encodes* the FGDC inter-section dependency graph.
//!
//! | Trait | Builds | Preconditions (tokens) | Postcondition |
//! |---|---|---|---|
//! | [`FgdcCitationFactory`] | §8 Citation | — | [`FgdcCitationInfoValid`] |
//! | [`FgdcTimePeriodFactory`] | §9 Time Period | — | [`FgdcTimePeriodInfoValid`] |
//! | [`FgdcContactFactory`] | §10 Contact | — | [`FgdcContactInfoValid`] |
//! | [`FgdcIdentificationFactory`] | §1 Identification | [`FgdcCitationInfoValid`] + [`FgdcTimePeriodInfoValid`] | [`FgdcIdentificationSectionValid`] |
//! | [`FgdcDataQualityFactory`] | §2 Data Quality | — | [`FgdcDataQualitySectionValid`] |
//! | [`FgdcSpatialOrgFactory`] | §3 Spatial Data Organization | — | [`FgdcSpatialDataOrgSectionValid`] |
//! | [`FgdcSpatialRefFactory`] | §4 Spatial Reference | — | [`FgdcSpatialReferenceSectionValid`] |
//! | [`FgdcEntityAttrFactory`] | §5 Entity & Attribute | — | [`FgdcEntityAttributeSectionValid`] |
//! | [`FgdcDistributionFactory`] | §6 Distribution | [`FgdcContactInfoValid`] | [`FgdcDistributionSectionValid`] |
//! | [`FgdcMetadataRefFactory`] | §7 Metadata Reference | [`FgdcContactInfoValid`] | [`FgdcMetadataReferenceSectionValid`] |
//! | [`FgdcRecordFactory`] | §0 Record | [`FgdcIdentificationSectionValid`] + [`FgdcMetadataReferenceSectionValid`] | [`FgdcRecordValid`] |
//!
//! ## Role 2 — Orthogonal concern traits (pure inspection)
//!
//! These traits inspect content that is always present, independent of whether
//! any validity token has been established.  A record can report its bounding
//! box even before the bounding constraints have been checked; keyword lists and
//! contact data can be read at any time.
//!
//! | Trait | Reports |
//! |---|---|
//! | [`FgdcBoundingMeta`] | West/east/south/north, has_g_polygon |
//! | [`FgdcKeywordMeta`] | Theme/place/stratum/temporal keyword lists |
//! | [`FgdcContactMeta`] | Person name, org name, phone, city, state |
//! | [`FgdcDistributionMeta`] | Distributor name, liability, format name, transfer size |
//! | [`FgdcMetadataMeta`] | Metadata date, standard name/version, review dates |
//!
//! ## Role 3 — Abstraction supertrait
//!
//! [`FgdcBackend`] composes all 26 sub-traits above and is incorporated
//! into [`crate::GisBackend`], hiding the interplay of FGDC §0–§10 behind a
//! single coherent interface.
//!
//! Source: Federal Geographic Data Committee, FGDC-STD-001-1998.
//!
//! [`FgdcCitationInfoValid`]: crate::FgdcCitationInfoValid
//! [`FgdcTimePeriodInfoValid`]: crate::FgdcTimePeriodInfoValid
//! [`FgdcContactInfoValid`]: crate::FgdcContactInfoValid
//! [`FgdcIdentificationSectionValid`]: crate::FgdcIdentificationSectionValid
//! [`FgdcDataQualitySectionValid`]: crate::FgdcDataQualitySectionValid
//! [`FgdcSpatialDataOrgSectionValid`]: crate::FgdcSpatialDataOrgSectionValid
//! [`FgdcSpatialReferenceSectionValid`]: crate::FgdcSpatialReferenceSectionValid
//! [`FgdcEntityAttributeSectionValid`]: crate::FgdcEntityAttributeSectionValid
//! [`FgdcDistributionSectionValid`]: crate::FgdcDistributionSectionValid
//! [`FgdcMetadataReferenceSectionValid`]: crate::FgdcMetadataReferenceSectionValid
//! [`FgdcRecordValid`]: crate::FgdcRecordValid

use elicitation::Established;

use crate::{
    FgdcAltitudeResolutionAtLeastOne, FgdcArcZoneIdentifier1To18,
    FgdcAttributeMeasurementResolutionPositive, FgdcBoundingEastCoordInRange,
    FgdcBoundingNorthCoordInRange, FgdcBoundingNorthGeqSouth, FgdcBoundingSouthCoordInRange,
    FgdcBoundingWestCoordInRange, FgdcCalendarDateIsYyyymmddOrToken, FgdcCitationDescriptor,
    FgdcCitationInfoValid, FgdcCitationPublicationDateIsYyyymmddOrToken,
    FgdcContactAddressTypeCodeValid, FgdcContactDescriptor, FgdcContactInfoValid,
    FgdcDataQualityDescriptor, FgdcDataQualitySectionValid, FgdcDepthResolutionAtLeastOne,
    FgdcDialupDataBitsSevenOrEight, FgdcDialupHighestBpsGtLowest, FgdcDialupLowestBpsGeq110,
    FgdcDialupParityCodeValid, FgdcDialupStopBitsOneOrTwo, FgdcDirectSpatialRefMethodCodeValid,
    FgdcDistributionDescriptor, FgdcDistributionSectionValid, FgdcEntityAttrDescriptor,
    FgdcEntityAttributeSectionValid, FgdcGPolygonOuterRingHasAtLeastFourPoints,
    FgdcGRingLatitudeInRange, FgdcGRingLongitudeInRange, FgdcGeographicCoordUnitsCodeValid,
    FgdcGeographicLatResolutionPositive, FgdcGeographicLonResolutionPositive,
    FgdcIdentificationDescriptor, FgdcIdentificationSectionValid, FgdcKeywordGroup,
    FgdcMapProjAzimuthalAngle0To360, FgdcMapProjCentralMeridianInRange,
    FgdcMapProjLatitudeOriginInRange, FgdcMapProjScaleFactorPositive,
    FgdcMapProjStandardParallelInRange, FgdcMetadataRefDescriptor,
    FgdcMetadataReferenceSectionValid, FgdcMetadataReviewDateAfterMetadataDate,
    FgdcMetadataSecurityClassificationCodeValid, FgdcMetadataTimeConventionCodeValid,
    FgdcOfflineMediaCodeValid, FgdcProcessStepDateFgdcFormat, FgdcProcessStepInfo,
    FgdcRangeDomainMinimumLeMaximum, FgdcRangeEndingDateAfterBeginning,
    FgdcRasterColumnCountPositive, FgdcRasterObjectTypeCodeValid, FgdcRasterRowCountPositive,
    FgdcRasterVerticalCountPositive, FgdcRecordDescriptor, FgdcRecordValid,
    FgdcRecordingDensityPositive, FgdcRecordingFormatCodeValid, FgdcSdtsObjectCountPositive,
    FgdcSdtsObjectTypeCodeValid, FgdcSecurityClassificationCodeValid, FgdcSourceInfo,
    FgdcSpatialDataOrgSectionValid, FgdcSpatialOrgDescriptor, FgdcSpatialRefDescriptor,
    FgdcSpatialReferenceSectionValid, FgdcStatusProgressCodeValid,
    FgdcStatusUpdateFrequencyCodeValid, FgdcTimePeriodDescriptor, FgdcTimePeriodInfoValid,
    FgdcTimePeriodKind, FgdcUpsZoneIdentifierCodeValid, FgdcUtmZoneNumberInRange,
    FgdcVpfObjectTypeCodeValid, FgdcVpfTopologyLevelZeroToThree, GisResult,
};

// ── Role 1a: Tier 1 leaf validators ──────────────────────────────────────────

/// Validate individual FGDC bounding coordinate constraints.
///
/// All five methods are independent FV targets.  The north ≥ south ordering
/// invariant (`validate_bounding_north_geq_south`) is a standalone leaf
/// because it is only meaningful once both range checks have already been
/// composed; FV harnesses call all five methods and compose the tokens.
///
/// Source: FGDC CSDGM §1.5.1 — Bounding Coordinates.
pub trait FgdcBoundingValidator: Send + Sync {
    /// Assert West_Bounding_Coordinate ∈ [-180.0, 180.0).
    ///
    /// Validates: `FgdcBoundingWestCoordInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.1.1.
    fn validate_bounding_west(
        &self,
        west: f64,
    ) -> GisResult<Established<FgdcBoundingWestCoordInRange>>;

    /// Assert East_Bounding_Coordinate ∈ [-180.0, 180.0].
    ///
    /// Validates: `FgdcBoundingEastCoordInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.1.2.
    fn validate_bounding_east(
        &self,
        east: f64,
    ) -> GisResult<Established<FgdcBoundingEastCoordInRange>>;

    /// Assert North_Bounding_Coordinate ∈ [-90.0, 90.0].
    ///
    /// Validates: `FgdcBoundingNorthCoordInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.1.3.
    fn validate_bounding_north(
        &self,
        north: f64,
    ) -> GisResult<Established<FgdcBoundingNorthCoordInRange>>;

    /// Assert South_Bounding_Coordinate ∈ [-90.0, 90.0].
    ///
    /// Validates: `FgdcBoundingSouthCoordInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.1.4.
    fn validate_bounding_south(
        &self,
        south: f64,
    ) -> GisResult<Established<FgdcBoundingSouthCoordInRange>>;

    /// Assert North_Bounding_Coordinate ≥ South_Bounding_Coordinate.
    ///
    /// This invariant requires both coordinates to be in their valid ranges
    /// before the ordering check is meaningful.  FV harnesses compose
    /// `Established<FgdcBoundingNorthCoordInRange>` and
    /// `Established<FgdcBoundingSouthCoordInRange>` before calling this method.
    ///
    /// Validates: `FgdcBoundingNorthGeqSouth`.
    ///
    /// Source: FGDC CSDGM §1.5.1.3.
    fn validate_bounding_north_geq_south(
        &self,
        south: f64,
        north: f64,
    ) -> GisResult<Established<FgdcBoundingNorthGeqSouth>>;

    /// Assert G-Ring_Latitude ∈ [-90.0, 90.0].
    ///
    /// Validates: `FgdcGRingLatitudeInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.2.1.1.1.
    fn validate_g_ring_latitude(
        &self,
        lat: f64,
    ) -> GisResult<Established<FgdcGRingLatitudeInRange>>;

    /// Assert G-Ring_Longitude ∈ [-180.0, 180.0).
    ///
    /// Validates: `FgdcGRingLongitudeInRange`.
    ///
    /// Source: FGDC CSDGM §1.5.2.1.1.2.
    fn validate_g_ring_longitude(
        &self,
        lon: f64,
    ) -> GisResult<Established<FgdcGRingLongitudeInRange>>;

    /// Assert a G-Polygon outer ring has at least four G-Ring points.
    ///
    /// Validates: `FgdcGPolygonOuterRingHasAtLeastFourPoints`.
    ///
    /// Source: FGDC CSDGM §1.5.2.1.
    fn validate_g_polygon_outer_ring_count(
        &self,
        count: usize,
    ) -> GisResult<Established<FgdcGPolygonOuterRingHasAtLeastFourPoints>>;
}

/// Validate FGDC-format date values and chronological ordering constraints.
///
/// Source: FGDC CSDGM §9 and various mandatory date elements.
pub trait FgdcDateValidator: Send + Sync {
    /// Assert a calendar date string is YYYYMMDD, "Unknown", or an FGDC token.
    ///
    /// Validates: `FgdcCalendarDateIsYyyymmddOrToken`.
    ///
    /// Source: FGDC CSDGM §9.1.1 — Single_Date/Time.Calendar_Date.
    fn validate_calendar_date(
        &self,
        date: &str,
    ) -> GisResult<Established<FgdcCalendarDateIsYyyymmddOrToken>>;

    /// Assert a Publication_Date is YYYYMMDD, "Unknown", or
    /// "Unpublished material".
    ///
    /// Validates: `FgdcCitationPublicationDateIsYyyymmddOrToken`.
    ///
    /// Source: FGDC CSDGM §8.2.
    fn validate_publication_date(
        &self,
        date: &str,
    ) -> GisResult<Established<FgdcCitationPublicationDateIsYyyymmddOrToken>>;

    /// Assert a time-range end date is chronologically after its begin date
    /// (unless either is the "Unknown" token).
    ///
    /// Validates: `FgdcRangeEndingDateAfterBeginning`.
    ///
    /// Source: FGDC CSDGM §9.3.
    fn validate_range_end_after_begin(
        &self,
        begin: &str,
        end: &str,
    ) -> GisResult<Established<FgdcRangeEndingDateAfterBeginning>>;

    /// Assert Metadata_Review_Date is later than Metadata_Date.
    ///
    /// Validates: `FgdcMetadataReviewDateAfterMetadataDate`.
    ///
    /// Source: FGDC CSDGM §7.2.
    fn validate_metadata_review_after_date(
        &self,
        metadata_date: &str,
        review_date: &str,
    ) -> GisResult<Established<FgdcMetadataReviewDateAfterMetadataDate>>;

    /// Assert Metadata_Future_Review_Date is later than Metadata_Review_Date.
    ///
    /// Validates: `FgdcMetadataFutureReviewDateAfterReviewDate`.
    ///
    /// Source: FGDC CSDGM §7.3.
    fn validate_future_review_after_review(
        &self,
        review_date: &str,
        future_review_date: &str,
    ) -> GisResult<Established<crate::FgdcMetadataFutureReviewDateAfterReviewDate>>;

    /// Assert a Process_Step date is YYYYMMDD, "Unknown", or "Not complete".
    ///
    /// Validates: `FgdcProcessStepDateFgdcFormat`.
    ///
    /// Source: FGDC CSDGM §2.5.2.3.
    fn validate_process_step_date(
        &self,
        date: &str,
    ) -> GisResult<Established<FgdcProcessStepDateFgdcFormat>>;
}

/// Validate FGDC domain-code fields in the Status and Reference sections.
///
/// Source: FGDC CSDGM §1.4, §7.7.
pub trait FgdcStatusValidator: Send + Sync {
    /// Assert Progress value is "Complete", "In work", or "Planned".
    ///
    /// Validates: `FgdcStatusProgressCodeValid`.
    ///
    /// Source: FGDC CSDGM §1.4.1.
    fn validate_progress_code(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcStatusProgressCodeValid>>;

    /// Assert Maintenance_and_Update_Frequency is a valid FGDC code or free
    /// text.
    ///
    /// Validates: `FgdcStatusUpdateFrequencyCodeValid`.
    ///
    /// Source: FGDC CSDGM §1.4.2.
    fn validate_update_frequency_code(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcStatusUpdateFrequencyCodeValid>>;

    /// Assert Metadata_Time_Convention is a valid FGDC code.
    ///
    /// Validates: `FgdcMetadataTimeConventionCodeValid`.
    ///
    /// Source: FGDC CSDGM §7.7.
    fn validate_time_convention_code(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcMetadataTimeConventionCodeValid>>;
}

/// Validate spatial data organization parameters (§3).
///
/// Source: FGDC CSDGM §3 — Spatial_Data_Organization_Information.
pub trait FgdcSpatialParamValidator: Send + Sync {
    /// Assert Direct_Spatial_Reference_Method is "Point", "Vector", or
    /// "Raster".
    ///
    /// Validates: `FgdcDirectSpatialRefMethodCodeValid`.
    ///
    /// Source: FGDC CSDGM §3.2.
    fn validate_direct_ref_method(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcDirectSpatialRefMethodCodeValid>>;

    /// Assert SDTS_Point_and_Vector_Object_Type is from the SDTS code domain.
    ///
    /// Validates: `FgdcSdtsObjectTypeCodeValid`.
    ///
    /// Source: FGDC CSDGM §3.3.1.1.
    fn validate_sdts_object_type(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcSdtsObjectTypeCodeValid>>;

    /// Assert Point_and_Vector_Object_Count > 0.
    ///
    /// Validates: `FgdcSdtsObjectCountPositive`.
    ///
    /// Source: FGDC CSDGM §3.3.1.2.
    fn validate_sdts_object_count(
        &self,
        count: u64,
    ) -> GisResult<Established<FgdcSdtsObjectCountPositive>>;

    /// Assert VPF_Point_and_Vector_Object_Type is "Node", "Edge", "Face", or
    /// "Text".
    ///
    /// Validates: `FgdcVpfObjectTypeCodeValid`.
    ///
    /// Source: FGDC CSDGM §3.3.2.2.1.
    fn validate_vpf_object_type(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcVpfObjectTypeCodeValid>>;

    /// Assert VPF_Topology_Level ∈ [0, 3].
    ///
    /// Validates: `FgdcVpfTopologyLevelZeroToThree`.
    ///
    /// Source: FGDC CSDGM §3.3.2.1.
    fn validate_vpf_topology_level(
        &self,
        level: u8,
    ) -> GisResult<Established<FgdcVpfTopologyLevelZeroToThree>>;

    /// Assert Raster_Object_Type is "Point", "Pixel", "Grid Cell", or "Voxel".
    ///
    /// Validates: `FgdcRasterObjectTypeCodeValid`.
    ///
    /// Source: FGDC CSDGM §3.4.1.
    fn validate_raster_object_type(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcRasterObjectTypeCodeValid>>;

    /// Assert Row_Count > 0.
    ///
    /// Validates: `FgdcRasterRowCountPositive`.
    ///
    /// Source: FGDC CSDGM §3.4.2.
    fn validate_raster_row_count(
        &self,
        count: u64,
    ) -> GisResult<Established<FgdcRasterRowCountPositive>>;

    /// Assert Column_Count > 0.
    ///
    /// Validates: `FgdcRasterColumnCountPositive`.
    ///
    /// Source: FGDC CSDGM §3.4.3.
    fn validate_raster_column_count(
        &self,
        count: u64,
    ) -> GisResult<Established<FgdcRasterColumnCountPositive>>;

    /// Assert Vertical_Count > 0.
    ///
    /// Validates: `FgdcRasterVerticalCountPositive`.
    ///
    /// Source: FGDC CSDGM §3.4.4.
    fn validate_raster_vertical_count(
        &self,
        count: u64,
    ) -> GisResult<Established<FgdcRasterVerticalCountPositive>>;
}

/// Validate map projection numeric parameters and grid zone identifiers (§4).
///
/// Source: FGDC CSDGM §4.1.2 — Map_Projection.
pub trait FgdcMapProjValidator: Send + Sync {
    /// Assert Latitude_Resolution > 0.0.
    ///
    /// Validates: `FgdcGeographicLatResolutionPositive`.
    ///
    /// Source: FGDC CSDGM §4.1.1.1.
    fn validate_lat_resolution(
        &self,
        res: f64,
    ) -> GisResult<Established<FgdcGeographicLatResolutionPositive>>;

    /// Assert Longitude_Resolution > 0.0.
    ///
    /// Validates: `FgdcGeographicLonResolutionPositive`.
    ///
    /// Source: FGDC CSDGM §4.1.1.2.
    fn validate_lon_resolution(
        &self,
        res: f64,
    ) -> GisResult<Established<FgdcGeographicLonResolutionPositive>>;

    /// Assert Geographic_Coordinate_Units is a valid FGDC code.
    ///
    /// Validates: `FgdcGeographicCoordUnitsCodeValid`.
    ///
    /// Source: FGDC CSDGM §4.1.1.3.
    fn validate_geographic_coord_units(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcGeographicCoordUnitsCodeValid>>;

    /// Assert Standard_Parallel ∈ [-90.0, 90.0].
    ///
    /// Validates: `FgdcMapProjStandardParallelInRange`.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.1.
    fn validate_standard_parallel(
        &self,
        lat: f64,
    ) -> GisResult<Established<FgdcMapProjStandardParallelInRange>>;

    /// Assert Longitude_of_Central_Meridian ∈ [-180.0, 180.0).
    ///
    /// Validates: `FgdcMapProjCentralMeridianInRange`.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.2.
    fn validate_central_meridian(
        &self,
        lon: f64,
    ) -> GisResult<Established<FgdcMapProjCentralMeridianInRange>>;

    /// Assert Latitude_of_Projection_Origin ∈ [-90.0, 90.0].
    ///
    /// Validates: `FgdcMapProjLatitudeOriginInRange`.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.3.
    fn validate_latitude_origin(
        &self,
        lat: f64,
    ) -> GisResult<Established<FgdcMapProjLatitudeOriginInRange>>;

    /// Assert a scale factor (at equator, centre line, origin, or central
    /// meridian) > 0.0.
    ///
    /// Validates: `FgdcMapProjScaleFactorPositive`.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.6/.10/.14/.17.
    fn validate_scale_factor(
        &self,
        factor: f64,
    ) -> GisResult<Established<FgdcMapProjScaleFactorPositive>>;

    /// Assert Azimuthal_Angle ∈ [0.0, 360.0).
    ///
    /// Validates: `FgdcMapProjAzimuthalAngle0To360`.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.11.1.
    fn validate_azimuthal_angle(
        &self,
        angle: f64,
    ) -> GisResult<Established<FgdcMapProjAzimuthalAngle0To360>>;

    /// Assert UTM_Zone_Number ∈ [1, 60] (N) or [-60, -1] (S).
    ///
    /// Validates: `FgdcUtmZoneNumberInRange`.
    ///
    /// Source: FGDC CSDGM §4.1.2.2.2.1.
    fn validate_utm_zone(&self, zone: i32) -> GisResult<Established<FgdcUtmZoneNumberInRange>>;

    /// Assert UPS_Zone_Identifier is "A", "B", "Y", or "Z".
    ///
    /// Validates: `FgdcUpsZoneIdentifierCodeValid`.
    ///
    /// Source: FGDC CSDGM §4.1.2.2.3.1.
    fn validate_ups_zone(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcUpsZoneIdentifierCodeValid>>;

    /// Assert ARC_System_Zone_Identifier ∈ [1, 18].
    ///
    /// Validates: `FgdcArcZoneIdentifier1To18`.
    ///
    /// Source: FGDC CSDGM §4.1.2.2.5.1.
    fn validate_arc_zone(&self, zone: u8) -> GisResult<Established<FgdcArcZoneIdentifier1To18>>;
}

/// Validate altitude and depth resolution constraints (§4.2).
///
/// Source: FGDC CSDGM §4.2 — Altitude_System_Definition /
/// Depth_System_Definition.
pub trait FgdcVertRefValidator: Send + Sync {
    /// Assert Altitude_System_Definition has at least one Altitude_Resolution
    /// value.
    ///
    /// Validates: `FgdcAltitudeResolutionAtLeastOne`.
    ///
    /// Source: FGDC CSDGM §4.2.
    fn validate_altitude_resolution_count(
        &self,
        count: usize,
    ) -> GisResult<Established<FgdcAltitudeResolutionAtLeastOne>>;

    /// Assert Depth_System_Definition has at least one Depth_Resolution value.
    ///
    /// Validates: `FgdcDepthResolutionAtLeastOne`.
    ///
    /// Source: FGDC CSDGM §4.2.
    fn validate_depth_resolution_count(
        &self,
        count: usize,
    ) -> GisResult<Established<FgdcDepthResolutionAtLeastOne>>;
}

/// Validate entity-attribute invariants (§5).
///
/// Source: FGDC CSDGM §5 — Entity_and_Attribute_Information.
pub trait FgdcEntityAttrValidator: Send + Sync {
    /// Assert Range_Domain_Minimum ≤ Range_Domain_Maximum.
    ///
    /// Validates: `FgdcRangeDomainMinimumLeMaximum`.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2.
    fn validate_range_domain_order(
        &self,
        minimum: &str,
        maximum: &str,
    ) -> GisResult<Established<FgdcRangeDomainMinimumLeMaximum>>;

    /// Assert Attribute_Measurement_Resolution > 0.0.
    ///
    /// Validates: `FgdcAttributeMeasurementResolutionPositive`.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2.4.
    fn validate_measurement_resolution(
        &self,
        res: f64,
    ) -> GisResult<Established<FgdcAttributeMeasurementResolutionPositive>>;
}

/// Validate distribution and dialup parameters (§6).
///
/// Source: FGDC CSDGM §6 — Distribution_Information.
pub trait FgdcDistributionParamValidator: Send + Sync {
    /// Assert Lowest_BPS ≥ 110.
    ///
    /// Validates: `FgdcDialupLowestBpsGeq110`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.1.
    fn validate_lowest_bps(&self, bps: u64) -> GisResult<Established<FgdcDialupLowestBpsGeq110>>;

    /// Assert Highest_BPS > Lowest_BPS.
    ///
    /// Validates: `FgdcDialupHighestBpsGtLowest`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.2.
    fn validate_highest_bps_gt_lowest(
        &self,
        lowest: u64,
        highest: u64,
    ) -> GisResult<Established<FgdcDialupHighestBpsGtLowest>>;

    /// Assert Number_DataBits is 7 or 8.
    ///
    /// Validates: `FgdcDialupDataBitsSevenOrEight`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.3.
    fn validate_data_bits(
        &self,
        bits: u8,
    ) -> GisResult<Established<FgdcDialupDataBitsSevenOrEight>>;

    /// Assert Number_StopBits is 1 or 2.
    ///
    /// Validates: `FgdcDialupStopBitsOneOrTwo`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.4.
    fn validate_stop_bits(&self, bits: u8) -> GisResult<Established<FgdcDialupStopBitsOneOrTwo>>;

    /// Assert Parity is "None", "Odd", "Even", "Mark", or "Space".
    ///
    /// Validates: `FgdcDialupParityCodeValid`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.5.
    fn validate_parity_code(&self, code: &str)
    -> GisResult<Established<FgdcDialupParityCodeValid>>;

    /// Assert Recording_Density > 0.0.
    ///
    /// Validates: `FgdcRecordingDensityPositive`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.2.1.
    fn validate_recording_density(
        &self,
        density: f64,
    ) -> GisResult<Established<FgdcRecordingDensityPositive>>;

    /// Assert Recording_Format is a valid FGDC code or free text.
    ///
    /// Validates: `FgdcRecordingFormatCodeValid`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.3.
    fn validate_recording_format(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcRecordingFormatCodeValid>>;

    /// Assert Offline_Media is a valid FGDC code or free text.
    ///
    /// Validates: `FgdcOfflineMediaCodeValid`.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.1.
    fn validate_offline_media(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcOfflineMediaCodeValid>>;
}

/// Validate security classification codes and cloud-cover value (§1.12, §2.6).
///
/// Source: FGDC CSDGM §1.12 — Security_Information; §2.6 — Cloud_Cover.
pub trait FgdcSecurityValidator: Send + Sync {
    /// Assert Security_Classification is a valid FGDC code or free text.
    ///
    /// Validates: `FgdcSecurityClassificationCodeValid`.
    ///
    /// Source: FGDC CSDGM §1.12.2.
    fn validate_security_classification(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcSecurityClassificationCodeValid>>;

    /// Assert Metadata_Security_Classification is a valid FGDC code or free
    /// text.
    ///
    /// Validates: `FgdcMetadataSecurityClassificationCodeValid`.
    ///
    /// Source: FGDC CSDGM §7.10.2.
    fn validate_metadata_security_classification(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcMetadataSecurityClassificationCodeValid>>;

    /// Assert Cloud_Cover ∈ [0, 100] or is the "Unknown" token.
    ///
    /// Validates: `FgdcCloudCoverZeroToHundred`.
    ///
    /// Source: FGDC CSDGM §2.6.
    fn validate_cloud_cover(
        &self,
        value: &str,
    ) -> GisResult<Established<crate::FgdcCloudCoverZeroToHundred>>;

    /// Assert Source_Scale_Denominator > 1.
    ///
    /// Validates: `FgdcSourceScaleDenominatorGt1`.
    ///
    /// Source: FGDC CSDGM §2.5.1.2.
    fn validate_source_scale_denominator(
        &self,
        denom: u64,
    ) -> GisResult<Established<crate::FgdcSourceScaleDenominatorGt1>>;
}

/// Validate the Contact_Address type code (§10).
///
/// Source: FGDC CSDGM §10.4.1 — Address_Type.
pub trait FgdcContactValidator: Send + Sync {
    /// Assert Address_Type is "mailing", "physical", "mailing and physical", or
    /// free text.
    ///
    /// Validates: `FgdcContactAddressTypeCodeValid`.
    ///
    /// Source: FGDC CSDGM §10.4.1.
    fn validate_contact_address_type(
        &self,
        code: &str,
    ) -> GisResult<Established<FgdcContactAddressTypeCodeValid>>;
}

// ── Role 1b: Tier 2 section factories ────────────────────────────────────────

/// Build and validate FGDC §8 Citation_Information.
///
/// This factory validates all leaf invariants internal to the Citation section
/// (originator, date, title) and emits `Established<FgdcCitationInfoValid>`.
/// The resulting token is a required precondition for
/// [`FgdcIdentificationFactory::build_identification`].
///
/// Source: FGDC CSDGM §8 — Citation_Information.
///
/// [`FgdcIdentificationFactory::build_identification`]:
///     FgdcIdentificationFactory::build_identification
pub trait FgdcCitationFactory: Send + Sync {
    /// Build a Citation_Information descriptor.
    ///
    /// `originators` must be non-empty and each originator non-empty.
    /// `pub_date` must be YYYYMMDD, "Unknown", or "Unpublished material".
    /// `title` must be non-empty.
    ///
    /// Validates (all internal): `FgdcCitationHasAtLeastOneOriginator`,
    /// `FgdcCitationOriginatorNonEmpty`, `FgdcCitationPublicationDatePresent`,
    /// `FgdcCitationPublicationDateIsYyyymmddOrToken`,
    /// `FgdcCitationTitlePresent`, `FgdcCitationTitleNonEmpty`
    /// → `FgdcCitationInfoValid`.
    ///
    /// Source: FGDC CSDGM §8 — Citation_Information.
    fn build_citation(
        &self,
        originators: Vec<String>,
        pub_date: String,
        title: String,
        pub_place: Option<String>,
        publisher: Option<String>,
        other_citation_details: Option<String>,
    ) -> GisResult<(FgdcCitationDescriptor, Established<FgdcCitationInfoValid>)>;
}

/// Build and validate FGDC §9 Time_Period_Information.
///
/// This factory validates the exclusive time-period-kind invariant and, for
/// ranges, the ordering invariant, then emits
/// `Established<FgdcTimePeriodInfoValid>`.
///
/// Source: FGDC CSDGM §9 — Time_Period_Information.
pub trait FgdcTimePeriodFactory: Send + Sync {
    /// Build a Time_Period_Information descriptor.
    ///
    /// For `FgdcTimePeriodKind::Range`, validates that the ending date is not
    /// before the beginning date.
    ///
    /// Validates: `FgdcTimePeriodTypeExclusive`,
    /// `FgdcCalendarDateIsYyyymmddOrToken`,
    /// `FgdcRangeEndingDateAfterBeginning` (range only)
    /// → `FgdcTimePeriodInfoValid`.
    ///
    /// Source: FGDC CSDGM §9 — Time_Period_Information.
    fn build_time_period(
        &self,
        kind: FgdcTimePeriodKind,
    ) -> GisResult<(
        FgdcTimePeriodDescriptor,
        Established<FgdcTimePeriodInfoValid>,
    )>;
}

/// Build and validate FGDC §10 Contact_Information.
///
/// This factory validates contact address type code and mandatory address
/// sub-elements, then emits `Established<FgdcContactInfoValid>`.
/// The token is a required precondition for
/// [`FgdcDistributionFactory`] and [`FgdcMetadataRefFactory`].
///
/// Source: FGDC CSDGM §10 — Contact_Information.
pub trait FgdcContactFactory: Send + Sync {
    /// Build a Contact_Information descriptor.
    ///
    /// Accepts a [`FgdcContactDescriptor`] whose fields map to CSDGM §10
    /// elements.  Either `person` or `organization` must be `Some(_)`.
    ///
    /// Validates: `FgdcContactHasPersonOrOrganizationPrimary`,
    /// `FgdcContactAddressTypeCodeValid`,
    /// `FgdcContactAddressCityPresent`,
    /// `FgdcContactAddressStatePresent`,
    /// `FgdcContactAddressPostalCodePresent`,
    /// `FgdcContactHasAtLeastOneAddress`,
    /// `FgdcContactHasAtLeastOneVoiceTelephone`
    /// → `FgdcContactInfoValid`.
    ///
    /// Source: FGDC CSDGM §10 — Contact_Information.
    fn build_contact(
        &self,
        input: FgdcContactDescriptor,
    ) -> GisResult<(FgdcContactDescriptor, Established<FgdcContactInfoValid>)>;
}

/// Build and validate FGDC §1 Identification_Information.
///
/// Requires pre-validated citation and time-period tokens; validates all leaf
/// invariants internal to the Identification section (bounding coordinates,
/// status codes, keywords) and emits
/// `Established<FgdcIdentificationSectionValid>`.
///
/// # Preconditions
///
/// - `Established<FgdcCitationInfoValid>` from [`FgdcCitationFactory`]
/// - `Established<FgdcTimePeriodInfoValid>` from [`FgdcTimePeriodFactory`]
///
/// Source: FGDC CSDGM §1 — Identification_Information.
pub trait FgdcIdentificationFactory: Send + Sync {
    /// Build an Identification_Information descriptor.
    ///
    /// `theme_keywords` must contain at least one group, each group having a
    /// non-empty thesaurus and at least one keyword.
    ///
    /// `west`, `east`, `south`, `north` must satisfy the FGDC bounding
    /// coordinate domain rules; north ≥ south is also verified internally.
    ///
    /// Validates internally: `FgdcDescriptionAbstractPresent`,
    /// `FgdcDescriptionPurposePresent`, `FgdcStatusProgressCodeValid`,
    /// `FgdcStatusUpdateFrequencyCodeValid`, `FgdcBoundingWestCoordInRange`,
    /// `FgdcBoundingEastCoordInRange`, `FgdcBoundingSouthCoordInRange`,
    /// `FgdcBoundingNorthCoordInRange`, `FgdcBoundingNorthGeqSouth`,
    /// `FgdcKeywordsHasAtLeastOneTheme`, `FgdcThemeHasKeywordThesaurus`,
    /// `FgdcThemeHasAtLeastOneKeyword`, `FgdcAccessConstraintsPresent`,
    /// `FgdcUseConstraintsPresent`.
    ///
    /// Postcondition: `FgdcIdentificationSectionValid`.
    ///
    /// Source: FGDC CSDGM §1 — Identification_Information.
    #[allow(clippy::too_many_arguments)]
    fn build_identification(
        &self,
        citation: Established<FgdcCitationInfoValid>,
        time_of_content: Established<FgdcTimePeriodInfoValid>,
        abstract_text: String,
        purpose: String,
        status_progress: String,
        update_frequency: String,
        west: f64,
        east: f64,
        south: f64,
        north: f64,
        theme_keywords: Vec<FgdcKeywordGroup>,
        place_keywords: Vec<FgdcKeywordGroup>,
        stratum_keywords: Vec<FgdcKeywordGroup>,
        temporal_keywords: Vec<FgdcKeywordGroup>,
        access_constraints: String,
        use_constraints: String,
        cloud_cover: Option<String>,
        browse_file_name: Option<String>,
        browse_file_description: Option<String>,
        browse_file_type: Option<String>,
        security_classification: Option<String>,
        security_classification_system: Option<String>,
        security_handling_description: Option<String>,
    ) -> GisResult<(
        FgdcIdentificationDescriptor,
        Established<FgdcIdentificationSectionValid>,
    )>;
}

/// Build and validate FGDC §2 Data_Quality_Information.
///
/// Validates that all mandatory §2 elements are present, including at least one
/// process step, and emits `Established<FgdcDataQualitySectionValid>`.
///
/// Source: FGDC CSDGM §2 — Data_Quality_Information.
pub trait FgdcDataQualityFactory: Send + Sync {
    /// Build a Data_Quality_Information descriptor.
    ///
    /// `process_steps` must be non-empty.
    ///
    /// Validates: `FgdcDataQualityLogicalConsistencyPresent`,
    /// `FgdcDataQualityCompletenessReportPresent`,
    /// `FgdcDataQualityLineagePresent`,
    /// `FgdcLineageHasAtLeastOneProcessStep`,
    /// `FgdcProcessStepDescriptionPresent`
    /// → `FgdcDataQualitySectionValid`.
    ///
    /// Source: FGDC CSDGM §2 — Data_Quality_Information.
    fn build_data_quality(
        &self,
        logical_consistency: String,
        completeness: String,
        lineage_statement: Option<String>,
        process_steps: Vec<FgdcProcessStepInfo>,
        sources: Vec<FgdcSourceInfo>,
    ) -> GisResult<(
        FgdcDataQualityDescriptor,
        Established<FgdcDataQualitySectionValid>,
    )>;
}

/// Build and validate FGDC §3 Spatial_Data_Organization_Information.
///
/// Validates direct reference method, SDTS/VPF/raster codes, and counts, then
/// emits `Established<FgdcSpatialDataOrgSectionValid>`.
///
/// Source: FGDC CSDGM §3 — Spatial_Data_Organization_Information.
pub trait FgdcSpatialOrgFactory: Send + Sync {
    /// Build a Spatial_Data_Organization_Information descriptor.
    ///
    /// Validates: `FgdcDirectSpatialRefMethodCodeValid`,
    /// `FgdcPvectInfoIsSdtsOrVpfExclusive` (when vector),
    /// `FgdcSdtsObjectTypeCodeValid`, `FgdcSdtsObjectCountPositive`,
    /// `FgdcVpfObjectTypeCodeValid`, `FgdcVpfTopologyLevelZeroToThree`,
    /// `FgdcRasterObjectTypeCodeValid`, `FgdcRasterRowCountPositive`,
    /// `FgdcRasterColumnCountPositive`, `FgdcRasterVerticalCountPositive`
    /// → `FgdcSpatialDataOrgSectionValid`.
    ///
    /// Source: FGDC CSDGM §3 — Spatial_Data_Organization_Information.
    fn build_spatial_org(
        &self,
        descriptor: FgdcSpatialOrgDescriptor,
    ) -> GisResult<(
        FgdcSpatialOrgDescriptor,
        Established<FgdcSpatialDataOrgSectionValid>,
    )>;
}

/// Build and validate FGDC §4 Spatial_Reference_Information.
///
/// Validates horizontal CRS type code, geographic resolution and units,
/// geodetic model parameters, and altitude/depth resolution, then emits
/// `Established<FgdcSpatialReferenceSectionValid>`.
///
/// Source: FGDC CSDGM §4 — Spatial_Reference_Information.
pub trait FgdcSpatialRefFactory: Send + Sync {
    /// Build a Spatial_Reference_Information descriptor.
    ///
    /// Validates: `FgdcHorizCrsTypeIsGeographicPlanarOrLocal`,
    /// `FgdcGeographicLatResolutionPositive`,
    /// `FgdcGeographicLonResolutionPositive`,
    /// `FgdcGeographicCoordUnitsCodeValid`,
    /// `FgdcGeodeticModelEllipsoidNamePresent`,
    /// `FgdcGeodeticModelSemiMajorAxisPositive`,
    /// `FgdcGeodeticModelFlatteningRatioDenominatorPositive`,
    /// `FgdcAltitudeResolutionAtLeastOne`,
    /// `FgdcAltitudeDistanceUnitsPresent`,
    /// `FgdcAltitudeEncodingMethodPresent`
    /// → `FgdcSpatialReferenceSectionValid`.
    ///
    /// Source: FGDC CSDGM §4 — Spatial_Reference_Information.
    fn build_spatial_ref(
        &self,
        descriptor: FgdcSpatialRefDescriptor,
    ) -> GisResult<(
        FgdcSpatialRefDescriptor,
        Established<FgdcSpatialReferenceSectionValid>,
    )>;
}

/// Build and validate FGDC §5 Entity_and_Attribute_Information.
///
/// Validates that each entity type has label/definition/source, each attribute
/// has label/definition/source/domain, and emits
/// `Established<FgdcEntityAttributeSectionValid>`.
///
/// Source: FGDC CSDGM §5 — Entity_and_Attribute_Information.
pub trait FgdcEntityAttrFactory: Send + Sync {
    /// Build Entity_and_Attribute_Information descriptors.
    ///
    /// `entities` must be non-empty.
    ///
    /// Validates: `FgdcEntityAttributeHasDetailedOrOverview`,
    /// `FgdcEntityTypeLabelPresent`, `FgdcEntityTypeDefinitionPresent`,
    /// `FgdcEntityTypeDefinitionSourcePresent`,
    /// `FgdcAttributeLabelPresent`, `FgdcAttributeDefinitionPresent`,
    /// `FgdcAttributeDefinitionSourcePresent`,
    /// `FgdcAttributeHasAtLeastOneDomain`,
    /// `FgdcAttributeDomainTypeExclusive`
    /// → `FgdcEntityAttributeSectionValid`.
    ///
    /// Source: FGDC CSDGM §5 — Entity_and_Attribute_Information.
    fn build_entity(
        &self,
        entities: Vec<FgdcEntityAttrDescriptor>,
    ) -> GisResult<(
        Vec<FgdcEntityAttrDescriptor>,
        Established<FgdcEntityAttributeSectionValid>,
    )>;
}

/// Build and validate FGDC §6 Distribution_Information.
///
/// Requires a pre-validated contact token for the Distributor contact record.
///
/// # Preconditions
///
/// - `Established<FgdcContactInfoValid>` from [`FgdcContactFactory`]
///
/// Source: FGDC CSDGM §6 — Distribution_Information.
pub trait FgdcDistributionFactory: Send + Sync {
    /// Build a Distribution_Information descriptor.
    ///
    /// Validates: `FgdcDistributionDistributorPresent`,
    /// `FgdcDistributionLiabilityPresent`,
    /// `FgdcStandardOrderFeesPresent`,
    /// `FgdcStandardOrderHasFormNondigitalOrDigital` (when form fields present),
    /// `FgdcDigitalFormatNamePresent` (when digital form present),
    /// `FgdcTransferSizePositive` (when transfer_size present)
    /// → `FgdcDistributionSectionValid`.
    ///
    /// Source: FGDC CSDGM §6 — Distribution_Information.
    fn build_distribution(
        &self,
        distributor_contact: Established<FgdcContactInfoValid>,
        input: FgdcDistributionDescriptor,
    ) -> GisResult<(
        FgdcDistributionDescriptor,
        Established<FgdcDistributionSectionValid>,
    )>;
}

/// Build and validate FGDC §7 Metadata_Reference_Information.
///
/// Requires a pre-validated contact token for the Metadata_Contact record.
///
/// # Preconditions
///
/// - `Established<FgdcContactInfoValid>` from [`FgdcContactFactory`]
///
/// Source: FGDC CSDGM §7 — Metadata_Reference_Information.
pub trait FgdcMetadataRefFactory: Send + Sync {
    /// Build a Metadata_Reference_Information descriptor.
    ///
    /// Validates: `FgdcMetadataDatePresent`,
    /// `FgdcCalendarDateIsYyyymmddOrToken`,
    /// `FgdcMetadataContactPresent`,
    /// `FgdcMetadataStandardNamePresent`,
    /// `FgdcMetadataStandardVersionPresent`,
    /// `FgdcMetadataReviewDateAfterMetadataDate` (when review_date present),
    /// `FgdcMetadataFutureReviewDateAfterReviewDate` (when future_review_date present)
    /// → `FgdcMetadataReferenceSectionValid`.
    ///
    /// Source: FGDC CSDGM §7 — Metadata_Reference_Information.
    fn build_metadata_ref(
        &self,
        metadata_contact: Established<FgdcContactInfoValid>,
        input: FgdcMetadataRefDescriptor,
    ) -> GisResult<(
        FgdcMetadataRefDescriptor,
        Established<FgdcMetadataReferenceSectionValid>,
    )>;
}

/// Assemble a complete FGDC CSDGM §0 Metadata record.
///
/// Takes the two mandatory section tokens and any optional section tokens;
/// validates the §0 top-level structure rule (identification + metadata
/// reference are present), and emits `Established<FgdcRecordValid>`.
///
/// # Preconditions (mandatory)
///
/// - `Established<FgdcIdentificationSectionValid>` from
///   [`FgdcIdentificationFactory`]
/// - `Established<FgdcMetadataReferenceSectionValid>` from
///   [`FgdcMetadataRefFactory`]
///
/// # Optional section tokens
///
/// When an optional section is included its validity token must also be
/// supplied.  The absence of an optional token simply means that section is
/// not included in the record.
///
/// Source: FGDC CSDGM §0 — Metadata.
pub trait FgdcRecordFactory: Send + Sync {
    /// Assemble a complete FGDC Metadata record.
    ///
    /// Validates: `FgdcMetadataHasIdentificationSection`,
    /// `FgdcMetadataHasMetadataReferenceSection`
    /// → `FgdcRecordValid`.
    ///
    /// Source: FGDC CSDGM §0 — Metadata.
    #[allow(clippy::too_many_arguments)]
    fn build_record(
        &self,
        identification: Established<FgdcIdentificationSectionValid>,
        metadata_ref: Established<FgdcMetadataReferenceSectionValid>,
        data_quality: Option<Established<FgdcDataQualitySectionValid>>,
        spatial_org: Option<Established<FgdcSpatialDataOrgSectionValid>>,
        spatial_ref: Option<Established<FgdcSpatialReferenceSectionValid>>,
        entity_attr: Option<Established<FgdcEntityAttributeSectionValid>>,
        distribution: Option<Established<FgdcDistributionSectionValid>>,
    ) -> GisResult<(FgdcRecordDescriptor, Established<FgdcRecordValid>)>;
}

// ── Role 2: Orthogonal concern traits ────────────────────────────────────────

/// Report bounding coordinate values independently of validation status.
///
/// These methods return raw field values and can be called at any time,
/// regardless of whether any `Established<FgdcBounding*>` token has been
/// obtained.
///
/// Source: FGDC CSDGM §1.5.1 — Bounding Coordinates.
pub trait FgdcBoundingMeta: Send + Sync {
    /// Western-most longitude in decimal degrees.
    ///
    /// Source: FGDC CSDGM §1.5.1.1.
    fn west(&self) -> f64;

    /// Eastern-most longitude in decimal degrees.
    ///
    /// Source: FGDC CSDGM §1.5.1.2.
    fn east(&self) -> f64;

    /// Southern-most latitude in decimal degrees.
    ///
    /// Source: FGDC CSDGM §1.5.1.4.
    fn south(&self) -> f64;

    /// Northern-most latitude in decimal degrees.
    ///
    /// Source: FGDC CSDGM §1.5.1.3.
    fn north(&self) -> f64;

    /// Whether a Data_Set_G-Polygon extent is declared.
    ///
    /// Source: FGDC CSDGM §1.5.2.
    fn has_g_polygon(&self) -> bool;
}

/// Report keyword content independently of validation status.
///
/// Source: FGDC CSDGM §1.6 — Keywords.
pub trait FgdcKeywordMeta: Send + Sync {
    /// All theme keywords from all Theme groups, flattened.
    ///
    /// Source: FGDC CSDGM §1.6.1.
    fn theme_keywords(&self) -> Vec<&str>;

    /// Theme keyword thesauri, one per Theme group.
    ///
    /// Source: FGDC CSDGM §1.6.1.
    fn theme_thesauri(&self) -> Vec<&str>;

    /// All place keywords from all Place groups, flattened (empty when absent).
    ///
    /// Source: FGDC CSDGM §1.6.2.
    fn place_keywords(&self) -> Vec<&str>;

    /// All stratum keywords from all Stratum groups, flattened (empty when absent).
    ///
    /// Source: FGDC CSDGM §1.6.3.
    fn stratum_keywords(&self) -> Vec<&str>;

    /// All temporal keywords from all Temporal groups, flattened (empty when
    /// absent).
    ///
    /// Source: FGDC CSDGM §1.6.4.
    fn temporal_keywords(&self) -> Vec<&str>;
}

/// Report contact identity fields independently of validation status.
///
/// Source: FGDC CSDGM §10 — Contact_Information.
pub trait FgdcContactMeta: Send + Sync {
    /// Contact person name, if declared.
    ///
    /// Source: FGDC CSDGM §10.1.
    fn contact_person(&self) -> Option<&str>;

    /// Contact organization name, if declared.
    ///
    /// Source: FGDC CSDGM §10.2.
    fn contact_organization(&self) -> Option<&str>;

    /// Primary voice telephone number.
    ///
    /// Source: FGDC CSDGM §10.5.
    fn contact_phone(&self) -> &str;

    /// City from the first Contact_Address.
    ///
    /// Source: FGDC CSDGM §10.4.3.
    fn contact_city(&self) -> &str;

    /// State or province from the first Contact_Address.
    ///
    /// Source: FGDC CSDGM §10.4.4.
    fn contact_state(&self) -> &str;
}

/// Report distribution descriptive fields independently of validation status.
///
/// Source: FGDC CSDGM §6 — Distribution_Information.
pub trait FgdcDistributionMeta: Send + Sync {
    /// Distributor name (person or organization from the Distributor contact).
    ///
    /// Source: FGDC CSDGM §6.1.
    fn distributor_name(&self) -> &str;

    /// Distribution liability statement.
    ///
    /// Source: FGDC CSDGM §6.2.
    fn liability(&self) -> &str;

    /// Digital format name, if declared.
    ///
    /// Source: FGDC CSDGM §6.4.2.1.1.
    fn format_name(&self) -> Option<&str>;

    /// Transfer size in megabytes, if declared.
    ///
    /// Source: FGDC CSDGM §6.4.2.1.7.
    fn transfer_size(&self) -> Option<f64>;

    /// Whether any digital transfer form is declared.
    ///
    /// Source: FGDC CSDGM §6.4.
    fn has_digital_form(&self) -> bool;
}

/// Report metadata reference descriptive fields independently of validation
/// status.
///
/// Source: FGDC CSDGM §7 — Metadata_Reference_Information.
pub trait FgdcMetadataMeta: Send + Sync {
    /// Metadata creation or last-update date (YYYYMMDD).
    ///
    /// Source: FGDC CSDGM §7.1.
    fn metadata_date(&self) -> &str;

    /// Name of the metadata standard.
    ///
    /// Source: FGDC CSDGM §7.4.
    fn metadata_standard_name(&self) -> &str;

    /// Version of the metadata standard.
    ///
    /// Source: FGDC CSDGM §7.5.
    fn metadata_standard_version(&self) -> &str;

    /// Date of the last metadata review, if declared.
    ///
    /// Source: FGDC CSDGM §7.2.
    fn metadata_review_date(&self) -> Option<&str>;

    /// Scheduled date of the next metadata review, if declared.
    ///
    /// Source: FGDC CSDGM §7.3.
    fn metadata_future_review_date(&self) -> Option<&str>;
}

// ── Role 3: Abstraction supertrait ───────────────────────────────────────────

/// Complete FGDC CSDGM backend — composes all FGDC sub-traits.
///
/// Any type that implements all FGDC sub-traits automatically implements
/// `FgdcBackend` via the blanket impl below.  Use the individual object-safe
/// sub-traits (`dyn FgdcCitationFactory`, etc.) for dynamic dispatch at
/// architectural boundaries.
///
/// Source: Federal Geographic Data Committee, FGDC-STD-001-1998.
pub trait FgdcBackend:
    FgdcBoundingValidator
    + FgdcDateValidator
    + FgdcStatusValidator
    + FgdcSpatialParamValidator
    + FgdcMapProjValidator
    + FgdcVertRefValidator
    + FgdcEntityAttrValidator
    + FgdcDistributionParamValidator
    + FgdcSecurityValidator
    + FgdcContactValidator
    + FgdcCitationFactory
    + FgdcTimePeriodFactory
    + FgdcContactFactory
    + FgdcIdentificationFactory
    + FgdcDataQualityFactory
    + FgdcSpatialOrgFactory
    + FgdcSpatialRefFactory
    + FgdcEntityAttrFactory
    + FgdcDistributionFactory
    + FgdcMetadataRefFactory
    + FgdcRecordFactory
    + FgdcBoundingMeta
    + FgdcKeywordMeta
    + FgdcContactMeta
    + FgdcDistributionMeta
    + FgdcMetadataMeta
    + Send
    + Sync
{
}

impl<T> FgdcBackend for T where
    T: FgdcBoundingValidator
        + FgdcDateValidator
        + FgdcStatusValidator
        + FgdcSpatialParamValidator
        + FgdcMapProjValidator
        + FgdcVertRefValidator
        + FgdcEntityAttrValidator
        + FgdcDistributionParamValidator
        + FgdcSecurityValidator
        + FgdcContactValidator
        + FgdcCitationFactory
        + FgdcTimePeriodFactory
        + FgdcContactFactory
        + FgdcIdentificationFactory
        + FgdcDataQualityFactory
        + FgdcSpatialOrgFactory
        + FgdcSpatialRefFactory
        + FgdcEntityAttrFactory
        + FgdcDistributionFactory
        + FgdcMetadataRefFactory
        + FgdcRecordFactory
        + FgdcBoundingMeta
        + FgdcKeywordMeta
        + FgdcContactMeta
        + FgdcDistributionMeta
        + FgdcMetadataMeta
        + Send
        + Sync
{
}
