# FGDC CSDGM Contract Implementation Notes

## ⚠ Pattern correction notice

The prop structs below use `#[derive(Prop)]` and `#[spec_reference(...)]`.
**Both are fabricated — they do not exist in the codebase.**

The correct pattern is (from `crates/elicit_db/src/contracts/iso_sql.rs`):

```rust
mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Brief description of the proposition.
    ///
    /// Source: FGDC CSDGM §X.Y — <section title>
    pub struct PropName;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream { quote! { /* structural: #name */ } }
                fn verus_proof() -> TokenStream { quote! { /* structural: #name */ } }
                fn creusot_proof() -> TokenStream { quote! { /* structural: #name */ } }
            }
        };
    }
    structural_prop!(PropName, "PropName");
}
pub use emit_impls::{PropName};
```

Use this file as a content reference only. All prop struct snippets below must be
converted to the `structural_prop!` pattern before being placed in
`crates/elicit_gis/src/contracts/fgdc.rs`.

---

## FGDC CSDGM Contract Checklist

### Section 1: Identification Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/below-title-element-citation-information>

**Required Elements:**

- [ ] **Citation Title** - Non-empty, descriptive title
- [ ] **Originator/Author** - At least one originator identified
- [ ] **Publication Date** - Valid date, not in future
- [ ] **Geospatial Data Presentation Form** - Format of data (vector, raster, table)
- [ ] **Online Linkage** - URL for online access (if applicable)
- [ ] **Other Citation Details** - Additional bibliographic information

**Contract Requirements:**

```rust
// Title requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/below-title-element-citation-information")]
pub struct FgdcTitleMandatory;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/below-title-element-citation-information")]
pub struct FgdcTitleDescriptive;

// Originator requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorRequired;

// Publication date requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateNotFuture;
```

### Section 2: Data Quality Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element>

**Required Elements:**

- [ ] **Attribute Accuracy Report** - Description of accuracy assessment
- [ ] **Logical Consistency Report** - Logical consistency description
- [ ] **Completeness Report** - Data completeness assessment
- [ ] **Positional Accuracy** - Horizontal/vertical positional accuracy
- [ ] **Lineage** - Data lineage and processing steps

**Contract Requirements:**

```rust
// Data quality assessment contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyAssessed;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency")]
pub struct FgdcLogicalConsistencyEvaluated;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyQuantified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageTraceable;
```

### Section 3: Spatial Data Organization Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-element>

**Required Elements:**

- [ ] **Indirect Spatial Reference** - Textual description of spatial reference
- [ ] **Direct Spatial Reference Method** - Coordinate system method
- [ ] **Point and Vector Object Information** - Geometric object descriptions
- [ ] **Raster Object Information** - Raster data characteristics

**Contract Requirements:**

```rust
// Spatial organization contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-indirect-spatial-reference")]
pub struct FgdcSpatialReferenceMethodSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-direct-spatial-reference")]
pub struct FgdcGeometricObjectsDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-point-vector")]
pub struct FgdcRasterCharacteristicsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-raster")]
pub struct FgdcTopologyLevelSpecified;
```

### Section 4: Spatial Reference Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element>

**Required Elements:**

- [ ] **Horizontal Coordinate System** - Coordinate system definition
- [ ] **Vertical Coordinate System** - Vertical datum and units
- [ ] **Planar Coordinate Information** - Planar encoding details
- [ ] **Geodetic Model** - Datum and ellipsoid parameters

**Contract Requirements:**

```rust
// Spatial reference contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-coordinate-system")]
pub struct FgdcHorizontalCoordinateSystemDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-geographic-coordinate-system")]
pub struct FgdcGeographicCoordinateParametersValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarEncodingMethodSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-vertical-coordinate-system")]
pub struct FgdcVerticalCoordinateSystemDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-geodetic-model")]
pub struct FgdcGeodeticModelParametersValid;
```

### Section 5: Entity and Attribute Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-element>

**Required Elements:**

- [ ] **Entity Type Definitions** - Description of entity types
- [ ] **Attribute Definitions** - Detailed attribute descriptions
- [ ] **Attribute Domain Values** - Valid value ranges/enumerations
- [ ] **Overview Descriptions** - Summary of entity/attribute relationships

**Contract Requirements:**

```rust
// Entity and attribute contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types")]
pub struct FgdcEntityTypesCompletelyDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions")]
pub struct FgdcAttributeDefinitionsComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values")]
pub struct FgdcAttributeDomainsSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-overview-descriptions")]
pub struct FgdcOverviewDescriptionsProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-attribute-citations")]
pub struct FgdcEntityAttributeCitationsValid;
```

### Section 6: Distribution Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-element>

**Required Elements:**

- [ ] **Distribution Liability Statement** - Disclaimer and liability notice
- [ ] **Technical Prerequisites** - Software/hardware requirements
- [ ] **Available Formats** - Data format specifications
- [ ] **Transfer Size** - File size information
- [ ] **Access Instructions** - How to obtain the data
- [ ] **Ordering Instructions** - Ordering process details

**Contract Requirements:**

```rust
// Distribution contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStated;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites")]
pub struct FgdcTechnicalPrerequisitesDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats")]
pub struct FgdcAvailableFormatsSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-access-instructions")]
pub struct FgdcAccessInstructionsProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions")]
pub struct FgdcOrderingProcessDocumented;
```

### Section 7: Metadata Reference Information

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-element>

**Required Elements:**

- [ ] **Metadata Standard Name** - Standard used (FGDC CSDGM)
- [ ] **Metadata Standard Version** - Version of standard
- [ ] **Metadata Creation Date** - When metadata was created
- [ ] **Metadata Review Date** - Last review date
- [ ] **Metadata Future Review Date** - Next scheduled review
- [ ] **Metadata Security Classification** - Security level
- [ ] **Metadata Access Constraints** - Access restrictions

**Contract Requirements:**

```rust
// Metadata reference contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataCreationDateDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates")]
pub struct FgdcMetadataReviewScheduleMaintained;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcMetadataSecurityClassificationValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-access-constraints")]
pub struct FgdcMetadataAccessConstraintsSpecified;
```

### Cross-Cutting Validation Contracts

```rust
// Composite and cross-cutting contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcAllMandatoryElementsPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcNoContradictoryInformation;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcDatesChronologicallyConsistent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcSpatialExtentsConsistent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcMetadataInternallyConsistent;

// Final compliance contract
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Complete", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698")]
pub struct FgdcCompliant;
```

### Quality Assurance Contracts

````rust
// Data quality specific contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcQualityAssessmentMethodologyDocumented;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcErrorMeasurementsQuantified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcConfidenceLevelsSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcProcessingStepsDocumented;

# FGDC CSDGM Contract Implementation Plan (Continued)

## Detailed Section Requirements with Specific Contracts

### Section 1: Identification Information (Detailed)
**Reference:** https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/below-title-element-citation-information

**Specific Element Contracts:**
```rust
// Title Element Requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleLengthValid;  // 1-255 characters

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleCharacterSetValid;  // ASCII/UTF-8 compliant

// Originator Requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorNameFormatValid;  // Proper name format

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorOrganizationValid;  // Valid organization name

// Publication Date Requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateFormatValid;  // YYYYMMDD or CCYYMMDD format

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateRangeValid;  // Valid calendar date

// Presentation Form Requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-presentation-form")]
pub struct FgdcPresentationFormValid;  // Enumerated values validation

// Online Linkage Requirements
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageProtocolValid;  // HTTP/HTTPS/FTP protocols

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 1.1.1.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageUrlFormatValid;  // Valid URL format
````

### Section 2: Data Quality Information (Detailed)

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element>

**Specific Quality Metric Contracts:**

```rust
// Attribute Accuracy Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy-report")]
pub struct FgdcAttributeAccuracyReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy-value")]
pub struct FgdcAttributeAccuracyValueQuantified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy-explanation")]
pub struct FgdcAttributeAccuracyExplanationProvided;

// Logical Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency-report")]
pub struct FgdcLogicalConsistencyReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency-value")]
pub struct FgdcLogicalConsistencyValueMeasured;

// Completeness Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness-report")]
pub struct FgdcCompletenessReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness-omission")]
pub struct FgdcCompletenessOmissionAssessed;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness-commission")]
pub struct FgdcCompletenessCommissionAssessed;

// Positional Accuracy Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy-horizontal")]
pub struct FgdcHorizontalPositionalAccuracyQuantified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy-vertical")]
pub struct FgdcVerticalPositionalAccuracyQuantified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.4.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy-method")]
pub struct FgdcPositionalAccuracyMethodDocumented;

// Lineage Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.5.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage-statement")]
pub struct FgdcLineageStatementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.5.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage-process-step")]
pub struct FgdcProcessStepDocumentationComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 2.5.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage-source")]
pub struct FgdcSourceDocumentationComplete;
```

### Section 3: Spatial Data Organization Information (Detailed)

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-element>

**Specific Organization Contracts:**

```rust
// Indirect Spatial Reference Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-indirect-spatial-reference")]
pub struct FgdcIndirectSpatialReferenceDescribed;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-indirect-spatial-reference")]
pub struct FgdcIndirectSpatialReferenceGeographicValid;

// Direct Spatial Reference Method Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-direct-spatial-reference")]
pub struct FgdcDirectSpatialReferenceMethodSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-direct-spatial-reference")]
pub struct FgdcDirectSpatialReferenceMethodValid;

// Point and Vector Object Information Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-point-vector-object-type")]
pub struct FgdcObjectTypeDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-point-vector-object-count")]
pub struct FgdcObjectCountValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-point-vector-topology")]
pub struct FgdcTopologyLevelSpecified;

// Raster Object Information Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-raster-row-count")]
pub struct FgdcRasterRowCountValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-raster-column-count")]
pub struct FgdcRasterColumnCountValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 3.4.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-data-organization-raster-vpf-topology")]
pub struct FgdcRasterVpfTopologyLevelValid;
```

### Section 4: Spatial Reference Information (Detailed)

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element>

**Specific Spatial Reference Contracts:**

````rust
// Horizontal Coordinate System Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-geographic-coordinate-system")]
pub struct FgdcGeographicCoordinateSystemDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateSystemDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-local-coordinate-system")]
pub struct FgdcLocalCoordinateSystemDefined;

// Geographic Coordinate System Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-latitude-resolution")]
pub struct FgdcLatitudeResolutionValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-longitude-resolution")]
pub struct FgdcLongitudeResolutionValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-geographic-coordinate-units")]
pub struct FgdcGeographicCoordinateUnitsValid;

// Planar Coordinate System Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-map-projection")]
pub struct FgdcMapProjectionDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-grid-coordinate-system")]
pub struct FgdcGridCoordinateSystemDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.1.2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-distance-units")]
pub struct FgdcPlanarDistanceUnitsValid;

// Vertical Coordinate System Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-altitude-datum")]
pub struct FgdcAltitudeDatumDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-altitude-resolution")]
pub struct FgdcAltitudeResolutionValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-altitude-distance-units")]
pub struct FgdcAltitudeDistanceUnitsValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.2.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-altitude-encoding-method")]
pub struct FgdcAltitudeEncodingMethodValid;

// Geodetic Model Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-datum")]
pub struct FgdcHorizontalDatumDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-ellipsoid")]
pub struct FgdcEllipsoidDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-semi-major-axis")]
pub struct FgdcSemiMajorAxisValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 4.3.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-denominator-flattening-ratio")]
pub struct FgdcDenominatorFlatteningRatioValid;

# FGDC CSDGM Contract Implementation Plan (Continued)

## Section 5: Entity and Attribute Information (Detailed)
**Reference:** https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-element

**Specific Entity and Attribute Contracts:**
```rust
// Entity Type Definition Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasValid;

// Detailed Entity Type Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label-name")]
pub struct FgdcEntityTypeLabelNameValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.1.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label-type")]
pub struct FgdcEntityTypeLabelTypeValid;

// Attribute Definition Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.2.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-storage-type")]
pub struct FgdcAttributeStorageTypeValid;

// Attribute Domain Value Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-unrepresentable")]
pub struct FgdcUnrepresentableDomainDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated")]
pub struct FgdcEnumeratedDomainDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range")]
pub struct FgdcRangeDomainDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-codeset")]
pub struct FgdcCodesetDomainDefined;

// Enumerated Domain Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-reference")]
pub struct FgdcEnumeratedReferenceValid;

// Range Domain Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-minimum")]
pub struct FgdcRangeMinimumValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-maximum")]
pub struct FgdcRangeMaximumValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-units")]
pub struct FgdcRangeUnitsValid;

// Codeset Domain Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-codeset-name")]
pub struct FgdcCodesetNameValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.3.4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-codeset-reference")]
pub struct FgdcCodesetReferenceValid;

// Overview Description Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-overview-descriptions-entity")]
pub struct FgdcOverviewEntityDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-overview-descriptions-attribute")]
pub struct FgdcOverviewAttributeDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.4.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-overview-descriptions-definition-source")]
pub struct FgdcOverviewDefinitionSourceValid;

// Entity Attribute Citation Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.5.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-attribute-citations-title")]
pub struct FgdcEntityAttributeCitationTitleValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.5.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-attribute-citations-date")]
pub struct FgdcEntityAttributeCitationDateValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 5.5.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-attribute-citations-originator")]
pub struct FgdcEntityAttributeCitationOriginatorValid;
````

## Section 6: Distribution Information (Detailed)

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-element>

**Specific Distribution Contracts:**

```rust
// Distribution Liability Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeDefined;

// Technical Prerequisites Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.2.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentDescribed;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.2.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.2.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsDefined;

// Available Formats Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.3.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueValid;

// Transfer Size Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-transfer-size")]
pub struct FgdcTransferSizeCalculated;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-transfer-size")]
pub struct FgdcTransferSizeUnitsValid;

// Access Instructions Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.5.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-access-instructions-standard-order-process")]
pub struct FgdcStandardOrderProcessDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.5.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-access-instructions-non-standard-order-process")]
pub struct FgdcNonStandardOrderProcessDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.5.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-access-instructions-turnaround")]
pub struct FgdcTurnaroundTimeSpecified;

// Ordering Instructions Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.6.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions-fees")]
pub struct FgdcOrderingFeesDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.6.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions-payment")]
pub struct FgdcPaymentInstructionsProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.6.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions-custom-order-process")]
pub struct FgdcCustomOrderProcessDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.6.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions-availability")]
pub struct FgdcAvailabilityStatementProvided;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 6.6.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-ordering-instructions-contact")]
pub struct FgdcOrderingContactDefined;
```

# FGDC CSDGM Contract Implementation Plan (Continued)

## Section 7: Metadata Reference Information (Detailed)

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-element>

**Specific Metadata Reference Contracts:**

```rust
// Metadata Standard Name Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.1.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.1.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionValid;

// Metadata Creation Date Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateFormatValid;

// Metadata Review Date Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.3.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateSpecified;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.3.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.3.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateValid;

// Metadata Security Classification Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.4.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationAssigned;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.4.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.4.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceValid;

// Metadata Access Constraints Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.5.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-access-constraints")]
pub struct FgdcAccessConstraintsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.5.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-access-constraints-use")]
pub struct FgdcUseConstraintsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.5.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-access-constraints-other")]
pub struct FgdcOtherConstraintsDefined;

// Metadata Contact Information Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.1", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-person")]
pub struct FgdcContactPersonDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.2", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-organization")]
pub struct FgdcContactOrganizationDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.3", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-position")]
pub struct FgdcContactPositionDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.4", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-address")]
pub struct FgdcContactAddressDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.5", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-voice")]
pub struct FgdcContactVoiceDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.6", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-facsimile")]
pub struct FgdcContactFacsimileDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Section 7.6.7", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-contact-email")]
pub struct FgdcContactEmailDefined;
```

## Cross-Cutting Validation and Quality Assurance Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698>

**Comprehensive Validation Contracts:**

```rust
// Temporal Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Temporal", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/temporal-element")]
pub struct FgdcTemporalExtentDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Temporal", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/temporal-element-single-date")]
pub struct FgdcSingleDateValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Temporal", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/temporal-element-range")]
pub struct FgdcDateRangeValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Temporal", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/temporal-element-period")]
pub struct FgdcPeriodValid;

// Spatial Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-extent")]
pub struct FgdcSpatialExtentDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-extent-bounding")]
pub struct FgdcBoundingCoordinatesValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-extent-polygon")]
pub struct FgdcPolygonCoordinatesValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-extent-gazetteer")]
pub struct FgdcGazetteerValid;

// Keyword Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Keywords", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/keywords-element")]
pub struct FgdcKeywordsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Keywords", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/keywords-element-theme")]
pub struct FgdcThemeKeywordsValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Keywords", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/keywords-element-place")]
pub struct FgdcPlaceKeywordsValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Keywords", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/keywords-element-stratum")]
pub struct FgdcStratumKeywordsValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Keywords", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/keywords-element-temporal")]
pub struct FgdcTemporalKeywordsValid;

// Data Presentation Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-presentation-form")]
pub struct FgdcPresentationFormValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-presentation-form-digital")]
pub struct FgdcDigitalFormValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-presentation-form-hardcopy")]
pub struct FgdcHardcopyFormValid;

// Citation Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information")]
pub struct FgdcCitationComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateComplete;

// Quality Metrics Consistency Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcQualityMetricsDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency")]
pub struct FgdcLogicalConsistencyDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyDefined;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Quality", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageDefined;
```

# FGDC CSDGM Contract Implementation Plan (Continued)

## Detailed Element-Level Contracts

### Citation Information Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information>

```rust
// Citation Element Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information")]
pub struct FgdcCitationElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information")]
pub struct FgdcCitationElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Citation Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information")]
pub struct FgdcCitationElementValid;

// Title Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Title Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Title Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Title Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Title Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleElementCharacterSetValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Title Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-title")]
pub struct FgdcTitleElementSyntaxValid;

// Originator Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Originator Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Originator Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Originator Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Originator Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-originator")]
pub struct FgdcOriginatorElementMultiplicityValid;

// Publication Date Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Publication Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Publication Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Publication Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateElementChronologicalValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Publication Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-publication-date")]
pub struct FgdcPublicationDateElementNotFuture;

// Presentation Form Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation Form Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-presentation-form")]
pub struct FgdcPresentationFormElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation Form Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-presentation-form")]
pub struct FgdcPresentationFormElementValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Presentation Form Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-presentation-form")]
pub struct FgdcPresentationFormElementEnumerated;

// Online Linkage Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Online Linkage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Online Linkage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Online Linkage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageElementProtocolValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Online Linkage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/citation-information-online-linkage")]
pub struct FgdcOnlineLinkageElementAccessibilityValid;
```

### Data Quality Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element>

```rust
// Data Quality Element Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Data Quality Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcDataQualityElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Data Quality Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcDataQualityElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Data Quality Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-element")]
pub struct FgdcDataQualityElementHierarchicalValid;

// Attribute Accuracy Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyElementReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyElementValuePresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-attribute-accuracy")]
pub struct FgdcAttributeAccuracyElementExplanationPresent;

// Logical Consistency Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Logical Consistency Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency")]
pub struct FgdcLogicalConsistencyElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Logical Consistency Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency")]
pub struct FgdcLogicalConsistencyElementReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Logical Consistency Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-logical-consistency")]
pub struct FgdcLogicalConsistencyElementValuePresent;

// Completeness Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Completeness Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Completeness Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessElementReportPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Completeness Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessElementOmissionPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Completeness Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-completeness")]
pub struct FgdcCompletenessElementCommissionPresent;

// Positional Accuracy Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Positional Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Positional Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyElementHorizontalPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Positional Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyElementVerticalPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Positional Accuracy Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-positional-accuracy")]
pub struct FgdcPositionalAccuracyElementMethodPresent;

// Lineage Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Lineage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Lineage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageElementStatementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Lineage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageElementProcessStepPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Lineage Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/data-quality-lineage")]
pub struct FgdcLineageElementSourcePresent;
```

### Spatial Reference Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element>

```rust
// Spatial Reference Element Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial Reference Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element")]
pub struct FgdcSpatialReferenceElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial Reference Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element")]
pub struct FgdcSpatialReferenceElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Spatial Reference Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-element")]
pub struct FgdcSpatialReferenceElementValid;

// Horizontal Coordinate System Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Horizontal Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-coordinate-system")]
pub struct FgdcHorizontalCoordinateSystemElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Horizontal Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-coordinate-system")]
pub struct FgdcHorizontalCoordinateSystemElementGeographicPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Horizontal Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-coordinate-system")]
pub struct FgdcHorizontalCoordinateSystemElementPlanarPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Horizontal Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-horizontal-coordinate-system")]
pub struct FgdcHorizontalCoordinateSystemElementLocalPresent;

// Vertical Coordinate System Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Vertical Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-vertical-coordinate-system")]
pub struct FgdcVerticalCoordinateSystemElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Vertical Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-vertical-coordinate-system")]
pub struct FgdcVerticalCoordinateSystemElementAltitudeDatumPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Vertical Coordinate System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-vertical-coordinate-system")]
pub struct FgdcVerticalCoordinateSystemElementDepthDatumPresent;

// Planar Coordinate Information Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementMapProjectionPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementGridCoordinatePresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementLocalPlanarPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementLocalSystemPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementPlanarDistanceUnitsPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Planar Coordinate Information Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/spatial-reference-planar-coordinate-information")]
pub struct FgdcPlanarCoordinateInformationElementEncodingMethodPresent;
```

# FGDC CSDGM Contract Implementation Plan (Continued)

## Entity and Attribute Element Detailed Contracts

### Entity Types Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types>

```rust
// Entity Types Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Types Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types")]
pub struct FgdcEntityTypesElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Types Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types")]
pub struct FgdcEntityTypesElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Types Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types")]
pub struct FgdcEntityTypesElementValid;

// Entity Type Label Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-label")]
pub struct FgdcEntityTypeLabelElementUnique;

// Entity Type Definition Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-definition")]
pub struct FgdcEntityTypeDefinitionElementSemanticValid;

// Entity Type Alias Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Alias Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Alias Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Alias Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Alias Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Entity Type Alias Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-entity-types-alias")]
pub struct FgdcEntityTypeAliasElementUnique;
```

### Attribute Definitions Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions>

```rust
// Attribute Definitions Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definitions Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions")]
pub struct FgdcAttributeDefinitionsElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definitions Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions")]
pub struct FgdcAttributeDefinitionsElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definitions Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions")]
pub struct FgdcAttributeDefinitionsElementValid;

// Attribute Identifier Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Identifier Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Identifier Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Identifier Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Identifier Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Identifier Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-identifier")]
pub struct FgdcAttributeIdentifierElementUnique;

// Attribute Label Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Label Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-label")]
pub struct FgdcAttributeLabelElementSemanticValid;

// Attribute Definition Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementSemanticValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-definition")]
pub struct FgdcAttributeDefinitionElementTechnicalValid;

// Attribute Storage Type Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Storage Type Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-storage-type")]
pub struct FgdcAttributeStorageTypeElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Storage Type Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-storage-type")]
pub struct FgdcAttributeStorageTypeElementValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Storage Type Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-storage-type")]
pub struct FgdcAttributeStorageTypeElementEnumerated;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Storage Type Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-definitions-storage-type")]
pub struct FgdcAttributeStorageTypeElementCompatible;
```

### Attribute Domain Values Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values>

```rust
// Attribute Domain Values Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Domain Values Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values")]
pub struct FgdcAttributeDomainValuesElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Domain Values Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values")]
pub struct FgdcAttributeDomainValuesElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Attribute Domain Values Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values")]
pub struct FgdcAttributeDomainValuesElementValid;

// Unrepresentable Domain Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Unrepresentable Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-unrepresentable")]
pub struct FgdcUnrepresentableDomainElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Unrepresentable Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-unrepresentable")]
pub struct FgdcUnrepresentableDomainElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Unrepresentable Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-unrepresentable")]
pub struct FgdcUnrepresentableDomainElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Unrepresentable Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-unrepresentable")]
pub struct FgdcUnrepresentableDomainElementSyntaxValid;

// Enumerated Domain Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated")]
pub struct FgdcEnumeratedDomainElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated")]
pub struct FgdcEnumeratedDomainElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated")]
pub struct FgdcEnumeratedDomainElementValid;

// Enumerated Domain Code Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Code Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Code Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Code Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Code Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Code Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-code")]
pub struct FgdcEnumeratedCodeElementUnique;

// Enumerated Domain Definition Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Enumerated Domain Definition Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-enumerated-definition")]
pub struct FgdcEnumeratedDefinitionElementSemanticValid;

// Range Domain Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range")]
pub struct FgdcRangeDomainElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range")]
pub struct FgdcRangeDomainElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range")]
pub struct FgdcRangeDomainElementValid;

// Range Domain Minimum Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Minimum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-minimum")]
pub struct FgdcRangeMinimumElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Minimum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-minimum")]
pub struct FgdcRangeMinimumElementValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Minimum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-minimum")]
pub struct FgdcRangeMinimumElementComparable;

// Range Domain Maximum Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Maximum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-maximum")]
pub struct FgdcRangeMaximumElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Maximum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-maximum")]
pub struct FgdcRangeMaximumElementValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Maximum Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-maximum")]
pub struct FgdcRangeMaximumElementComparable;

// Range Domain Units Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Units Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-units")]
pub struct FgdcRangeUnitsElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Units Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-units")]
pub struct FgdcRangeUnitsElementValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Units Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-units")]
pub struct FgdcRangeUnitsElementStandard;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Range Domain Units Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/entity-attribute-attribute-domain-values-range-units")]
pub struct FgdcRangeUnitsElementCompatible;
```

# FGDC CSDGM Contract Implementation Plan (Continued)

## Distribution Element Detailed Contracts

### Distribution Liability Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability>

```rust
// Distribution Liability Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityElementValid;

// Distribution Liability Statement Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Statement Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Statement Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Statement Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Statement Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Statement Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityStatementElementLegalValid;

// Distribution Liability Scope Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Scope Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Scope Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Scope Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Scope Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Distribution Liability Scope Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-distribution-liability")]
pub struct FgdcDistributionLiabilityScopeElementCoverageValid;
```

### Technical Prerequisites Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites>

```rust
// Technical Prerequisites Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Prerequisites Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites")]
pub struct FgdcTechnicalPrerequisitesElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Prerequisites Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites")]
pub struct FgdcTechnicalPrerequisitesElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Prerequisites Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites")]
pub struct FgdcTechnicalPrerequisitesElementValid;

// Technical Environment Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Environment Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Environment Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Environment Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Environment Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Technical Environment Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-environment")]
pub struct FgdcTechnicalEnvironmentElementTechnicalValid;

// Software Requirements Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Software Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Software Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Software Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Software Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Software Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-software")]
pub struct FgdcSoftwareRequirementsElementVersionValid;

// Hardware Requirements Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Hardware Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Hardware Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Hardware Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Hardware Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Hardware Requirements Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-technical-prerequisites-hardware")]
pub struct FgdcHardwareRequirementsElementSpecificationsValid;
```

### Available Formats Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats>

```rust
// Available Formats Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Available Formats Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats")]
pub struct FgdcAvailableFormatsElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Available Formats Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats")]
pub struct FgdcAvailableFormatsElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Available Formats Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats")]
pub struct FgdcAvailableFormatsElementValid;

// Format Name Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementStandardValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-name")]
pub struct FgdcFormatNameElementRecognized;

// Format Version Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-version")]
pub struct FgdcFormatVersionElementSemanticValid;

// Format Specification Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Specification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Specification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Specification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Specification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Format Specification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-specification")]
pub struct FgdcFormatSpecificationElementReferenceValid;

// File Decompression Technique Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM File Decompression Technique Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM File Decompression Technique Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM File Decompression Technique Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM File Decompression Technique Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM File Decompression Technique Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/distribution-available-formats-file-decompression-technique")]
pub struct FgdcFileDecompressionTechniqueElementStandardValid;
```

# FGDC CSDGM Contract Implementation Plan (Continued)

## Metadata Reference Element Detailed Contracts

### Metadata Standard Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name>

```rust
// Metadata Standard Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardElementValid;

// Metadata Standard Name Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementRecognized;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Name Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-name")]
pub struct FgdcMetadataStandardNameElementCurrent;

// Metadata Standard Version Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementSemanticValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Standard Version Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-standard-version")]
pub struct FgdcMetadataStandardVersionElementCompatible;
```

### Metadata Creation Date Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date>

```rust
// Metadata Creation Date Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateElementValid;

// Metadata Creation Date Value Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Value", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValuePresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Value", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValueFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Value", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValueChronologicalValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Value", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValueNotFuture;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Value", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateValuePrecisionValid;

// Metadata Creation Date Type Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Type", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateTypePresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Type", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateTypeValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Type", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateTypeEnumerated;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Creation Date Type", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-creation-date")]
pub struct FgdcMetadataCreationDateTypeCompatible;
```

### Metadata Review Dates Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates>

```rust
// Metadata Review Dates Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Review Dates Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates")]
pub struct FgdcMetadataReviewDatesElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Review Dates Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates")]
pub struct FgdcMetadataReviewDatesElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Review Dates Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates")]
pub struct FgdcMetadataReviewDatesElementValid;

// Future Review Date Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Future Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Future Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Future Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Future Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateElementChronologicalValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Future Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-future")]
pub struct FgdcFutureReviewDateElementReasonable;

// Last Review Date Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Last Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Last Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Last Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Last Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateElementChronologicalValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Last Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-last")]
pub struct FgdcLastReviewDateElementNotFuture;

// Next Review Date Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Next Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Next Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Next Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateElementFormatValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Next Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateElementChronologicalValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Next Review Date Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-review-dates-next")]
pub struct FgdcNextReviewDateElementReasonable;
```

### Metadata Security Classification Element Contracts

**Reference:** <https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification>

```rust
// Metadata Security Classification Structure Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Security Classification Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcMetadataSecurityClassificationElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Security Classification Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcMetadataSecurityClassificationElementComplete;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Metadata Security Classification Structure", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcMetadataSecurityClassificationElementValid;

// Security Classification Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementStandardValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification")]
pub struct FgdcSecurityClassificationElementEnumerated;

// Security Classification System Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification System Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-system")]
pub struct FgdcSecurityClassificationSystemElementRecognized;

// Security Classification Code Reference Element Contracts
#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Code Reference Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceElementPresent;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Code Reference Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceElementNonEmpty;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Code Reference Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceElementLengthValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Code Reference Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceElementSyntaxValid;

#[derive(Prop)]
#[spec_reference("FGDC CSDGM Security Classification Code Reference Element", "https://www.fgdc.gov/standards/projects/FGDC-standards-projects/metadata/base-metadata/v2_0698/metadata-reference-security-classification-coderef")]
pub struct FgdcSecurityClassificationCodeReferenceElementReferenceValid;
```
