//! ASC 606 — Revenue from Contracts with Customers.
//!
//! Full coverage of the five-step revenue recognition model and its supporting
//! criteria, plus disclosure requirements.
//!
//! Source: FASB ASC 606 — <https://asc.fasb.org/606>
// ── Step 1: Identify the Contract ────────────────────────────────────────

/// A contract has been identified that meets all five criteria of ASC 606-10-25-1.
///
/// Source: ASC 606-10-25-1 — Identifying the Contract
#[derive(elicitation::Prop)]
pub struct ContractIdentified;

/// The contract has commercial substance, approved by parties, and collectibility is probable.
///
/// Source: ASC 606-10-25-1(a)–(e) — Contract Criteria
#[derive(elicitation::Prop)]
pub struct ContractCriteriaMet;

/// Multiple contracts with the same customer have been assessed for combination.
///
/// Source: ASC 606-10-25-9 — Combining Contracts
#[derive(elicitation::Prop)]
pub struct ContractCombinationAssessed;

/// A contract modification has been assessed and accounted for (new contract, cumulative catch-up, or prospective).
///
/// Source: ASC 606-10-25-10 — Contract Modifications
#[derive(elicitation::Prop)]
pub struct ContractModificationAccountedFor;

/// Collectibility of consideration is probable at contract inception.
///
/// Source: ASC 606-10-25-1(e) — Collectibility Threshold
#[derive(elicitation::Prop)]
pub struct CollectibilityProbable;

// ── Step 2: Identify Performance Obligations ──────────────────────────────

/// All distinct performance obligations in the contract are identified.
///
/// Source: ASC 606-10-25-14 — Identifying Performance Obligations
#[derive(elicitation::Prop)]
pub struct PerformanceObligationsIdentified;

/// Each performance obligation represents a distinct good or service.
///
/// Source: ASC 606-10-25-19 — Distinct Goods or Services
#[derive(elicitation::Prop)]
pub struct DistinctGoodOrServiceDetermined;

/// A series of distinct goods or services is accounted for as a single performance obligation when appropriate.
///
/// Source: ASC 606-10-25-15 — Series of Distinct Goods or Services
#[derive(elicitation::Prop)]
pub struct SeriesPerformanceObligationAccountedFor;

/// Principal vs. agent determination is made for each performance obligation.
///
/// Source: ASC 606-10-55-36 — Principal vs. Agent Considerations
#[derive(elicitation::Prop)]
pub struct PrincipalVsAgentDetermined;

// ── Step 3: Determine the Transaction Price ───────────────────────────────

/// The transaction price is determined for the contract.
///
/// Source: ASC 606-10-32-2 — Determining the Transaction Price
#[derive(elicitation::Prop)]
pub struct TransactionPriceDetermined;

/// Variable consideration is estimated using the expected value or most likely amount method.
///
/// Source: ASC 606-10-32-8 — Variable Consideration
#[derive(elicitation::Prop)]
pub struct VariableConsiderationEstimated;

/// Variable consideration is constrained to the amount unlikely to result in a significant revenue reversal.
///
/// Source: ASC 606-10-32-11 — Constraint on Variable Consideration
#[derive(elicitation::Prop)]
pub struct VariableConsiderationConstraintApplied;

/// Significant financing component is assessed and interest income/expense adjusted when material.
///
/// Source: ASC 606-10-32-15 — Significant Financing Component
#[derive(elicitation::Prop)]
pub struct SignificantFinancingComponentAssessed;

/// Noncash consideration is measured at fair value at contract inception.
///
/// Source: ASC 606-10-32-21 — Noncash Consideration
#[derive(elicitation::Prop)]
pub struct NonCashConsiderationMeasured;

/// Consideration payable to the customer is accounted for as a reduction of the transaction price.
///
/// Source: ASC 606-10-32-25 — Consideration Payable to a Customer
#[derive(elicitation::Prop)]
pub struct ConsiderationPayableToCustomerDeducted;

// ── Step 4: Allocate the Transaction Price ────────────────────────────────

/// The transaction price is allocated to each performance obligation based on relative standalone selling prices.
///
/// Source: ASC 606-10-32-28 — Allocating the Transaction Price
#[derive(elicitation::Prop)]
pub struct TransactionPriceAllocated;

/// Standalone selling price is determined for each distinct performance obligation.
///
/// Source: ASC 606-10-32-31 — Standalone Selling Price
#[derive(elicitation::Prop)]
pub struct StandaloneSellingPriceDetermined;

/// The residual approach is used only when the standalone selling price is highly variable or uncertain.
///
/// Source: ASC 606-10-32-34 — Residual Approach
#[derive(elicitation::Prop)]
pub struct ResidualApproachApplied;

/// Transaction price allocation is adjusted when a contract modification occurs.
///
/// Source: ASC 606-10-25-12 — Allocation Adjustments for Modifications
#[derive(elicitation::Prop)]
pub struct AllocationAdjustedForModification;

// ── Step 5: Recognize Revenue ─────────────────────────────────────────────

/// Revenue is recognized at a point in time when control of the good or service transfers.
///
/// Source: ASC 606-10-25-30 — Point-in-Time Recognition
#[derive(elicitation::Prop)]
pub struct RevenueRecognizedAtPointInTime;

/// Revenue is recognized over time because one of the three over-time criteria is met.
///
/// Source: ASC 606-10-25-27 — Over-Time Recognition Criteria
#[derive(elicitation::Prop)]
pub struct RevenueRecognizedOverTime;

/// At least one of the three over-time recognition criteria is satisfied.
///
/// Source: ASC 606-10-25-27(a)–(c) — Over-Time Criteria
#[derive(elicitation::Prop)]
pub struct OverTimeCriteriaMet;

/// An input or output method is selected and applied consistently to measure progress.
///
/// Source: ASC 606-10-55-16 — Methods to Measure Progress
#[derive(elicitation::Prop)]
pub struct ProgressMeasurementMethodSelected;

/// Contract asset (unbilled revenue) or contract liability (deferred revenue) is recorded correctly.
///
/// Source: ASC 606-10-45 — Contract Assets and Liabilities Presentation
#[derive(elicitation::Prop)]
pub struct ContractBalanceRecordedCorrectly;

// ── Disclosure requirements ───────────────────────────────────────────────

/// Revenue is disaggregated into categories depicting how economic factors affect revenue.
///
/// Source: ASC 606-10-50-5 — Disaggregation of Revenue
#[derive(elicitation::Prop)]
pub struct RevenueDisaggregated;

/// Opening and closing balances of contract assets and liabilities are disclosed.
///
/// Source: ASC 606-10-50-8 — Contract Balances Disclosure
#[derive(elicitation::Prop)]
pub struct ContractBalancesDisclosed;

/// Remaining performance obligations and expected timing of recognition are disclosed.
///
/// Source: ASC 606-10-50-13 — Remaining Performance Obligations
#[derive(elicitation::Prop)]
pub struct RemainingPerformanceObligationsDisclosed;

/// Revenue recognition policies and judgments are disclosed.
///
/// Source: ASC 606-10-50-1 — Disclosure Objective
#[derive(elicitation::Prop)]
pub struct RevenueRecognitionPolicyDisclosed;

/// Revenue recognized from satisfying performance obligations in prior periods is disclosed.
///
/// Source: ASC 606-10-50-12 — Revenue from Prior-Period POs
#[derive(elicitation::Prop)]
pub struct PriorPeriodPerformanceObligationRevenueDisclosed;
