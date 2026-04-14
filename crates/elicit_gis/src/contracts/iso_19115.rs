//! ISO 19115-1:2014 propositions — Geographic information — Metadata.
//!
//! Source: ISO 19115-1:2014 — all §references are to that standard.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: $name */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: $name */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: $name */ }
                }
            }
        };
    }

    /// fileIdentifier is optional (0..1); when provided, should be a UUID string uniquely
    /// identifying this metadata record across systems.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / fileIdentifier
    pub struct MdMetadataFileIdentifierOptional;
    structural_prop!(
        MdMetadataFileIdentifierOptional,
        "MdMetadataFileIdentifierOptional"
    );

    /// language is conditional (0..*); required when metadata content uses a human language;
    /// values shall be ISO 639-2 three-letter lowercase language codes.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / language
    pub struct MdMetadataLanguageConditional;
    structural_prop!(
        MdMetadataLanguageConditional,
        "MdMetadataLanguageConditional"
    );

    /// characterSet is conditional (0..*); required when the character encoding is not UTF-8;
    /// value shall be a code from MD_CharacterSetCode enumeration.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / characterSet
    pub struct MdMetadataCharacterSetConditional;
    structural_prop!(
        MdMetadataCharacterSetConditional,
        "MdMetadataCharacterSetConditional"
    );

    /// parentIdentifier is optional (0..1); when provided, shall be the UUID of the parent
    /// metadata record in a hierarchical metadata set.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / parentIdentifier
    pub struct MdMetadataParentIdentifierOptional;
    structural_prop!(
        MdMetadataParentIdentifierOptional,
        "MdMetadataParentIdentifierOptional"
    );

    /// hierarchyLevel is optional (0..*); when the resource is not a dataset, at least one
    /// MD_ScopeCode value shall be provided to indicate the resource scope.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / hierarchyLevel
    pub struct MdMetadataHierarchyLevelScopeCode;
    structural_prop!(
        MdMetadataHierarchyLevelScopeCode,
        "MdMetadataHierarchyLevelScopeCode"
    );

    /// hierarchyLevelName is optional (0..*); each entry corresponds to one entry in
    /// hierarchyLevel; cardinalities must match.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / hierarchyLevelName
    pub struct MdMetadataHierarchyLevelNameMatchesLevel;
    structural_prop!(
        MdMetadataHierarchyLevelNameMatchesLevel,
        "MdMetadataHierarchyLevelNameMatchesLevel"
    );

    /// contact is mandatory (1..*); at least one CI_Responsibility shall be provided;
    /// identifies the party responsible for the metadata record itself.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / contact
    pub struct MdMetadataContactMandatory;
    structural_prop!(MdMetadataContactMandatory, "MdMetadataContactMandatory");

    /// contact constraint: at least one CI_Responsibility in the contact array shall have
    /// a non-null party name (CI_Individual.name or CI_Organisation.name).
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / contact
    pub struct MdMetadataContactPartyNameNonNull;
    structural_prop!(
        MdMetadataContactPartyNameNonNull,
        "MdMetadataContactPartyNameNonNull"
    );

    /// dateInfo is mandatory (1..*); at least one CI_Date shall document when the metadata
    /// record was created, published, or revised; ISO 8601 format required.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / dateInfo
    pub struct MdMetadataDateInfoMandatory;
    structural_prop!(MdMetadataDateInfoMandatory, "MdMetadataDateInfoMandatory");

    /// metadataStandardName is optional (0..1); when documenting conformance to a specific
    /// metadata standard, provide the standard name as a CharacterString.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / metadataStandardName
    pub struct MdMetadataStandardNameOptional;
    structural_prop!(
        MdMetadataStandardNameOptional,
        "MdMetadataStandardNameOptional"
    );

    /// metadataStandardVersion is optional (0..1); paired with metadataStandardName to
    /// document the version of the metadata standard applied.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / metadataStandardVersion
    pub struct MdMetadataStandardVersionOptional;
    structural_prop!(
        MdMetadataStandardVersionOptional,
        "MdMetadataStandardVersionOptional"
    );

    /// locale is optional (0..*); each PT_Locale entry documents a language/encoding
    /// combination used in the metadata record.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / locale
    pub struct MdMetadataLocaleOptional;
    structural_prop!(MdMetadataLocaleOptional, "MdMetadataLocaleOptional");

    /// spatialRepresentationInfo is optional (0..*); documents the digital mechanisms used
    /// to represent spatial information in the described resource.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / spatialRepresentationInfo
    pub struct MdMetadataSpatialRepresentationInfoOptional;
    structural_prop!(
        MdMetadataSpatialRepresentationInfoOptional,
        "MdMetadataSpatialRepresentationInfoOptional"
    );

    /// referenceSystemInfo is optional (0..*); describes the reference system used by the
    /// described resource; omit only if no spatial data is involved.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / referenceSystemInfo
    pub struct MdMetadataReferenceSystemInfoOptional;
    structural_prop!(
        MdMetadataReferenceSystemInfoOptional,
        "MdMetadataReferenceSystemInfoOptional"
    );

    /// identificationInfo is mandatory (1..*); at least one MD_Identification subclass
    /// (typically MD_DataIdentification) shall describe the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / identificationInfo
    pub struct MdMetadataIdentificationInfoMandatory;
    structural_prop!(
        MdMetadataIdentificationInfoMandatory,
        "MdMetadataIdentificationInfoMandatory"
    );

    /// distributionInfo is optional (0..*); documents how the resource can be obtained;
    /// provide when the resource is publicly or commercially available.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / distributionInfo
    pub struct MdMetadataDistributionInfoOptional;
    structural_prop!(
        MdMetadataDistributionInfoOptional,
        "MdMetadataDistributionInfoOptional"
    );

    /// dataQualityInfo is optional (0..*); each DQ_DataQuality element reports quality
    /// assessment results for the described resource or sub-resource.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / dataQualityInfo
    pub struct MdMetadataDataQualityInfoOptional;
    structural_prop!(
        MdMetadataDataQualityInfoOptional,
        "MdMetadataDataQualityInfoOptional"
    );

    /// resourceLineage is optional (0..*); each LI_Lineage element traces the history,
    /// provenance, and processing steps of the described resource.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / resourceLineage
    pub struct MdMetadataResourceLineageOptional;
    structural_prop!(
        MdMetadataResourceLineageOptional,
        "MdMetadataResourceLineageOptional"
    );

    /// metadataConstraints is optional (0..*); legal and security constraints governing
    /// access and use of the metadata record itself (not the described resource).
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / metadataConstraints
    pub struct MdMetadataConstraintsOptional;
    structural_prop!(
        MdMetadataConstraintsOptional,
        "MdMetadataConstraintsOptional"
    );

    /// metadataMaintenance is optional (0..1); describes the frequency and scope of
    /// future updates to the metadata record.
    ///
    /// Source: ISO 19115-1:2014 §6.2 — MD_Metadata / metadataMaintenance
    pub struct MdMetadataMaintenanceOptional;
    structural_prop!(
        MdMetadataMaintenanceOptional,
        "MdMetadataMaintenanceOptional"
    );

    /// title is mandatory (1); shall be a non-empty CharacterString; the empty string
    /// is explicitly prohibited — every CI_Citation must have a descriptive title.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / title
    pub struct CiCitationTitleMandatory;
    structural_prop!(CiCitationTitleMandatory, "CiCitationTitleMandatory");

    /// title shall never be the empty string; this constraint applies everywhere CI_Citation
    /// appears: identificationInfo, thesaurusName, formatSpecificationCitation, lineage docs.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / title
    pub struct CiCitationTitleNonEmpty;
    structural_prop!(CiCitationTitleNonEmpty, "CiCitationTitleNonEmpty");

    /// alternateTitle is optional (0..*); provides additional titles or acronyms by which
    /// the resource is known; each must be a non-empty CharacterString.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / alternateTitle
    pub struct CiCitationAlternateTitleOptional;
    structural_prop!(
        CiCitationAlternateTitleOptional,
        "CiCitationAlternateTitleOptional"
    );

    /// date is mandatory (1..*); at least one CI_Date shall be provided documenting when
    /// the cited resource was created, published, revised, or otherwise dated.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / date
    pub struct CiCitationDateMandatory;
    structural_prop!(CiCitationDateMandatory, "CiCitationDateMandatory");

    /// edition is optional (0..1); version designation of the cited resource
    /// as a free-form CharacterString (e.g., "2nd ed.", "v3.1").
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / edition
    pub struct CiCitationEditionOptional;
    structural_prop!(CiCitationEditionOptional, "CiCitationEditionOptional");

    /// editionDate is optional (0..1); the date on which the cited edition was published;
    /// shall conform to ISO 8601 DateTime or Date format.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / editionDate
    pub struct CiCitationEditionDateOptional;
    structural_prop!(
        CiCitationEditionDateOptional,
        "CiCitationEditionDateOptional"
    );

    /// identifier is optional (0..*); each MD_Identifier provides an authority-scoped
    /// unique identifier for the cited resource (DOI, ISBN, catalog number, etc.).
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / identifier
    pub struct CiCitationIdentifierOptional;
    structural_prop!(CiCitationIdentifierOptional, "CiCitationIdentifierOptional");

    /// citedResponsibleParty is optional (0..*); documents the parties responsible for
    /// the cited resource (authors, editors, publishers, distributors, etc.).
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / citedResponsibleParty
    pub struct CiCitationResponsiblePartyOptional;
    structural_prop!(
        CiCitationResponsiblePartyOptional,
        "CiCitationResponsiblePartyOptional"
    );

    /// presentationForm is optional (0..*); specifies the physical or digital form in which
    /// the cited resource is available; values from CI_PresentationFormCode enumeration.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / presentationForm
    pub struct CiCitationPresentationFormOptional;
    structural_prop!(
        CiCitationPresentationFormOptional,
        "CiCitationPresentationFormOptional"
    );

    /// series is optional (0..1); identifies a series or aggregate dataset of which the
    /// cited resource is a part; includes series name, issue identification, and page.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / series
    pub struct CiCitationSeriesOptional;
    structural_prop!(CiCitationSeriesOptional, "CiCitationSeriesOptional");

    /// otherCitationDetails is optional (0..1); free-text bibliographic information not
    /// captured by other CI_Citation attributes (e.g., edition notes, publisher address).
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / otherCitationDetails
    pub struct CiCitationOtherDetailsOptional;
    structural_prop!(
        CiCitationOtherDetailsOptional,
        "CiCitationOtherDetailsOptional"
    );

    /// collectiveTitle is optional (0..1); the title of the series or collection to which
    /// the cited resource belongs when there is no formal CI_Series entry.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / collectiveTitle
    pub struct CiCitationCollectiveTitleOptional;
    structural_prop!(
        CiCitationCollectiveTitleOptional,
        "CiCitationCollectiveTitleOptional"
    );

    /// ISBN is optional (0..1); the International Standard Book Number; shall conform to
    /// ISBN-10 (xxxxxxxxxx) or ISBN-13 (xxx-xxxxxxxxxx) format with valid check digit.
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / ISBN
    pub struct CiCitationIsbnFormatValid;
    structural_prop!(CiCitationIsbnFormatValid, "CiCitationIsbnFormatValid");

    /// ISSN is optional (0..1); the International Standard Serial Number; shall conform to
    /// ISSN format (nnnn-nnnx) where x is a check character (0-9 or X).
    ///
    /// Source: ISO 19115-1:2014 §6.5 — CI_Citation / ISSN
    pub struct CiCitationIssnFormatValid;
    structural_prop!(CiCitationIssnFormatValid, "CiCitationIssnFormatValid");

    /// date is mandatory (1); the temporal value of the event; shall be in ISO 8601 format
    /// (date: YYYY-MM-DD, or dateTime: YYYY-MM-DDThh:mm:ssZ or with offset).
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_Date / date
    pub struct CiDateValueMandatory;
    structural_prop!(CiDateValueMandatory, "CiDateValueMandatory");

    /// dateType is mandatory (1); shall be one of the values in CI_DateTypeCode;
    /// identifies the event or condition that the date describes.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_Date / dateType
    pub struct CiDateTypeCodeMandatory;
    structural_prop!(CiDateTypeCodeMandatory, "CiDateTypeCodeMandatory");

    /// CI_DateTypeCode: creation — the date identifies when the resource was first created;
    /// should be the earliest date in any dateInfo set for a given resource.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / creation
    pub struct CiDateTypeCreation;
    structural_prop!(CiDateTypeCreation, "CiDateTypeCreation");

    /// CI_DateTypeCode: publication — the date the resource was published or made
    /// publicly available; may differ from creation date by years.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / publication
    pub struct CiDateTypePublication;
    structural_prop!(CiDateTypePublication, "CiDateTypePublication");

    /// CI_DateTypeCode: revision — the date the resource content was substantively
    /// changed; triggers an update to the metadata record.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / revision
    pub struct CiDateTypeRevision;
    structural_prop!(CiDateTypeRevision, "CiDateTypeRevision");

    /// CI_DateTypeCode: expiry — the date after which the resource should not be used
    /// or is no longer considered current.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / expiry
    pub struct CiDateTypeExpiry;
    structural_prop!(CiDateTypeExpiry, "CiDateTypeExpiry");

    /// CI_DateTypeCode: lastUpdate — the date the resource or its metadata was last
    /// updated, regardless of whether it was a substantive revision.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / lastUpdate
    pub struct CiDateTypeLastUpdate;
    structural_prop!(CiDateTypeLastUpdate, "CiDateTypeLastUpdate");

    /// CI_DateTypeCode: lastRevision — date of the last formal revision process applied
    /// to the resource (distinct from ad-hoc updates).
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / lastRevision
    pub struct CiDateTypeLastRevision;
    structural_prop!(CiDateTypeLastRevision, "CiDateTypeLastRevision");

    /// CI_DateTypeCode: nextUpdate — the date when the next update to the resource
    /// is planned or scheduled.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / nextUpdate
    pub struct CiDateTypeNextUpdate;
    structural_prop!(CiDateTypeNextUpdate, "CiDateTypeNextUpdate");

    /// CI_DateTypeCode: unavailable — the date after which the resource is no longer
    /// available through normal distribution channels.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / unavailable
    pub struct CiDateTypeUnavailable;
    structural_prop!(CiDateTypeUnavailable, "CiDateTypeUnavailable");

    /// CI_DateTypeCode: inForce — the date on which the resource (e.g., a regulation
    /// or standard) came into force or effect.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / inForce
    pub struct CiDateTypeInForce;
    structural_prop!(CiDateTypeInForce, "CiDateTypeInForce");

    /// CI_DateTypeCode: adopted — the date on which the resource was formally adopted
    /// by the responsible authority.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / adopted
    pub struct CiDateTypeAdopted;
    structural_prop!(CiDateTypeAdopted, "CiDateTypeAdopted");

    /// CI_DateTypeCode: deprecated — the date on which the resource was declared
    /// deprecated and should no longer be used for new work.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / deprecated
    pub struct CiDateTypeDeprecated;
    structural_prop!(CiDateTypeDeprecated, "CiDateTypeDeprecated");

    /// CI_DateTypeCode: superseded — the date on which the resource was superseded
    /// by a newer version or replacement resource.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / superseded
    pub struct CiDateTypeSuperseded;
    structural_prop!(CiDateTypeSuperseded, "CiDateTypeSuperseded");

    /// CI_DateTypeCode: validityBegins — the start of the period during which the resource
    /// content is valid; paired with validityExpires to define a validity interval.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / validityBegins
    pub struct CiDateTypeValidityBegins;
    structural_prop!(CiDateTypeValidityBegins, "CiDateTypeValidityBegins");

    /// CI_DateTypeCode: validityExpires — the end of the period during which the resource
    /// content is valid; validityBegins <= validityExpires shall hold.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / validityExpires
    pub struct CiDateTypeValidityExpires;
    structural_prop!(CiDateTypeValidityExpires, "CiDateTypeValidityExpires");

    /// CI_DateTypeCode: released — the date the resource was formally released to users
    /// or customers (may differ from publication date).
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / released
    pub struct CiDateTypeReleased;
    structural_prop!(CiDateTypeReleased, "CiDateTypeReleased");

    /// CI_DateTypeCode: distribution — the date the resource was distributed to a
    /// specific audience or distribution channel.
    ///
    /// Source: ISO 19115-1:2014 §6.6 — CI_DateTypeCode / distribution
    pub struct CiDateTypeDistribution;
    structural_prop!(CiDateTypeDistribution, "CiDateTypeDistribution");

    /// role is mandatory (1); identifies the function performed by the party with respect
    /// to the resource; value shall be from the CI_RoleCode enumeration.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Responsibility / role
    pub struct CiResponsibilityRoleMandatory;
    structural_prop!(
        CiResponsibilityRoleMandatory,
        "CiResponsibilityRoleMandatory"
    );

    /// extent is optional (0..*); when the responsibility is spatially or temporally
    /// limited, EX_Extent documents that geographic or temporal scope.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Responsibility / extent
    pub struct CiResponsibilityExtentOptional;
    structural_prop!(
        CiResponsibilityExtentOptional,
        "CiResponsibilityExtentOptional"
    );

    /// party is mandatory (1..*); at least one CI_Individual or CI_Organisation shall be
    /// identified; both name fields may not simultaneously be null.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Responsibility / party
    pub struct CiResponsibilityPartyMandatory;
    structural_prop!(
        CiResponsibilityPartyMandatory,
        "CiResponsibilityPartyMandatory"
    );

    /// party name constraint: among all parties listed in a CI_Responsibility, at least
    /// one shall carry a non-null CI_Individual.name or CI_Organisation.name value.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Responsibility / party / name
    pub struct CiResponsibilityPartyNameNonNull;
    structural_prop!(
        CiResponsibilityPartyNameNonNull,
        "CiResponsibilityPartyNameNonNull"
    );

    /// CI_RoleCode: resourceProvider — the party that supplies the physical media or
    /// channel through which the resource is delivered to users.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / resourceProvider
    pub struct CiRoleResourceProvider;
    structural_prop!(CiRoleResourceProvider, "CiRoleResourceProvider");

    /// CI_RoleCode: custodian — the party that accepts accountability and responsibility
    /// for the data and ensures appropriate care and maintenance of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / custodian
    pub struct CiRoleCustodian;
    structural_prop!(CiRoleCustodian, "CiRoleCustodian");

    /// CI_RoleCode: owner — the party that owns the resource; may differ from the
    /// custodian who manages it day-to-day.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / owner
    pub struct CiRoleOwner;
    structural_prop!(CiRoleOwner, "CiRoleOwner");

    /// CI_RoleCode: user — a party who uses the resource; typically not a producer;
    /// may trigger documentation of user communities in the metadata.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / user
    pub struct CiRoleUser;
    structural_prop!(CiRoleUser, "CiRoleUser");

    /// CI_RoleCode: distributor — the party who distributes the resource; handles
    /// ordering, packaging, and delivery to end users.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / distributor
    pub struct CiRoleDistributor;
    structural_prop!(CiRoleDistributor, "CiRoleDistributor");

    /// CI_RoleCode: originator — the party responsible for creating the resource;
    /// may differ from publisher if creation and dissemination roles are separated.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / originator
    pub struct CiRoleOriginator;
    structural_prop!(CiRoleOriginator, "CiRoleOriginator");

    /// CI_RoleCode: pointOfContact — the party to contact for acquiring knowledge about
    /// or acquisition of the resource; primary user-facing contact.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / pointOfContact
    pub struct CiRolePointOfContact;
    structural_prop!(CiRolePointOfContact, "CiRolePointOfContact");

    /// CI_RoleCode: principalInvestigator — the key party responsible for gathering
    /// information and conducting research that produced the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / principalInvestigator
    pub struct CiRolePrincipalInvestigator;
    structural_prop!(CiRolePrincipalInvestigator, "CiRolePrincipalInvestigator");

    /// CI_RoleCode: processor — the party who has processed the data in a manner such
    /// that the resource has been modified (e.g., re-projected, resampled, merged).
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / processor
    pub struct CiRoleProcessor;
    structural_prop!(CiRoleProcessor, "CiRoleProcessor");

    /// CI_RoleCode: publisher — the party who published the resource; formally
    /// disseminates the resource to the public or a defined audience.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / publisher
    pub struct CiRolePublisher;
    structural_prop!(CiRolePublisher, "CiRolePublisher");

    /// CI_RoleCode: author — the party who authored the resource content; credited
    /// for intellectual creation of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / author
    pub struct CiRoleAuthor;
    structural_prop!(CiRoleAuthor, "CiRoleAuthor");

    /// CI_RoleCode: sponsor — the party who financially supports or underwrites
    /// production of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / sponsor
    pub struct CiRoleSponsor;
    structural_prop!(CiRoleSponsor, "CiRoleSponsor");

    /// CI_RoleCode: coAuthor — a co-author of the resource; used when multiple parties
    /// share authorial credit with the primary author.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / coAuthor
    pub struct CiRoleCoAuthor;
    structural_prop!(CiRoleCoAuthor, "CiRoleCoAuthor");

    /// CI_RoleCode: collaborator — a party who contributed to the resource but does not
    /// meet the authorial threshold for co-author credit.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / collaborator
    pub struct CiRoleCollaborator;
    structural_prop!(CiRoleCollaborator, "CiRoleCollaborator");

    /// CI_RoleCode: editor — the party who reviewed, revised, or compiled the resource
    /// content for publication or release.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / editor
    pub struct CiRoleEditor;
    structural_prop!(CiRoleEditor, "CiRoleEditor");

    /// CI_RoleCode: mediator — an entity that intermediates access to the resource
    /// and for whose use the resource is intended or useful.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / mediator
    pub struct CiRoleMediator;
    structural_prop!(CiRoleMediator, "CiRoleMediator");

    /// CI_RoleCode: rightsHolder — the party owning or managing rights over the resource;
    /// may differ from the owner if rights have been licensed or transferred.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / rightsHolder
    pub struct CiRoleRightsHolder;
    structural_prop!(CiRoleRightsHolder, "CiRoleRightsHolder");

    /// CI_RoleCode: contributor — a party who contributed to the intellectual content
    /// of the resource in a manner not captured by other role codes.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / contributor
    pub struct CiRoleContributor;
    structural_prop!(CiRoleContributor, "CiRoleContributor");

    /// CI_RoleCode: funder — the party who provided financial support for producing
    /// the resource; may include grant agencies, foundations, or corporate sponsors.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / funder
    pub struct CiRoleFunder;
    structural_prop!(CiRoleFunder, "CiRoleFunder");

    /// CI_RoleCode: stakeholder — a party with an interest in the resource or in
    /// decisions affecting the resource, without necessarily having a production role.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_RoleCode / stakeholder
    pub struct CiRoleStakeholder;
    structural_prop!(CiRoleStakeholder, "CiRoleStakeholder");

    /// citation is mandatory (1); shall reference a CI_Citation with a non-empty title;
    /// provides the formal bibliographic identity of the described resource.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / citation
    pub struct MdIdentificationCitationMandatory;
    structural_prop!(
        MdIdentificationCitationMandatory,
        "MdIdentificationCitationMandatory"
    );

    /// abstract is mandatory (1); shall be a non-empty CharacterString providing a
    /// brief, plain-language description of the resource content and purpose.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / abstract
    pub struct MdIdentificationAbstractMandatory;
    structural_prop!(
        MdIdentificationAbstractMandatory,
        "MdIdentificationAbstractMandatory"
    );

    /// abstract shall not be the empty string; even a single sentence describing the
    /// resource satisfies this constraint; whitespace-only strings are not valid.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / abstract
    pub struct MdIdentificationAbstractNonEmpty;
    structural_prop!(
        MdIdentificationAbstractNonEmpty,
        "MdIdentificationAbstractNonEmpty"
    );

    /// purpose is optional (0..1); a summary of the intentions with which the resource
    /// was developed; complements abstract with motivational context.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / purpose
    pub struct MdIdentificationPurposeOptional;
    structural_prop!(
        MdIdentificationPurposeOptional,
        "MdIdentificationPurposeOptional"
    );

    /// credit is optional (0..*); free-text acknowledgements of parties who contributed
    /// to the resource but are not captured in CI_Responsibility roles.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / credit
    pub struct MdIdentificationCreditOptional;
    structural_prop!(
        MdIdentificationCreditOptional,
        "MdIdentificationCreditOptional"
    );

    /// status is optional (0..*); one or more MD_ProgressCode values indicating the
    /// current development or availability state of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / status
    pub struct MdIdentificationStatusOptional;
    structural_prop!(
        MdIdentificationStatusOptional,
        "MdIdentificationStatusOptional"
    );

    /// MD_ProgressCode: completed — production of the resource has been completed
    /// and it is available for use; no further updates are planned.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / completed
    pub struct MdProgressCodeCompleted;
    structural_prop!(MdProgressCodeCompleted, "MdProgressCodeCompleted");

    /// MD_ProgressCode: historicalArchive — the resource has been stored in an offline
    /// or archival facility; may not be immediately accessible.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / historicalArchive
    pub struct MdProgressCodeHistoricalArchive;
    structural_prop!(
        MdProgressCodeHistoricalArchive,
        "MdProgressCodeHistoricalArchive"
    );

    /// MD_ProgressCode: obsolete — the resource is no longer relevant or useful;
    /// users should seek a replacement or more current version.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / obsolete
    pub struct MdProgressCodeObsolete;
    structural_prop!(MdProgressCodeObsolete, "MdProgressCodeObsolete");

    /// MD_ProgressCode: onGoing — the resource is continually being updated with new
    /// data; data currency is a defining characteristic of this status.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / onGoing
    pub struct MdProgressCodeOnGoing;
    structural_prop!(MdProgressCodeOnGoing, "MdProgressCodeOnGoing");

    /// MD_ProgressCode: planned — the resource does not yet exist but is scheduled
    /// for future production; useful for resource discovery in advance.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / planned
    pub struct MdProgressCodePlanned;
    structural_prop!(MdProgressCodePlanned, "MdProgressCodePlanned");

    /// MD_ProgressCode: required — the resource is needed but does not exist; flags
    /// a gap in the data holdings of the documenting organisation.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / required
    pub struct MdProgressCodeRequired;
    structural_prop!(MdProgressCodeRequired, "MdProgressCodeRequired");

    /// MD_ProgressCode: underDevelopment — the resource is being actively produced
    /// but is not yet complete; intermediate versions may be available.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / underDevelopment
    pub struct MdProgressCodeUnderDevelopment;
    structural_prop!(
        MdProgressCodeUnderDevelopment,
        "MdProgressCodeUnderDevelopment"
    );

    /// MD_ProgressCode: final — the resource is the definitive, authoritative version;
    /// no further changes are expected outside of formal revision processes.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / final
    pub struct MdProgressCodeFinal;
    structural_prop!(MdProgressCodeFinal, "MdProgressCodeFinal");

    /// MD_ProgressCode: pending — the resource is awaiting an action or decision
    /// before it can advance to another state.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / pending
    pub struct MdProgressCodePending;
    structural_prop!(MdProgressCodePending, "MdProgressCodePending");

    /// MD_ProgressCode: retired — the resource is no longer being maintained;
    /// it remains available but may not reflect current conditions.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / retired
    pub struct MdProgressCodeRetired;
    structural_prop!(MdProgressCodeRetired, "MdProgressCodeRetired");

    /// MD_ProgressCode: superseded — the resource has been replaced by a newer version
    /// or a different resource that serves the same purpose.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / superseded
    pub struct MdProgressCodeSuperseded;
    structural_prop!(MdProgressCodeSuperseded, "MdProgressCodeSuperseded");

    /// MD_ProgressCode: tentative — the resource is available on a provisional basis;
    /// content may change significantly before it is finalised.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / tentative
    pub struct MdProgressCodeTentative;
    structural_prop!(MdProgressCodeTentative, "MdProgressCodeTentative");

    /// MD_ProgressCode: valid — the resource has been assessed as accurate and current;
    /// typically follows a formal validation or certification process.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / valid
    pub struct MdProgressCodeValid;
    structural_prop!(MdProgressCodeValid, "MdProgressCodeValid");

    /// MD_ProgressCode: accepted — the resource has been accepted as part of an
    /// official set; commonly used in standards or regulatory contexts.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / accepted
    pub struct MdProgressCodeAccepted;
    structural_prop!(MdProgressCodeAccepted, "MdProgressCodeAccepted");

    /// MD_ProgressCode: notAccepted — the resource was reviewed but was not accepted
    /// into an official set; documents failed submissions.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / notAccepted
    pub struct MdProgressCodeNotAccepted;
    structural_prop!(MdProgressCodeNotAccepted, "MdProgressCodeNotAccepted");

    /// MD_ProgressCode: withdrawn — the resource was accepted but has since been
    /// formally withdrawn from the official set.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / withdrawn
    pub struct MdProgressCodeWithdrawn;
    structural_prop!(MdProgressCodeWithdrawn, "MdProgressCodeWithdrawn");

    /// MD_ProgressCode: proposed — the resource has been proposed for acceptance into
    /// an official set but has not yet been reviewed.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / proposed
    pub struct MdProgressCodeProposed;
    structural_prop!(MdProgressCodeProposed, "MdProgressCodeProposed");

    /// MD_ProgressCode: deprecated — the resource is still available but its use is
    /// discouraged; a preferred alternative should be referenced.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_ProgressCode / deprecated
    pub struct MdProgressCodeDeprecated;
    structural_prop!(MdProgressCodeDeprecated, "MdProgressCodeDeprecated");

    /// pointOfContact is optional (0..*); identifies parties to contact for questions
    /// about the described resource; distinct from MD_Metadata.contact.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / pointOfContact
    pub struct MdIdentificationPointOfContactOptional;
    structural_prop!(
        MdIdentificationPointOfContactOptional,
        "MdIdentificationPointOfContactOptional"
    );

    /// resourceMaintenance is optional (0..*); describes the update frequency and scope
    /// for ongoing maintenance of the described resource.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / resourceMaintenance
    pub struct MdIdentificationResourceMaintenanceOptional;
    structural_prop!(
        MdIdentificationResourceMaintenanceOptional,
        "MdIdentificationResourceMaintenanceOptional"
    );

    /// graphicOverview is optional (0..*); each MD_BrowseGraphic provides a thumbnail
    /// or overview image illustrating the content of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / graphicOverview
    pub struct MdIdentificationGraphicOverviewOptional;
    structural_prop!(
        MdIdentificationGraphicOverviewOptional,
        "MdIdentificationGraphicOverviewOptional"
    );

    /// resourceFormat is optional (0..*); each MD_Format entry documents a format in
    /// which the resource is available; cite the format specification.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / resourceFormat
    pub struct MdIdentificationResourceFormatOptional;
    structural_prop!(
        MdIdentificationResourceFormatOptional,
        "MdIdentificationResourceFormatOptional"
    );

    /// descriptiveKeywords is optional (0..*); each MD_Keywords entry provides a set
    /// of keywords with optional type classification and thesaurus citation.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / descriptiveKeywords
    pub struct MdIdentificationDescriptiveKeywordsOptional;
    structural_prop!(
        MdIdentificationDescriptiveKeywordsOptional,
        "MdIdentificationDescriptiveKeywordsOptional"
    );

    /// resourceConstraints is optional (0..*); each MD_Constraints or MD_LegalConstraints
    /// entry documents access or use restrictions on the described resource.
    ///
    /// Source: ISO 19115-1:2014 §6.13 — MD_Identification / resourceConstraints
    pub struct MdIdentificationResourceConstraintsOptional;
    structural_prop!(
        MdIdentificationResourceConstraintsOptional,
        "MdIdentificationResourceConstraintsOptional"
    );

    /// language is conditional (1..*) for datasets; at least one ISO 639-2 language code
    /// shall be provided if the dataset content is expressed in a human language.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / language
    pub struct MdDataIdentificationLanguageConditional;
    structural_prop!(
        MdDataIdentificationLanguageConditional,
        "MdDataIdentificationLanguageConditional"
    );

    /// characterSet is conditional (0..*); required when the dataset character encoding
    /// is not UTF-8; value shall be from MD_CharacterSetCode.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / characterSet
    pub struct MdDataIdentificationCharacterSetConditional;
    structural_prop!(
        MdDataIdentificationCharacterSetConditional,
        "MdDataIdentificationCharacterSetConditional"
    );

    /// topicCategory is conditional (0..*) for datasets and series; at least one
    /// MD_TopicCategoryCode shall classify the general theme of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / topicCategory
    pub struct MdDataIdentificationTopicCategoryConditional;
    structural_prop!(
        MdDataIdentificationTopicCategoryConditional,
        "MdDataIdentificationTopicCategoryConditional"
    );

    /// MD_TopicCategoryCode: farming — agriculture, irrigation, aquaculture,
    /// plantations, herding, pests and diseases affecting crops and livestock.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / farming
    pub struct MdTopicCategoryFarming;
    structural_prop!(MdTopicCategoryFarming, "MdTopicCategoryFarming");

    /// MD_TopicCategoryCode: biota — flora and/or fauna in natural environment;
    /// wildlife, vegetation, biological sciences, ecology, wilderness.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / biota
    pub struct MdTopicCategoryBiota;
    structural_prop!(MdTopicCategoryBiota, "MdTopicCategoryBiota");

    /// MD_TopicCategoryCode: boundaries — legal land descriptions; political and
    /// administrative boundaries; census enumeration areas.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / boundaries
    pub struct MdTopicCategoryBoundaries;
    structural_prop!(MdTopicCategoryBoundaries, "MdTopicCategoryBoundaries");

    /// MD_TopicCategoryCode: climatologyMeteorologyAtmosphere — atmospheric processes
    /// and phenomena; climate; meteorology; weather; atmospheric conditions.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / climatologyMeteorologyAtmosphere
    pub struct MdTopicCategoryClimatologyMeteorologyAtmosphere;
    structural_prop!(
        MdTopicCategoryClimatologyMeteorologyAtmosphere,
        "MdTopicCategoryClimatologyMeteorologyAtmosphere"
    );

    /// MD_TopicCategoryCode: economy — economic activities, conditions and employment;
    /// labour, revenue, commerce, industry, tourism.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / economy
    pub struct MdTopicCategoryEconomy;
    structural_prop!(MdTopicCategoryEconomy, "MdTopicCategoryEconomy");

    /// MD_TopicCategoryCode: elevation — heights above or below sea level;
    /// altitude, depth, digital elevation models, bathymetric charts.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / elevation
    pub struct MdTopicCategoryElevation;
    structural_prop!(MdTopicCategoryElevation, "MdTopicCategoryElevation");

    /// MD_TopicCategoryCode: environment — environmental resources, protection and
    /// conservation; pollution, waste storage, environmental impact assessment.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / environment
    pub struct MdTopicCategoryEnvironment;
    structural_prop!(MdTopicCategoryEnvironment, "MdTopicCategoryEnvironment");

    /// MD_TopicCategoryCode: geoscientificInformation — earth sciences; geology,
    /// minerals, seismology, volcanic activity, landslides, gravity.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / geoscientificInformation
    pub struct MdTopicCategoryGeoscientificInformation;
    structural_prop!(
        MdTopicCategoryGeoscientificInformation,
        "MdTopicCategoryGeoscientificInformation"
    );

    /// MD_TopicCategoryCode: health — health, health services, human ecology, and
    /// safety; disease, hygiene, substance abuse, mental and physical health.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / health
    pub struct MdTopicCategoryHealth;
    structural_prop!(MdTopicCategoryHealth, "MdTopicCategoryHealth");

    /// MD_TopicCategoryCode: imageryBaseMapsEarthCover — base maps; land cover;
    /// topographic maps; classified and unclassified images, annotations.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / imageryBaseMapsEarthCover
    pub struct MdTopicCategoryImageryBaseMapsEarthCover;
    structural_prop!(
        MdTopicCategoryImageryBaseMapsEarthCover,
        "MdTopicCategoryImageryBaseMapsEarthCover"
    );

    /// MD_TopicCategoryCode: intelligenceMilitary — military bases, structures,
    /// activities; nuclear power plants; troop movements.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / intelligenceMilitary
    pub struct MdTopicCategoryIntelligenceMilitary;
    structural_prop!(
        MdTopicCategoryIntelligenceMilitary,
        "MdTopicCategoryIntelligenceMilitary"
    );

    /// MD_TopicCategoryCode: inlandWaters — inland water features, drainage systems
    /// and their characteristics; rivers, glaciers, saline lakes, water utilisation.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / inlandWaters
    pub struct MdTopicCategoryInlandWaters;
    structural_prop!(MdTopicCategoryInlandWaters, "MdTopicCategoryInlandWaters");

    /// MD_TopicCategoryCode: location — positional information and services;
    /// addresses, geodetic networks, control points, postal zones.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / location
    pub struct MdTopicCategoryLocation;
    structural_prop!(MdTopicCategoryLocation, "MdTopicCategoryLocation");

    /// MD_TopicCategoryCode: oceans — features and characteristics of salt water
    /// bodies (excluding inland waters); tides, tsunamis, coastal information.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / oceans
    pub struct MdTopicCategoryOceans;
    structural_prop!(MdTopicCategoryOceans, "MdTopicCategoryOceans");

    /// MD_TopicCategoryCode: planningCadastre — information used for appropriate
    /// future use of land; land use maps, zoning, cadastral surveys.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / planningCadastre
    pub struct MdTopicCategoryPlanningCadastre;
    structural_prop!(
        MdTopicCategoryPlanningCadastre,
        "MdTopicCategoryPlanningCadastre"
    );

    /// MD_TopicCategoryCode: society — characteristics of society and cultures;
    /// natural settlements, anthropology, archaeology, education, demographics.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / society
    pub struct MdTopicCategorySociety;
    structural_prop!(MdTopicCategorySociety, "MdTopicCategorySociety");

    /// MD_TopicCategoryCode: structure — man-made construction; buildings, museums,
    /// churches, factories, housing, monuments, shops, towers.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / structure
    pub struct MdTopicCategoryStructure;
    structural_prop!(MdTopicCategoryStructure, "MdTopicCategoryStructure");

    /// MD_TopicCategoryCode: transportation — means and aids for conveying persons
    /// and/or goods; roads, airports, shipping routes, tunnels, nautical charts.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / transportation
    pub struct MdTopicCategoryTransportation;
    structural_prop!(
        MdTopicCategoryTransportation,
        "MdTopicCategoryTransportation"
    );

    /// MD_TopicCategoryCode: utilitiesCommunication — energy, water and waste
    /// systems, communications infrastructure; electricity, gas, water supply.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / utilitiesCommunication
    pub struct MdTopicCategoryUtilitiesCommunication;
    structural_prop!(
        MdTopicCategoryUtilitiesCommunication,
        "MdTopicCategoryUtilitiesCommunication"
    );

    /// MD_TopicCategoryCode: extraTerrestrial — regions more than 60 km above
    /// the Earth's surface; celestial bodies; outer space environments.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / extraTerrestrial
    pub struct MdTopicCategoryExtraTerrestrial;
    structural_prop!(
        MdTopicCategoryExtraTerrestrial,
        "MdTopicCategoryExtraTerrestrial"
    );

    /// MD_TopicCategoryCode: disaster — information related to disaster occurrences
    /// and responses; earthquakes, floods, droughts, tsunamis, emergency management.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_TopicCategoryCode / disaster
    pub struct MdTopicCategoryDisaster;
    structural_prop!(MdTopicCategoryDisaster, "MdTopicCategoryDisaster");

    /// extent is conditional (0..*); required when the resource has a geographic, temporal,
    /// or vertical extent; shall provide at least one geographic bounding box for datasets.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / extent
    pub struct MdDataIdentificationExtentConditional;
    structural_prop!(
        MdDataIdentificationExtentConditional,
        "MdDataIdentificationExtentConditional"
    );

    /// MD_SpatialRepresentationTypeCode: vector — vector data representing point, line,
    /// and polygon geometries; the most common type for discrete features.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / vector
    pub struct MdSpatialRepTypeVector;
    structural_prop!(MdSpatialRepTypeVector, "MdSpatialRepTypeVector");

    /// MD_SpatialRepresentationTypeCode: grid — raster/grid data representing continuous
    /// phenomena as a regular array of cells with associated values.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / grid
    pub struct MdSpatialRepTypeGrid;
    structural_prop!(MdSpatialRepTypeGrid, "MdSpatialRepTypeGrid");

    /// MD_SpatialRepresentationTypeCode: textTable — tabular or textual representation
    /// of spatial features without geometric encoding.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / textTable
    pub struct MdSpatialRepTypeTextTable;
    structural_prop!(MdSpatialRepTypeTextTable, "MdSpatialRepTypeTextTable");

    /// MD_SpatialRepresentationTypeCode: tin — triangulated irregular network;
    /// a vector surface model using irregular triangles.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / tin
    pub struct MdSpatialRepTypeTin;
    structural_prop!(MdSpatialRepTypeTin, "MdSpatialRepTypeTin");

    /// MD_SpatialRepresentationTypeCode: stereoModel — three-dimensional view formed
    /// from two images of the same area taken from different angles.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / stereoModel
    pub struct MdSpatialRepTypeStereoModel;
    structural_prop!(MdSpatialRepTypeStereoModel, "MdSpatialRepTypeStereoModel");

    /// MD_SpatialRepresentationTypeCode: video — scene from a video recording;
    /// temporal sequence of images covering a spatial area.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_SpatialRepresentationTypeCode / video
    pub struct MdSpatialRepTypeVideo;
    structural_prop!(MdSpatialRepTypeVideo, "MdSpatialRepTypeVideo");

    /// spatialResolution is optional (0..*); each MD_Resolution entry expresses the
    /// nominal scale, ground sample distance, or angular distance of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / spatialResolution
    pub struct MdDataIdentificationSpatialResolutionOptional;
    structural_prop!(
        MdDataIdentificationSpatialResolutionOptional,
        "MdDataIdentificationSpatialResolutionOptional"
    );

    /// supplementalInformation is optional (0..1); any other descriptive information
    /// about the resource that does not fit into structured attributes.
    ///
    /// Source: ISO 19115-1:2014 §6.12 — MD_DataIdentification / supplementalInformation
    pub struct MdDataIdentificationSupplementalInfoOptional;
    structural_prop!(
        MdDataIdentificationSupplementalInfoOptional,
        "MdDataIdentificationSupplementalInfoOptional"
    );

    /// keyword is mandatory (1..*); at least one non-empty CharacterString keyword shall
    /// be present; all keywords in one MD_Keywords instance should share a common type.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_Keywords / keyword
    pub struct MdKeywordsKeywordMandatory;
    structural_prop!(MdKeywordsKeywordMandatory, "MdKeywordsKeywordMandatory");

    /// type is optional (0..1); classifies the nature of all keywords in this group;
    /// value from MD_KeywordTypeCode; omitting implies the type is unspecified.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_Keywords / type
    pub struct MdKeywordsTypeOptional;
    structural_prop!(MdKeywordsTypeOptional, "MdKeywordsTypeOptional");

    /// thesaurusName is optional (0..1); when keywords are drawn from a controlled
    /// vocabulary, this CI_Citation identifies the vocabulary and its edition.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_Keywords / thesaurusName
    pub struct MdKeywordsThesaurusNameOptional;
    structural_prop!(
        MdKeywordsThesaurusNameOptional,
        "MdKeywordsThesaurusNameOptional"
    );

    /// keywordClass is optional (0..1); provides an ontology-referenced class to which
    /// the keywords belong; extends keyword semantics beyond a simple type code.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_Keywords / keywordClass
    pub struct MdKeywordsKeywordClassOptional;
    structural_prop!(
        MdKeywordsKeywordClassOptional,
        "MdKeywordsKeywordClassOptional"
    );

    /// MD_KeywordTypeCode: discipline — the applied sciences or disciplines to which
    /// the resource relates (e.g., hydrology, forestry, seismology).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / discipline
    pub struct MdKeywordTypeDiscipline;
    structural_prop!(MdKeywordTypeDiscipline, "MdKeywordTypeDiscipline");

    /// MD_KeywordTypeCode: place — the geographic location where the resource content
    /// is relevant; geographic names, administrative units, feature names.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / place
    pub struct MdKeywordTypePlace;
    structural_prop!(MdKeywordTypePlace, "MdKeywordTypePlace");

    /// MD_KeywordTypeCode: stratum — the layer(s) of any deposited substance or
    /// layer(s) within the body of water described by the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / stratum
    pub struct MdKeywordTypeStratum;
    structural_prop!(MdKeywordTypeStratum, "MdKeywordTypeStratum");

    /// MD_KeywordTypeCode: temporal — a named time period described by the resource;
    /// distinguishes time-period keywords from spatial extent keywords.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / temporal
    pub struct MdKeywordTypeTemporal;
    structural_prop!(MdKeywordTypeTemporal, "MdKeywordTypeTemporal");

    /// MD_KeywordTypeCode: theme — the subject or topic of the resource content;
    /// the most broadly applicable keyword type for general subject matter.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / theme
    pub struct MdKeywordTypeTheme;
    structural_prop!(MdKeywordTypeTheme, "MdKeywordTypeTheme");

    /// MD_KeywordTypeCode: dataCentre — identifies a data centre or data repository
    /// where the resource is held or available.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / dataCentre
    pub struct MdKeywordTypeDataCentre;
    structural_prop!(MdKeywordTypeDataCentre, "MdKeywordTypeDataCentre");

    /// MD_KeywordTypeCode: featureType — the type of geographic feature described by
    /// the resource (e.g., building, river, road, land parcel).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / featureType
    pub struct MdKeywordTypeFeatureType;
    structural_prop!(MdKeywordTypeFeatureType, "MdKeywordTypeFeatureType");

    /// MD_KeywordTypeCode: instrument — the sensor or measuring instrument used to
    /// collect the data (e.g., LIDAR, Landsat OLI, rain gauge, CTD sensor).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / instrument
    pub struct MdKeywordTypeInstrument;
    structural_prop!(MdKeywordTypeInstrument, "MdKeywordTypeInstrument");

    /// MD_KeywordTypeCode: platform — the platform carrying the instrument that
    /// collected the data (e.g., satellite, aircraft, vessel, ground station).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / platform
    pub struct MdKeywordTypePlatform;
    structural_prop!(MdKeywordTypePlatform, "MdKeywordTypePlatform");

    /// MD_KeywordTypeCode: process — the data-collection or processing methodology
    /// applied to produce the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / process
    pub struct MdKeywordTypeProcess;
    structural_prop!(MdKeywordTypeProcess, "MdKeywordTypeProcess");

    /// MD_KeywordTypeCode: project — the project under whose auspices the resource was
    /// created (e.g., CORINE, Global Land Cover 2000, SRTM).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / project
    pub struct MdKeywordTypeProject;
    structural_prop!(MdKeywordTypeProject, "MdKeywordTypeProject");

    /// MD_KeywordTypeCode: service — the type of service described by the resource
    /// (e.g., WMS, WFS, WCS, download service, processing service).
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / service
    pub struct MdKeywordTypeService;
    structural_prop!(MdKeywordTypeService, "MdKeywordTypeService");

    /// MD_KeywordTypeCode: product — a product identifier or name; identifies
    /// commercial or official data products described by the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / product
    pub struct MdKeywordTypeProduct;
    structural_prop!(MdKeywordTypeProduct, "MdKeywordTypeProduct");

    /// MD_KeywordTypeCode: subTopicCategory — a refinement of MD_TopicCategoryCode;
    /// used when a topic category code is too broad for precise classification.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / subTopicCategory
    pub struct MdKeywordTypeSubTopicCategory;
    structural_prop!(
        MdKeywordTypeSubTopicCategory,
        "MdKeywordTypeSubTopicCategory"
    );

    /// MD_KeywordTypeCode: taxon — taxonomic information for biological resources;
    /// identifies species, genus, family, or other taxonomic ranks.
    ///
    /// Source: ISO 19115-1:2014 §6.14 — MD_KeywordTypeCode / taxon
    pub struct MdKeywordTypeTaxon;
    structural_prop!(MdKeywordTypeTaxon, "MdKeywordTypeTaxon");

    /// description is optional (0..1); a prose description of the extent in natural
    /// language; may complement or substitute for structured extent elements.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent / description
    pub struct ExExtentDescriptionOptional;
    structural_prop!(ExExtentDescriptionOptional, "ExExtentDescriptionOptional");

    /// geographicElement is conditional (0..*); shall be present when the resource has
    /// geographic extent; EX_GeographicBoundingBox is the most common subclass.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent / geographicElement
    pub struct ExExtentGeographicElementConditional;
    structural_prop!(
        ExExtentGeographicElementConditional,
        "ExExtentGeographicElementConditional"
    );

    /// temporalElement is optional (0..*); each EX_TemporalExtent documents a time
    /// period or instant during which the resource content is relevant.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent / temporalElement
    pub struct ExExtentTemporalElementOptional;
    structural_prop!(
        ExExtentTemporalElementOptional,
        "ExExtentTemporalElementOptional"
    );

    /// verticalElement is optional (0..*); each EX_VerticalExtent documents a range
    /// of heights or depths covered by the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent / verticalElement
    pub struct ExExtentVerticalElementOptional;
    structural_prop!(
        ExExtentVerticalElementOptional,
        "ExExtentVerticalElementOptional"
    );

    /// at least one of geographicElement, temporalElement, or verticalElement shall be
    /// present in any EX_Extent instance; a description-only extent is not sufficient.
    ///
    /// Source: ISO 19115-1:2014 §6.16 — EX_Extent (constraint)
    pub struct ExExtentAtLeastOneElementRequired;
    structural_prop!(
        ExExtentAtLeastOneElementRequired,
        "ExExtentAtLeastOneElementRequired"
    );

    /// extentTypeCode is optional (0..1); Boolean indicating inclusion (true) or
    /// exclusion (false) of the described area; default is true (inclusion).
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox / extentTypeCode
    pub struct ExBboxExtentTypeCodeOptional;
    structural_prop!(ExBboxExtentTypeCodeOptional, "ExBboxExtentTypeCodeOptional");

    /// westBoundLongitude is mandatory (1); western-most coordinate of the bounding box
    /// in decimal degrees; shall be in the range [-180.0, 180.0] inclusive.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox / westBoundLongitude
    pub struct ExBboxWestBoundMandatory;
    structural_prop!(ExBboxWestBoundMandatory, "ExBboxWestBoundMandatory");

    /// eastBoundLongitude is mandatory (1); eastern-most coordinate of the bounding box
    /// in decimal degrees; shall be in the range [-180.0, 180.0] inclusive.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox / eastBoundLongitude
    pub struct ExBboxEastBoundMandatory;
    structural_prop!(ExBboxEastBoundMandatory, "ExBboxEastBoundMandatory");

    /// southBoundLatitude is mandatory (1); southern-most coordinate in decimal degrees;
    /// shall be in the range [-90.0, 90.0] inclusive.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox / southBoundLatitude
    pub struct ExBboxSouthBoundMandatory;
    structural_prop!(ExBboxSouthBoundMandatory, "ExBboxSouthBoundMandatory");

    /// northBoundLatitude is mandatory (1); northern-most coordinate in decimal degrees;
    /// shall be in the range [-90.0, 90.0] inclusive.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox / northBoundLatitude
    pub struct ExBboxNorthBoundMandatory;
    structural_prop!(ExBboxNorthBoundMandatory, "ExBboxNorthBoundMandatory");

    /// longitude range constraint: both westBoundLongitude and eastBoundLongitude shall
    /// be in the closed interval [-180.0, 180.0]; values outside this range are invalid.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox (constraint)
    pub struct ExBboxLongitudeRange;
    structural_prop!(ExBboxLongitudeRange, "ExBboxLongitudeRange");

    /// latitude range constraint: both southBoundLatitude and northBoundLatitude shall
    /// be in the closed interval [-90.0, 90.0]; values outside this range are invalid.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox (constraint)
    pub struct ExBboxLatitudeRange;
    structural_prop!(ExBboxLatitudeRange, "ExBboxLatitudeRange");

    /// south-north ordering constraint: southBoundLatitude shall be less than or equal
    /// to northBoundLatitude; equal values represent a single parallel.
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox (constraint)
    pub struct ExBboxSouthLeNorth;
    structural_prop!(ExBboxSouthLeNorth, "ExBboxSouthLeNorth");

    /// antimeridian crossing constraint: westBoundLongitude may exceed eastBoundLongitude
    /// only when the bounding box crosses the antimeridian (180°/-180° line).
    ///
    /// Source: ISO 19115-1:2014 §6.17 — EX_GeographicBoundingBox (constraint)
    pub struct ExBboxWestLeEastOrAntimeridian;
    structural_prop!(
        ExBboxWestLeEastOrAntimeridian,
        "ExBboxWestLeEastOrAntimeridian"
    );

    /// extent is mandatory (1); shall be a TM_Instant (single point) or TM_Period
    /// (begin/end interval); temporal values shall conform to ISO 8601.
    ///
    /// Source: ISO 19115-1:2014 §6.18 — EX_TemporalExtent / extent
    pub struct ExTemporalExtentExtentMandatory;
    structural_prop!(
        ExTemporalExtentExtentMandatory,
        "ExTemporalExtentExtentMandatory"
    );

    /// TM_Period ordering constraint: when extent is a TM_Period, the begin instant
    /// shall be chronologically less than or equal to the end instant.
    ///
    /// Source: ISO 19115-1:2014 §6.18 — EX_TemporalExtent / extent (constraint)
    pub struct ExTemporalExtentPeriodBeginLeEnd;
    structural_prop!(
        ExTemporalExtentPeriodBeginLeEnd,
        "ExTemporalExtentPeriodBeginLeEnd"
    );

    /// minimumValue is mandatory (1); the minimum vertical extent value in units of
    /// the vertical CRS; negative values denote depths below the reference surface.
    ///
    /// Source: ISO 19115-1:2014 §6.19 — EX_VerticalExtent / minimumValue
    pub struct ExVerticalExtentMinimumMandatory;
    structural_prop!(
        ExVerticalExtentMinimumMandatory,
        "ExVerticalExtentMinimumMandatory"
    );

    /// maximumValue is mandatory (1); the maximum vertical extent value in units of
    /// the vertical CRS; shall be greater than or equal to minimumValue.
    ///
    /// Source: ISO 19115-1:2014 §6.19 — EX_VerticalExtent / maximumValue
    pub struct ExVerticalExtentMaximumMandatory;
    structural_prop!(
        ExVerticalExtentMaximumMandatory,
        "ExVerticalExtentMaximumMandatory"
    );

    /// verticalCRSId is optional (0..1); a reference to the SC_CRS that defines the
    /// vertical datum, units, and direction (up/down positive) for the extent values.
    ///
    /// Source: ISO 19115-1:2014 §6.19 — EX_VerticalExtent / verticalCRSId
    pub struct ExVerticalExtentCrsOptional;
    structural_prop!(ExVerticalExtentCrsOptional, "ExVerticalExtentCrsOptional");

    /// min-max ordering constraint: minimumValue shall be less than or equal to
    /// maximumValue; equal values represent a single horizontal surface or plane.
    ///
    /// Source: ISO 19115-1:2014 §6.19 — EX_VerticalExtent (constraint)
    pub struct ExVerticalExtentMinLeMax;
    structural_prop!(ExVerticalExtentMinLeMax, "ExVerticalExtentMinLeMax");

    /// formatSpecificationCitation is mandatory (1); a CI_Citation identifying the
    /// standard, specification, or document that defines this format.
    ///
    /// Source: ISO 19115-1:2014 §6.21 — MD_Format / formatSpecificationCitation
    pub struct MdFormatSpecificationCitationMandatory;
    structural_prop!(
        MdFormatSpecificationCitationMandatory,
        "MdFormatSpecificationCitationMandatory"
    );

    /// amendmentNumber is optional (0..1); the amendment or patch number of the format
    /// version being described (e.g., "Amd. 1", "Corr. 2").
    ///
    /// Source: ISO 19115-1:2014 §6.21 — MD_Format / amendmentNumber
    pub struct MdFormatAmendmentNumberOptional;
    structural_prop!(
        MdFormatAmendmentNumberOptional,
        "MdFormatAmendmentNumberOptional"
    );

    /// fileDecompressionTechnique is optional (0..1); the algorithm or process used
    /// to decompress the digital resource (e.g., ZIP, GZIP, LZW, bzip2).
    ///
    /// Source: ISO 19115-1:2014 §6.21 — MD_Format / fileDecompressionTechnique
    pub struct MdFormatFileDecompressionOptional;
    structural_prop!(
        MdFormatFileDecompressionOptional,
        "MdFormatFileDecompressionOptional"
    );

    /// medium is optional (0..*); each MD_Medium entry describes the physical medium
    /// on which the resource is stored or delivered (e.g., DVD, USB, cloud storage).
    ///
    /// Source: ISO 19115-1:2014 §6.21 — MD_Format / medium
    pub struct MdFormatMediumOptional;
    structural_prop!(MdFormatMediumOptional, "MdFormatMediumOptional");

    /// formatDistributor is optional (0..*); each MD_Distributor entry identifies a
    /// party that distributes the resource in this specific format.
    ///
    /// Source: ISO 19115-1:2014 §6.21 — MD_Format / formatDistributor
    pub struct MdFormatDistributorOptional;
    structural_prop!(MdFormatDistributorOptional, "MdFormatDistributorOptional");

    /// useLimitation is optional (0..*); free-text descriptions of limitations affecting
    /// the fitness of the resource for a particular use (not legal restrictions).
    ///
    /// Source: ISO 19115-1:2014 §6.22 — MD_Constraints / useLimitation
    pub struct MdConstraintsUseLimitationOptional;
    structural_prop!(
        MdConstraintsUseLimitationOptional,
        "MdConstraintsUseLimitationOptional"
    );

    /// constraintApplicationScope is optional (0..1); an MD_Scope that qualifies to
    /// which part of the resource or metadata the constraints apply.
    ///
    /// Source: ISO 19115-1:2014 §6.22 — MD_Constraints / constraintApplicationScope
    pub struct MdConstraintsApplicationScopeOptional;
    structural_prop!(
        MdConstraintsApplicationScopeOptional,
        "MdConstraintsApplicationScopeOptional"
    );

    /// accessConstraints is optional (0..*); one or more MD_RestrictionCode values
    /// documenting restrictions on access to the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_LegalConstraints / accessConstraints
    pub struct MdLegalConstraintsAccessConstraintsOptional;
    structural_prop!(
        MdLegalConstraintsAccessConstraintsOptional,
        "MdLegalConstraintsAccessConstraintsOptional"
    );

    /// useConstraints is optional (0..*); one or more MD_RestrictionCode values
    /// documenting restrictions on use of the resource once accessed.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_LegalConstraints / useConstraints
    pub struct MdLegalConstraintsUseConstraintsOptional;
    structural_prop!(
        MdLegalConstraintsUseConstraintsOptional,
        "MdLegalConstraintsUseConstraintsOptional"
    );

    /// otherConstraints is conditional (0..*); required when any restriction code is
    /// otherRestrictions; provides the specific constraint text.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_LegalConstraints / otherConstraints
    pub struct MdLegalConstraintsOtherConstraintsConditional;
    structural_prop!(
        MdLegalConstraintsOtherConstraintsConditional,
        "MdLegalConstraintsOtherConstraintsConditional"
    );

    /// MD_RestrictionCode: copyright — exclusive right to the publication, production,
    /// or sale of the rights to a literary, dramatic, musical, or artistic work.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / copyright
    pub struct MdRestrictionCopyrightCode;
    structural_prop!(MdRestrictionCopyrightCode, "MdRestrictionCopyrightCode");

    /// MD_RestrictionCode: patent — government has granted exclusive right to make,
    /// sell, use or license an invention or discovery.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / patent
    pub struct MdRestrictionPatentCode;
    structural_prop!(MdRestrictionPatentCode, "MdRestrictionPatentCode");

    /// MD_RestrictionCode: patentPending — a patent application has been filed but
    /// the patent has not yet been granted.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / patentPending
    pub struct MdRestrictionPatentPendingCode;
    structural_prop!(
        MdRestrictionPatentPendingCode,
        "MdRestrictionPatentPendingCode"
    );

    /// MD_RestrictionCode: trademark — a name, symbol, or other device identifying a
    /// product has been legally registered or established by use.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / trademark
    pub struct MdRestrictionTrademarkCode;
    structural_prop!(MdRestrictionTrademarkCode, "MdRestrictionTrademarkCode");

    /// MD_RestrictionCode: licence — formal permission to do something; resource is
    /// available under a specific licence agreement.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / licence
    pub struct MdRestrictionLicenceCode;
    structural_prop!(MdRestrictionLicenceCode, "MdRestrictionLicenceCode");

    /// MD_RestrictionCode: intellectualPropertyRights — rights from intangible property
    /// that is a result of creativity, such as trade secrets, design rights.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / intellectualPropertyRights
    pub struct MdRestrictionIntellectualPropertyCode;
    structural_prop!(
        MdRestrictionIntellectualPropertyCode,
        "MdRestrictionIntellectualPropertyCode"
    );

    /// MD_RestrictionCode: restricted — withheld from general circulation or disclosure;
    /// must be paired with otherConstraints to explain the restriction.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / restricted
    pub struct MdRestrictionRestrictedCode;
    structural_prop!(MdRestrictionRestrictedCode, "MdRestrictionRestrictedCode");

    /// MD_RestrictionCode: otherRestrictions — restriction not listed in the
    /// enumeration; otherConstraints shall be non-empty when this code is used.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / otherRestrictions
    pub struct MdRestrictionOtherRestrictionsCode;
    structural_prop!(
        MdRestrictionOtherRestrictionsCode,
        "MdRestrictionOtherRestrictionsCode"
    );

    /// MD_RestrictionCode: unrestricted — no restriction; freely available to all
    /// users; equivalent to a public domain or open access declaration.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / unrestricted
    pub struct MdRestrictionUnrestrictedCode;
    structural_prop!(
        MdRestrictionUnrestrictedCode,
        "MdRestrictionUnrestrictedCode"
    );

    /// MD_RestrictionCode: licenceUnrestricted — available under a licence that
    /// imposes no significant use restrictions on the recipient.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / licenceUnrestricted
    pub struct MdRestrictionLicenceUnrestrictedCode;
    structural_prop!(
        MdRestrictionLicenceUnrestrictedCode,
        "MdRestrictionLicenceUnrestrictedCode"
    );

    /// MD_RestrictionCode: licenceEndUser — end user licence agreement must be accepted
    /// before the resource may be used; applies per user or per organisation.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / licenceEndUser
    pub struct MdRestrictionLicenceEndUserCode;
    structural_prop!(
        MdRestrictionLicenceEndUserCode,
        "MdRestrictionLicenceEndUserCode"
    );

    /// MD_RestrictionCode: licenceDistributor — licence restricts redistribution;
    /// the distributor cannot sub-license without specific permission.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / licenceDistributor
    pub struct MdRestrictionLicenceDistributorCode;
    structural_prop!(
        MdRestrictionLicenceDistributorCode,
        "MdRestrictionLicenceDistributorCode"
    );

    /// MD_RestrictionCode: private — only available to the data producer or associated
    /// internal parties; not available for external distribution.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / private
    pub struct MdRestrictionPrivateCode;
    structural_prop!(MdRestrictionPrivateCode, "MdRestrictionPrivateCode");

    /// MD_RestrictionCode: statutory — restriction imposed by law (statute);
    /// differs from intellectual property rights in its legislative basis.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / statutory
    pub struct MdRestrictionStatutoryCode;
    structural_prop!(MdRestrictionStatutoryCode, "MdRestrictionStatutoryCode");

    /// MD_RestrictionCode: confidential — not available to the public, restricted to
    /// specific authorised parties; often used in government contexts.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / confidential
    pub struct MdRestrictionConfidentialCode;
    structural_prop!(
        MdRestrictionConfidentialCode,
        "MdRestrictionConfidentialCode"
    );

    /// MD_RestrictionCode: SBU — sensitive but unclassified; requires controlled
    /// handling but does not meet the threshold for formal classification.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / SBU
    pub struct MdRestrictionSbuCode;
    structural_prop!(MdRestrictionSbuCode, "MdRestrictionSbuCode");

    /// MD_RestrictionCode: in-confidence — resource is provided in confidence and
    /// shall not be disclosed or redistributed without explicit consent.
    ///
    /// Source: ISO 19115-1:2014 §6.23 — MD_RestrictionCode / in-confidence
    pub struct MdRestrictionInConfidenceCode;
    structural_prop!(
        MdRestrictionInConfidenceCode,
        "MdRestrictionInConfidenceCode"
    );

    /// statement is conditional (0..1); a general explanation of the data producer's
    /// knowledge about the lineage; required when lineage scope is dataset or series.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage / statement
    pub struct LiLineageStatementConditional;
    structural_prop!(
        LiLineageStatementConditional,
        "LiLineageStatementConditional"
    );

    /// scope is optional (0..1); an MD_Scope that specifies the level or part of the
    /// resource to which the lineage information applies.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage / scope
    pub struct LiLineageScopeOptional;
    structural_prop!(LiLineageScopeOptional, "LiLineageScopeOptional");

    /// additionalDocumentation is optional (0..*); CI_Citation references to documents
    /// that provide further information about the lineage of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage / additionalDocumentation
    pub struct LiLineageAdditionalDocumentationOptional;
    structural_prop!(
        LiLineageAdditionalDocumentationOptional,
        "LiLineageAdditionalDocumentationOptional"
    );

    /// source is optional (0..*); each LI_Source describes a data source from which
    /// the described resource was derived.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage / source
    pub struct LiLineageSourceOptional;
    structural_prop!(LiLineageSourceOptional, "LiLineageSourceOptional");

    /// processStep is optional (0..*); each LI_ProcessStep documents a transformation
    /// or process event in the production history of the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage / processStep
    pub struct LiLineageProcessStepOptional;
    structural_prop!(LiLineageProcessStepOptional, "LiLineageProcessStepOptional");

    /// at least one of statement, source, or processStep shall be provided in any
    /// LI_Lineage; a completely empty lineage section is not conformant.
    ///
    /// Source: ISO 19115-1:2014 §6.26 — LI_Lineage (constraint)
    pub struct LiLineageAtLeastOneProvided;
    structural_prop!(LiLineageAtLeastOneProvided, "LiLineageAtLeastOneProvided");

    /// description is mandatory (1); a non-empty CharacterString describing what was
    /// done in this step; shall explain the process method, not just name a tool.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / description
    pub struct LiProcessStepDescriptionMandatory;
    structural_prop!(
        LiProcessStepDescriptionMandatory,
        "LiProcessStepDescriptionMandatory"
    );

    /// description shall not be empty; a process step with an empty description is
    /// not informative and violates the mandatory attribute constraint.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / description
    pub struct LiProcessStepDescriptionNonEmpty;
    structural_prop!(
        LiProcessStepDescriptionNonEmpty,
        "LiProcessStepDescriptionNonEmpty"
    );

    /// rationale is optional (0..1); the reason why the process step was performed;
    /// provides context for why this transformation or operation was applied.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / rationale
    pub struct LiProcessStepRationaleOptional;
    structural_prop!(
        LiProcessStepRationaleOptional,
        "LiProcessStepRationaleOptional"
    );

    /// stepDateTime is optional (0..1); the date and time the process step was performed;
    /// shall conform to ISO 8601 when provided.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / stepDateTime
    pub struct LiProcessStepDateTimeOptional;
    structural_prop!(
        LiProcessStepDateTimeOptional,
        "LiProcessStepDateTimeOptional"
    );

    /// processor is optional (0..*); the party responsible for carrying out the
    /// process step; identifies who performed the described operation.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / processor
    pub struct LiProcessStepProcessorOptional;
    structural_prop!(
        LiProcessStepProcessorOptional,
        "LiProcessStepProcessorOptional"
    );

    /// reference is optional (0..*); CI_Citation references to documentation, standards,
    /// or algorithms that describe the process method in more detail.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / reference
    pub struct LiProcessStepReferenceOptional;
    structural_prop!(
        LiProcessStepReferenceOptional,
        "LiProcessStepReferenceOptional"
    );

    /// scope is optional (0..1); an MD_Scope that qualifies which part of the resource
    /// was affected by this process step.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / scope
    pub struct LiProcessStepScopeOptional;
    structural_prop!(LiProcessStepScopeOptional, "LiProcessStepScopeOptional");

    /// source is optional (0..*); each LI_Source identifies an input dataset or resource
    /// that was consumed or read by this process step.
    ///
    /// Source: ISO 19115-1:2014 §6.27 — LI_ProcessStep / source
    pub struct LiProcessStepSourceOptional;
    structural_prop!(LiProcessStepSourceOptional, "LiProcessStepSourceOptional");

    /// description is conditional (0..1); required when sourceCitation is absent;
    /// provides a textual description of the source dataset.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source / description
    pub struct LiSourceDescriptionConditional;
    structural_prop!(
        LiSourceDescriptionConditional,
        "LiSourceDescriptionConditional"
    );

    /// sourceSpatialResolution is optional (0..1); the spatial resolution or scale of
    /// the source dataset; helps assess positional accuracy propagation.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source / sourceSpatialResolution
    pub struct LiSourceSpatialResolutionOptional;
    structural_prop!(
        LiSourceSpatialResolutionOptional,
        "LiSourceSpatialResolutionOptional"
    );

    /// sourceReferenceSystem is optional (0..1); the coordinate reference system of
    /// the source dataset; useful when the source CRS differs from the product CRS.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source / sourceReferenceSystem
    pub struct LiSourceReferenceSystemOptional;
    structural_prop!(
        LiSourceReferenceSystemOptional,
        "LiSourceReferenceSystemOptional"
    );

    /// sourceCitation is optional (0..1); a CI_Citation formally identifying the source
    /// dataset; required when description is absent.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source / sourceCitation
    pub struct LiSourceCitationOptional;
    structural_prop!(LiSourceCitationOptional, "LiSourceCitationOptional");

    /// sourceStep is optional (0..*); process steps that were applied to this source
    /// to produce an intermediate or final output.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source / sourceStep
    pub struct LiSourceStepOptional;
    structural_prop!(LiSourceStepOptional, "LiSourceStepOptional");

    /// source constraint: at least one of description or sourceCitation shall be present;
    /// a completely undescribed source provides no traceability value.
    ///
    /// Source: ISO 19115-1:2014 §6.28 — LI_Source (constraint)
    pub struct LiSourceDescriptionOrCitationRequired;
    structural_prop!(
        LiSourceDescriptionOrCitationRequired,
        "LiSourceDescriptionOrCitationRequired"
    );

    /// scope is mandatory (1); an MD_Scope that identifies the level (dataset, feature,
    /// attribute) and extent to which the quality information applies.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality / scope
    pub struct DqDataQualityScopeMandatory;
    structural_prop!(DqDataQualityScopeMandatory, "DqDataQualityScopeMandatory");

    /// report is optional (0..*); each DQ_Element subclass documents one quality
    /// dimension; multiple reports may be present for different quality aspects.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality / report
    pub struct DqDataQualityReportOptional;
    structural_prop!(DqDataQualityReportOptional, "DqDataQualityReportOptional");

    /// standaloneQuality is optional (0..1); a reference to a separate MD_Metadata
    /// record that describes quality information for this resource in detail.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_DataQuality / standaloneQuality
    pub struct DqDataQualityStandaloneOptional;
    structural_prop!(
        DqDataQualityStandaloneOptional,
        "DqDataQualityStandaloneOptional"
    );

    /// DQ_CompletenessOmission — measures the absence of features, attributes,
    /// or relationships that exist in the real world but are missing from the dataset.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_CompletenessOmission
    pub struct DqCompletenessOmission;
    structural_prop!(DqCompletenessOmission, "DqCompletenessOmission");

    /// DQ_CompletenessCommission — measures the presence of excess features, attributes,
    /// or relationships in the dataset that do not exist in the real world.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_CompletenessCommission
    pub struct DqCompletenessCommission;
    structural_prop!(DqCompletenessCommission, "DqCompletenessCommission");

    /// DQ_ConceptualConsistency — measures adherence of data to rules of the
    /// conceptual schema; conformance to the application schema.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_ConceptualConsistency
    pub struct DqConceptualConsistency;
    structural_prop!(DqConceptualConsistency, "DqConceptualConsistency");

    /// DQ_DomainConsistency — measures adherence of values to the value domains
    /// defined in the schema; values shall be in the defined valid range or set.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_DomainConsistency
    pub struct DqDomainConsistency;
    structural_prop!(DqDomainConsistency, "DqDomainConsistency");

    /// DQ_FormatConsistency — measures the degree to which data is stored in accordance
    /// with the physical structure of the dataset format specification.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_FormatConsistency
    pub struct DqFormatConsistency;
    structural_prop!(DqFormatConsistency, "DqFormatConsistency");

    /// DQ_TopologicalConsistency — measures the correctness of the explicitly encoded
    /// topological characteristics of the dataset (adjacency, connectivity, planarity).
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_TopologicalConsistency
    pub struct DqTopologicalConsistency;
    structural_prop!(DqTopologicalConsistency, "DqTopologicalConsistency");

    /// DQ_AbsoluteExternalPositionalAccuracy — measures the closeness of reported
    /// coordinate values to values accepted as being true (absolute XYZ accuracy).
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_AbsoluteExternalPositionalAccuracy
    pub struct DqAbsoluteExternalPositionalAccuracy;
    structural_prop!(
        DqAbsoluteExternalPositionalAccuracy,
        "DqAbsoluteExternalPositionalAccuracy"
    );

    /// DQ_RelativeInternalPositionalAccuracy — measures the closeness of the relative
    /// positions of features to their respective positions in the real world.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_RelativeInternalPositionalAccuracy
    pub struct DqRelativeInternalPositionalAccuracy;
    structural_prop!(
        DqRelativeInternalPositionalAccuracy,
        "DqRelativeInternalPositionalAccuracy"
    );

    /// DQ_GriddedDataPositionalAccuracy — measures the closeness of gridded data
    /// position values to values accepted as being true (raster cell positional accuracy).
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_GriddedDataPositionalAccuracy
    pub struct DqGriddedDataPositionalAccuracy;
    structural_prop!(
        DqGriddedDataPositionalAccuracy,
        "DqGriddedDataPositionalAccuracy"
    );

    /// DQ_ThematicClassificationCorrectness — measures the accuracy of assigned
    /// thematic categories compared to the true categories in the real world.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_ThematicClassificationCorrectness
    pub struct DqThematicClassificationCorrectness;
    structural_prop!(
        DqThematicClassificationCorrectness,
        "DqThematicClassificationCorrectness"
    );

    /// DQ_NonQuantitativeAttributeCorrectness — measures the correctness of non-numeric
    /// attribute values compared to a reference; applies to categorical or coded attributes.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_NonQuantitativeAttributeCorrectness
    pub struct DqNonQuantitativeAttributeCorrectness;
    structural_prop!(
        DqNonQuantitativeAttributeCorrectness,
        "DqNonQuantitativeAttributeCorrectness"
    );

    /// DQ_QuantitativeAttributeAccuracy — measures the accuracy of numeric attribute
    /// values compared to accepted true values (e.g., elevation, temperature).
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_QuantitativeAttributeAccuracy
    pub struct DqQuantitativeAttributeAccuracy;
    structural_prop!(
        DqQuantitativeAttributeAccuracy,
        "DqQuantitativeAttributeAccuracy"
    );

    /// DQ_TemporalConsistency — measures the correctness of the order of events and
    /// the correctness of temporal references in the dataset.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_TemporalConsistency
    pub struct DqTemporalConsistency;
    structural_prop!(DqTemporalConsistency, "DqTemporalConsistency");

    /// DQ_TemporalValidity — measures whether time values fall within the valid
    /// temporal range or domain defined for the dataset.
    ///
    /// Source: ISO 19115-1:2014 §6.29 — DQ_Element / DQ_TemporalValidity
    pub struct DqTemporalValidity;
    structural_prop!(DqTemporalValidity, "DqTemporalValidity");

    /// topologyLevel is optional (0..1) in MD_VectorSpatialRepresentation; identifies
    /// the degree of complexity of the spatial relationships in the dataset.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_VectorSpatialRepresentation / topologyLevel
    pub struct MdVectorSpatialRepTopologyLevel;
    structural_prop!(
        MdVectorSpatialRepTopologyLevel,
        "MdVectorSpatialRepTopologyLevel"
    );

    /// geometricObjects is optional (0..*) in MD_VectorSpatialRepresentation; each
    /// MD_GeometricObjects entry counts and classifies the geometric primitives present.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_VectorSpatialRepresentation / geometricObjects
    pub struct MdVectorSpatialRepGeometricObjects;
    structural_prop!(
        MdVectorSpatialRepGeometricObjects,
        "MdVectorSpatialRepGeometricObjects"
    );

    /// numberOfDimensions is mandatory (1) in MD_GridSpatialRepresentation; a positive
    /// integer specifying the number of independent spatial or temporal axes.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_GridSpatialRepresentation / numberOfDimensions
    pub struct MdGridSpatialRepNumberOfDimensions;
    structural_prop!(
        MdGridSpatialRepNumberOfDimensions,
        "MdGridSpatialRepNumberOfDimensions"
    );

    /// axisDimensionProperties is optional (0..*) in MD_GridSpatialRepresentation;
    /// each MD_Dimension entry describes one spatial axis (name, size, resolution).
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_GridSpatialRepresentation / axisDimensionProperties
    pub struct MdGridSpatialRepAxisDimensionProperties;
    structural_prop!(
        MdGridSpatialRepAxisDimensionProperties,
        "MdGridSpatialRepAxisDimensionProperties"
    );

    /// cellGeometry is mandatory (1) in MD_GridSpatialRepresentation; MD_CellGeometryCode
    /// specifies whether grid values represent areas (area) or points (point).
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_GridSpatialRepresentation / cellGeometry
    pub struct MdGridSpatialRepCellGeometry;
    structural_prop!(MdGridSpatialRepCellGeometry, "MdGridSpatialRepCellGeometry");

    /// transformationParameterAvailability is mandatory (1) in MD_GridSpatialRepresentation;
    /// Boolean indicating whether affine transformation parameters are provided.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_GridSpatialRepresentation / transformationParameterAvailability
    pub struct MdGridSpatialRepTransformationAvailable;
    structural_prop!(
        MdGridSpatialRepTransformationAvailable,
        "MdGridSpatialRepTransformationAvailable"
    );

    /// referenceSystemIdentifier is optional (0..1); an RS_Identifier (authority + code)
    /// uniquely identifying the CRS, e.g., EPSG:4326 for WGS 84 geographic.
    ///
    /// Source: ISO 19115-1:2014 §6.35 — MD_ReferenceSystem / referenceSystemIdentifier
    pub struct MdReferenceSystemIdentifierOptional;
    structural_prop!(
        MdReferenceSystemIdentifierOptional,
        "MdReferenceSystemIdentifierOptional"
    );

    /// referenceSystemType is optional (0..1); an MD_ReferenceSystemTypeCode classifying
    /// the type of CRS (geographic 2D, projected, vertical, compound, engineering, etc.).
    ///
    /// Source: ISO 19115-1:2014 §6.35 — MD_ReferenceSystem / referenceSystemType
    pub struct MdReferenceSystemTypeOptional;
    structural_prop!(
        MdReferenceSystemTypeOptional,
        "MdReferenceSystemTypeOptional"
    );

    /// language is mandatory (1); the language code for this locale; shall be an ISO
    /// 639-2 three-letter lowercase code (e.g., "eng", "fra", "deu", "zho").
    ///
    /// Source: ISO 19115-1:2014 §6.36 — PT_Locale / language
    pub struct PtLocaleLanguageMandatory;
    structural_prop!(PtLocaleLanguageMandatory, "PtLocaleLanguageMandatory");

    /// country is optional (0..1); narrows the locale to a specific national variant
    /// of the language; shall be an ISO 3166-1 alpha-2 or alpha-3 code.
    ///
    /// Source: ISO 19115-1:2014 §6.36 — PT_Locale / country
    pub struct PtLocaleCountryOptional;
    structural_prop!(PtLocaleCountryOptional, "PtLocaleCountryOptional");

    /// characterEncoding is mandatory (1); the character encoding used for text in this
    /// locale; value from MD_CharacterSetCode (e.g., utf8, latin1, utf16).
    ///
    /// Source: ISO 19115-1:2014 §6.36 — PT_Locale / characterEncoding
    pub struct PtLocaleCharacterEncodingMandatory;
    structural_prop!(
        PtLocaleCharacterEncodingMandatory,
        "PtLocaleCharacterEncodingMandatory"
    );

    /// language code format constraint: the language value shall be exactly three
    /// lowercase ASCII letters conforming to ISO 639-2/T terminological codes.
    ///
    /// Source: ISO 19115-1:2014 §6.36 — PT_Locale / language (constraint)
    pub struct PtLocaleLanguageCodeThreeLetterLowercase;
    structural_prop!(
        PtLocaleLanguageCodeThreeLetterLowercase,
        "PtLocaleLanguageCodeThreeLetterLowercase"
    );

    /// linkage is mandatory (1); a URL providing online access to the resource or
    /// information about the resource; shall be a valid RFC 3986 URL.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / linkage
    pub struct CiOnlineResourceLinkageMandatory;
    structural_prop!(
        CiOnlineResourceLinkageMandatory,
        "CiOnlineResourceLinkageMandatory"
    );

    /// linkage validation constraint: the URL shall be a well-formed RFC 3986 URI;
    /// the scheme (http, https, ftp, etc.) shall be explicitly provided.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / linkage
    pub struct CiOnlineResourceLinkageValidUrl;
    structural_prop!(
        CiOnlineResourceLinkageValidUrl,
        "CiOnlineResourceLinkageValidUrl"
    );

    /// protocol is optional (0..1); the connection protocol used to access the resource
    /// (e.g., "OGC:WMS", "OGC:WFS", "WWW:DOWNLOAD-1.0-http--download").
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / protocol
    pub struct CiOnlineResourceProtocolOptional;
    structural_prop!(
        CiOnlineResourceProtocolOptional,
        "CiOnlineResourceProtocolOptional"
    );

    /// applicationProfile is optional (0..1); the name of an application profile that
    /// can be used with the online resource; relevant for OGC services.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / applicationProfile
    pub struct CiOnlineResourceApplicationProfileOptional;
    structural_prop!(
        CiOnlineResourceApplicationProfileOptional,
        "CiOnlineResourceApplicationProfileOptional"
    );

    /// name is optional (0..1); a human-readable name for the online resource;
    /// used as a label in user interfaces and catalogs.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / name
    pub struct CiOnlineResourceNameOptional;
    structural_prop!(CiOnlineResourceNameOptional, "CiOnlineResourceNameOptional");

    /// description is optional (0..1); a detailed description of what the online
    /// resource provides or how it should be used.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / description
    pub struct CiOnlineResourceDescriptionOptional;
    structural_prop!(
        CiOnlineResourceDescriptionOptional,
        "CiOnlineResourceDescriptionOptional"
    );

    /// function is optional (0..1); classifies the function performed by the online
    /// resource; value from CI_OnLineFunctionCode.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnlineResource / function
    pub struct CiOnlineResourceFunctionOptional;
    structural_prop!(
        CiOnlineResourceFunctionOptional,
        "CiOnlineResourceFunctionOptional"
    );

    /// CI_OnLineFunctionCode: download — online instructions for transferring data
    /// from one storage device or system to another.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / download
    pub struct CiOnlineFunctionDownload;
    structural_prop!(CiOnlineFunctionDownload, "CiOnlineFunctionDownload");

    /// CI_OnLineFunctionCode: information — online information about the resource;
    /// typically documentation, fact sheets, or landing pages.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / information
    pub struct CiOnlineFunctionInformation;
    structural_prop!(CiOnlineFunctionInformation, "CiOnlineFunctionInformation");

    /// CI_OnLineFunctionCode: offlineAccess — online instructions for requesting the
    /// resource from the distributor (the resource itself is not online).
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / offlineAccess
    pub struct CiOnlineFunctionOfflineAccess;
    structural_prop!(
        CiOnlineFunctionOfflineAccess,
        "CiOnlineFunctionOfflineAccess"
    );

    /// CI_OnLineFunctionCode: order — online order form for obtaining the resource;
    /// implies a procurement process rather than direct download.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / order
    pub struct CiOnlineFunctionOrder;
    structural_prop!(CiOnlineFunctionOrder, "CiOnlineFunctionOrder");

    /// CI_OnLineFunctionCode: search — online query interface for searching related
    /// resources in a catalog or discovery service.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / search
    pub struct CiOnlineFunctionSearch;
    structural_prop!(CiOnlineFunctionSearch, "CiOnlineFunctionSearch");

    /// CI_OnLineFunctionCode: completeMetadata — complete metadata for the resource
    /// is available at this URL; points to a full metadata record endpoint.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / completeMetadata
    pub struct CiOnlineFunctionCompleteMetadata;
    structural_prop!(
        CiOnlineFunctionCompleteMetadata,
        "CiOnlineFunctionCompleteMetadata"
    );

    /// CI_OnLineFunctionCode: browseGraphic — a graphic or image illustrating the
    /// content of the resource is available at this URL.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / browseGraphic
    pub struct CiOnlineFunctionBrowseGraphic;
    structural_prop!(
        CiOnlineFunctionBrowseGraphic,
        "CiOnlineFunctionBrowseGraphic"
    );

    /// CI_OnLineFunctionCode: upload — online resource allows uploading data to the
    /// service or provider; implies a data submission endpoint.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / upload
    pub struct CiOnlineFunctionUpload;
    structural_prop!(CiOnlineFunctionUpload, "CiOnlineFunctionUpload");

    /// CI_OnLineFunctionCode: emailService — an email service for obtaining the resource
    /// or further information; the linkage URL is a mailto: URI.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / emailService
    pub struct CiOnlineFunctionEmailService;
    structural_prop!(CiOnlineFunctionEmailService, "CiOnlineFunctionEmailService");

    /// CI_OnLineFunctionCode: browsing — online browsing of the resource data or
    /// content; typically a web map viewer or interactive viewer.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / browsing
    pub struct CiOnlineFunctionBrowsing;
    structural_prop!(CiOnlineFunctionBrowsing, "CiOnlineFunctionBrowsing");

    /// CI_OnLineFunctionCode: fileAccess — online access to a specific file; direct
    /// link to a downloadable file asset, distinct from a download service.
    ///
    /// Source: ISO 19115-1:2014 §6.38 — CI_OnLineFunctionCode / fileAccess
    pub struct CiOnlineFunctionFileAccess;
    structural_prop!(CiOnlineFunctionFileAccess, "CiOnlineFunctionFileAccess");

    /// CI_Party is abstract; only CI_Individual or CI_Organisation are instantiated;
    /// no direct CI_Party instances shall appear in a metadata record.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Party {abstract}
    pub struct CiPartyIsAbstract;
    structural_prop!(CiPartyIsAbstract, "CiPartyIsAbstract");

    /// name is optional (0..1) on CI_Individual; when absent, positionName should
    /// be provided; together they identify the person.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Individual / name
    pub struct CiIndividualNameOptional;
    structural_prop!(CiIndividualNameOptional, "CiIndividualNameOptional");

    /// positionName is optional (0..1); used when the individual name is confidential
    /// or when the role is identified by position rather than person.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Individual / positionName
    pub struct CiIndividualPositionNameOptional;
    structural_prop!(
        CiIndividualPositionNameOptional,
        "CiIndividualPositionNameOptional"
    );

    /// At least one of CI_Individual.name or CI_Individual.positionName should be
    /// non-null so that the individual can be identified or contacted.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Individual / name + positionName
    pub struct CiIndividualNameOrPositionRequired;
    structural_prop!(
        CiIndividualNameOrPositionRequired,
        "CiIndividualNameOrPositionRequired"
    );

    /// contactInfo is optional (0..*) on CI_Individual; zero or more CI_Contact entries
    /// providing phone, address, email, and online resource details.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Individual / contactInfo
    pub struct CiIndividualContactInfoOptional;
    structural_prop!(
        CiIndividualContactInfoOptional,
        "CiIndividualContactInfoOptional"
    );

    /// name is optional (0..1) on CI_Organisation; when provided it shall be non-empty;
    /// identifies the corporate body, agency, or project team.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Organisation / name
    pub struct CiOrganisationNameOptional;
    structural_prop!(CiOrganisationNameOptional, "CiOrganisationNameOptional");

    /// individual is optional (0..*); specific persons within the organisation who are
    /// relevant to the responsibility; may be empty when only the org is identified.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Organisation / individual
    pub struct CiOrganisationIndividualOptional;
    structural_prop!(
        CiOrganisationIndividualOptional,
        "CiOrganisationIndividualOptional"
    );

    /// contactInfo is optional (0..*) on CI_Organisation; zero or more CI_Contact entries
    /// providing the organisation's phone, address, email, and online resources.
    ///
    /// Source: ISO 19115-1:2014 §6.7 — CI_Organisation / contactInfo
    pub struct CiOrganisationContactInfoOptional;
    structural_prop!(
        CiOrganisationContactInfoOptional,
        "CiOrganisationContactInfoOptional"
    );

    /// phone is optional (0..*) on CI_Contact; zero or more CI_Telephone entries;
    /// each entry covers one phone number and its type.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Contact / phone
    pub struct CiContactPhoneOptional;
    structural_prop!(CiContactPhoneOptional, "CiContactPhoneOptional");

    /// address is optional (0..*) on CI_Contact; zero or more CI_Address entries
    /// providing the postal address of the party.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Contact / address
    pub struct CiContactAddressOptional;
    structural_prop!(CiContactAddressOptional, "CiContactAddressOptional");

    /// onlineResource is optional (0..*) on CI_Contact; links to the party's
    /// website, data portal, or other online presence.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Contact / onlineResource
    pub struct CiContactOnlineResourceOptional;
    structural_prop!(
        CiContactOnlineResourceOptional,
        "CiContactOnlineResourceOptional"
    );

    /// hoursOfService is optional (0..1); free-text description of periods when
    /// the party can be contacted (e.g., "Mon-Fri 09:00-17:00 UTC").
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Contact / hoursOfService
    pub struct CiContactHoursOfServiceOptional;
    structural_prop!(
        CiContactHoursOfServiceOptional,
        "CiContactHoursOfServiceOptional"
    );

    /// contactInstructions is optional (0..1); supplementary instructions on how
    /// to reach the party (e.g., preferred channel, escalation procedure).
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Contact / contactInstructions
    pub struct CiContactInstructionsOptional;
    structural_prop!(
        CiContactInstructionsOptional,
        "CiContactInstructionsOptional"
    );

    /// number is mandatory (1) on CI_Telephone; the telephone number string shall
    /// be non-empty; E.164 format is recommended.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Telephone / number
    pub struct CiTelephoneNumberMandatory;
    structural_prop!(CiTelephoneNumberMandatory, "CiTelephoneNumberMandatory");

    /// A CI_Telephone.number value shall not be the empty string; a telephone
    /// record with an empty number conveys no contact information.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Telephone / number non-empty
    pub struct CiTelephoneNumberNonEmpty;
    structural_prop!(CiTelephoneNumberNonEmpty, "CiTelephoneNumberNonEmpty");

    /// CI_TelephoneTypeCode: voice — a voice telephone number for direct speech.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_TelephoneTypeCode / voice
    pub struct CiTelephoneTypeVoice;
    structural_prop!(CiTelephoneTypeVoice, "CiTelephoneTypeVoice");

    /// CI_TelephoneTypeCode: facsimile — a facsimile (fax) telephone number.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_TelephoneTypeCode / facsimile
    pub struct CiTelephoneFacsimile;
    structural_prop!(CiTelephoneFacsimile, "CiTelephoneFacsimile");

    /// CI_TelephoneTypeCode: sms — a short-message-service (SMS/text) number.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_TelephoneTypeCode / sms
    pub struct CiTelephoneSms;
    structural_prop!(CiTelephoneSms, "CiTelephoneSms");

    /// deliveryPoint is optional (0..*) on CI_Address; street address lines such
    /// as building name, street number, and street name.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / deliveryPoint
    pub struct CiAddressDeliveryPointOptional;
    structural_prop!(
        CiAddressDeliveryPointOptional,
        "CiAddressDeliveryPointOptional"
    );

    /// city is optional (0..1) on CI_Address; the name of the city or locality
    /// as it would appear on a postal envelope.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / city
    pub struct CiAddressCityOptional;
    structural_prop!(CiAddressCityOptional, "CiAddressCityOptional");

    /// administrativeArea is optional (0..1); state, province, county, or
    /// equivalent administrative subdivision of the country.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / administrativeArea
    pub struct CiAddressAdministrativeAreaOptional;
    structural_prop!(
        CiAddressAdministrativeAreaOptional,
        "CiAddressAdministrativeAreaOptional"
    );

    /// postalCode is optional (0..1); the postal/ZIP code for the address;
    /// format varies by country.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / postalCode
    pub struct CiAddressPostalCodeOptional;
    structural_prop!(CiAddressPostalCodeOptional, "CiAddressPostalCodeOptional");

    /// country is optional (0..1) on CI_Address; when provided shall be an
    /// ISO 3166-1 alpha-2 or alpha-3 country code.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / country
    pub struct CiAddressCountryOptional;
    structural_prop!(CiAddressCountryOptional, "CiAddressCountryOptional");

    /// electronicMailAddress is optional (0..*); one or more email addresses for
    /// the party; each entry shall follow RFC 5321 syntax.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / electronicMailAddress
    pub struct CiAddressEmailOptional;
    structural_prop!(CiAddressEmailOptional, "CiAddressEmailOptional");

    /// When CI_Address.country is provided its value shall be an ISO 3166-1
    /// alpha-2 (two uppercase letters) or alpha-3 (three uppercase letters) code.
    ///
    /// Source: ISO 19115-1:2014 §6.8 — CI_Address / country ISO 3166 constraint
    pub struct CiAddressCountryIsIso3166;
    structural_prop!(CiAddressCountryIsIso3166, "CiAddressCountryIsIso3166");

    /// fileName is mandatory (1) on MD_BrowseGraphic; the URI or file-system path
    /// of the graphic file shall be provided.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — MD_BrowseGraphic / fileName
    pub struct MdBrowseGraphicFileNameMandatory;
    structural_prop!(
        MdBrowseGraphicFileNameMandatory,
        "MdBrowseGraphicFileNameMandatory"
    );

    /// MD_BrowseGraphic.fileName shall not be an empty string; an empty path
    /// cannot be resolved to a graphic resource.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — MD_BrowseGraphic / fileName non-empty
    pub struct MdBrowseGraphicFileNameNonEmpty;
    structural_prop!(
        MdBrowseGraphicFileNameNonEmpty,
        "MdBrowseGraphicFileNameNonEmpty"
    );

    /// fileDescription is optional (0..1); a plain-language caption or description
    /// of what the graphic depicts.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — MD_BrowseGraphic / fileDescription
    pub struct MdBrowseGraphicFileDescriptionOptional;
    structural_prop!(
        MdBrowseGraphicFileDescriptionOptional,
        "MdBrowseGraphicFileDescriptionOptional"
    );

    /// fileType is optional (0..1); when provided, should be a MIME type string
    /// (e.g., "image/png", "image/jpeg", "image/svg+xml").
    ///
    /// Source: ISO 19115-1:2014 §6.11 — MD_BrowseGraphic / fileType
    pub struct MdBrowseGraphicFileTypeOptional;
    structural_prop!(
        MdBrowseGraphicFileTypeOptional,
        "MdBrowseGraphicFileTypeOptional"
    );

    /// linkage is optional (0..*); CI_OnlineResource entries giving alternative
    /// online access paths to the browse graphic.
    ///
    /// Source: ISO 19115-1:2014 §6.11 — MD_BrowseGraphic / linkage
    pub struct MdBrowseGraphicLinkageOptional;
    structural_prop!(
        MdBrowseGraphicLinkageOptional,
        "MdBrowseGraphicLinkageOptional"
    );

    /// name is conditional (0..1) on MD_AssociatedResource; required when
    /// metadataReference is absent; identifies the associated resource by citation.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — MD_AssociatedResource / name
    pub struct MdAssociatedResourceNameConditional;
    structural_prop!(
        MdAssociatedResourceNameConditional,
        "MdAssociatedResourceNameConditional"
    );

    /// metadataReference is conditional (0..1); required when name is absent;
    /// points to the metadata record of the associated resource.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — MD_AssociatedResource / metadataReference
    pub struct MdAssociatedResourceMetadataRefConditional;
    structural_prop!(
        MdAssociatedResourceMetadataRefConditional,
        "MdAssociatedResourceMetadataRefConditional"
    );

    /// At least one of name or metadataReference shall be present in every
    /// MD_AssociatedResource instance.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — MD_AssociatedResource / name+metadataReference
    pub struct MdAssociatedResourceNameOrMetaRefRequired;
    structural_prop!(
        MdAssociatedResourceNameOrMetaRefRequired,
        "MdAssociatedResourceNameOrMetaRefRequired"
    );

    /// associationType is mandatory (1); the DS_AssociationTypeCode value shall
    /// be drawn from the defined enumeration.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — MD_AssociatedResource / associationType
    pub struct MdAssociatedResourceAssociationTypeMandatory;
    structural_prop!(
        MdAssociatedResourceAssociationTypeMandatory,
        "MdAssociatedResourceAssociationTypeMandatory"
    );

    /// initiativeType is optional (0..1); when provided, a DS_InitiativeTypeCode
    /// value classifying the type of scientific initiative.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — MD_AssociatedResource / initiativeType
    pub struct MdAssociatedResourceInitiativeTypeOptional;
    structural_prop!(
        MdAssociatedResourceInitiativeTypeOptional,
        "MdAssociatedResourceInitiativeTypeOptional"
    );

    /// DS_AssociationTypeCode: crossReference — reference from one dataset to
    /// another that is not a hierarchical relationship.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_AssociationTypeCode / crossReference
    pub struct DsAssociationTypeCrossReference;
    structural_prop!(
        DsAssociationTypeCrossReference,
        "DsAssociationTypeCrossReference"
    );

    /// DS_AssociationTypeCode: largerWorkCitation — the described resource is
    /// a component of the cited larger work.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_AssociationTypeCode / largerWorkCitation
    pub struct DsAssociationTypeLargerWorkCitation;
    structural_prop!(
        DsAssociationTypeLargerWorkCitation,
        "DsAssociationTypeLargerWorkCitation"
    );

    /// DS_AssociationTypeCode: partOfSeamlessDatabase — the resource is a tile
    /// or partition of a seamless multi-tile database.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_AssociationTypeCode / partOfSeamlessDatabase
    pub struct DsAssociationTypePartOfSeamlessDatabase;
    structural_prop!(
        DsAssociationTypePartOfSeamlessDatabase,
        "DsAssociationTypePartOfSeamlessDatabase"
    );

    /// DS_AssociationTypeCode: isComposedOf — the described resource is composed
    /// of the cited component resources.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_AssociationTypeCode / isComposedOf
    pub struct DsAssociationTypeIsComposedOf;
    structural_prop!(
        DsAssociationTypeIsComposedOf,
        "DsAssociationTypeIsComposedOf"
    );

    /// DS_AssociationTypeCode: revisionOf — the described resource is a revision
    /// or update of the cited earlier resource.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_AssociationTypeCode / revisionOf
    pub struct DsAssociationTypeRevisionOf;
    structural_prop!(DsAssociationTypeRevisionOf, "DsAssociationTypeRevisionOf");

    /// DS_InitiativeTypeCode: project — a project with defined scope, budget,
    /// and timeline.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_InitiativeTypeCode / project
    pub struct DsInitiativeTypeProject;
    structural_prop!(DsInitiativeTypeProject, "DsInitiativeTypeProject");

    /// DS_InitiativeTypeCode: mission — a scientific or operational mission,
    /// typically a satellite or field campaign.
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_InitiativeTypeCode / mission
    pub struct DsInitiativeTypeMission;
    structural_prop!(DsInitiativeTypeMission, "DsInitiativeTypeMission");

    /// DS_InitiativeTypeCode: platform — a specific sensor platform (satellite,
    /// aircraft, vessel).
    ///
    /// Source: ISO 19115-1:2014 §6.15 — DS_InitiativeTypeCode / platform
    pub struct DsInitiativeTypePlatform;
    structural_prop!(DsInitiativeTypePlatform, "DsInitiativeTypePlatform");

    /// polygon is mandatory (1..*) on EX_BoundingPolygon; at least one geometry
    /// object shall be provided; the polygon array shall be non-empty.
    ///
    /// Source: ISO 19115-1:2014 §6.20 — EX_BoundingPolygon / polygon
    pub struct ExBoundingPolygonPolygonMandatory;
    structural_prop!(
        ExBoundingPolygonPolygonMandatory,
        "ExBoundingPolygonPolygonMandatory"
    );

    /// The polygon array of EX_BoundingPolygon shall contain at least one non-null
    /// geometry; an empty array cannot define a geographic extent.
    ///
    /// Source: ISO 19115-1:2014 §6.20 — EX_BoundingPolygon / polygon multiplicity
    pub struct ExBoundingPolygonAtLeastOneGeometry;
    structural_prop!(
        ExBoundingPolygonAtLeastOneGeometry,
        "ExBoundingPolygonAtLeastOneGeometry"
    );

    /// extentTypeCode is optional (0..1); when true the polygon describes an inclusion
    /// area; when false it describes an exclusion zone within a larger extent.
    ///
    /// Source: ISO 19115-1:2014 §6.20 — EX_BoundingPolygon / extentTypeCode
    pub struct ExBoundingPolygonExtentTypeCodeOptional;
    structural_prop!(
        ExBoundingPolygonExtentTypeCodeOptional,
        "ExBoundingPolygonExtentTypeCodeOptional"
    );

    /// Each element of the polygon array shall be a valid GM_Object geometry;
    /// invalid or degenerate geometries shall not be recorded as bounding polygons.
    ///
    /// Source: ISO 19115-1:2014 §6.20 — EX_BoundingPolygon / polygon validity
    pub struct ExBoundingPolygonGeometryIsValid;
    structural_prop!(
        ExBoundingPolygonGeometryIsValid,
        "ExBoundingPolygonGeometryIsValid"
    );

    /// classification is mandatory (1) on MD_SecurityConstraints; the security
    /// classification level shall be drawn from MD_ClassificationCode.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_SecurityConstraints / classification
    pub struct MdSecurityConstraintsClassificationMandatory;
    structural_prop!(
        MdSecurityConstraintsClassificationMandatory,
        "MdSecurityConstraintsClassificationMandatory"
    );

    /// userNote is optional (0..1); a plain-text caveat or declassification
    /// instruction associated with the security classification.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_SecurityConstraints / userNote
    pub struct MdSecurityConstraintsUserNoteOptional;
    structural_prop!(
        MdSecurityConstraintsUserNoteOptional,
        "MdSecurityConstraintsUserNoteOptional"
    );

    /// classificationSystem is optional (0..1); the name of the classification
    /// system under which the code was assigned.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_SecurityConstraints / classificationSystem
    pub struct MdSecurityConstraintsClassSystemOptional;
    structural_prop!(
        MdSecurityConstraintsClassSystemOptional,
        "MdSecurityConstraintsClassSystemOptional"
    );

    /// handlingDescription is optional (0..1); additional instructions for handling,
    /// dissemination, or storage of the classified resource.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_SecurityConstraints / handlingDescription
    pub struct MdSecurityConstraintsHandlingDescOptional;
    structural_prop!(
        MdSecurityConstraintsHandlingDescOptional,
        "MdSecurityConstraintsHandlingDescOptional"
    );

    /// MD_ClassificationCode: unclassified — no restrictions on access or use;
    /// the resource may be freely distributed.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / unclassified
    pub struct MdClassificationUnclassified;
    structural_prop!(MdClassificationUnclassified, "MdClassificationUnclassified");

    /// MD_ClassificationCode: restricted — distribution limited to specific parties
    /// or purposes; not for general public release.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / restricted
    pub struct MdClassificationRestricted;
    structural_prop!(MdClassificationRestricted, "MdClassificationRestricted");

    /// MD_ClassificationCode: confidential — sensitive information whose disclosure
    /// could damage national interests or personal privacy.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / confidential
    pub struct MdClassificationConfidential;
    structural_prop!(MdClassificationConfidential, "MdClassificationConfidential");

    /// MD_ClassificationCode: secret — highly sensitive; unauthorised disclosure
    /// could cause serious damage to national security.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / secret
    pub struct MdClassificationSecret;
    structural_prop!(MdClassificationSecret, "MdClassificationSecret");

    /// MD_ClassificationCode: topSecret — the highest civilian classification level;
    /// unauthorised disclosure could cause exceptionally grave damage.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / topSecret
    pub struct MdClassificationTopSecret;
    structural_prop!(MdClassificationTopSecret, "MdClassificationTopSecret");

    /// MD_ClassificationCode: sensitiveButUnclassified — not formally classified
    /// but requires controlled handling to protect sensitive content.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / sensitiveButUnclassified
    pub struct MdClassificationSensitiveButUnclassified;
    structural_prop!(
        MdClassificationSensitiveButUnclassified,
        "MdClassificationSensitiveButUnclassified"
    );

    /// MD_ClassificationCode: forOfficialUseOnly — for internal government use;
    /// not to be released outside the originating department without authorisation.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / forOfficialUseOnly
    pub struct MdClassificationForOfficialUseOnly;
    structural_prop!(
        MdClassificationForOfficialUseOnly,
        "MdClassificationForOfficialUseOnly"
    );

    /// MD_ClassificationCode: protected — resource is protected under specific
    /// legislation or regulation restricting distribution.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / protected
    pub struct MdClassificationProtected;
    structural_prop!(MdClassificationProtected, "MdClassificationProtected");

    /// MD_ClassificationCode: limitedDistribution — distribution is limited to
    /// a named set of authorised recipients or organisations.
    ///
    /// Source: ISO 19115-1:2014 §6.24 — MD_ClassificationCode / limitedDistribution
    pub struct MdClassificationLimitedDistribution;
    structural_prop!(
        MdClassificationLimitedDistribution,
        "MdClassificationLimitedDistribution"
    );

    /// MD_Resolution is a union type; exactly one resolution form shall be
    /// present — equivalentScale, distance, vertical, angularDistance, or levelOfDetail.
    ///
    /// Source: ISO 19115-1:2014 §6.25 — MD_Resolution / union constraint
    pub struct MdResolutionEquivalentScaleOrDistance;
    structural_prop!(
        MdResolutionEquivalentScaleOrDistance,
        "MdResolutionEquivalentScaleOrDistance"
    );

    /// denominator is mandatory (1) on MD_RepresentativeFraction; the integer
    /// scale denominator shall be provided (e.g., 50000 for 1:50 000).
    ///
    /// Source: ISO 19115-1:2014 §6.25 — MD_RepresentativeFraction / denominator
    pub struct MdRepresentativeFractionDenominatorMandatory;
    structural_prop!(
        MdRepresentativeFractionDenominatorMandatory,
        "MdRepresentativeFractionDenominatorMandatory"
    );

    /// MD_RepresentativeFraction.denominator shall be a positive integer (> 0);
    /// a denominator of zero or less is not a valid scale.
    ///
    /// Source: ISO 19115-1:2014 §6.25 — MD_RepresentativeFraction / denominator positive
    pub struct MdRepresentativeFractionDenominatorPositive;
    structural_prop!(
        MdRepresentativeFractionDenominatorPositive,
        "MdRepresentativeFractionDenominatorPositive"
    );

    /// When the distance form of MD_Resolution is used, the distance value shall
    /// be a positive real number with an associated unit of measure.
    ///
    /// Source: ISO 19115-1:2014 §6.25 — MD_Resolution / distance positive
    pub struct MdResolutionDistanceIsPositive;
    structural_prop!(
        MdResolutionDistanceIsPositive,
        "MdResolutionDistanceIsPositive"
    );

    /// A larger equivalentScale denominator indicates coarser resolution:
    /// 1:1 000 000 is less detailed than 1:25 000 (denominator 25000 < 1000000).
    ///
    /// Source: ISO 19115-1:2014 §6.25 — MD_Resolution / scale semantics
    pub struct MdResolutionScaleImpliesSmallIsCoarse;
    structural_prop!(
        MdResolutionScaleImpliesSmallIsCoarse,
        "MdResolutionScaleImpliesSmallIsCoarse"
    );

    /// dimensionName is mandatory (1) on MD_Dimension; value shall be drawn from
    /// MD_DimensionNameTypeCode (row, column, vertical, track, crossTrack, line,
    /// sample, time).
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_Dimension / dimensionName
    pub struct MdDimensionNameMandatory;
    structural_prop!(MdDimensionNameMandatory, "MdDimensionNameMandatory");

    /// dimensionSize is mandatory (1) on MD_Dimension; the number of elements along
    /// this axis shall be provided as a positive integer.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_Dimension / dimensionSize
    pub struct MdDimensionSizeMandatory;
    structural_prop!(MdDimensionSizeMandatory, "MdDimensionSizeMandatory");

    /// MD_Dimension.dimensionSize shall be a positive integer (> 0); a grid axis
    /// with zero or fewer cells is not physically meaningful.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_Dimension / dimensionSize positive
    pub struct MdDimensionSizePositive;
    structural_prop!(MdDimensionSizePositive, "MdDimensionSizePositive");

    /// resolution is optional (0..1) on MD_Dimension; when provided, gives the
    /// ground sample distance or time step for the axis (with unit of measure).
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_Dimension / resolution
    pub struct MdDimensionResolutionOptional;
    structural_prop!(
        MdDimensionResolutionOptional,
        "MdDimensionResolutionOptional"
    );

    /// MD_DimensionNameTypeCode: row — y-direction axis in image/grid coordinates.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_DimensionNameTypeCode / row
    pub struct MdDimensionNameRow;
    structural_prop!(MdDimensionNameRow, "MdDimensionNameRow");

    /// MD_DimensionNameTypeCode: column — x-direction axis in image/grid coordinates.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_DimensionNameTypeCode / column
    pub struct MdDimensionNameColumn;
    structural_prop!(MdDimensionNameColumn, "MdDimensionNameColumn");

    /// MD_DimensionNameTypeCode: vertical — altitude or depth axis (z-direction).
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_DimensionNameTypeCode / vertical
    pub struct MdDimensionNameVertical;
    structural_prop!(MdDimensionNameVertical, "MdDimensionNameVertical");

    /// MD_DimensionNameTypeCode: time — temporal axis; used in time-series grids.
    ///
    /// Source: ISO 19115-1:2014 §6.30 — MD_DimensionNameTypeCode / time
    pub struct MdDimensionNameTime;
    structural_prop!(MdDimensionNameTime, "MdDimensionNameTime");

    /// maintenanceAndUpdateFrequency is mandatory (1); the frequency code governs
    /// expected update cadence; shall be drawn from MD_MaintenanceFrequencyCode.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / maintenanceAndUpdateFrequency
    pub struct MdMaintenanceFrequencyMandatory;
    structural_prop!(
        MdMaintenanceFrequencyMandatory,
        "MdMaintenanceFrequencyMandatory"
    );

    /// maintenanceDate is optional (0..*); CI_Date entries documenting when
    /// past or planned updates occurred or are scheduled.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / maintenanceDate
    pub struct MdMaintenanceDateOptional;
    structural_prop!(MdMaintenanceDateOptional, "MdMaintenanceDateOptional");

    /// userDefinedMaintenanceFrequency is conditional (0..1); required when
    /// maintenanceAndUpdateFrequency = userDefined; provides a TM_PeriodDuration.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / userDefinedMaintenanceFrequency
    pub struct MdMaintenanceUserDefinedFreqConditional;
    structural_prop!(
        MdMaintenanceUserDefinedFreqConditional,
        "MdMaintenanceUserDefinedFreqConditional"
    );

    /// maintenanceScope is optional (0..*); MD_Scope entries restricting which
    /// part of the resource the maintenance schedule applies to.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / maintenanceScope
    pub struct MdMaintenanceScopeOptional;
    structural_prop!(MdMaintenanceScopeOptional, "MdMaintenanceScopeOptional");

    /// maintenanceNote is optional (0..*); free-text descriptions of maintenance
    /// activities performed or planned for the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / maintenanceNote
    pub struct MdMaintenanceNoteOptional;
    structural_prop!(MdMaintenanceNoteOptional, "MdMaintenanceNoteOptional");

    /// contact is optional (0..*); CI_Responsibility entries for parties responsible
    /// for maintaining the resource on the stated schedule.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceInformation / contact
    pub struct MdMaintenanceContactOptional;
    structural_prop!(MdMaintenanceContactOptional, "MdMaintenanceContactOptional");

    /// MD_MaintenanceFrequencyCode: continual — data is repeatedly and frequently
    /// updated; changes are made as soon as new information is available.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / continual
    pub struct MdMaintenanceFrequencyContinual;
    structural_prop!(
        MdMaintenanceFrequencyContinual,
        "MdMaintenanceFrequencyContinual"
    );

    /// MD_MaintenanceFrequencyCode: daily — data is updated each day.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / daily
    pub struct MdMaintenanceFrequencyDaily;
    structural_prop!(MdMaintenanceFrequencyDaily, "MdMaintenanceFrequencyDaily");

    /// MD_MaintenanceFrequencyCode: weekly — data is updated on a weekly basis.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / weekly
    pub struct MdMaintenanceFrequencyWeekly;
    structural_prop!(MdMaintenanceFrequencyWeekly, "MdMaintenanceFrequencyWeekly");

    /// MD_MaintenanceFrequencyCode: monthly — data is updated each month.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / monthly
    pub struct MdMaintenanceFrequencyMonthly;
    structural_prop!(
        MdMaintenanceFrequencyMonthly,
        "MdMaintenanceFrequencyMonthly"
    );

    /// MD_MaintenanceFrequencyCode: quarterly — data is updated every three months.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / quarterly
    pub struct MdMaintenanceFrequencyQuarterly;
    structural_prop!(
        MdMaintenanceFrequencyQuarterly,
        "MdMaintenanceFrequencyQuarterly"
    );

    /// MD_MaintenanceFrequencyCode: annually — data is updated once a year.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / annually
    pub struct MdMaintenanceFrequencyAnnually;
    structural_prop!(
        MdMaintenanceFrequencyAnnually,
        "MdMaintenanceFrequencyAnnually"
    );

    /// MD_MaintenanceFrequencyCode: asNeeded — data is updated when deemed necessary
    /// by the data custodian; no fixed schedule.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / asNeeded
    pub struct MdMaintenanceFrequencyAsNeeded;
    structural_prop!(
        MdMaintenanceFrequencyAsNeeded,
        "MdMaintenanceFrequencyAsNeeded"
    );

    /// MD_MaintenanceFrequencyCode: irregular — data is updated at irregular
    /// intervals; the intervals are not predictable.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / irregular
    pub struct MdMaintenanceFrequencyIrregular;
    structural_prop!(
        MdMaintenanceFrequencyIrregular,
        "MdMaintenanceFrequencyIrregular"
    );

    /// MD_MaintenanceFrequencyCode: notPlanned — no further updates are planned;
    /// the dataset is considered complete and closed.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / notPlanned
    pub struct MdMaintenanceFrequencyNotPlanned;
    structural_prop!(
        MdMaintenanceFrequencyNotPlanned,
        "MdMaintenanceFrequencyNotPlanned"
    );

    /// MD_MaintenanceFrequencyCode: unknown — the update frequency is not known
    /// to the metadata author.
    ///
    /// Source: ISO 19115-1:2014 §6.37 — MD_MaintenanceFrequencyCode / unknown
    pub struct MdMaintenanceFrequencyUnknown;
    structural_prop!(
        MdMaintenanceFrequencyUnknown,
        "MdMaintenanceFrequencyUnknown"
    );

    /// distributionFormat is optional (0..*) on MD_Distribution; zero or more
    /// MD_Format entries describing the available distribution formats.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distribution / distributionFormat
    pub struct MdDistributionFormatOptional;
    structural_prop!(MdDistributionFormatOptional, "MdDistributionFormatOptional");

    /// distributor is optional (0..*) on MD_Distribution; zero or more
    /// MD_Distributor entries documenting who distributes the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distribution / distributor
    pub struct MdDistributionDistributorOptional;
    structural_prop!(
        MdDistributionDistributorOptional,
        "MdDistributionDistributorOptional"
    );

    /// transferOptions is optional (0..*) on MD_Distribution; zero or more
    /// MD_DigitalTransferOptions entries describing available download methods.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distribution / transferOptions
    pub struct MdDistributionTransferOptionsOptional;
    structural_prop!(
        MdDistributionTransferOptionsOptional,
        "MdDistributionTransferOptionsOptional"
    );

    /// An MD_Distribution with no format, distributor, or transferOptions provides
    /// no actionable access information; at least one element should be populated.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distribution / non-empty constraint
    pub struct MdDistributionAtLeastOneElement;
    structural_prop!(
        MdDistributionAtLeastOneElement,
        "MdDistributionAtLeastOneElement"
    );

    /// distributorContact is mandatory (1) on MD_Distributor; a CI_Responsibility
    /// identifying the distributing party shall be provided.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distributor / distributorContact
    pub struct MdDistributorContactMandatory;
    structural_prop!(
        MdDistributorContactMandatory,
        "MdDistributorContactMandatory"
    );

    /// distributionOrderProcess is optional (0..*) on MD_Distributor; information
    /// about the process for ordering the resource from this distributor.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distributor / distributionOrderProcess
    pub struct MdDistributorOrderProcessOptional;
    structural_prop!(
        MdDistributorOrderProcessOptional,
        "MdDistributorOrderProcessOptional"
    );

    /// distributorFormat is optional (0..*) on MD_Distributor; MD_Format entries
    /// specific to what this distributor provides.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distributor / distributorFormat
    pub struct MdDistributorFormatOptional;
    structural_prop!(MdDistributorFormatOptional, "MdDistributorFormatOptional");

    /// distributorTransferOptions is optional (0..*) on MD_Distributor; transfer
    /// options specific to what this distributor can provide.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Distributor / distributorTransferOptions
    pub struct MdDistributorTransferOptionsOptional;
    structural_prop!(
        MdDistributorTransferOptionsOptional,
        "MdDistributorTransferOptionsOptional"
    );

    /// When transferSize is provided on MD_DigitalTransferOptions, its value shall
    /// be a positive real number expressed in megabytes (MB).
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_DigitalTransferOptions / transferSize positive
    pub struct MdTransferOptionsSizePositive;
    structural_prop!(
        MdTransferOptionsSizePositive,
        "MdTransferOptionsSizePositive"
    );

    /// onLine is optional (0..*) on MD_DigitalTransferOptions; CI_OnlineResource
    /// entries giving URLs or endpoints for downloading the resource.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_DigitalTransferOptions / onLine
    pub struct MdTransferOptionsOnlineOptional;
    structural_prop!(
        MdTransferOptionsOnlineOptional,
        "MdTransferOptionsOnlineOptional"
    );

    /// offLine is optional (0..*) on MD_DigitalTransferOptions; MD_Medium entries
    /// documenting physical media on which the resource may be distributed.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_DigitalTransferOptions / offLine
    pub struct MdTransferOptionsOfflineOptional;
    structural_prop!(
        MdTransferOptionsOfflineOptional,
        "MdTransferOptionsOfflineOptional"
    );

    /// densityUnits on MD_Medium is conditional (0..1); required when density is
    /// provided; specifies the unit of the density measurement.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Medium / densityUnits conditional
    pub struct MdMediumDensityUnitsConditional;
    structural_prop!(
        MdMediumDensityUnitsConditional,
        "MdMediumDensityUnitsConditional"
    );

    /// volumes is optional (0..1) on MD_Medium; when provided, the number of items
    /// in the media collection shall be a non-negative integer.
    ///
    /// Source: ISO 19115-1:2014 §6.39 — MD_Medium / volumes
    pub struct MdMediumVolumesOptional;
    structural_prop!(MdMediumVolumesOptional, "MdMediumVolumesOptional");

    /// MD_ScopeCode: dataset — the scope is a single geographic dataset;
    /// the most common value for standard GIS data files.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / dataset
    pub struct MdScopeDataset;
    structural_prop!(MdScopeDataset, "MdScopeDataset");

    /// MD_ScopeCode: series — the scope is an aggregate series of related datasets
    /// (e.g., a national topographic series).
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / series
    pub struct MdScopeSeries;
    structural_prop!(MdScopeSeries, "MdScopeSeries");

    /// MD_ScopeCode: service — the scope is a service interface (OWS, WMS, WFS,
    /// etc.) rather than a data file.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / service
    pub struct MdScopeService;
    structural_prop!(MdScopeService, "MdScopeService");

    /// MD_ScopeCode: software — the scope is a computer program or application
    /// that processes or generates geographic data.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / software
    pub struct MdScopeSoftware;
    structural_prop!(MdScopeSoftware, "MdScopeSoftware");

    /// MD_ScopeCode: model — the scope is a copy of data with altered structure
    /// or content, typically a processed derivative product.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / model
    pub struct MdScopeModel;
    structural_prop!(MdScopeModel, "MdScopeModel");

    /// MD_ScopeCode: initiative — the scope is a broad-scale scientific or
    /// operational data collection initiative.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / initiative
    pub struct MdScopeInitiative;
    structural_prop!(MdScopeInitiative, "MdScopeInitiative");

    /// MD_ScopeCode: featureType — the scope is a geographic feature type
    /// definition (schema / class level).
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / featureType
    pub struct MdScopeFeatureType;
    structural_prop!(MdScopeFeatureType, "MdScopeFeatureType");

    /// MD_ScopeCode: feature — the scope is an individual geographic feature
    /// instance.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / feature
    pub struct MdScopeFeature;
    structural_prop!(MdScopeFeature, "MdScopeFeature");

    /// MD_ScopeCode: attributeType — the scope is a feature attribute type
    /// definition.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / attributeType
    pub struct MdScopeAttributeType;
    structural_prop!(MdScopeAttributeType, "MdScopeAttributeType");

    /// MD_ScopeCode: attribute — the scope is an individual feature attribute
    /// value.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / attribute
    pub struct MdScopeAttribute;
    structural_prop!(MdScopeAttribute, "MdScopeAttribute");

    /// MD_ScopeCode: tile — the scope is a tile or sheet of a larger tiled
    /// dataset.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / tile
    pub struct MdScopeTile;
    structural_prop!(MdScopeTile, "MdScopeTile");

    /// MD_ScopeCode: fieldSession — the scope is a single field data collection
    /// session or survey event.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / fieldSession
    pub struct MdScopeFieldSession;
    structural_prop!(MdScopeFieldSession, "MdScopeFieldSession");

    /// MD_ScopeCode: collectionSession — the scope is a collection session
    /// encompassing multiple field events.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / collectionSession
    pub struct MdScopeCollectionSession;
    structural_prop!(MdScopeCollectionSession, "MdScopeCollectionSession");

    /// MD_ScopeCode: nonGeographicDataset — the scope is a dataset that has no
    /// geographic extent (tabular, statistical, or thematic non-spatial data).
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / nonGeographicDataset
    pub struct MdScopeNonGeographicDataset;
    structural_prop!(MdScopeNonGeographicDataset, "MdScopeNonGeographicDataset");

    /// MD_ScopeCode: dimensionGroup — the scope is a dimension group within a
    /// multidimensional grid dataset.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_ScopeCode / dimensionGroup
    pub struct MdScopeDimensionGroup;
    structural_prop!(MdScopeDimensionGroup, "MdScopeDimensionGroup");

    /// MD_CharacterSetCode: utf8 — UTF-8 variable-width Unicode encoding;
    /// the default encoding assumed when characterSet is absent.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / utf8
    pub struct MdCharsetUtf8;
    structural_prop!(MdCharsetUtf8, "MdCharsetUtf8");

    /// MD_CharacterSetCode: utf16 — UTF-16 wide-character Unicode encoding;
    /// used when BOM-marked UTF-16 files are distributed.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / utf16
    pub struct MdCharsetUtf16;
    structural_prop!(MdCharsetUtf16, "MdCharsetUtf16");

    /// MD_CharacterSetCode: utf32 — UTF-32 fixed-width Unicode encoding.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / utf32
    pub struct MdCharsetUtf32;
    structural_prop!(MdCharsetUtf32, "MdCharsetUtf32");

    /// MD_CharacterSetCode: 8859part1 — ISO-8859-1 Latin-1, Western European;
    /// covers English, French, German, Spanish, Portuguese, and others.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part1
    pub struct MdCharsetLatin1;
    structural_prop!(MdCharsetLatin1, "MdCharsetLatin1");

    /// MD_CharacterSetCode: 8859part2 — ISO-8859-2 Latin-2, Central European;
    /// covers Czech, Polish, Slovak, Hungarian, Romanian.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part2
    pub struct MdCharsetLatin2;
    structural_prop!(MdCharsetLatin2, "MdCharsetLatin2");

    /// MD_CharacterSetCode: 8859part5 — ISO-8859-5, Cyrillic script; covers
    /// Russian, Bulgarian, Serbian, and other Cyrillic-script languages.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part5
    pub struct MdCharsetCyrillic;
    structural_prop!(MdCharsetCyrillic, "MdCharsetCyrillic");

    /// MD_CharacterSetCode: 8859part6 — ISO-8859-6, Arabic script.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part6
    pub struct MdCharsetArabic;
    structural_prop!(MdCharsetArabic, "MdCharsetArabic");

    /// MD_CharacterSetCode: 8859part7 — ISO-8859-7, Greek script.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part7
    pub struct MdCharsetGreek;
    structural_prop!(MdCharsetGreek, "MdCharsetGreek");

    /// MD_CharacterSetCode: 8859part8 — ISO-8859-8, Hebrew script.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part8
    pub struct MdCharsetHebrew;
    structural_prop!(MdCharsetHebrew, "MdCharsetHebrew");

    /// MD_CharacterSetCode: 8859part9 — ISO-8859-9 Latin-5, Turkish; replaces
    /// rarely used Icelandic letters with Turkish-specific characters.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / 8859part9
    pub struct MdCharsetLatin5;
    structural_prop!(MdCharsetLatin5, "MdCharsetLatin5");

    /// MD_CharacterSetCode: ucs2 — ISO/IEC 10646-1 UCS-2 fixed 2-byte encoding;
    /// covers the Basic Multilingual Plane.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / ucs2
    pub struct MdCharsetUcs2;
    structural_prop!(MdCharsetUcs2, "MdCharsetUcs2");

    /// MD_CharacterSetCode: ucs4 — ISO/IEC 10646-1 UCS-4 fixed 4-byte encoding;
    /// covers the full Unicode character space.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / ucs4
    pub struct MdCharsetUcs4;
    structural_prop!(MdCharsetUcs4, "MdCharsetUcs4");

    /// MD_CharacterSetCode: shiftJIS — Shift-JIS double-byte encoding for
    /// Japanese; widely used in Japanese GIS data from legacy systems.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / shiftJIS
    pub struct MdCharsetShiftJis;
    structural_prop!(MdCharsetShiftJis, "MdCharsetShiftJis");

    /// MD_CharacterSetCode: eucJP — EUC-JP encoding for Japanese; common in
    /// Unix/Linux Japanese environments.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / eucJP
    pub struct MdCharsetEucJp;
    structural_prop!(MdCharsetEucJp, "MdCharsetEucJp");

    /// MD_CharacterSetCode: big5 — Big5 double-byte encoding for Traditional
    /// Chinese; used in Taiwan and Hong Kong.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / big5
    pub struct MdCharsetBig5;
    structural_prop!(MdCharsetBig5, "MdCharsetBig5");

    /// MD_CharacterSetCode: GB2312 — GB-2312 encoding for Simplified Chinese;
    /// used in mainland China GIS data.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / GB2312
    pub struct MdCharsetGb2312;
    structural_prop!(MdCharsetGb2312, "MdCharsetGb2312");

    /// MD_CharacterSetCode: usAscii — 7-bit US-ASCII; the most restrictive
    /// encoding; suitable only for plain English metadata.
    ///
    /// Source: ISO 19115-1:2014 Annex B — MD_CharacterSetCode / usAscii
    pub struct MdCharsetUsAscii;
    structural_prop!(MdCharsetUsAscii, "MdCharsetUsAscii");

    /// All temporal values in the metadata record shall conform to ISO 8601;
    /// acceptable formats: YYYY, YYYY-MM, YYYY-MM-DD, YYYY-MM-DDThh:mm:ssZ.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — temporal value format
    pub struct Iso19115DateIso8601Format;
    structural_prop!(Iso19115DateIso8601Format, "Iso19115DateIso8601Format");

    /// All LanguageCode values in the metadata record shall be ISO 639-2 three-letter
    /// lowercase terminological (2/T) codes; bibliographic (2/B) codes are not used.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — language code format
    pub struct Iso19115LanguageCodeIso6392;
    structural_prop!(Iso19115LanguageCodeIso6392, "Iso19115LanguageCodeIso6392");

    /// All CountryCode values shall be ISO 3166-1 alpha-2 (two uppercase letters) or
    /// alpha-3 (three uppercase letters) codes; numeric codes are not used.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — country code format
    pub struct Iso19115CountryCodeIso3166;
    structural_prop!(Iso19115CountryCodeIso3166, "Iso19115CountryCodeIso3166");

    /// CI_Citation.title shall never be the empty string in any context where
    /// CI_Citation appears: identification, thesaurus, format, lineage documentation.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — title non-empty constraint
    pub struct Iso19115CitationTitleNeverEmpty;
    structural_prop!(
        Iso19115CitationTitleNeverEmpty,
        "Iso19115CitationTitleNeverEmpty"
    );

    /// LI_ProcessStep.description shall never be the empty string; a step without a
    /// description cannot communicate what transformation was applied.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — process step description non-empty
    pub struct Iso19115ProcessStepDescriptionNeverEmpty;
    structural_prop!(
        Iso19115ProcessStepDescriptionNeverEmpty,
        "Iso19115ProcessStepDescriptionNeverEmpty"
    );

    /// Among all CI_Responsibility instances in MD_Metadata.contact, at least one party
    /// shall carry a non-null, non-empty CI_Individual.name or CI_Organisation.name.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — contact party name
    pub struct Iso19115ContactPartyNameNonNull;
    structural_prop!(
        Iso19115ContactPartyNameNonNull,
        "Iso19115ContactPartyNameNonNull"
    );

    /// A conformant metadata record shall contain all mandatory (M) attributes defined
    /// by the standard; absence of any M attribute is a conformance failure.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — mandatory element presence
    pub struct Iso19115AllMandatoryElementsPresent;
    structural_prop!(
        Iso19115AllMandatoryElementsPresent,
        "Iso19115AllMandatoryElementsPresent"
    );

    /// Conditional (C) attributes shall be present whenever the triggering condition
    /// holds; absence of a conditional attribute when its trigger is satisfied is
    /// a conformance failure equal in severity to absence of a mandatory attribute.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — conditional element trigger
    pub struct Iso19115ConditionalElementsTriggered;
    structural_prop!(
        Iso19115ConditionalElementsTriggered,
        "Iso19115ConditionalElementsTriggered"
    );

    /// All code-list attribute values shall be drawn from the code lists defined in
    /// ISO 19115-1 Annex B; values not in the defined enumeration are non-conformant.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — enumeration value validity
    pub struct Iso19115EnumerationValuesValid;
    structural_prop!(
        Iso19115EnumerationValuesValid,
        "Iso19115EnumerationValuesValid"
    );

    /// Multiplicity constraints shall be respected: exactly-1 (1) attributes shall
    /// not be absent or appear more than once; 1..* attributes shall have at least one.
    ///
    /// Source: ISO 19115-1:2014 (cross-cutting) — multiplicity constraint
    pub struct Iso19115MultiplicityConstraintsMet;
    structural_prop!(
        Iso19115MultiplicityConstraintsMet,
        "Iso19115MultiplicityConstraintsMet"
    );
}

pub use emit_impls::{
    CiAddressAdministrativeAreaOptional, CiAddressCityOptional, CiAddressCountryIsIso3166,
    CiAddressCountryOptional, CiAddressDeliveryPointOptional, CiAddressEmailOptional,
    CiAddressPostalCodeOptional, CiCitationAlternateTitleOptional,
    CiCitationCollectiveTitleOptional, CiCitationDateMandatory, CiCitationEditionDateOptional,
    CiCitationEditionOptional, CiCitationIdentifierOptional, CiCitationIsbnFormatValid,
    CiCitationIssnFormatValid, CiCitationOtherDetailsOptional, CiCitationPresentationFormOptional,
    CiCitationResponsiblePartyOptional, CiCitationSeriesOptional, CiCitationTitleMandatory,
    CiCitationTitleNonEmpty, CiContactAddressOptional, CiContactHoursOfServiceOptional,
    CiContactInstructionsOptional, CiContactOnlineResourceOptional, CiContactPhoneOptional,
    CiDateTypeAdopted, CiDateTypeCodeMandatory, CiDateTypeCreation, CiDateTypeDeprecated,
    CiDateTypeDistribution, CiDateTypeExpiry, CiDateTypeInForce, CiDateTypeLastRevision,
    CiDateTypeLastUpdate, CiDateTypeNextUpdate, CiDateTypePublication, CiDateTypeReleased,
    CiDateTypeRevision, CiDateTypeSuperseded, CiDateTypeUnavailable, CiDateTypeValidityBegins,
    CiDateTypeValidityExpires, CiDateValueMandatory, CiIndividualContactInfoOptional,
    CiIndividualNameOptional, CiIndividualNameOrPositionRequired, CiIndividualPositionNameOptional,
    CiOnlineFunctionBrowseGraphic, CiOnlineFunctionBrowsing, CiOnlineFunctionCompleteMetadata,
    CiOnlineFunctionDownload, CiOnlineFunctionEmailService, CiOnlineFunctionFileAccess,
    CiOnlineFunctionInformation, CiOnlineFunctionOfflineAccess, CiOnlineFunctionOrder,
    CiOnlineFunctionSearch, CiOnlineFunctionUpload, CiOnlineResourceApplicationProfileOptional,
    CiOnlineResourceDescriptionOptional, CiOnlineResourceFunctionOptional,
    CiOnlineResourceLinkageMandatory, CiOnlineResourceLinkageValidUrl,
    CiOnlineResourceNameOptional, CiOnlineResourceProtocolOptional,
    CiOrganisationContactInfoOptional, CiOrganisationIndividualOptional,
    CiOrganisationNameOptional, CiPartyIsAbstract, CiResponsibilityExtentOptional,
    CiResponsibilityPartyMandatory, CiResponsibilityPartyNameNonNull,
    CiResponsibilityRoleMandatory, CiRoleAuthor, CiRoleCoAuthor, CiRoleCollaborator,
    CiRoleContributor, CiRoleCustodian, CiRoleDistributor, CiRoleEditor, CiRoleFunder,
    CiRoleMediator, CiRoleOriginator, CiRoleOwner, CiRolePointOfContact,
    CiRolePrincipalInvestigator, CiRoleProcessor, CiRolePublisher, CiRoleResourceProvider,
    CiRoleRightsHolder, CiRoleSponsor, CiRoleStakeholder, CiRoleUser, CiTelephoneFacsimile,
    CiTelephoneNumberMandatory, CiTelephoneNumberNonEmpty, CiTelephoneSms, CiTelephoneTypeVoice,
    DqAbsoluteExternalPositionalAccuracy, DqCompletenessCommission, DqCompletenessOmission,
    DqConceptualConsistency, DqDataQualityReportOptional, DqDataQualityScopeMandatory,
    DqDataQualityStandaloneOptional, DqDomainConsistency, DqFormatConsistency,
    DqGriddedDataPositionalAccuracy, DqNonQuantitativeAttributeCorrectness,
    DqQuantitativeAttributeAccuracy, DqRelativeInternalPositionalAccuracy, DqTemporalConsistency,
    DqTemporalValidity, DqThematicClassificationCorrectness, DqTopologicalConsistency,
    DsAssociationTypeCrossReference, DsAssociationTypeIsComposedOf,
    DsAssociationTypeLargerWorkCitation, DsAssociationTypePartOfSeamlessDatabase,
    DsAssociationTypeRevisionOf, DsInitiativeTypeMission, DsInitiativeTypePlatform,
    DsInitiativeTypeProject, ExBboxEastBoundMandatory, ExBboxExtentTypeCodeOptional,
    ExBboxLatitudeRange, ExBboxLongitudeRange, ExBboxNorthBoundMandatory,
    ExBboxSouthBoundMandatory, ExBboxSouthLeNorth, ExBboxWestBoundMandatory,
    ExBboxWestLeEastOrAntimeridian, ExBoundingPolygonAtLeastOneGeometry,
    ExBoundingPolygonExtentTypeCodeOptional, ExBoundingPolygonGeometryIsValid,
    ExBoundingPolygonPolygonMandatory, ExExtentAtLeastOneElementRequired,
    ExExtentDescriptionOptional, ExExtentGeographicElementConditional,
    ExExtentTemporalElementOptional, ExExtentVerticalElementOptional,
    ExTemporalExtentExtentMandatory, ExTemporalExtentPeriodBeginLeEnd, ExVerticalExtentCrsOptional,
    ExVerticalExtentMaximumMandatory, ExVerticalExtentMinLeMax, ExVerticalExtentMinimumMandatory,
    Iso19115AllMandatoryElementsPresent, Iso19115CitationTitleNeverEmpty,
    Iso19115ConditionalElementsTriggered, Iso19115ContactPartyNameNonNull,
    Iso19115CountryCodeIso3166, Iso19115DateIso8601Format, Iso19115EnumerationValuesValid,
    Iso19115LanguageCodeIso6392, Iso19115MultiplicityConstraintsMet,
    Iso19115ProcessStepDescriptionNeverEmpty, LiLineageAdditionalDocumentationOptional,
    LiLineageAtLeastOneProvided, LiLineageProcessStepOptional, LiLineageScopeOptional,
    LiLineageSourceOptional, LiLineageStatementConditional, LiProcessStepDateTimeOptional,
    LiProcessStepDescriptionMandatory, LiProcessStepDescriptionNonEmpty,
    LiProcessStepProcessorOptional, LiProcessStepRationaleOptional, LiProcessStepReferenceOptional,
    LiProcessStepScopeOptional, LiProcessStepSourceOptional, LiSourceCitationOptional,
    LiSourceDescriptionConditional, LiSourceDescriptionOrCitationRequired,
    LiSourceReferenceSystemOptional, LiSourceSpatialResolutionOptional, LiSourceStepOptional,
    MdAssociatedResourceAssociationTypeMandatory, MdAssociatedResourceInitiativeTypeOptional,
    MdAssociatedResourceMetadataRefConditional, MdAssociatedResourceNameConditional,
    MdAssociatedResourceNameOrMetaRefRequired, MdBrowseGraphicFileDescriptionOptional,
    MdBrowseGraphicFileNameMandatory, MdBrowseGraphicFileNameNonEmpty,
    MdBrowseGraphicFileTypeOptional, MdBrowseGraphicLinkageOptional, MdCharsetArabic,
    MdCharsetBig5, MdCharsetCyrillic, MdCharsetEucJp, MdCharsetGb2312, MdCharsetGreek,
    MdCharsetHebrew, MdCharsetLatin1, MdCharsetLatin2, MdCharsetLatin5, MdCharsetShiftJis,
    MdCharsetUcs2, MdCharsetUcs4, MdCharsetUsAscii, MdCharsetUtf8, MdCharsetUtf16, MdCharsetUtf32,
    MdClassificationConfidential, MdClassificationForOfficialUseOnly,
    MdClassificationLimitedDistribution, MdClassificationProtected, MdClassificationRestricted,
    MdClassificationSecret, MdClassificationSensitiveButUnclassified, MdClassificationTopSecret,
    MdClassificationUnclassified, MdConstraintsApplicationScopeOptional,
    MdConstraintsUseLimitationOptional, MdDataIdentificationCharacterSetConditional,
    MdDataIdentificationExtentConditional, MdDataIdentificationLanguageConditional,
    MdDataIdentificationSpatialResolutionOptional, MdDataIdentificationSupplementalInfoOptional,
    MdDataIdentificationTopicCategoryConditional, MdDimensionNameColumn, MdDimensionNameMandatory,
    MdDimensionNameRow, MdDimensionNameTime, MdDimensionNameVertical,
    MdDimensionResolutionOptional, MdDimensionSizeMandatory, MdDimensionSizePositive,
    MdDistributionAtLeastOneElement, MdDistributionDistributorOptional,
    MdDistributionFormatOptional, MdDistributionTransferOptionsOptional,
    MdDistributorContactMandatory, MdDistributorFormatOptional, MdDistributorOrderProcessOptional,
    MdDistributorTransferOptionsOptional, MdFormatAmendmentNumberOptional,
    MdFormatDistributorOptional, MdFormatFileDecompressionOptional, MdFormatMediumOptional,
    MdFormatSpecificationCitationMandatory, MdGridSpatialRepAxisDimensionProperties,
    MdGridSpatialRepCellGeometry, MdGridSpatialRepNumberOfDimensions,
    MdGridSpatialRepTransformationAvailable, MdIdentificationAbstractMandatory,
    MdIdentificationAbstractNonEmpty, MdIdentificationCitationMandatory,
    MdIdentificationCreditOptional, MdIdentificationDescriptiveKeywordsOptional,
    MdIdentificationGraphicOverviewOptional, MdIdentificationPointOfContactOptional,
    MdIdentificationPurposeOptional, MdIdentificationResourceConstraintsOptional,
    MdIdentificationResourceFormatOptional, MdIdentificationResourceMaintenanceOptional,
    MdIdentificationStatusOptional, MdKeywordTypeDataCentre, MdKeywordTypeDiscipline,
    MdKeywordTypeFeatureType, MdKeywordTypeInstrument, MdKeywordTypePlace, MdKeywordTypePlatform,
    MdKeywordTypeProcess, MdKeywordTypeProduct, MdKeywordTypeProject, MdKeywordTypeService,
    MdKeywordTypeStratum, MdKeywordTypeSubTopicCategory, MdKeywordTypeTaxon, MdKeywordTypeTemporal,
    MdKeywordTypeTheme, MdKeywordsKeywordClassOptional, MdKeywordsKeywordMandatory,
    MdKeywordsThesaurusNameOptional, MdKeywordsTypeOptional,
    MdLegalConstraintsAccessConstraintsOptional, MdLegalConstraintsOtherConstraintsConditional,
    MdLegalConstraintsUseConstraintsOptional, MdMaintenanceContactOptional,
    MdMaintenanceDateOptional, MdMaintenanceFrequencyAnnually, MdMaintenanceFrequencyAsNeeded,
    MdMaintenanceFrequencyContinual, MdMaintenanceFrequencyDaily, MdMaintenanceFrequencyIrregular,
    MdMaintenanceFrequencyMandatory, MdMaintenanceFrequencyMonthly,
    MdMaintenanceFrequencyNotPlanned, MdMaintenanceFrequencyQuarterly,
    MdMaintenanceFrequencyUnknown, MdMaintenanceFrequencyWeekly, MdMaintenanceNoteOptional,
    MdMaintenanceScopeOptional, MdMaintenanceUserDefinedFreqConditional,
    MdMediumDensityUnitsConditional, MdMediumVolumesOptional, MdMetadataCharacterSetConditional,
    MdMetadataConstraintsOptional, MdMetadataContactMandatory, MdMetadataContactPartyNameNonNull,
    MdMetadataDataQualityInfoOptional, MdMetadataDateInfoMandatory,
    MdMetadataDistributionInfoOptional, MdMetadataFileIdentifierOptional,
    MdMetadataHierarchyLevelNameMatchesLevel, MdMetadataHierarchyLevelScopeCode,
    MdMetadataIdentificationInfoMandatory, MdMetadataLanguageConditional, MdMetadataLocaleOptional,
    MdMetadataMaintenanceOptional, MdMetadataParentIdentifierOptional,
    MdMetadataReferenceSystemInfoOptional, MdMetadataResourceLineageOptional,
    MdMetadataSpatialRepresentationInfoOptional, MdMetadataStandardNameOptional,
    MdMetadataStandardVersionOptional, MdProgressCodeAccepted, MdProgressCodeCompleted,
    MdProgressCodeDeprecated, MdProgressCodeFinal, MdProgressCodeHistoricalArchive,
    MdProgressCodeNotAccepted, MdProgressCodeObsolete, MdProgressCodeOnGoing,
    MdProgressCodePending, MdProgressCodePlanned, MdProgressCodeProposed, MdProgressCodeRequired,
    MdProgressCodeRetired, MdProgressCodeSuperseded, MdProgressCodeTentative,
    MdProgressCodeUnderDevelopment, MdProgressCodeValid, MdProgressCodeWithdrawn,
    MdReferenceSystemIdentifierOptional, MdReferenceSystemTypeOptional,
    MdRepresentativeFractionDenominatorMandatory, MdRepresentativeFractionDenominatorPositive,
    MdResolutionDistanceIsPositive, MdResolutionEquivalentScaleOrDistance,
    MdResolutionScaleImpliesSmallIsCoarse, MdRestrictionConfidentialCode,
    MdRestrictionCopyrightCode, MdRestrictionInConfidenceCode,
    MdRestrictionIntellectualPropertyCode, MdRestrictionLicenceCode,
    MdRestrictionLicenceDistributorCode, MdRestrictionLicenceEndUserCode,
    MdRestrictionLicenceUnrestrictedCode, MdRestrictionOtherRestrictionsCode,
    MdRestrictionPatentCode, MdRestrictionPatentPendingCode, MdRestrictionPrivateCode,
    MdRestrictionRestrictedCode, MdRestrictionSbuCode, MdRestrictionStatutoryCode,
    MdRestrictionTrademarkCode, MdRestrictionUnrestrictedCode, MdScopeAttribute,
    MdScopeAttributeType, MdScopeCollectionSession, MdScopeDataset, MdScopeDimensionGroup,
    MdScopeFeature, MdScopeFeatureType, MdScopeFieldSession, MdScopeInitiative, MdScopeModel,
    MdScopeNonGeographicDataset, MdScopeSeries, MdScopeService, MdScopeSoftware, MdScopeTile,
    MdSecurityConstraintsClassSystemOptional, MdSecurityConstraintsClassificationMandatory,
    MdSecurityConstraintsHandlingDescOptional, MdSecurityConstraintsUserNoteOptional,
    MdSpatialRepTypeGrid, MdSpatialRepTypeStereoModel, MdSpatialRepTypeTextTable,
    MdSpatialRepTypeTin, MdSpatialRepTypeVector, MdSpatialRepTypeVideo, MdTopicCategoryBiota,
    MdTopicCategoryBoundaries, MdTopicCategoryClimatologyMeteorologyAtmosphere,
    MdTopicCategoryDisaster, MdTopicCategoryEconomy, MdTopicCategoryElevation,
    MdTopicCategoryEnvironment, MdTopicCategoryExtraTerrestrial, MdTopicCategoryFarming,
    MdTopicCategoryGeoscientificInformation, MdTopicCategoryHealth,
    MdTopicCategoryImageryBaseMapsEarthCover, MdTopicCategoryInlandWaters,
    MdTopicCategoryIntelligenceMilitary, MdTopicCategoryLocation, MdTopicCategoryOceans,
    MdTopicCategoryPlanningCadastre, MdTopicCategorySociety, MdTopicCategoryStructure,
    MdTopicCategoryTransportation, MdTopicCategoryUtilitiesCommunication,
    MdTransferOptionsOfflineOptional, MdTransferOptionsOnlineOptional,
    MdTransferOptionsSizePositive, MdVectorSpatialRepGeometricObjects,
    MdVectorSpatialRepTopologyLevel, PtLocaleCharacterEncodingMandatory, PtLocaleCountryOptional,
    PtLocaleLanguageCodeThreeLetterLowercase, PtLocaleLanguageMandatory,
};
