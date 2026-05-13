//! Disclosure requirement propositions.
//!
//! Each proposition represents a specific footnote or supplemental disclosure
//! that GAAP requires when applicable. These complement the recognition and
//! measurement props in the ASC series modules.
//!
//! Source: FASB ASC §50 disclosure subsections across the codification
// ── General and cross-cutting disclosures ─────────────────────────────────

/// Summary of significant accounting policies note is included.
///
/// Source: ASC 235-10-50-1 — Notes to Financial Statements
#[derive(elicitation::Prop)]
pub struct SignificantAccountingPoliciesDisclosed;

/// All related-party transactions are identified and disclosed.
///
/// Source: ASC 850-10-50-1 — Related Party Disclosures
#[derive(elicitation::Prop)]
pub struct RelatedPartyTransactionsDisclosed;

/// Subsequent events are reviewed through the issuance date and material events disclosed.
///
/// Source: ASC 855-10-50-2 — Subsequent Events Disclosure
#[derive(elicitation::Prop)]
pub struct SubsequentEventsDisclosed;

/// Commitments and contingencies are disclosed in the balance sheet caption and notes.
///
/// Source: ASC 440-10-50 — Commitments Disclosure; ASC 450-20-50 — Contingencies
#[derive(elicitation::Prop)]
pub struct CommitmentsAndContingenciesDisclosed;

/// Recently issued accounting standards and their expected impact are disclosed.
///
/// Source: ASC 250-10-50-1 — New Accounting Standards
#[derive(elicitation::Prop)]
pub struct NewAccountingStandardsDisclosed;

/// Concentrations of credit risk are disclosed for all significant counterparties.
///
/// Source: ASC 825-10-50-21 — Concentrations of Credit Risk
#[derive(elicitation::Prop)]
pub struct ConcentrationRisksDisclosed;

/// Liquidity risk, available funding sources, and any going concern indicators are disclosed.
///
/// Source: ASC 205-40-50 — Going Concern; ASC 275-10-50 — Liquidity Risks
#[derive(elicitation::Prop)]
pub struct LiquidityRisksDisclosed;

// ── Revenue and contract disclosures ──────────────────────────────────────

/// Revenue recognition policy, including the nature of performance obligations, is disclosed.
///
/// Source: ASC 606-10-50-1 — Revenue Recognition Policy
#[derive(elicitation::Prop)]
pub struct RevenueRecognitionPolicyNote;

/// Revenue is disaggregated into categories that depict economic factors.
///
/// Source: ASC 606-10-50-5 — Disaggregation of Revenue
#[derive(elicitation::Prop)]
pub struct RevenueDisaggregationNote;

/// Contract asset and liability opening and closing balances are disclosed.
///
/// Source: ASC 606-10-50-8 — Contract Balances
#[derive(elicitation::Prop)]
pub struct ContractBalanceNote;

/// Remaining performance obligations and expected recognition timing are disclosed.
///
/// Source: ASC 606-10-50-13 — Remaining Performance Obligations
#[derive(elicitation::Prop)]
pub struct RemainingPerformanceObligationNote;

// ── Asset and investment disclosures ──────────────────────────────────────

/// Goodwill rollforward by reportable segment is disclosed.
///
/// Source: ASC 350-20-50-1 — Goodwill Rollforward
#[derive(elicitation::Prop)]
pub struct GoodwillRollforwardDisclosed;

/// Intangible assets subject to amortization and indefinite-lived are separately disclosed.
///
/// Source: ASC 350-30-50-1 — Intangible Assets Disclosure
#[derive(elicitation::Prop)]
pub struct IntangibleAssetsDisclosed;

/// Depreciation method and range of useful lives for each PP&E class are disclosed.
///
/// Source: ASC 360-10-50-1 — PP&E Disclosure
#[derive(elicitation::Prop)]
pub struct PpeDepreciationPolicyDisclosed;

// ── Debt and equity disclosures ───────────────────────────────────────────

/// Debt covenant terms, required ratios, and compliance status are disclosed.
///
/// Source: ASC 470-10-50-1 — Debt Covenants
#[derive(elicitation::Prop)]
pub struct DebtCovenantsDisclosed;

/// Aggregate annual maturities of long-term debt for the next five years are disclosed.
///
/// Source: ASC 470-10-50-1 — Debt Maturity Schedule
#[derive(elicitation::Prop)]
pub struct DebtMaturityScheduleDisclosed;

/// Preferred stock terms (liquidation preference, dividend rate, conversion rights) are disclosed.
///
/// Source: ASC 505-10-50-4 — Preferred Stock Disclosures
#[derive(elicitation::Prop)]
pub struct PreferredStockDisclosures;

// ── Income tax disclosures ────────────────────────────────────────────────

/// Deferred tax asset and liability components are disclosed.
///
/// Source: ASC 740-10-50-2 — Deferred Tax Components
#[derive(elicitation::Prop)]
pub struct DeferredTaxComponentsDisclosed;

/// Effective tax rate reconciliation from statutory rate to reported rate is disclosed.
///
/// Source: ASC 740-10-50-12 — Effective Tax Rate Reconciliation
#[derive(elicitation::Prop)]
pub struct EffectiveTaxRateReconciliationDisclosed;

/// Unrecognized tax benefits and the potential impact on the effective tax rate are disclosed.
///
/// Source: ASC 740-10-50-15 — Uncertain Tax Positions Disclosure
#[derive(elicitation::Prop)]
pub struct UncertainTaxBenefitsDisclosed;

/// Material tax jurisdictions subject to examination are disclosed.
///
/// Source: ASC 740-10-50-15 — Tax Jurisdictions
#[derive(elicitation::Prop)]
pub struct TaxJurisdictionsDisclosed;

// ── Pension and post-retirement benefit disclosures ───────────────────────

/// Pension and OPEB benefit obligations and plan assets are disclosed.
///
/// Source: ASC 715-20-50 — Defined Benefit Plan Disclosures
#[derive(elicitation::Prop)]
pub struct PensionObligationDisclosed;

/// Net periodic benefit cost components are disclosed.
///
/// Source: ASC 715-20-50-1(h) — Net Periodic Benefit Cost
#[derive(elicitation::Prop)]
pub struct NetPeriodicBenefitCostDisclosed;

// ── Derivative and hedging disclosures ────────────────────────────────────

/// Derivative instruments, hedging strategy, and fair value amounts are disclosed.
///
/// Source: ASC 815-10-50-1 — Derivatives and Hedging Disclosures
#[derive(elicitation::Prop)]
pub struct DerivativeAndHedgingDisclosed;

/// Tabular disclosure of derivatives' fair value and gain/loss by category is included.
///
/// Source: ASC 815-10-50-1A — Quantitative Derivative Disclosures
#[derive(elicitation::Prop)]
pub struct DerivativeFairValueTableDisclosed;

// ── Lease disclosures ─────────────────────────────────────────────────────

/// Lease supplemental quantitative disclosures (cost, cash paid, ROU assets) are included.
///
/// Source: ASC 842-20-50-4 — Quantitative Lease Disclosures
#[derive(elicitation::Prop)]
pub struct LeaseQuantitativeDisclosed;

/// Future undiscounted lease payments reconciled to the lease liability are disclosed.
///
/// Source: ASC 842-20-50-6 — Maturity Analysis of Lease Liabilities
#[derive(elicitation::Prop)]
pub struct LeaseLiabilityMaturityDisclosed;

// ── Stock compensation disclosures ────────────────────────────────────────

/// Stock compensation plan description, assumptions, and expense are disclosed.
///
/// Source: ASC 718-10-50-1 — Stock Compensation Disclosures
#[derive(elicitation::Prop)]
pub struct StockCompensationPlanDisclosed;

/// Unrecognized compensation cost and expected recognition period are disclosed.
///
/// Source: ASC 718-10-50-2(i) — Unrecognized Compensation Cost
#[derive(elicitation::Prop)]
pub struct UnrecognizedCompensationCostDisclosed;

// ── Segment disclosures ───────────────────────────────────────────────────

/// Revenue, profit or loss, and total assets by reportable segment are disclosed.
///
/// Source: ASC 280-10-50-22 — Segment Information
#[derive(elicitation::Prop)]
pub struct SegmentInformationDisclosed;

/// Entity-wide disclosures (products, geographic areas, major customers) are included.
///
/// Source: ASC 280-10-50-38 — Entity-Wide Disclosures
#[derive(elicitation::Prop)]
pub struct EntityWideDisclosuresIncluded;

// ── Fair value disclosures ────────────────────────────────────────────────

/// Valuation techniques and inputs used for recurring and nonrecurring FV measurements are disclosed.
///
/// Source: ASC 820-10-50-2 — Fair Value Measurement Disclosures
#[derive(elicitation::Prop)]
pub struct FairValueMeasurementMethodsDisclosed;

/// Rollforward of Level 3 fair value measurements is disclosed.
///
/// Source: ASC 820-10-50-2(d) — Level 3 Rollforward
#[derive(elicitation::Prop)]
pub struct Level3FairValueRollforwardDisclosed;

// ── Interim disclosures ───────────────────────────────────────────────────

/// Material changes from the prior annual report are disclosed in interim financial statements.
///
/// Source: ASC 270-10-50 — Interim Disclosures
#[derive(elicitation::Prop)]
pub struct InterimSignificantChangesDisclosed;
