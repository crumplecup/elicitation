//! Liability descriptor types — payables, contingencies, and debt.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount, PeriodDate};

// ── General liabilities ───────────────────────────────────────────────────────

/// Descriptor for a trade payable or accrued liability.
///
/// The factory asserts `TradeAccountsPayableAccrued` when the obligation is
/// recorded at the invoice amount in the correct period.
///
/// Source: ASC 405 — Liabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct LiabilityDescriptor {
    /// Ledger account holding the liability.
    pub account: AccountId,
    /// Amount owed.
    pub amount: MonetaryAmount,
    /// Optional due date (ISO 8601).
    pub due_date: Option<PeriodDate>,
    /// Free-text description (e.g. `"Accrued wages payable"`).
    pub description: String,
    /// Whether this is a current liability (due within twelve months).
    pub is_current: bool,
}

// ── Contingencies ─────────────────────────────────────────────────────────────

/// Probability assessment for a loss contingency.
///
/// Source: ASC 450-20 — Loss Contingencies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ContingencyProbability {
    /// The future event is likely to occur.
    Probable,
    /// The chance of occurrence is more than remote but less than probable.
    ReasonablyPossible,
    /// The chance of occurrence is slight.
    Remote,
}

/// Descriptor for a contingent liability.
///
/// The factory asserts `LossContingencyAssessed` (always), `ProbableLossAccrued`
/// when the loss is probable and estimable, and `ReasonablyPossibleLossDisclosed`
/// when the loss is reasonably possible.
///
/// Source: ASC 450-20 — Loss Contingencies.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ContingencyDescriptor {
    /// Ledger account to accrue the liability against (if probable and estimable).
    pub account: AccountId,
    /// Estimated range minimum, if estimable.
    pub estimated_minimum: Option<MonetaryAmount>,
    /// Estimated range maximum or best estimate, if estimable.
    pub estimated_maximum: Option<MonetaryAmount>,
    /// Probability assessment as of the balance-sheet date.
    pub probability: ContingencyProbability,
    /// Free-text description (e.g. `"Environmental remediation at Site A"`).
    pub description: String,
}

// ── Debt instruments ──────────────────────────────────────────────────────────

/// Descriptor for a borrowing arrangement (note payable, bond, credit facility).
///
/// The factory asserts `DebtClassifiedCorrectly` when the current/non-current
/// split is correct, `DebtIssuanceCostsDeferred` when issuance costs net the
/// carrying amount, and `EffectiveInterestMethodUsed` when amortization is
/// computed at the original effective rate.
///
/// Source: ASC 470 — Debt; ASC 835-30 — Imputation of Interest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DebtDescriptor {
    /// Ledger account for this debt instrument.
    pub account: AccountId,
    /// Face (principal) amount.
    pub face_amount: MonetaryAmount,
    /// Unamortized debt issuance costs (positive = contra amount).
    pub unamortized_issuance_costs: MonetaryAmount,
    /// Unamortized discount/(premium) (positive = discount).
    pub unamortized_discount_premium: MonetaryAmount,
    /// Net carrying amount (face − unamortized discount − unamortized costs + unamortized premium).
    pub carrying_amount: MonetaryAmount,
    /// Coupon (stated) interest rate.
    pub stated_rate: f64,
    /// Effective (yield) interest rate at issuance.
    pub effective_rate: f64,
    /// Contractual maturity date (ISO 8601).
    pub maturity_date: PeriodDate,
    /// Whether the balance is classified as current.
    pub is_current: bool,
}
