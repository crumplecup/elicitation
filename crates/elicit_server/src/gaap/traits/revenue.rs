//! `GaapRevenueFactory` — ASC 606 five-step revenue recognition factory (Role 1a).
//! `GaapRevenueMeta`    — contract backlog reporter (Role 2).

use futures::future::BoxFuture;

use crate::gaap::asc_606::{
    ContractIdentified, PerformanceObligationsIdentified, RevenueRecognizedAtPointInTime,
    RevenueRecognizedOverTime, TransactionPriceAllocated, TransactionPriceDetermined,
};
use crate::gaap::errors::GaapResult;
use crate::gaap::proof_composition::{
    Asc606OverTimeEvidence, Asc606PointInTimeEvidence, Asc606Steps1To3Evidence,
    ContractIdentificationEvidence,
};
use crate::gaap::types::{
    AllocationDescriptor, ContractStatus, FinancialPeriod, PerformanceObligationDescriptor,
    RevenueContractDescriptor, RevenueRecognitionDescriptor, TransactionPriceDescriptor,
};
use elicitation::Established;

// ── Role 1a: ASC 606 five-step factory ───────────────────────────────────────

/// ASC 606 five-step revenue recognition factory.
///
/// The five steps are encoded as a sequential dependency chain: each step
/// returns an `Established<P>` token that is a required input to the next step.
/// A backend cannot skip a step — the type system enforces the order.
///
/// Source: ASC 606-10-05-4 — The Five-Step Model.
pub trait GaapRevenueFactory: Send + Sync {
    // ── Step 1 — Identify the contract ───────────────────────────────────────

    /// Step 1: Identify the customer contract.
    ///
    /// Verifies that the five ASC 606-10-25-1 criteria are met and returns
    /// `ContractIdentified`.
    ///
    /// Source: ASC 606-10-25-1 — Identifying the Contract.
    fn identify_contract(
        &self,
        contract: RevenueContractDescriptor,
    ) -> GaapResult<(RevenueContractDescriptor, Established<ContractIdentified>)>;

    // ── Step 2 — Identify performance obligations ─────────────────────────────

    /// Step 2: Identify performance obligations in the contract.
    ///
    /// Requires `ContractIdentified` (Step 1 token) plus the candidate
    /// obligations. Returns `PerformanceObligationsIdentified`.
    ///
    /// Source: ASC 606-10-25-14 — Identifying Performance Obligations.
    fn identify_performance_obligations(
        &self,
        contract_token: Established<ContractIdentified>,
        obligations: Vec<PerformanceObligationDescriptor>,
    ) -> GaapResult<(
        Vec<PerformanceObligationDescriptor>,
        Established<PerformanceObligationsIdentified>,
    )>;

    // ── Step 3 — Determine the transaction price ──────────────────────────────

    /// Step 3: Determine the transaction price.
    ///
    /// Requires both Step 1 and Step 2 tokens via `ContractIdentificationEvidence`.
    /// Returns `TransactionPriceDetermined`.
    ///
    /// Source: ASC 606-10-32-2 — Determining the Transaction Price.
    fn determine_transaction_price(
        &self,
        evidence: ContractIdentificationEvidence,
        price: TransactionPriceDescriptor,
    ) -> GaapResult<(
        TransactionPriceDescriptor,
        Established<TransactionPriceDetermined>,
    )>;

    // ── Step 4 — Allocate the transaction price ───────────────────────────────

    /// Step 4: Allocate the transaction price to performance obligations.
    ///
    /// Requires Steps 1–3 tokens via `Asc606Steps1To3Evidence`.
    /// Returns `TransactionPriceAllocated`.
    ///
    /// Source: ASC 606-10-32-28 — Allocating the Transaction Price.
    fn allocate_transaction_price(
        &self,
        evidence: Asc606Steps1To3Evidence,
        allocation: AllocationDescriptor,
    ) -> GaapResult<(AllocationDescriptor, Established<TransactionPriceAllocated>)>;

    // ── Step 5a — Recognize revenue at a point in time ───────────────────────

    /// Step 5 (point-in-time): Recognize revenue when control transfers at a
    /// single point in time.
    ///
    /// Requires `Asc606PointInTimeEvidence` (Steps 1–4 + point-in-time indicator).
    /// Returns `RevenueRecognizedAtPointInTime`.
    ///
    /// Source: ASC 606-10-25-30 — Performance Obligations Satisfied at a Point in Time.
    fn recognize_revenue_at_point_in_time(
        &self,
        evidence: Asc606PointInTimeEvidence,
        recognition: RevenueRecognitionDescriptor,
    ) -> GaapResult<(
        RevenueRecognitionDescriptor,
        Established<RevenueRecognizedAtPointInTime>,
    )>;

    // ── Step 5b — Recognize revenue over time ────────────────────────────────

    /// Step 5 (over-time): Recognize revenue as control transfers over a period.
    ///
    /// Requires `Asc606OverTimeEvidence` (Steps 1–4 + over-time criteria).
    /// Returns `RevenueRecognizedOverTime`.
    ///
    /// Source: ASC 606-10-25-27 — Performance Obligations Satisfied Over Time.
    fn recognize_revenue_over_time(
        &self,
        evidence: Asc606OverTimeEvidence,
        recognition: RevenueRecognitionDescriptor,
    ) -> GaapResult<(
        RevenueRecognitionDescriptor,
        Established<RevenueRecognizedOverTime>,
    )>;
}

// ── Role 2: contract backlog reporter ────────────────────────────────────────

/// Orthogonal contract backlog and revenue pipeline reporter.
///
/// Source: ASC 606-10-50-13 — Remaining Performance Obligations.
pub trait GaapRevenueMeta: Send + Sync {
    /// Return all contracts with the given lifecycle status.
    fn contracts_by_status(
        &self,
        status: ContractStatus,
    ) -> BoxFuture<'_, GaapResult<Vec<RevenueContractDescriptor>>>;

    /// Return all contracts with revenue recognized in the given period.
    fn recognized_in_period(
        &self,
        period: FinancialPeriod,
    ) -> BoxFuture<'_, GaapResult<Vec<RevenueRecognitionDescriptor>>>;
}
