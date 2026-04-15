//! Tax accounting descriptor types — deferred taxes, effective tax rate.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount};

// ── Deferred taxes ────────────────────────────────────────────────────────────

/// Descriptor for a single deferred tax position.
///
/// The factory asserts `DeferredTaxAssetRecognized` or
/// `DeferredTaxLiabilityRecognized` based on the temporary difference direction,
/// and `ValuationAllowanceAssessed` when realizability is evaluated.
///
/// Source: ASC 740 — Income Taxes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DeferredTaxDescriptor {
    /// Ledger account for the deferred tax asset or liability.
    pub account: AccountId,
    /// Temporary difference between book and tax basis.
    ///
    /// Positive → future taxable amount (deferred tax liability direction).
    /// Negative → future deductible amount (deferred tax asset direction).
    pub temporary_difference: MonetaryAmount,
    /// Enacted statutory tax rate applied to the temporary difference.
    pub enacted_rate: f64,
    /// Computed deferred tax asset (negative temporary difference × rate).
    pub deferred_tax_asset: Option<MonetaryAmount>,
    /// Computed deferred tax liability (positive temporary difference × rate).
    pub deferred_tax_liability: Option<MonetaryAmount>,
    /// Valuation allowance against a deferred tax asset (positive = contra).
    pub valuation_allowance: Option<MonetaryAmount>,
    /// Free-text description (e.g. `"Depreciation timing difference"`).
    pub description: String,
}

// ── Tax rate reconciliation ───────────────────────────────────────────────────

/// A single reconciling item between the statutory and effective tax rate.
///
/// Source: ASC 740-10-50-12 — Rate Reconciliation Disclosure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TaxReconciliationItem {
    /// Description (e.g. `"State taxes net of federal benefit"`, `"R&D credits"`).
    pub description: String,
    /// Amount of the reconciling item.
    pub amount: MonetaryAmount,
    /// Rate impact (amount / pre-tax income).
    pub rate_impact: f64,
}

/// Descriptor for the effective-tax-rate reconciliation.
///
/// The factory asserts `TaxRateReconciles` when the reconciliation from the
/// statutory rate to the effective rate is mathematically consistent.
///
/// Source: ASC 740-10-50-12.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TaxRateDescriptor {
    /// Statutory (enacted) federal tax rate.
    pub statutory_rate: f64,
    /// Effective tax rate as reported.
    pub effective_rate: f64,
    /// Reconciling items between statutory and effective rates.
    pub reconciling_items: Vec<TaxReconciliationItem>,
    /// Pre-tax income used as the denominator.
    pub pre_tax_income: MonetaryAmount,
    /// Income tax expense.
    pub income_tax_expense: MonetaryAmount,
}
