//! Orthogonal concern traits for ISO 19111 `IO_IdentifiedObject` and scope.
//!
//! These traits operate independently of validity — you can query name, aliases,
//! scope, and domain of validity for any object regardless of whether its CRS
//! or geometry is well-formed.
//!
//! # Role
//!
//! `Iso19111Identified` and `Iso19111Scoped` are **orthogonal concern** traits:
//! they isolate metadata and provenance reporting as a separate axis from
//! geometric or CRS validity.  An implementation may return a name even for
//! an incompletely constructed object.
//!
//! # Object safety
//!
//! All method signatures use concrete `Established<P>` types (no generics),
//! making these traits `dyn`-compatible.
//!
//! Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject; §6 — SC_CRS scope /
//! domain of validity.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuthorityCode, CrsDomainOfValidityExtentTypes, CrsScopeDescribesIntendedUse, DomainExtent,
    GisResult, IdentifiedObjectAliasNoDuplicates, IdentifiedObjectAliasNoNullEntries,
    IdentifiedObjectAliasOptionalList, IdentifiedObjectIdentifierEntryComplete,
    IdentifiedObjectPrimaryNameNonEmpty,
};

/// Queries identity metadata for any ISO 19111 identified object.
///
/// Corresponds to the `IO_IdentifiedObject` UML class (§6.2), which is
/// inherited by CRS, datum, coordinate system, coordinate operation, and
/// unit-of-measure types.
///
/// These methods are callable regardless of structural validity — a
/// partially constructed object still has a name and identifiers.
///
/// Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject.
pub trait Iso19111Identified: Send + Sync {
    /// Return the primary (canonical) name of this object and prove it
    /// is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject.name (mandatory).
    fn primary_name(
        &self,
    ) -> BoxFuture<'_, GisResult<(String, Established<IdentifiedObjectPrimaryNameNonEmpty>)>>;

    /// Return all aliases for this object and prove the list is optionally
    /// present (may be empty).
    ///
    /// Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject.alias (0..*).
    fn aliases(
        &self,
    ) -> BoxFuture<'_, GisResult<(Vec<String>, Established<IdentifiedObjectAliasOptionalList>)>>;

    /// Given an alias list token, prove that no entry is a null/empty string.
    ///
    /// Requires `aliases_token` from [`aliases`].
    ///
    /// Source: ISO 19111:2019 §6.2 — alias entries must be non-null strings.
    ///
    /// [`aliases`]: Self::aliases
    fn aliases_no_nulls(
        &self,
        aliases_token: Established<IdentifiedObjectAliasOptionalList>,
    ) -> BoxFuture<'_, GisResult<Established<IdentifiedObjectAliasNoNullEntries>>>;

    /// Given a no-nulls alias token, prove that no two aliases are identical.
    ///
    /// Requires `no_nulls_token` from [`aliases_no_nulls`].
    ///
    /// Source: ISO 19111:2019 §18.4 — no duplicate aliases.
    ///
    /// [`aliases_no_nulls`]: Self::aliases_no_nulls
    fn aliases_no_duplicates(
        &self,
        no_nulls_token: Established<IdentifiedObjectAliasNoNullEntries>,
    ) -> BoxFuture<'_, GisResult<Established<IdentifiedObjectAliasNoDuplicates>>>;

    /// Return the optional remarks string for this object.
    ///
    /// Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject.remarks (optional).
    fn remarks(&self) -> BoxFuture<'_, GisResult<Option<String>>>;

    /// Return all authority-code identifiers registered for this object.
    ///
    /// Source: ISO 19111:2019 §6.2 — IO_IdentifiedObject.identifier (0..*).
    fn identifiers(&self) -> BoxFuture<'_, GisResult<Vec<AuthorityCode>>>;

    /// Given a specific identifier, prove that both its authority name and
    /// code fields are non-empty.
    ///
    /// Source: ISO 19111:2019 §6.2 — RS_Identifier requires both authority
    /// and code.
    fn identifier_complete(
        &self,
        id: &AuthorityCode,
    ) -> BoxFuture<'_, GisResult<Established<IdentifiedObjectIdentifierEntryComplete>>>;
}

/// Queries scope and domain of validity for any ISO 19111 CRS or coordinate
/// operation.
///
/// These properties are orthogonal to structural validity: an object may have
/// a well-defined intended use description even if its internal geometry or
/// parameter checks have not yet been performed.
///
/// Source: ISO 19111:2019 §6 — SC_CRS.scope / SC_CRS.domainOfValidity.
pub trait Iso19111Scoped: Send + Sync {
    /// Return the scope — a free-text description of the intended use — and
    /// prove it is non-empty and describes an intended use.
    ///
    /// Source: ISO 19111:2019 §6 — SC_CRS.scope (mandatory, 1..*).
    fn scope(
        &self,
    ) -> BoxFuture<'_, GisResult<(String, Established<CrsScopeDescribesIntendedUse>)>>;

    /// Return the optional domain of validity for this object.
    ///
    /// `None` implies global applicability.
    ///
    /// Source: ISO 19111:2019 §6 — SC_CRS.domainOfValidity (optional).
    fn domain_of_validity(&self) -> BoxFuture<'_, GisResult<Option<DomainExtent>>>;

    /// Given a domain extent, prove that it is expressed using at least one
    /// recognised extent type (geographic, temporal, or vertical bounding box).
    ///
    /// Source: ISO 19111:2019 §6 — domain of validity extent types.
    fn domain_extent_types(
        &self,
        domain: &DomainExtent,
    ) -> BoxFuture<'_, GisResult<Established<CrsDomainOfValidityExtentTypes>>>;
}
