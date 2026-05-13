//! `GaapDisclosureFactory` — footnote preparation factory (Role 1a).
//! `GaapDisclosureMeta`    — disclosure-checklist reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::asc_200::GoingConcernEvaluated;
use crate::gaap::asc_800::FinancialInstrumentFairValueDisclosed;
use crate::gaap::disclosure::{
    LeaseLiabilityMaturityDisclosed, RevenueDisaggregationNote, SegmentInformationDisclosed,
    SignificantAccountingPoliciesDisclosed,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::types::{DisclosureRequirement, FootnoteDescriptor};
use elicitation::Established;

// ── Role 1a: footnote preparation factory ────────────────────────────────────

/// Factory for preparing required financial statement footnote disclosures.
///
/// Each method takes a `FootnoteDescriptor` plus any required evidence and
/// returns an `Established<P>` token proving the disclosure is present.
///
/// Source: ASC 235 — Notes to Financial Statements; various §50 paragraphs.
pub trait GaapDisclosureFactory: Send + Sync {
    /// Prepare Note 1 — Significant Accounting Policies.
    ///
    /// Returns `SignificantAccountingPoliciesDisclosed`.
    ///
    /// Source: ASC 235-10-50-1.
    fn disclose_accounting_policies(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(
        FootnoteDescriptor,
        Established<SignificantAccountingPoliciesDisclosed>,
    )>;

    /// Prepare the ASC 606 revenue disaggregation and backlog disclosure.
    ///
    /// Returns `RevenueDisaggregationNote`.
    ///
    /// Source: ASC 606-10-50-5.
    fn disclose_revenue_recognition(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(FootnoteDescriptor, Established<RevenueDisaggregationNote>)>;

    /// Prepare the ASC 820 fair value hierarchy table.
    ///
    /// Returns `FinancialInstrumentFairValueDisclosed`.
    ///
    /// Source: ASC 820-10-50-2.
    fn disclose_fair_values(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(
        FootnoteDescriptor,
        Established<FinancialInstrumentFairValueDisclosed>,
    )>;

    /// Prepare the ASC 842 lease maturity analysis and ROU/liability table.
    ///
    /// Returns `LeaseLiabilityMaturityDisclosed`.
    ///
    /// Source: ASC 842-20-50-6.
    fn disclose_leases(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(
        FootnoteDescriptor,
        Established<LeaseLiabilityMaturityDisclosed>,
    )>;

    /// Prepare going-concern evaluation disclosure.
    ///
    /// Returns `GoingConcernEvaluated`.
    ///
    /// Source: ASC 205-40-50-1.
    fn disclose_going_concern(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(FootnoteDescriptor, Established<GoingConcernEvaluated>)>;

    /// Prepare segment reporting disclosures.
    ///
    /// Returns `SegmentInformationDisclosed`.
    ///
    /// Source: ASC 280-10-50-1.
    fn disclose_segments(
        &self,
        note: FootnoteDescriptor,
    ) -> GaapResult<(FootnoteDescriptor, Established<SegmentInformationDisclosed>)>;
}

// ── Role 2: disclosure-checklist reporter ────────────────────────────────────

/// Orthogonal disclosure completeness reporter.
///
/// Queries which required disclosures have and have not been satisfied.
pub trait GaapDisclosureMeta: Send + Sync {
    /// Return all outstanding (unsatisfied) disclosure requirements.
    fn outstanding_disclosures(&self) -> BoxFuture<'_, GaapResult<Vec<DisclosureRequirement>>>;

    /// Return all satisfied disclosure requirements.
    fn satisfied_disclosures(&self) -> BoxFuture<'_, GaapResult<Vec<DisclosureRequirement>>>;
}
