//! FGDC CSDGM propositions — Content Standard for Digital Geospatial Metadata.
//!
//! Source: Federal Geographic Data Committee, FGDC-STD-001-1998.
//! Online reference: <https://www.fgdc.gov/metadata/csdgm/>
//! All section-references are to numbered sections of that standard.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // ── §0 Metadata — top-level record structure ─────────────────────────────

    /// Identification_Information is a mandatory top-level element.
    ///
    /// Source: FGDC CSDGM §0 — `Metadata = Identification_Information + ...`
    pub struct FgdcMetadataHasIdentificationSection;

    /// Metadata_Reference_Information is a mandatory top-level element.
    ///
    /// Source: FGDC CSDGM §0 — `... + Metadata_Reference_Information`
    pub struct FgdcMetadataHasMetadataReferenceSection;

    // ── §1 Identification Information ────────────────────────────────────────
    // §1.1 / §8 Citation Information

    /// Citation_Information requires at least one Originator.
    ///
    /// Source: FGDC CSDGM §8 — `Citation_Information = 1{Originator}n + ...`
    pub struct FgdcCitationHasAtLeastOneOriginator;

    /// Originator value is non-empty (domain includes "Unknown", but not blank).
    ///
    /// Source: FGDC CSDGM §8.1 — domain: "Unknown" free text
    pub struct FgdcCitationOriginatorNonEmpty;

    /// Publication_Date is mandatory in Citation_Information.
    ///
    /// Source: FGDC CSDGM §8 — `... + Publication_Date + ...`
    pub struct FgdcCitationPublicationDatePresent;

    /// Publication_Date must be YYYYMMDD, "Unknown", or "Unpublished material".
    ///
    /// Source: FGDC CSDGM §8.2 — domain: "Unknown" "Unpublished material" free date
    pub struct FgdcCitationPublicationDateIsYyyymmddOrToken;

    /// Title is mandatory in Citation_Information.
    ///
    /// Source: FGDC CSDGM §8 — `... + Title + ...`
    pub struct FgdcCitationTitlePresent;

    /// Title is a non-empty free-text string.
    ///
    /// Source: FGDC CSDGM §8.4 — domain: free text
    pub struct FgdcCitationTitleNonEmpty;

    // §1.2 Description

    /// Abstract is mandatory in Description.
    ///
    /// Source: FGDC CSDGM §1.2 — `Description = Abstract + Purpose + ...`
    pub struct FgdcDescriptionAbstractPresent;

    /// Purpose is mandatory in Description.
    ///
    /// Source: FGDC CSDGM §1.2 — `Description = Abstract + Purpose + ...`
    pub struct FgdcDescriptionPurposePresent;

    // §1.3 Time Period of Content

    /// Time_Period_Information is mandatory in Time_Period_of_Content.
    ///
    /// Source: FGDC CSDGM §1.3 — `Time_Period_of_Content = Time_Period_Information + ...`
    pub struct FgdcTimeOfContentTimePeriodPresent;

    /// Currentness_Reference must be "ground condition", "publication date", or free text.
    ///
    /// Source: FGDC CSDGM §1.3.1 — domain: "ground condition" "publication date" free text
    pub struct FgdcTimeOfContentCurrentnessReferenceValid;

    // §1.4 Status

    /// Progress must be one of the three FGDC code values.
    ///
    /// Source: FGDC CSDGM §1.4.1 — domain: "Complete" "In work" "Planned"
    pub struct FgdcStatusProgressCodeValid;

    /// Maintenance_and_Update_Frequency must be a valid FGDC code or free text.
    ///
    /// Source: FGDC CSDGM §1.4.2 — domain: "Continually" "Daily" "Weekly" "Monthly"
    /// "Annually" "Unknown" "As needed" "Irregular" "None planned" free text
    pub struct FgdcStatusUpdateFrequencyCodeValid;

    // §1.5.1 Bounding Coordinates

    /// West_Bounding_Coordinate must be in [-180.0, 180.0).
    ///
    /// Source: FGDC CSDGM §1.5.1.1 — domain: -180.0 <= West Bounding Coordinate < 180.0
    pub struct FgdcBoundingWestCoordInRange;

    /// East_Bounding_Coordinate must be in [-180.0, 180.0].
    ///
    /// Source: FGDC CSDGM §1.5.1.2 — domain: -180.0 <= East Bounding Coordinate <= 180.0
    pub struct FgdcBoundingEastCoordInRange;

    /// North_Bounding_Coordinate must be in [-90.0, 90.0].
    ///
    /// Source: FGDC CSDGM §1.5.1.3 — domain: -90.0 <= North Bounding Coordinate <= 90.0
    pub struct FgdcBoundingNorthCoordInRange;

    /// South_Bounding_Coordinate must be in [-90.0, 90.0].
    ///
    /// Source: FGDC CSDGM §1.5.1.4 — domain: -90.0 <= South Bounding Coordinate <= 90.0
    pub struct FgdcBoundingSouthCoordInRange;

    /// North_Bounding_Coordinate must be >= South_Bounding_Coordinate.
    ///
    /// Source: FGDC CSDGM §1.5.1.3 — North Bounding Coordinate >= South Bounding Coordinate
    pub struct FgdcBoundingNorthGeqSouth;

    // §1.5.2 Data Set G-Polygon (optional)

    /// When a G-Polygon is provided the outer G-Ring must have at least four G-Ring points.
    ///
    /// Source: FGDC CSDGM §1.5.2.1 — `[4{G-Ring_Point}n | G-Ring]`
    pub struct FgdcGPolygonOuterRingHasAtLeastFourPoints;

    /// G-Ring latitude component must be in [-90.0, 90.0].
    ///
    /// Source: FGDC CSDGM §1.5.2.1.1.1 — domain: -90.0 <= G-Ring Latitude <= 90.0
    pub struct FgdcGRingLatitudeInRange;

    /// G-Ring longitude component must be in [-180.0, 180.0).
    ///
    /// Source: FGDC CSDGM §1.5.2.1.1.2 — domain: -180.0 <= G-Ring Longitude < 180.0
    pub struct FgdcGRingLongitudeInRange;

    // §1.6 Keywords

    /// Keywords must contain at least one Theme group.
    ///
    /// Source: FGDC CSDGM §1.6 — `Keywords = 1{Theme}n + ...`
    pub struct FgdcKeywordsHasAtLeastOneTheme;

    /// Each Theme group must provide a keyword thesaurus.
    ///
    /// Source: FGDC CSDGM §1.6.1 — `Theme = Theme_Keyword_Thesaurus + 1{Theme_Keyword}n`
    pub struct FgdcThemeHasKeywordThesaurus;

    /// Each Theme group must have at least one keyword.
    ///
    /// Source: FGDC CSDGM §1.6.1 — `Theme = Theme_Keyword_Thesaurus + 1{Theme_Keyword}n`
    pub struct FgdcThemeHasAtLeastOneKeyword;

    /// A Place keyword group, when present, must include a thesaurus.
    ///
    /// Source: FGDC CSDGM §1.6.2 — `Place = Place_Keyword_Thesaurus + 1{Place_Keyword}n`
    pub struct FgdcPlaceHasKeywordThesaurus;

    /// A Place keyword group, when present, must include at least one keyword.
    ///
    /// Source: FGDC CSDGM §1.6.2 — `Place = Place_Keyword_Thesaurus + 1{Place_Keyword}n`
    pub struct FgdcPlaceHasAtLeastOneKeyword;

    /// A Stratum keyword group, when present, must include a thesaurus.
    ///
    /// Source: FGDC CSDGM §1.6.3 — `Stratum = Stratum_Keyword_Thesaurus + 1{Stratum_Keyword}n`
    pub struct FgdcStratumHasKeywordThesaurus;

    /// A Stratum keyword group, when present, must include at least one keyword.
    ///
    /// Source: FGDC CSDGM §1.6.3 — `Stratum = Stratum_Keyword_Thesaurus + 1{Stratum_Keyword}n`
    pub struct FgdcStratumHasAtLeastOneKeyword;

    /// A Temporal keyword group, when present, must include a thesaurus.
    ///
    /// Source: FGDC CSDGM §1.6.4 — `Temporal = Temporal_Keyword_Thesaurus + 1{Temporal_Keyword}n`
    pub struct FgdcTemporalKeywordHasThesaurus;

    /// A Temporal keyword group, when present, must include at least one keyword.
    ///
    /// Source: FGDC CSDGM §1.6.4 — `Temporal = Temporal_Keyword_Thesaurus + 1{Temporal_Keyword}n`
    pub struct FgdcTemporalKeywordHasAtLeastOneKeyword;

    // §1.7 Access Constraints

    /// Access_Constraints is mandatory (domain: "None" | free text).
    ///
    /// Source: FGDC CSDGM §1.7 — mandatory, no surrounding parentheses in production rule
    pub struct FgdcAccessConstraintsPresent;

    // §1.8 Use Constraints

    /// Use_Constraints is mandatory (domain: "None" | free text).
    ///
    /// Source: FGDC CSDGM §1.8 — mandatory, no surrounding parentheses in production rule
    pub struct FgdcUseConstraintsPresent;

    // §1.10 Browse Graphic (optional; all sub-elements mandatory when present)

    /// Browse_Graphic_File_Name is mandatory when Browse_Graphic is present.
    ///
    /// Source: FGDC CSDGM §1.10 — `Browse_Graphic = Browse_Graphic_File_Name + ...`
    pub struct FgdcBrowseGraphicFileNamePresent;

    /// Browse_Graphic_File_Description is mandatory when Browse_Graphic is present.
    ///
    /// Source: FGDC CSDGM §1.10 — `... + Browse_Graphic_File_Description + ...`
    pub struct FgdcBrowseGraphicFileDescriptionPresent;

    /// Browse_Graphic_File_Type is mandatory when Browse_Graphic is present.
    ///
    /// Source: FGDC CSDGM §1.10 — `... + Browse_Graphic_File_Type`
    pub struct FgdcBrowseGraphicFileTypePresent;

    // §1.12 Security Information (optional; all sub-elements mandatory when present)

    /// Security_Classification_System is mandatory when Security_Information is present.
    ///
    /// Source: FGDC CSDGM §1.12 — `Security_Information = Security_Classification_System + ...`
    pub struct FgdcSecurityClassificationSystemPresent;

    /// Security_Classification must be a valid FGDC code or free text.
    ///
    /// Source: FGDC CSDGM §1.12.2 — domain: "Top secret" "Secret" "Confidential"
    /// "Restricted" "Unclassified" "Sensitive" free text
    pub struct FgdcSecurityClassificationCodeValid;

    /// Security_Handling_Description is mandatory when Security_Information is present.
    ///
    /// Source: FGDC CSDGM §1.12 — `... + Security_Handling_Description`
    pub struct FgdcSecurityHandlingDescriptionPresent;

    // ── §2 Data Quality Information (optional section) ────────────────────────

    /// Logical_Consistency_Report is mandatory when the Data_Quality section is present.
    ///
    /// Source: FGDC CSDGM §2 — `Data_Quality_Information = ... + Logical_Consistency_Report + ...`
    pub struct FgdcDataQualityLogicalConsistencyPresent;

    /// Completeness_Report is mandatory when the Data_Quality section is present.
    ///
    /// Source: FGDC CSDGM §2 — `... + Completeness_Report + ...`
    pub struct FgdcDataQualityCompletenessReportPresent;

    /// Lineage is mandatory when the Data_Quality section is present.
    ///
    /// Source: FGDC CSDGM §2 — `... + Lineage`
    pub struct FgdcDataQualityLineagePresent;

    // §2.1 Attribute Accuracy (optional within §2)

    /// Attribute_Accuracy_Report is mandatory when Attribute_Accuracy is present.
    ///
    /// Source: FGDC CSDGM §2.1 — `Attribute_Accuracy = Attribute_Accuracy_Report + ...`
    pub struct FgdcAttributeAccuracyReportPresent;

    /// Attribute_Accuracy_Value and Attribute_Accuracy_Explanation must appear together.
    ///
    /// Source: FGDC CSDGM §2.1.2 — both elements mandatory in the compound assessment
    pub struct FgdcQaaValueAndExplanationPaired;

    // §2.4 Positional Accuracy (optional within §2)

    /// When Positional_Accuracy is present it must include at least one H or V component.
    ///
    /// Source: FGDC CSDGM §2.4 — `Positional_Accuracy = 0{Horizontal}1 + 0{Vertical}1`
    pub struct FgdcPositionalAccuracyHasAtLeastOneComponent;

    /// Quantitative_Horizontal_Positional_Accuracy_Assessment: Value and Explanation
    /// must appear together.
    ///
    /// Source: FGDC CSDGM §2.4.1.2 — both elements mandatory in the compound assessment
    pub struct FgdcHorizAccuracyAssessmentPaired;

    /// Quantitative_Vertical_Positional_Accuracy_Assessment: Value and Explanation
    /// must appear together.
    ///
    /// Source: FGDC CSDGM §2.4.2.2 — both elements mandatory in the compound assessment
    pub struct FgdcVertAccuracyAssessmentPaired;

    // §2.5 Lineage

    /// Lineage must contain at least one Process_Step.
    ///
    /// Source: FGDC CSDGM §2.5 — `Lineage = 0{Source_Information}n + 1{Process_Step}n`
    pub struct FgdcLineageHasAtLeastOneProcessStep;

    // §2.5.1 Source Information (optional; all sub-elements mandatory when present)

    /// Source_Citation is mandatory when Source_Information is present.
    ///
    /// Source: FGDC CSDGM §2.5.1 — `Source_Information = Source_Citation + ...`
    pub struct FgdcSourceCitationPresent;

    /// Type_of_Source_Media is mandatory when Source_Information is present.
    ///
    /// Source: FGDC CSDGM §2.5.1 — `... + Type_of_Source_Media + ...`
    pub struct FgdcSourceMediaTypePresent;

    /// Source_Time_Period_of_Content is mandatory when Source_Information is present.
    ///
    /// Source: FGDC CSDGM §2.5.1 — `... + Source_Time_Period_of_Content + ...`
    pub struct FgdcSourceTimePeriodPresent;

    /// Source_Citation_Abbreviation is mandatory when Source_Information is present.
    ///
    /// Source: FGDC CSDGM §2.5.1 — `... + Source_Citation_Abbreviation + ...`
    pub struct FgdcSourceCitationAbbreviationPresent;

    /// Source_Contribution is mandatory when Source_Information is present.
    ///
    /// Source: FGDC CSDGM §2.5.1 — `... + Source_Contribution`
    pub struct FgdcSourceContributionPresent;

    /// Source_Scale_Denominator, when present, must be > 1.
    ///
    /// Source: FGDC CSDGM §2.5.1.2 — domain: Source Scale Denominator > 1
    pub struct FgdcSourceScaleDenominatorGt1;

    // §2.5.2 Process Step

    /// Process_Description is mandatory in each Process_Step.
    ///
    /// Source: FGDC CSDGM §2.5.2 — `Process_Step = Process_Description + ...`
    pub struct FgdcProcessStepDescriptionPresent;

    /// Process_Date is mandatory in each Process_Step.
    ///
    /// Source: FGDC CSDGM §2.5.2 — `... + Process_Date + ...`
    pub struct FgdcProcessStepDatePresent;

    /// Process_Date must be YYYYMMDD, "Unknown", or "Not complete".
    ///
    /// Source: FGDC CSDGM §2.5.2.3 — domain: "Unknown" "Not complete" free date
    pub struct FgdcProcessStepDateFgdcFormat;

    // §2.6 Cloud Cover (optional)

    /// Cloud_Cover, when present, must be an integer in [0, 100] or "Unknown".
    ///
    /// Source: FGDC CSDGM §2.6 — domain: 0 <= Cloud Cover <= 100 "Unknown"
    pub struct FgdcCloudCoverZeroToHundred;

    // ── §3 Spatial Data Organization Information (optional section) ───────────

    /// Direct_Spatial_Reference_Method must be "Point", "Vector", or "Raster".
    ///
    /// Source: FGDC CSDGM §3.2 — domain: "Point" "Vector" "Raster"
    pub struct FgdcDirectSpatialRefMethodCodeValid;

    /// Point_and_Vector_Object_Information uses either SDTS or VPF terms — not both.
    ///
    /// Source: FGDC CSDGM §3.3 — `[1{SDTS_Terms_Description}n | VPF_Terms_Description]`
    pub struct FgdcPvectInfoIsSdtsOrVpfExclusive;

    /// SDTS_Point_and_Vector_Object_Type must be from the SDTS domain.
    ///
    /// Source: FGDC CSDGM §3.3.1.1 — domain: enumerated SDTS object type codes
    pub struct FgdcSdtsObjectTypeCodeValid;

    /// Point_and_Vector_Object_Count, when present, must be > 0.
    ///
    /// Source: FGDC CSDGM §3.3.1.2 — domain: Point and Vector Object Count > 0
    pub struct FgdcSdtsObjectCountPositive;

    /// VPF_Topology_Level must be in [0, 3].
    ///
    /// Source: FGDC CSDGM §3.3.2.1 — domain: 0 <= VPF Topology Level <= 3
    pub struct FgdcVpfTopologyLevelZeroToThree;

    /// VPF_Point_and_Vector_Object_Type must be "Node", "Edge", "Face", or "Text".
    ///
    /// Source: FGDC CSDGM §3.3.2.2.1 — domain: "Node" "Edge" "Face" "Text"
    pub struct FgdcVpfObjectTypeCodeValid;

    /// Raster_Object_Type must be "Point", "Pixel", "Grid Cell", or "Voxel".
    ///
    /// Source: FGDC CSDGM §3.4.1 — domain: "Point" "Pixel" "Grid Cell" "Voxel"
    pub struct FgdcRasterObjectTypeCodeValid;

    /// Row_Count must be > 0.
    ///
    /// Source: FGDC CSDGM §3.4.2 — domain: Row Count > 0
    pub struct FgdcRasterRowCountPositive;

    /// Column_Count must be > 0.
    ///
    /// Source: FGDC CSDGM §3.4.3 — domain: Column Count > 0
    pub struct FgdcRasterColumnCountPositive;

    /// Vertical_Count, when present, must be > 0.
    ///
    /// Source: FGDC CSDGM §3.4.4 — domain: Depth Count > 0
    pub struct FgdcRasterVerticalCountPositive;

    // ── §4 Spatial Reference Information (optional section) ──────────────────

    /// Horizontal_Coordinate_System_Definition must use exactly one of Geographic,
    /// Planar (one or more), or Local — an exclusive choice.
    ///
    /// Source: FGDC CSDGM §4.1 — `[Geographic | 1{Planar}n | Local]`
    pub struct FgdcHorizCrsTypeIsGeographicPlanarOrLocal;

    // §4.1.1 Geographic

    /// Latitude_Resolution must be > 0.0.
    ///
    /// Source: FGDC CSDGM §4.1.1.1 — domain: Latitude Resolution > 0.0
    pub struct FgdcGeographicLatResolutionPositive;

    /// Longitude_Resolution must be > 0.0.
    ///
    /// Source: FGDC CSDGM §4.1.1.2 — domain: Longitude Resolution > 0.0
    pub struct FgdcGeographicLonResolutionPositive;

    /// Geographic_Coordinate_Units must be one of the FGDC-defined code values.
    ///
    /// Source: FGDC CSDGM §4.1.1.3 — domain: "Decimal degrees" "Decimal minutes"
    /// "Decimal seconds" "Degrees and decimal minutes"
    /// "Degrees, minutes, and decimal seconds" "Radians" "Grads"
    pub struct FgdcGeographicCoordUnitsCodeValid;

    // §4.1.2 Map Projection parameters

    /// Map_Projection_Name is mandatory within a Map_Projection element.
    ///
    /// Source: FGDC CSDGM §4.1.2.1 — `Map_Projection = Map_Projection_Name + [...]`
    pub struct FgdcMapProjNamePresent;

    /// Standard_Parallel must be in [-90.0, 90.0].
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.1 — domain: -90.0 <= Standard Parallel <= 90.0
    pub struct FgdcMapProjStandardParallelInRange;

    /// Longitude_of_Central_Meridian must be in [-180.0, 180.0).
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.2 — domain: -180.0 <= Central Meridian < 180.0
    pub struct FgdcMapProjCentralMeridianInRange;

    /// Latitude_of_Projection_Origin must be in [-90.0, 90.0].
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.3 — domain: -90.0 <= Latitude of Projection Origin <= 90.0
    pub struct FgdcMapProjLatitudeOriginInRange;

    /// Scale factors (at equator, center line, projection origin, central meridian)
    /// must all be > 0.0.
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.6/.10/.14/.17
    pub struct FgdcMapProjScaleFactorPositive;

    /// Azimuthal_Angle must be in [0.0, 360.0).
    ///
    /// Source: FGDC CSDGM §4.1.2.1.23.11.1 — domain: 0.0 <= Azimuthal Angle < 360.0
    pub struct FgdcMapProjAzimuthalAngle0To360;

    // §4.1.2.2 Grid Coordinate System

    /// UTM_Zone_Number must be in [1, 60] (north) or [-60, -1] (south).
    ///
    /// Source: FGDC CSDGM §4.1.2.2.2.1 — domain: 1 <= N <= 60; -60 <= S <= -1
    pub struct FgdcUtmZoneNumberInRange;

    /// UPS_Zone_Identifier must be "A", "B", "Y", or "Z".
    ///
    /// Source: FGDC CSDGM §4.1.2.2.3.1 — domain: "A" "B" "Y" "Z"
    pub struct FgdcUpsZoneIdentifierCodeValid;

    /// ARC_System_Zone_Identifier must be in [1, 18].
    ///
    /// Source: FGDC CSDGM §4.1.2.2.5.1 — domain: 1 <= ARC Zone <= 18
    pub struct FgdcArcZoneIdentifier1To18;

    // §4.1 Geodetic Model (optional, within Horizontal CRS)

    /// Ellipsoid_Name is mandatory when Geodetic_Model is present.
    ///
    /// Source: FGDC CSDGM §4.1 — `Geodetic_Model = [Horizontal_Datum_Name] + Ellipsoid_Name + ...`
    pub struct FgdcGeodeticModelEllipsoidNamePresent;

    /// Semi-major_Axis must be > 0.
    ///
    /// Source: FGDC CSDGM §4.1 — implied numeric constraint; ellipsoid semi-major axis > 0
    pub struct FgdcGeodeticModelSemiMajorAxisPositive;

    /// Denominator_of_Flattening_Ratio must be > 0.
    ///
    /// Source: FGDC CSDGM §4.1 — implied constraint; denominator > 0
    pub struct FgdcGeodeticModelFlatteningRatioDenominatorPositive;

    // §4.2 Altitude System Definition (optional)

    /// Altitude_Datum_Name is mandatory when Altitude_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `Altitude_System_Definition = Altitude_Datum_Name + ...`
    pub struct FgdcAltitudeDatumNamePresent;

    /// Altitude_System_Definition must have at least one Altitude_Resolution value.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + 1{Altitude_Resolution}n + ...`
    pub struct FgdcAltitudeResolutionAtLeastOne;

    /// Altitude_Distance_Units is mandatory when Altitude_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + Altitude_Distance_Units + ...`
    pub struct FgdcAltitudeDistanceUnitsPresent;

    /// Altitude_Encoding_Method is mandatory when Altitude_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + Altitude_Encoding_Method`
    pub struct FgdcAltitudeEncodingMethodPresent;

    // §4.2 Depth System Definition (optional)

    /// Depth_Datum_Name is mandatory when Depth_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `Depth_System_Definition = Depth_Datum_Name + ...`
    pub struct FgdcDepthDatumNamePresent;

    /// Depth_System_Definition must have at least one Depth_Resolution value.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + 1{Depth_Resolution}n + ...`
    pub struct FgdcDepthResolutionAtLeastOne;

    /// Depth_Distance_Units is mandatory when Depth_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + Depth_Distance_Units + ...`
    pub struct FgdcDepthDistanceUnitsPresent;

    /// Depth_Encoding_Method is mandatory when Depth_System_Definition is present.
    ///
    /// Source: FGDC CSDGM §4.2 — `... + Depth_Encoding_Method`
    pub struct FgdcDepthEncodingMethodPresent;

    // ── §5 Entity and Attribute Information (optional section) ────────────────

    /// Entity_and_Attribute_Information must contain Detailed_Description,
    /// Overview_Description, or both.
    ///
    /// Source: FGDC CSDGM §5 — `[1{Detailed_Description}n | 1{Overview_Description}n | both]`
    pub struct FgdcEntityAttributeHasDetailedOrOverview;

    // §5.1.1 Entity Type

    /// Entity_Type_Label is mandatory in each Entity_Type.
    ///
    /// Source: FGDC CSDGM §5.1.1 — `Entity_Type = Entity_Type_Label + ...`
    pub struct FgdcEntityTypeLabelPresent;

    /// Entity_Type_Definition is mandatory in each Entity_Type.
    ///
    /// Source: FGDC CSDGM §5.1.1 — `... + Entity_Type_Definition + ...`
    pub struct FgdcEntityTypeDefinitionPresent;

    /// Entity_Type_Definition_Source is mandatory in each Entity_Type.
    ///
    /// Source: FGDC CSDGM §5.1.1 — `... + Entity_Type_Definition_Source`
    pub struct FgdcEntityTypeDefinitionSourcePresent;

    // §5.1.2 Attribute

    /// Attribute_Label is mandatory in each Attribute.
    ///
    /// Source: FGDC CSDGM §5.1.2 — `Attribute = Attribute_Label + ...`
    pub struct FgdcAttributeLabelPresent;

    /// Attribute_Definition is mandatory in each Attribute.
    ///
    /// Source: FGDC CSDGM §5.1.2 — `... + Attribute_Definition + ...`
    pub struct FgdcAttributeDefinitionPresent;

    /// Attribute_Definition_Source is mandatory in each Attribute.
    ///
    /// Source: FGDC CSDGM §5.1.2 — `... + Attribute_Definition_Source + ...`
    pub struct FgdcAttributeDefinitionSourcePresent;

    /// Each Attribute must have at least one Attribute_Domain_Values element.
    ///
    /// Source: FGDC CSDGM §5.1.2 — `... + 1{Attribute_Domain_Values}n + ...`
    pub struct FgdcAttributeHasAtLeastOneDomain;

    /// Attribute_Domain_Values uses exactly one of: Enumerated_Domain, Range_Domain,
    /// Codeset_Domain, or Unrepresentable_Domain.
    ///
    /// Source: FGDC CSDGM §5.1.2.4 — `[Enumerated | Range | Codeset | Unrepresentable]`
    pub struct FgdcAttributeDomainTypeExclusive;

    // §5.1.2.4.1 Enumerated Domain

    /// Enumerated_Domain must have at least one value entry.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.1 — `1{Enumerated_Domain_Value + ...}n`
    pub struct FgdcEnumeratedDomainHasAtLeastOneValue;

    /// Enumerated_Domain_Value must be present in each entry.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.1
    pub struct FgdcEnumeratedDomainValuePresent;

    /// Enumerated_Domain_Value_Definition must be present in each entry.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.1
    pub struct FgdcEnumeratedDomainValueDefinitionPresent;

    /// Enumerated_Domain_Value_Definition_Source must be present in each entry.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.1
    pub struct FgdcEnumeratedDomainValueDefinitionSourcePresent;

    // §5.1.2.4.2 Range Domain

    /// Range_Domain_Minimum is mandatory in a Range_Domain.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2 — `Range_Domain = Range_Domain_Minimum + ...`
    pub struct FgdcRangeDomainMinimumPresent;

    /// Range_Domain_Maximum is mandatory in a Range_Domain.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2 — `... + Range_Domain_Maximum + ...`
    pub struct FgdcRangeDomainMaximumPresent;

    /// Range_Domain_Minimum must be <= Range_Domain_Maximum.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2 — implied ordering invariant from min/max semantics
    pub struct FgdcRangeDomainMinimumLeMaximum;

    /// Attribute_Measurement_Resolution, when present, must be > 0.0.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.2.4 — domain: Attribute Measurement Resolution > 0.0
    pub struct FgdcAttributeMeasurementResolutionPositive;

    // §5.1.2.4.3 Codeset Domain

    /// Codeset_Name is mandatory in a Codeset_Domain.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.3 — `Codeset_Domain = Codeset_Name + Codeset_Source`
    pub struct FgdcCodesetNamePresent;

    /// Codeset_Source is mandatory in a Codeset_Domain.
    ///
    /// Source: FGDC CSDGM §5.1.2.4.3 — `Codeset_Domain = Codeset_Name + Codeset_Source`
    pub struct FgdcCodesetSourcePresent;

    // §5.2 Overview Description

    /// Entity_and_Attribute_Overview text is mandatory in Overview_Description.
    ///
    /// Source: FGDC CSDGM §5.2 — `Overview_Description = Entity_and_Attribute_Overview + ...`
    pub struct FgdcOverviewDescriptionTextPresent;

    /// Overview_Description must have at least one Entity_and_Attribute_Detail_Citation.
    ///
    /// Source: FGDC CSDGM §5.2 — `... + 1{Entity_and_Attribute_Detail_Citation}n`
    pub struct FgdcOverviewDetailCitationAtLeastOne;

    // ── §6 Distribution Information (optional, repeatable) ───────────────────

    /// Distributor is mandatory when Distribution_Information is present.
    ///
    /// Source: FGDC CSDGM §6 — `Distribution_Information = Distributor + ...`
    pub struct FgdcDistributionDistributorPresent;

    /// Distribution_Liability is mandatory when Distribution_Information is present.
    ///
    /// Source: FGDC CSDGM §6 — `... + Distribution_Liability + ...`
    pub struct FgdcDistributionLiabilityPresent;

    /// Standard_Order_Process must include either a Non-digital form or at least one
    /// Digital form — an exclusive choice.
    ///
    /// Source: FGDC CSDGM §6.4 — `[Non-digital_Form | 1{Digital_Form}n]`
    pub struct FgdcStandardOrderHasFormNondigitalOrDigital;

    /// Fees is mandatory in each Standard_Order_Process.
    ///
    /// Source: FGDC CSDGM §6.4 — `... + Fees + ...`
    pub struct FgdcStandardOrderFeesPresent;

    // §6.4.2.1 Digital Transfer Information

    /// Format_Name is mandatory in Digital_Transfer_Information.
    ///
    /// Source: FGDC CSDGM §6.4.2.1 — `Digital_Transfer_Information = Format_Name + ...`
    pub struct FgdcDigitalFormatNamePresent;

    /// Transfer_Size, when present, must be > 0.0.
    ///
    /// Source: FGDC CSDGM §6.4.2.1.7 — domain: Transfer Size > 0.0
    pub struct FgdcTransferSizePositive;

    // §6.4.2.2.1.1.2 Dialup Instructions

    /// Lowest_BPS must be >= 110.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.1 — domain: Lowest BPS >= 110
    pub struct FgdcDialupLowestBpsGeq110;

    /// Highest_BPS, when present, must be > Lowest_BPS.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.2 — domain: Highest BPS > Lowest BPS
    pub struct FgdcDialupHighestBpsGtLowest;

    /// Number_DataBits must be 7 or 8.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.3 — domain: 7 <= Number DataBits <= 8
    pub struct FgdcDialupDataBitsSevenOrEight;

    /// Number_StopBits must be 1 or 2.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.4 — domain: 1 <= Number StopBits <= 2
    pub struct FgdcDialupStopBitsOneOrTwo;

    /// Parity must be "None", "Odd", "Even", "Mark", or "Space".
    ///
    /// Source: FGDC CSDGM §6.4.2.2.1.1.2.5 — domain: "None" "Odd" "Even" "Mark" "Space"
    pub struct FgdcDialupParityCodeValid;

    // §6.4.2.2.2 Offline Option

    /// Offline_Media must be a valid FGDC code or free text.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.1 — domain: "CD-ROM" "3-1/2 inch floppy disk"
    /// "5-1/4 inch floppy disk" "9-track tape" "4 mm cartridge tape"
    /// "8 mm cartridge tape" "1/4-inch cartridge tape" free text
    pub struct FgdcOfflineMediaCodeValid;

    /// Recording_Density, when present, must be > 0.0.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.2.1 — domain: Recording Density > 0.0
    pub struct FgdcRecordingDensityPositive;

    /// Recording_Format must be a valid FGDC code or free text.
    ///
    /// Source: FGDC CSDGM §6.4.2.2.2.3 — domain: "cpio" "tar" "High Sierra" "ISO 9660"
    /// "ISO 9660 with Rock Ridge extensions" "ISO 9660 with Apple HFS extensions" free text
    pub struct FgdcRecordingFormatCodeValid;

    // ── §7 Metadata Reference Information (mandatory section) ─────────────────

    /// Metadata_Date is mandatory in Metadata_Reference_Information.
    ///
    /// Source: FGDC CSDGM §7 — `Metadata_Reference_Information = Metadata_Date + ...`
    pub struct FgdcMetadataDatePresent;

    /// Metadata_Contact is mandatory in Metadata_Reference_Information.
    ///
    /// Source: FGDC CSDGM §7 — `... + Metadata_Contact + ...`
    pub struct FgdcMetadataContactPresent;

    /// Metadata_Standard_Name is mandatory in Metadata_Reference_Information.
    ///
    /// Source: FGDC CSDGM §7 — `... + Metadata_Standard_Name + ...`
    pub struct FgdcMetadataStandardNamePresent;

    /// Metadata_Standard_Version is mandatory in Metadata_Reference_Information.
    ///
    /// Source: FGDC CSDGM §7 — `... + Metadata_Standard_Version + ...`
    pub struct FgdcMetadataStandardVersionPresent;

    /// Metadata_Review_Date, when present, must be later than Metadata_Date.
    ///
    /// Source: FGDC CSDGM §7.2 — domain: Metadata Review Date later than Metadata Date
    pub struct FgdcMetadataReviewDateAfterMetadataDate;

    /// Metadata_Future_Review_Date, when present, must be later than Metadata_Review_Date.
    ///
    /// Source: FGDC CSDGM §7.3 — domain: Future Review Date later than Metadata Review Date
    pub struct FgdcMetadataFutureReviewDateAfterReviewDate;

    /// Metadata_Time_Convention, when present, must be a valid FGDC code.
    ///
    /// Source: FGDC CSDGM §7.7 — domain: "local time"
    /// "local time with time differential factor" "universal time"
    pub struct FgdcMetadataTimeConventionCodeValid;

    /// Metadata_Security_Classification, when present, must be a valid FGDC code or free text.
    ///
    /// Source: FGDC CSDGM §7.10.2 — domain: "Top secret" "Secret" "Confidential"
    /// "Restricted" "Unclassified" "Sensitive" free text
    pub struct FgdcMetadataSecurityClassificationCodeValid;

    // ── §9 Time Period Information ─────────────────────────────────────────────

    /// Time_Period_Information must use exactly one form: Single, Multiple, or Range.
    ///
    /// Source: FGDC CSDGM §9 — `[Single_Date/Time | Multiple_Dates/Times | Range_of_Dates/Times]`
    pub struct FgdcTimePeriodTypeExclusive;

    /// Calendar_Date must be YYYYMMDD (full or partial) or "Unknown".
    ///
    /// Source: FGDC CSDGM §9.1.1 — domain: "Unknown" free date (FGDC date format)
    pub struct FgdcCalendarDateIsYyyymmddOrToken;

    /// In a Range_of_Dates, Ending_Date must be chronologically after Beginning_Date
    /// (unless either is the "Unknown" token).
    ///
    /// Source: FGDC CSDGM §9.3 — implied by range semantics
    pub struct FgdcRangeEndingDateAfterBeginning;

    // ── §10 Contact Information ───────────────────────────────────────────────

    /// Contact_Information must have either Contact_Person_Primary or
    /// Contact_Organization_Primary — an exclusive choice.
    ///
    /// Source: FGDC CSDGM §10 — `[Contact_Person_Primary | Contact_Organization_Primary]`
    pub struct FgdcContactHasPersonOrOrganizationPrimary;

    /// Contact_Information must have at least one Contact_Address.
    ///
    /// Source: FGDC CSDGM §10 — `... + 1{Contact_Address}n + ...`
    pub struct FgdcContactHasAtLeastOneAddress;

    /// Contact_Information must have at least one Contact_Voice_Telephone.
    ///
    /// Source: FGDC CSDGM §10 — `... + 1{Contact_Voice_Telephone}n + ...`
    pub struct FgdcContactHasAtLeastOneVoiceTelephone;

    /// Address_Type must be "mailing", "physical", "mailing and physical", or free text.
    ///
    /// Source: FGDC CSDGM §10.4.1 — domain: "mailing" "physical" "mailing and physical" free text
    pub struct FgdcContactAddressTypeCodeValid;

    /// City is mandatory in each Contact_Address.
    ///
    /// Source: FGDC CSDGM §10.4 — `Contact_Address = Address_Type + [Address] + City + ...`
    pub struct FgdcContactAddressCityPresent;

    /// State_or_Province is mandatory in each Contact_Address.
    ///
    /// Source: FGDC CSDGM §10.4 — `... + State_or_Province + ...`
    pub struct FgdcContactAddressStatePresent;

    /// Postal_Code is mandatory in each Contact_Address.
    ///
    /// Source: FGDC CSDGM §10.4 — `... + Postal_Code + ...`
    pub struct FgdcContactAddressPostalCodePresent;

    // ── Aggregate validity seam props ─────────────────────────────────────────

    /// All mandatory section 1 Identification elements are present and individually valid.
    ///
    /// Source: FGDC CSDGM §1 — section completeness seam
    pub struct FgdcIdentificationSectionValid;

    /// All mandatory section 2 Data Quality elements are present when the section is included.
    ///
    /// Source: FGDC CSDGM §2 — section completeness seam
    pub struct FgdcDataQualitySectionValid;

    /// All section 3 Spatial Data Organization constraints are satisfied when included.
    ///
    /// Source: FGDC CSDGM §3 — section completeness seam
    pub struct FgdcSpatialDataOrgSectionValid;

    /// All section 4 Spatial Reference constraints are satisfied when the section is included.
    ///
    /// Source: FGDC CSDGM §4 — section completeness seam
    pub struct FgdcSpatialReferenceSectionValid;

    /// All section 5 Entity and Attribute constraints are satisfied when included.
    ///
    /// Source: FGDC CSDGM §5 — section completeness seam
    pub struct FgdcEntityAttributeSectionValid;

    /// All mandatory section 6 Distribution elements are present when the section is included.
    ///
    /// Source: FGDC CSDGM §6 — section completeness seam
    pub struct FgdcDistributionSectionValid;

    /// All mandatory section 7 Metadata Reference elements are present and valid.
    ///
    /// Source: FGDC CSDGM §7 — section completeness seam
    pub struct FgdcMetadataReferenceSectionValid;

    /// All mandatory section 8 Citation Information elements are present and valid.
    ///
    /// Source: FGDC CSDGM §8 — citation completeness seam
    pub struct FgdcCitationInfoValid;

    /// Time_Period_Information uses exactly one form and all elements are valid.
    ///
    /// Source: FGDC CSDGM §9 — time period completeness seam
    pub struct FgdcTimePeriodInfoValid;

    /// Contact_Information has all mandatory elements present and valid.
    ///
    /// Source: FGDC CSDGM §10 — contact completeness seam
    pub struct FgdcContactInfoValid;

    /// All FGDC-mandatory elements across all present sections satisfy their constraints;
    /// the metadata record is a conformant FGDC CSDGM record.
    ///
    /// Source: FGDC CSDGM §0 — full record conformance
    pub struct FgdcRecordValid;

    // ── Prop implementations ─────────────────────────────────────────────────

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by FGDC CSDGM contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by FGDC CSDGM contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by FGDC CSDGM contract */ }
                }
            }
        };
    }

    // §0
    structural_prop!(
        FgdcMetadataHasIdentificationSection,
        "FgdcMetadataHasIdentificationSection"
    );
    structural_prop!(
        FgdcMetadataHasMetadataReferenceSection,
        "FgdcMetadataHasMetadataReferenceSection"
    );
    // §1 Citation / §8
    structural_prop!(
        FgdcCitationHasAtLeastOneOriginator,
        "FgdcCitationHasAtLeastOneOriginator"
    );
    structural_prop!(
        FgdcCitationOriginatorNonEmpty,
        "FgdcCitationOriginatorNonEmpty"
    );
    structural_prop!(
        FgdcCitationPublicationDatePresent,
        "FgdcCitationPublicationDatePresent"
    );
    structural_prop!(
        FgdcCitationPublicationDateIsYyyymmddOrToken,
        "FgdcCitationPublicationDateIsYyyymmddOrToken"
    );
    structural_prop!(FgdcCitationTitlePresent, "FgdcCitationTitlePresent");
    structural_prop!(FgdcCitationTitleNonEmpty, "FgdcCitationTitleNonEmpty");
    // §1 Description
    structural_prop!(
        FgdcDescriptionAbstractPresent,
        "FgdcDescriptionAbstractPresent"
    );
    structural_prop!(
        FgdcDescriptionPurposePresent,
        "FgdcDescriptionPurposePresent"
    );
    // §1 Time of Content
    structural_prop!(
        FgdcTimeOfContentTimePeriodPresent,
        "FgdcTimeOfContentTimePeriodPresent"
    );
    structural_prop!(
        FgdcTimeOfContentCurrentnessReferenceValid,
        "FgdcTimeOfContentCurrentnessReferenceValid"
    );
    // §1 Status
    structural_prop!(FgdcStatusProgressCodeValid, "FgdcStatusProgressCodeValid");
    structural_prop!(
        FgdcStatusUpdateFrequencyCodeValid,
        "FgdcStatusUpdateFrequencyCodeValid"
    );
    // §1 Bounding Coordinates
    structural_prop!(FgdcBoundingWestCoordInRange, "FgdcBoundingWestCoordInRange");
    structural_prop!(FgdcBoundingEastCoordInRange, "FgdcBoundingEastCoordInRange");
    structural_prop!(
        FgdcBoundingNorthCoordInRange,
        "FgdcBoundingNorthCoordInRange"
    );
    structural_prop!(
        FgdcBoundingSouthCoordInRange,
        "FgdcBoundingSouthCoordInRange"
    );
    structural_prop!(FgdcBoundingNorthGeqSouth, "FgdcBoundingNorthGeqSouth");
    // §1 G-Polygon
    structural_prop!(
        FgdcGPolygonOuterRingHasAtLeastFourPoints,
        "FgdcGPolygonOuterRingHasAtLeastFourPoints"
    );
    structural_prop!(FgdcGRingLatitudeInRange, "FgdcGRingLatitudeInRange");
    structural_prop!(FgdcGRingLongitudeInRange, "FgdcGRingLongitudeInRange");
    // §1 Keywords
    structural_prop!(
        FgdcKeywordsHasAtLeastOneTheme,
        "FgdcKeywordsHasAtLeastOneTheme"
    );
    structural_prop!(FgdcThemeHasKeywordThesaurus, "FgdcThemeHasKeywordThesaurus");
    structural_prop!(
        FgdcThemeHasAtLeastOneKeyword,
        "FgdcThemeHasAtLeastOneKeyword"
    );
    structural_prop!(FgdcPlaceHasKeywordThesaurus, "FgdcPlaceHasKeywordThesaurus");
    structural_prop!(
        FgdcPlaceHasAtLeastOneKeyword,
        "FgdcPlaceHasAtLeastOneKeyword"
    );
    structural_prop!(
        FgdcStratumHasKeywordThesaurus,
        "FgdcStratumHasKeywordThesaurus"
    );
    structural_prop!(
        FgdcStratumHasAtLeastOneKeyword,
        "FgdcStratumHasAtLeastOneKeyword"
    );
    structural_prop!(
        FgdcTemporalKeywordHasThesaurus,
        "FgdcTemporalKeywordHasThesaurus"
    );
    structural_prop!(
        FgdcTemporalKeywordHasAtLeastOneKeyword,
        "FgdcTemporalKeywordHasAtLeastOneKeyword"
    );
    // §1 Constraints
    structural_prop!(FgdcAccessConstraintsPresent, "FgdcAccessConstraintsPresent");
    structural_prop!(FgdcUseConstraintsPresent, "FgdcUseConstraintsPresent");
    // §1 Browse Graphic
    structural_prop!(
        FgdcBrowseGraphicFileNamePresent,
        "FgdcBrowseGraphicFileNamePresent"
    );
    structural_prop!(
        FgdcBrowseGraphicFileDescriptionPresent,
        "FgdcBrowseGraphicFileDescriptionPresent"
    );
    structural_prop!(
        FgdcBrowseGraphicFileTypePresent,
        "FgdcBrowseGraphicFileTypePresent"
    );
    // §1 Security Information
    structural_prop!(
        FgdcSecurityClassificationSystemPresent,
        "FgdcSecurityClassificationSystemPresent"
    );
    structural_prop!(
        FgdcSecurityClassificationCodeValid,
        "FgdcSecurityClassificationCodeValid"
    );
    structural_prop!(
        FgdcSecurityHandlingDescriptionPresent,
        "FgdcSecurityHandlingDescriptionPresent"
    );
    // §2 Data Quality
    structural_prop!(
        FgdcDataQualityLogicalConsistencyPresent,
        "FgdcDataQualityLogicalConsistencyPresent"
    );
    structural_prop!(
        FgdcDataQualityCompletenessReportPresent,
        "FgdcDataQualityCompletenessReportPresent"
    );
    structural_prop!(
        FgdcDataQualityLineagePresent,
        "FgdcDataQualityLineagePresent"
    );
    structural_prop!(
        FgdcAttributeAccuracyReportPresent,
        "FgdcAttributeAccuracyReportPresent"
    );
    structural_prop!(
        FgdcQaaValueAndExplanationPaired,
        "FgdcQaaValueAndExplanationPaired"
    );
    structural_prop!(
        FgdcPositionalAccuracyHasAtLeastOneComponent,
        "FgdcPositionalAccuracyHasAtLeastOneComponent"
    );
    structural_prop!(
        FgdcHorizAccuracyAssessmentPaired,
        "FgdcHorizAccuracyAssessmentPaired"
    );
    structural_prop!(
        FgdcVertAccuracyAssessmentPaired,
        "FgdcVertAccuracyAssessmentPaired"
    );
    // §2 Lineage
    structural_prop!(
        FgdcLineageHasAtLeastOneProcessStep,
        "FgdcLineageHasAtLeastOneProcessStep"
    );
    // §2 Source Information
    structural_prop!(FgdcSourceCitationPresent, "FgdcSourceCitationPresent");
    structural_prop!(FgdcSourceMediaTypePresent, "FgdcSourceMediaTypePresent");
    structural_prop!(FgdcSourceTimePeriodPresent, "FgdcSourceTimePeriodPresent");
    structural_prop!(
        FgdcSourceCitationAbbreviationPresent,
        "FgdcSourceCitationAbbreviationPresent"
    );
    structural_prop!(
        FgdcSourceContributionPresent,
        "FgdcSourceContributionPresent"
    );
    structural_prop!(
        FgdcSourceScaleDenominatorGt1,
        "FgdcSourceScaleDenominatorGt1"
    );
    // §2 Process Step
    structural_prop!(
        FgdcProcessStepDescriptionPresent,
        "FgdcProcessStepDescriptionPresent"
    );
    structural_prop!(FgdcProcessStepDatePresent, "FgdcProcessStepDatePresent");
    structural_prop!(
        FgdcProcessStepDateFgdcFormat,
        "FgdcProcessStepDateFgdcFormat"
    );
    // §2 Cloud Cover
    structural_prop!(FgdcCloudCoverZeroToHundred, "FgdcCloudCoverZeroToHundred");
    // §3 Spatial Data Organization
    structural_prop!(
        FgdcDirectSpatialRefMethodCodeValid,
        "FgdcDirectSpatialRefMethodCodeValid"
    );
    structural_prop!(
        FgdcPvectInfoIsSdtsOrVpfExclusive,
        "FgdcPvectInfoIsSdtsOrVpfExclusive"
    );
    structural_prop!(FgdcSdtsObjectTypeCodeValid, "FgdcSdtsObjectTypeCodeValid");
    structural_prop!(FgdcSdtsObjectCountPositive, "FgdcSdtsObjectCountPositive");
    structural_prop!(
        FgdcVpfTopologyLevelZeroToThree,
        "FgdcVpfTopologyLevelZeroToThree"
    );
    structural_prop!(FgdcVpfObjectTypeCodeValid, "FgdcVpfObjectTypeCodeValid");
    structural_prop!(
        FgdcRasterObjectTypeCodeValid,
        "FgdcRasterObjectTypeCodeValid"
    );
    structural_prop!(FgdcRasterRowCountPositive, "FgdcRasterRowCountPositive");
    structural_prop!(
        FgdcRasterColumnCountPositive,
        "FgdcRasterColumnCountPositive"
    );
    structural_prop!(
        FgdcRasterVerticalCountPositive,
        "FgdcRasterVerticalCountPositive"
    );
    // §4 Spatial Reference
    structural_prop!(
        FgdcHorizCrsTypeIsGeographicPlanarOrLocal,
        "FgdcHorizCrsTypeIsGeographicPlanarOrLocal"
    );
    structural_prop!(
        FgdcGeographicLatResolutionPositive,
        "FgdcGeographicLatResolutionPositive"
    );
    structural_prop!(
        FgdcGeographicLonResolutionPositive,
        "FgdcGeographicLonResolutionPositive"
    );
    structural_prop!(
        FgdcGeographicCoordUnitsCodeValid,
        "FgdcGeographicCoordUnitsCodeValid"
    );
    structural_prop!(FgdcMapProjNamePresent, "FgdcMapProjNamePresent");
    structural_prop!(
        FgdcMapProjStandardParallelInRange,
        "FgdcMapProjStandardParallelInRange"
    );
    structural_prop!(
        FgdcMapProjCentralMeridianInRange,
        "FgdcMapProjCentralMeridianInRange"
    );
    structural_prop!(
        FgdcMapProjLatitudeOriginInRange,
        "FgdcMapProjLatitudeOriginInRange"
    );
    structural_prop!(
        FgdcMapProjScaleFactorPositive,
        "FgdcMapProjScaleFactorPositive"
    );
    structural_prop!(
        FgdcMapProjAzimuthalAngle0To360,
        "FgdcMapProjAzimuthalAngle0To360"
    );
    structural_prop!(FgdcUtmZoneNumberInRange, "FgdcUtmZoneNumberInRange");
    structural_prop!(
        FgdcUpsZoneIdentifierCodeValid,
        "FgdcUpsZoneIdentifierCodeValid"
    );
    structural_prop!(FgdcArcZoneIdentifier1To18, "FgdcArcZoneIdentifier1To18");
    structural_prop!(
        FgdcGeodeticModelEllipsoidNamePresent,
        "FgdcGeodeticModelEllipsoidNamePresent"
    );
    structural_prop!(
        FgdcGeodeticModelSemiMajorAxisPositive,
        "FgdcGeodeticModelSemiMajorAxisPositive"
    );
    structural_prop!(
        FgdcGeodeticModelFlatteningRatioDenominatorPositive,
        "FgdcGeodeticModelFlatteningRatioDenominatorPositive"
    );
    structural_prop!(FgdcAltitudeDatumNamePresent, "FgdcAltitudeDatumNamePresent");
    structural_prop!(
        FgdcAltitudeResolutionAtLeastOne,
        "FgdcAltitudeResolutionAtLeastOne"
    );
    structural_prop!(
        FgdcAltitudeDistanceUnitsPresent,
        "FgdcAltitudeDistanceUnitsPresent"
    );
    structural_prop!(
        FgdcAltitudeEncodingMethodPresent,
        "FgdcAltitudeEncodingMethodPresent"
    );
    structural_prop!(FgdcDepthDatumNamePresent, "FgdcDepthDatumNamePresent");
    structural_prop!(
        FgdcDepthResolutionAtLeastOne,
        "FgdcDepthResolutionAtLeastOne"
    );
    structural_prop!(
        FgdcDepthDistanceUnitsPresent,
        "FgdcDepthDistanceUnitsPresent"
    );
    structural_prop!(
        FgdcDepthEncodingMethodPresent,
        "FgdcDepthEncodingMethodPresent"
    );
    // §5 Entity and Attribute
    structural_prop!(
        FgdcEntityAttributeHasDetailedOrOverview,
        "FgdcEntityAttributeHasDetailedOrOverview"
    );
    structural_prop!(FgdcEntityTypeLabelPresent, "FgdcEntityTypeLabelPresent");
    structural_prop!(
        FgdcEntityTypeDefinitionPresent,
        "FgdcEntityTypeDefinitionPresent"
    );
    structural_prop!(
        FgdcEntityTypeDefinitionSourcePresent,
        "FgdcEntityTypeDefinitionSourcePresent"
    );
    structural_prop!(FgdcAttributeLabelPresent, "FgdcAttributeLabelPresent");
    structural_prop!(
        FgdcAttributeDefinitionPresent,
        "FgdcAttributeDefinitionPresent"
    );
    structural_prop!(
        FgdcAttributeDefinitionSourcePresent,
        "FgdcAttributeDefinitionSourcePresent"
    );
    structural_prop!(
        FgdcAttributeHasAtLeastOneDomain,
        "FgdcAttributeHasAtLeastOneDomain"
    );
    structural_prop!(
        FgdcAttributeDomainTypeExclusive,
        "FgdcAttributeDomainTypeExclusive"
    );
    structural_prop!(
        FgdcEnumeratedDomainHasAtLeastOneValue,
        "FgdcEnumeratedDomainHasAtLeastOneValue"
    );
    structural_prop!(
        FgdcEnumeratedDomainValuePresent,
        "FgdcEnumeratedDomainValuePresent"
    );
    structural_prop!(
        FgdcEnumeratedDomainValueDefinitionPresent,
        "FgdcEnumeratedDomainValueDefinitionPresent"
    );
    structural_prop!(
        FgdcEnumeratedDomainValueDefinitionSourcePresent,
        "FgdcEnumeratedDomainValueDefinitionSourcePresent"
    );
    structural_prop!(
        FgdcRangeDomainMinimumPresent,
        "FgdcRangeDomainMinimumPresent"
    );
    structural_prop!(
        FgdcRangeDomainMaximumPresent,
        "FgdcRangeDomainMaximumPresent"
    );
    structural_prop!(
        FgdcRangeDomainMinimumLeMaximum,
        "FgdcRangeDomainMinimumLeMaximum"
    );
    structural_prop!(
        FgdcAttributeMeasurementResolutionPositive,
        "FgdcAttributeMeasurementResolutionPositive"
    );
    structural_prop!(FgdcCodesetNamePresent, "FgdcCodesetNamePresent");
    structural_prop!(FgdcCodesetSourcePresent, "FgdcCodesetSourcePresent");
    structural_prop!(
        FgdcOverviewDescriptionTextPresent,
        "FgdcOverviewDescriptionTextPresent"
    );
    structural_prop!(
        FgdcOverviewDetailCitationAtLeastOne,
        "FgdcOverviewDetailCitationAtLeastOne"
    );
    // §6 Distribution
    structural_prop!(
        FgdcDistributionDistributorPresent,
        "FgdcDistributionDistributorPresent"
    );
    structural_prop!(
        FgdcDistributionLiabilityPresent,
        "FgdcDistributionLiabilityPresent"
    );
    structural_prop!(
        FgdcStandardOrderHasFormNondigitalOrDigital,
        "FgdcStandardOrderHasFormNondigitalOrDigital"
    );
    structural_prop!(FgdcStandardOrderFeesPresent, "FgdcStandardOrderFeesPresent");
    structural_prop!(FgdcDigitalFormatNamePresent, "FgdcDigitalFormatNamePresent");
    structural_prop!(FgdcTransferSizePositive, "FgdcTransferSizePositive");
    structural_prop!(FgdcDialupLowestBpsGeq110, "FgdcDialupLowestBpsGeq110");
    structural_prop!(FgdcDialupHighestBpsGtLowest, "FgdcDialupHighestBpsGtLowest");
    structural_prop!(
        FgdcDialupDataBitsSevenOrEight,
        "FgdcDialupDataBitsSevenOrEight"
    );
    structural_prop!(FgdcDialupStopBitsOneOrTwo, "FgdcDialupStopBitsOneOrTwo");
    structural_prop!(FgdcDialupParityCodeValid, "FgdcDialupParityCodeValid");
    structural_prop!(FgdcOfflineMediaCodeValid, "FgdcOfflineMediaCodeValid");
    structural_prop!(FgdcRecordingDensityPositive, "FgdcRecordingDensityPositive");
    structural_prop!(FgdcRecordingFormatCodeValid, "FgdcRecordingFormatCodeValid");
    // §7 Metadata Reference
    structural_prop!(FgdcMetadataDatePresent, "FgdcMetadataDatePresent");
    structural_prop!(FgdcMetadataContactPresent, "FgdcMetadataContactPresent");
    structural_prop!(
        FgdcMetadataStandardNamePresent,
        "FgdcMetadataStandardNamePresent"
    );
    structural_prop!(
        FgdcMetadataStandardVersionPresent,
        "FgdcMetadataStandardVersionPresent"
    );
    structural_prop!(
        FgdcMetadataReviewDateAfterMetadataDate,
        "FgdcMetadataReviewDateAfterMetadataDate"
    );
    structural_prop!(
        FgdcMetadataFutureReviewDateAfterReviewDate,
        "FgdcMetadataFutureReviewDateAfterReviewDate"
    );
    structural_prop!(
        FgdcMetadataTimeConventionCodeValid,
        "FgdcMetadataTimeConventionCodeValid"
    );
    structural_prop!(
        FgdcMetadataSecurityClassificationCodeValid,
        "FgdcMetadataSecurityClassificationCodeValid"
    );
    // §9 Time Period
    structural_prop!(FgdcTimePeriodTypeExclusive, "FgdcTimePeriodTypeExclusive");
    structural_prop!(
        FgdcCalendarDateIsYyyymmddOrToken,
        "FgdcCalendarDateIsYyyymmddOrToken"
    );
    structural_prop!(
        FgdcRangeEndingDateAfterBeginning,
        "FgdcRangeEndingDateAfterBeginning"
    );
    // §10 Contact
    structural_prop!(
        FgdcContactHasPersonOrOrganizationPrimary,
        "FgdcContactHasPersonOrOrganizationPrimary"
    );
    structural_prop!(
        FgdcContactHasAtLeastOneAddress,
        "FgdcContactHasAtLeastOneAddress"
    );
    structural_prop!(
        FgdcContactHasAtLeastOneVoiceTelephone,
        "FgdcContactHasAtLeastOneVoiceTelephone"
    );
    structural_prop!(
        FgdcContactAddressTypeCodeValid,
        "FgdcContactAddressTypeCodeValid"
    );
    structural_prop!(
        FgdcContactAddressCityPresent,
        "FgdcContactAddressCityPresent"
    );
    structural_prop!(
        FgdcContactAddressStatePresent,
        "FgdcContactAddressStatePresent"
    );
    structural_prop!(
        FgdcContactAddressPostalCodePresent,
        "FgdcContactAddressPostalCodePresent"
    );
    // Aggregate seams
    structural_prop!(
        FgdcIdentificationSectionValid,
        "FgdcIdentificationSectionValid"
    );
    structural_prop!(FgdcDataQualitySectionValid, "FgdcDataQualitySectionValid");
    structural_prop!(
        FgdcSpatialDataOrgSectionValid,
        "FgdcSpatialDataOrgSectionValid"
    );
    structural_prop!(
        FgdcSpatialReferenceSectionValid,
        "FgdcSpatialReferenceSectionValid"
    );
    structural_prop!(
        FgdcEntityAttributeSectionValid,
        "FgdcEntityAttributeSectionValid"
    );
    structural_prop!(FgdcDistributionSectionValid, "FgdcDistributionSectionValid");
    structural_prop!(
        FgdcMetadataReferenceSectionValid,
        "FgdcMetadataReferenceSectionValid"
    );
    structural_prop!(FgdcCitationInfoValid, "FgdcCitationInfoValid");
    structural_prop!(FgdcTimePeriodInfoValid, "FgdcTimePeriodInfoValid");
    structural_prop!(FgdcContactInfoValid, "FgdcContactInfoValid");
    structural_prop!(FgdcRecordValid, "FgdcRecordValid");
}

pub use emit_impls::{
    FgdcAccessConstraintsPresent, FgdcAltitudeDatumNamePresent, FgdcAltitudeDistanceUnitsPresent,
    FgdcAltitudeEncodingMethodPresent, FgdcAltitudeResolutionAtLeastOne,
    FgdcArcZoneIdentifier1To18, FgdcAttributeAccuracyReportPresent, FgdcAttributeDefinitionPresent,
    FgdcAttributeDefinitionSourcePresent, FgdcAttributeDomainTypeExclusive,
    FgdcAttributeHasAtLeastOneDomain, FgdcAttributeLabelPresent,
    FgdcAttributeMeasurementResolutionPositive, FgdcBoundingEastCoordInRange,
    FgdcBoundingNorthCoordInRange, FgdcBoundingNorthGeqSouth, FgdcBoundingSouthCoordInRange,
    FgdcBoundingWestCoordInRange, FgdcBrowseGraphicFileDescriptionPresent,
    FgdcBrowseGraphicFileNamePresent, FgdcBrowseGraphicFileTypePresent,
    FgdcCalendarDateIsYyyymmddOrToken, FgdcCitationHasAtLeastOneOriginator, FgdcCitationInfoValid,
    FgdcCitationOriginatorNonEmpty, FgdcCitationPublicationDateIsYyyymmddOrToken,
    FgdcCitationPublicationDatePresent, FgdcCitationTitleNonEmpty, FgdcCitationTitlePresent,
    FgdcCloudCoverZeroToHundred, FgdcCodesetNamePresent, FgdcCodesetSourcePresent,
    FgdcContactAddressCityPresent, FgdcContactAddressPostalCodePresent,
    FgdcContactAddressStatePresent, FgdcContactAddressTypeCodeValid,
    FgdcContactHasAtLeastOneAddress, FgdcContactHasAtLeastOneVoiceTelephone,
    FgdcContactHasPersonOrOrganizationPrimary, FgdcContactInfoValid,
    FgdcDataQualityCompletenessReportPresent, FgdcDataQualityLineagePresent,
    FgdcDataQualityLogicalConsistencyPresent, FgdcDataQualitySectionValid,
    FgdcDepthDatumNamePresent, FgdcDepthDistanceUnitsPresent, FgdcDepthEncodingMethodPresent,
    FgdcDepthResolutionAtLeastOne, FgdcDescriptionAbstractPresent, FgdcDescriptionPurposePresent,
    FgdcDialupDataBitsSevenOrEight, FgdcDialupHighestBpsGtLowest, FgdcDialupLowestBpsGeq110,
    FgdcDialupParityCodeValid, FgdcDialupStopBitsOneOrTwo, FgdcDigitalFormatNamePresent,
    FgdcDirectSpatialRefMethodCodeValid, FgdcDistributionDistributorPresent,
    FgdcDistributionLiabilityPresent, FgdcDistributionSectionValid,
    FgdcEntityAttributeHasDetailedOrOverview, FgdcEntityAttributeSectionValid,
    FgdcEntityTypeDefinitionPresent, FgdcEntityTypeDefinitionSourcePresent,
    FgdcEntityTypeLabelPresent, FgdcEnumeratedDomainHasAtLeastOneValue,
    FgdcEnumeratedDomainValueDefinitionPresent, FgdcEnumeratedDomainValueDefinitionSourcePresent,
    FgdcEnumeratedDomainValuePresent, FgdcGPolygonOuterRingHasAtLeastFourPoints,
    FgdcGRingLatitudeInRange, FgdcGRingLongitudeInRange, FgdcGeodeticModelEllipsoidNamePresent,
    FgdcGeodeticModelFlatteningRatioDenominatorPositive, FgdcGeodeticModelSemiMajorAxisPositive,
    FgdcGeographicCoordUnitsCodeValid, FgdcGeographicLatResolutionPositive,
    FgdcGeographicLonResolutionPositive, FgdcHorizAccuracyAssessmentPaired,
    FgdcHorizCrsTypeIsGeographicPlanarOrLocal, FgdcIdentificationSectionValid,
    FgdcKeywordsHasAtLeastOneTheme, FgdcLineageHasAtLeastOneProcessStep,
    FgdcMapProjAzimuthalAngle0To360, FgdcMapProjCentralMeridianInRange,
    FgdcMapProjLatitudeOriginInRange, FgdcMapProjNamePresent, FgdcMapProjScaleFactorPositive,
    FgdcMapProjStandardParallelInRange, FgdcMetadataContactPresent, FgdcMetadataDatePresent,
    FgdcMetadataFutureReviewDateAfterReviewDate, FgdcMetadataHasIdentificationSection,
    FgdcMetadataHasMetadataReferenceSection, FgdcMetadataReferenceSectionValid,
    FgdcMetadataReviewDateAfterMetadataDate, FgdcMetadataSecurityClassificationCodeValid,
    FgdcMetadataStandardNamePresent, FgdcMetadataStandardVersionPresent,
    FgdcMetadataTimeConventionCodeValid, FgdcOfflineMediaCodeValid,
    FgdcOverviewDescriptionTextPresent, FgdcOverviewDetailCitationAtLeastOne,
    FgdcPlaceHasAtLeastOneKeyword, FgdcPlaceHasKeywordThesaurus,
    FgdcPositionalAccuracyHasAtLeastOneComponent, FgdcProcessStepDateFgdcFormat,
    FgdcProcessStepDatePresent, FgdcProcessStepDescriptionPresent,
    FgdcPvectInfoIsSdtsOrVpfExclusive, FgdcQaaValueAndExplanationPaired,
    FgdcRangeDomainMaximumPresent, FgdcRangeDomainMinimumLeMaximum, FgdcRangeDomainMinimumPresent,
    FgdcRangeEndingDateAfterBeginning, FgdcRasterColumnCountPositive,
    FgdcRasterObjectTypeCodeValid, FgdcRasterRowCountPositive, FgdcRasterVerticalCountPositive,
    FgdcRecordValid, FgdcRecordingDensityPositive, FgdcRecordingFormatCodeValid,
    FgdcSdtsObjectCountPositive, FgdcSdtsObjectTypeCodeValid, FgdcSecurityClassificationCodeValid,
    FgdcSecurityClassificationSystemPresent, FgdcSecurityHandlingDescriptionPresent,
    FgdcSourceCitationAbbreviationPresent, FgdcSourceCitationPresent,
    FgdcSourceContributionPresent, FgdcSourceMediaTypePresent, FgdcSourceScaleDenominatorGt1,
    FgdcSourceTimePeriodPresent, FgdcSpatialDataOrgSectionValid, FgdcSpatialReferenceSectionValid,
    FgdcStandardOrderFeesPresent, FgdcStandardOrderHasFormNondigitalOrDigital,
    FgdcStatusProgressCodeValid, FgdcStatusUpdateFrequencyCodeValid,
    FgdcStratumHasAtLeastOneKeyword, FgdcStratumHasKeywordThesaurus,
    FgdcTemporalKeywordHasAtLeastOneKeyword, FgdcTemporalKeywordHasThesaurus,
    FgdcThemeHasAtLeastOneKeyword, FgdcThemeHasKeywordThesaurus,
    FgdcTimeOfContentCurrentnessReferenceValid, FgdcTimeOfContentTimePeriodPresent,
    FgdcTimePeriodInfoValid, FgdcTimePeriodTypeExclusive, FgdcTransferSizePositive,
    FgdcUpsZoneIdentifierCodeValid, FgdcUseConstraintsPresent, FgdcUtmZoneNumberInRange,
    FgdcVertAccuracyAssessmentPaired, FgdcVpfObjectTypeCodeValid, FgdcVpfTopologyLevelZeroToThree,
};
