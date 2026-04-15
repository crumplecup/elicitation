//! `GaapEquityFactory` — stockholders' equity factory (Role 1a).

use crate::gaap::asc_500::{
    OciAccumulatedSeparately, StockholdersEquityPresented, TreasuryStockAccountedFor,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::mathematical::RetainedEarningsRollforward;
use crate::gaap::proof_composition::RetainedEarningsEvidence;
use crate::gaap::types::{
    EquityDescriptor, MonetaryAmount, OciDescriptor, TreasuryStockDescriptor,
};
use elicitation::Established;

// ── Role 1a: stockholders' equity factory ────────────────────────────────────

/// Factory for recognizing and presenting stockholders' equity components.
///
/// Source: ASC 505 — Equity; ASC 220 — Comprehensive Income.
pub trait GaapEquityFactory: Send + Sync {
    // ── Equity components ─────────────────────────────────────────────────────

    /// Record an equity component and assert that all required equity lines
    /// are presented.
    ///
    /// Returns `StockholdersEquityPresented`.
    ///
    /// Source: ASC 505-10-45 — Equity Presentation.
    fn present_equity_component(
        &self,
        equity: EquityDescriptor,
    ) -> GaapResult<(EquityDescriptor, Established<StockholdersEquityPresented>)>;

    // ── OCI ───────────────────────────────────────────────────────────────────

    /// Record OCI items and assert that accumulated OCI is presented as a
    /// separate equity component.
    ///
    /// Returns `OciAccumulatedSeparately`.
    ///
    /// Source: ASC 220-10-45-1.
    fn record_oci(
        &self,
        oci: OciDescriptor,
    ) -> GaapResult<(OciDescriptor, Established<OciAccumulatedSeparately>)>;

    // ── Treasury stock ────────────────────────────────────────────────────────

    /// Record a treasury stock repurchase.
    ///
    /// Returns `TreasuryStockAccountedFor`.
    ///
    /// Source: ASC 505-30-30-1 — Cost Method.
    fn record_treasury_stock(
        &self,
        treasury: TreasuryStockDescriptor,
    ) -> GaapResult<(
        TreasuryStockDescriptor,
        Established<TreasuryStockAccountedFor>,
    )>;

    // ── Retained earnings rollforward ─────────────────────────────────────────

    /// Verify the retained earnings rollforward.
    ///
    /// Requires `RetainedEarningsEvidence` (net income + dividends proofs).
    /// Returns `RetainedEarningsRollforward`.
    ///
    /// Source: `RetainedEarningsRollforward` prop — mathematical invariant.
    fn verify_retained_earnings_rollforward(
        &self,
        evidence: RetainedEarningsEvidence,
        opening_balance: MonetaryAmount,
        net_income: MonetaryAmount,
        dividends_paid: MonetaryAmount,
        closing_balance: MonetaryAmount,
    ) -> GaapResult<Established<RetainedEarningsRollforward>>;

    // ── Stock split ───────────────────────────────────────────────────────────

    /// Record a stock split or reverse stock split.
    ///
    /// The factory adjusts par value and shares outstanding while leaving total
    /// equity unchanged.  Returns the updated equity descriptor.
    ///
    /// Source: ASC 505-20 — Stock Dividends and Stock Splits.
    fn record_stock_split(
        &self,
        equity: EquityDescriptor,
        split_ratio: f64,
    ) -> GaapResult<EquityDescriptor>;
}
