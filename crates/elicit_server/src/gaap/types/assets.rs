//! Asset descriptor types — receivables, investments, inventory, PP&E.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{AccountId, MonetaryAmount, PeriodDate};

// ── Receivables ───────────────────────────────────────────────────────────────

/// Descriptor for a trade or non-trade receivable.
///
/// The factory asserts `ReceivableRecordedAtAmortizedCost` when the carrying
/// amount reflects the amortized cost net of the allowance for credit losses.
///
/// Source: ASC 310 — Receivables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ReceivableDescriptor {
    /// Ledger account holding the receivable.
    pub account: AccountId,
    /// Original face (gross) amount.
    pub face_amount: MonetaryAmount,
    /// Allowance for credit losses (positive = contra amount).
    pub allowance_for_credit_loss: Option<MonetaryAmount>,
    /// Net carrying amount (face − allowance).
    pub carrying_amount: MonetaryAmount,
    /// Optional maturity or due date.
    pub maturity_date: Option<PeriodDate>,
}

// ── Investment securities ─────────────────────────────────────────────────────

/// ASC 320 / ASC 321 classification for an investment security.
///
/// Source: ASC 320 — Investments—Debt Securities;
///         ASC 321 — Investments—Equity Securities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum SecurityClassification {
    /// Debt security: intent and ability to sell in the near term (ASC 320).
    TradingDebt,
    /// Debt security: available-for-sale (ASC 320).
    AvailableForSale,
    /// Debt security: held-to-maturity (ASC 320).
    HeldToMaturity,
    /// Equity security: measured at fair value through net income (ASC 321).
    EquityFairValueNi,
    /// Equity security: equity method investment (ASC 323).
    EquityMethod,
}

/// Descriptor for an investment security.
///
/// The factory asserts `DebtSecurityClassified` or `EquitySecurityAtFairValue`
/// based on the classification.
///
/// Source: ASC 320, ASC 321, ASC 323.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SecurityDescriptor {
    /// Ledger account for this security position.
    pub account: AccountId,
    /// Classification.
    pub classification: SecurityClassification,
    /// Original cost (amortized cost for debt securities, cost for equity).
    pub cost: MonetaryAmount,
    /// Current fair value.
    pub fair_value: MonetaryAmount,
    /// Unrealized gain/(loss) = fair_value − cost.
    pub unrealized_gain_loss: MonetaryAmount,
}

// ── Inventory ─────────────────────────────────────────────────────────────────

/// Cost flow assumption for inventory measurement.
///
/// Source: ASC 330 — Inventory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CostFlowMethod {
    /// First-in, first-out.
    Fifo,
    /// Last-in, first-out.
    Lifo,
    /// Weighted-average cost.
    WeightedAverage,
    /// Specific identification.
    SpecificIdentification,
}

/// Descriptor for an inventory balance.
///
/// The factory asserts `InventoryAtLowerOfCostOrNrv` when
/// carrying_amount ≤ min(cost, net_realizable_value).
///
/// Source: ASC 330-10-35 — Subsequent Measurement of Inventory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct InventoryDescriptor {
    /// Ledger account for inventory.
    pub account: AccountId,
    /// Historical cost.
    pub cost: MonetaryAmount,
    /// Net realizable value (estimated selling price less completion/selling costs).
    pub net_realizable_value: MonetaryAmount,
    /// Carrying amount (lower of cost or NRV).
    pub carrying_amount: MonetaryAmount,
    /// Cost flow assumption applied.
    pub cost_flow_method: CostFlowMethod,
    /// LIFO reserve (difference between FIFO and LIFO cost), if LIFO.
    pub lifo_reserve: Option<MonetaryAmount>,
}

// ── Property, plant and equipment ─────────────────────────────────────────────

/// Depreciation method applied to a PP&E asset.
///
/// Source: ASC 360-10-35 — Depreciation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum DepreciationMethod {
    /// Equal charge each period.
    StraightLine,
    /// Accelerated: fixed rate × declining book value.
    DecliningBalance,
    /// Accelerated: sum-of-years-digits formula.
    SumOfYearsDigits,
    /// Activity-based: charge per unit produced.
    UnitsOfProduction,
}

/// Descriptor for a property, plant, and equipment asset.
///
/// The factory asserts `PpeCarriedAtCost` when cost is the basis and
/// `DepreciationAccumulatesCorrectly` when the accumulated depreciation
/// rollforward is consistent.
///
/// Source: ASC 360 — Property, Plant, and Equipment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PpeDescriptor {
    /// Ledger account for this asset.
    pub account: AccountId,
    /// Historical cost at acquisition.
    pub cost: MonetaryAmount,
    /// Accumulated depreciation to date.
    pub accumulated_depreciation: MonetaryAmount,
    /// Net carrying amount (cost − accumulated depreciation).
    pub carrying_amount: MonetaryAmount,
    /// Estimated useful life in years.
    pub useful_life_years: u32,
    /// Estimated salvage value at end of useful life.
    pub salvage_value: MonetaryAmount,
    /// Depreciation method.
    pub depreciation_method: DepreciationMethod,
    /// Date placed in service (ISO 8601).
    pub placed_in_service_date: String,
}
