//! `GaapAssetFactory` — asset-measurement factory (Role 1a).
//! `GaapAssetMeta`    — asset-ledger reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::asc_300::{
    DebtSecurityClassified, EquitySecurityAtFairValue, InventoryAtLowerOfCostOrNrv,
    PpeCarriedAtCost, ReceivableRecordedAtAmortizedCost,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::types::{
    AccountId, FinancialPeriod, InventoryDescriptor, MonetaryAmount, PpeDescriptor,
    ReceivableDescriptor, SecurityDescriptor,
};
use elicitation::Established;

// ── Role 1a: asset measurement factory ───────────────────────────────────────

/// Factory for recognizing and measuring financial and non-financial assets.
///
/// Source: ASC 310 — Receivables; ASC 320/321 — Investments;
///         ASC 330 — Inventory; ASC 360 — PP&E.
pub trait GaapAssetFactory: Send + Sync {
    // ── Receivables ───────────────────────────────────────────────────────────

    /// Record a trade or non-trade receivable.
    ///
    /// Returns `ReceivableRecordedAtAmortizedCost`.
    ///
    /// Source: ASC 310-10-35 — Subsequent Measurement.
    fn record_receivable(
        &self,
        receivable: ReceivableDescriptor,
    ) -> GaapResult<(
        ReceivableDescriptor,
        Established<ReceivableRecordedAtAmortizedCost>,
    )>;

    // ── Investment securities ─────────────────────────────────────────────────

    /// Classify and record a debt security.
    ///
    /// Returns `DebtSecurityClassified`.
    ///
    /// Source: ASC 320-10-25 — Classification.
    fn record_debt_security(
        &self,
        security: SecurityDescriptor,
    ) -> GaapResult<(SecurityDescriptor, Established<DebtSecurityClassified>)>;

    /// Record an equity security at fair value through net income.
    ///
    /// Returns `EquitySecurityAtFairValue`.
    ///
    /// Source: ASC 321-10-35-1.
    fn record_equity_security(
        &self,
        security: SecurityDescriptor,
    ) -> GaapResult<(SecurityDescriptor, Established<EquitySecurityAtFairValue>)>;

    // ── Inventory ─────────────────────────────────────────────────────────────

    /// Measure inventory at the lower of cost or net realizable value.
    ///
    /// Returns `InventoryAtLowerOfCostOrNrv`.
    ///
    /// Source: ASC 330-10-35-1B.
    fn measure_inventory(
        &self,
        inventory: InventoryDescriptor,
    ) -> GaapResult<(
        InventoryDescriptor,
        Established<InventoryAtLowerOfCostOrNrv>,
    )>;

    // ── PP&E ──────────────────────────────────────────────────────────────────

    /// Recognize a PP&E asset at historical cost.
    ///
    /// Returns `PpeCarriedAtCost`.
    ///
    /// Source: ASC 360-10-30-1.
    fn recognize_ppe(
        &self,
        asset: PpeDescriptor,
    ) -> GaapResult<(PpeDescriptor, Established<PpeCarriedAtCost>)>;

    /// Test a PP&E asset for impairment.
    ///
    /// Returns the asset with an updated carrying amount reflecting any
    /// write-down to fair value.
    ///
    /// Source: ASC 360-10-35-17 — Impairment or Disposal of Long-Lived Assets.
    fn test_ppe_impairment(
        &self,
        asset: PpeDescriptor,
        recoverable_amount: MonetaryAmount,
    ) -> GaapResult<PpeDescriptor>;
}

// ── Role 2: asset-ledger reporter ────────────────────────────────────────────

/// Orthogonal asset-ledger reporter.
pub trait GaapAssetMeta: Send + Sync {
    /// Return all receivables with a balance at the given period-end date.
    fn receivables_at_period_end(
        &self,
        period: FinancialPeriod,
    ) -> BoxFuture<'_, GaapResult<Vec<ReceivableDescriptor>>>;

    /// Return inventory balances at the given period-end date.
    fn inventory_balances(
        &self,
        account: AccountId,
        period: FinancialPeriod,
    ) -> BoxFuture<'_, GaapResult<InventoryDescriptor>>;
}
