//! ASC 606 revenue recognition descriptor types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::gaap::types::{MonetaryAmount, PeriodDate};

// ── Contract ──────────────────────────────────────────────────────────────────

/// Raw input data describing a customer contract for ASC 606 Step 1.
///
/// The factory asserts `ContractIdentified` when all five ASC 606-10-25-1
/// criteria are satisfied.
///
/// Source: ASC 606-10-25-1 — Identifying the Contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RevenueContractDescriptor {
    /// Unique contract reference.
    pub contract_id: String,
    /// Customer identifier.
    pub customer_id: String,
    /// ISO 8601 date the parties approved and committed to the contract.
    pub contract_date: PeriodDate,
    /// Total stated consideration.
    pub stated_consideration: MonetaryAmount,
    /// Free-text description of the arrangement.
    pub description: String,
    /// Whether collectibility of the consideration is assessed as probable.
    pub collectibility_probable: bool,
    /// Whether the contract has commercial substance.
    pub commercial_substance: bool,
}

// ── Performance obligation ────────────────────────────────────────────────────

/// Descriptor for a single identified performance obligation within a contract.
///
/// The factory asserts `PerformanceObligationsIdentified` when all obligations
/// in a contract are enumerated and each passes the distinct-good-or-service
/// test.
///
/// Source: ASC 606-10-25-14 — Identifying Performance Obligations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PerformanceObligationDescriptor {
    /// Unique obligation reference within the contract.
    pub obligation_id: String,
    /// Parent contract reference.
    pub contract_id: String,
    /// Free-text description of the promised good or service.
    pub description: String,
    /// Whether the good or service is distinct (passes ASC 606-10-25-19).
    pub is_distinct: bool,
    /// Standalone selling price for this obligation, used in Step 4 allocation.
    pub standalone_selling_price: MonetaryAmount,
}

// ── Transaction price ─────────────────────────────────────────────────────────

/// Descriptor for the determined transaction price (ASC 606 Step 3).
///
/// The factory asserts `TransactionPriceDetermined` when the components sum
/// correctly and any variable consideration is constrained per ASC 606-10-32-11.
///
/// Source: ASC 606-10-32-2 — Determining the Transaction Price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TransactionPriceDescriptor {
    /// Parent contract reference.
    pub contract_id: String,
    /// Fixed portion of the consideration.
    pub fixed_consideration: MonetaryAmount,
    /// Variable consideration estimate (after constraint), if any.
    pub variable_consideration: Option<MonetaryAmount>,
    /// Significant financing component adjustment, if any.
    pub financing_component: Option<MonetaryAmount>,
    /// Non-cash consideration at fair value, if any.
    pub non_cash_consideration: Option<MonetaryAmount>,
    /// Consideration payable to customer deduction, if any.
    pub consideration_payable_to_customer: Option<MonetaryAmount>,
    /// Final determined transaction price (sum of above components).
    pub total_transaction_price: MonetaryAmount,
}

// ── Allocation ────────────────────────────────────────────────────────────────

/// Descriptor for the allocated transaction price across performance obligations
/// (ASC 606 Step 4).
///
/// The factory asserts `TransactionPriceAllocated` when the allocated amounts
/// sum to the total transaction price and each allocation reflects relative SSP.
///
/// Source: ASC 606-10-32-28 — Allocating the Transaction Price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AllocationDescriptor {
    /// Parent contract reference.
    pub contract_id: String,
    /// Obligation IDs in the same order as `allocated_prices`.
    pub obligation_ids: Vec<String>,
    /// Allocated price for each obligation (parallel to `obligation_ids`).
    pub allocated_prices: Vec<MonetaryAmount>,
    /// Total should equal the transaction price.
    pub total_allocated: MonetaryAmount,
}

// ── Revenue recognition receipt ────────────────────────────────────────────────

/// Descriptor for a revenue recognition event (ASC 606 Step 5).
///
/// The factory asserts either `RevenueRecognizedAtPointInTime` or
/// `RevenueRecognizedOverTime` depending on which Step 5 path applies.
///
/// Source: ASC 606-10-25-23 — Satisfying Performance Obligations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RevenueRecognitionDescriptor {
    /// Parent contract reference.
    pub contract_id: String,
    /// Performance obligation being satisfied.
    pub obligation_id: String,
    /// Amount of revenue recognized.
    pub amount: MonetaryAmount,
    /// ISO 8601 date or period on which revenue is recognized.
    pub recognition_date: PeriodDate,
    /// True when control transfers at a point in time; false for over-time.
    pub is_point_in_time: bool,
    /// For over-time recognition: percentage of completion (0.0–1.0).
    pub completion_percentage: Option<f64>,
}

// ── Contract lifecycle status ──────────────────────────────────────────────────

/// Lifecycle state of a customer contract.
///
/// Source: ASC 606 contract asset/liability model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ContractStatus {
    /// Contract is active; performance obligations remain unsatisfied.
    Active,
    /// All performance obligations have been fully satisfied.
    Completed,
    /// Contract was terminated before all obligations were satisfied.
    Cancelled,
    /// Contract has been modified per ASC 606-10-25-18.
    Modified,
}
