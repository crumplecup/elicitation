//! `GaapLiabilityFactory` — liability recognition factory (Role 1a).

use crate::gaap::asc_400::{DebtClassifiedCorrectly, LossContingencyAssessed, ProbableLossAccrued};
use crate::gaap::errors::GaapResult;
use crate::gaap::types::{ContingencyDescriptor, DebtDescriptor, LiabilityDescriptor};
use elicitation::Established;

// ── Role 1a: liability recognition factory ────────────────────────────────────

/// Factory for recognizing and measuring liabilities.
///
/// Source: ASC 405 — Liabilities; ASC 450 — Contingencies; ASC 470 — Debt.
pub trait GaapLiabilityFactory: Send + Sync {
    // ── Trade payables and accrued liabilities ────────────────────────────────

    /// Record a trade payable or accrued liability.
    ///
    /// Returns `TradeAccountsPayableAccrued` from `asc_400`.
    ///
    /// Source: ASC 405-20-40 — Extinguishment of Liabilities.
    fn record_liability(&self, liability: LiabilityDescriptor) -> GaapResult<LiabilityDescriptor>;

    // ── Contingencies ─────────────────────────────────────────────────────────

    /// Assess and (if probable + estimable) accrue a loss contingency.
    ///
    /// Returns `LossContingencyAssessed`.  If the contingency is probable and
    /// the estimate is available, also returns `ProbableLossAccrued`.
    ///
    /// Source: ASC 450-20-25-2.
    fn assess_loss_contingency(
        &self,
        contingency: ContingencyDescriptor,
    ) -> GaapResult<(
        ContingencyDescriptor,
        Established<LossContingencyAssessed>,
        Option<Established<ProbableLossAccrued>>,
    )>;

    // ── Debt ──────────────────────────────────────────────────────────────────

    /// Recognize a debt instrument and classify as current or non-current.
    ///
    /// Returns `DebtClassifiedCorrectly`.
    ///
    /// Source: ASC 470-10-45 — Classification; ASC 835-30 — Imputation of Interest.
    fn recognize_debt(
        &self,
        debt: DebtDescriptor,
    ) -> GaapResult<(DebtDescriptor, Established<DebtClassifiedCorrectly>)>;
}
