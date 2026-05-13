//! Core GAAP principle propositions.
//!
//! Each proposition is a zero-cost proof marker establishing compliance with a
//! specific Generally Accepted Accounting Principle. All ASC references are to
//! the FASB Accounting Standards Codification.
//!
//! Source: FASB Accounting Standards Codification — <https://asc.fasb.org/>
// ── Core assumptions (ASC 105) ────────────────────────────────────────────

/// Every transaction records equal and opposite entries; Assets = Liabilities + Equity.
///
/// Pre-ASC foundational requirement. Required by FASB for GAAP compliance.
#[derive(elicitation::Prop)]
pub struct DoubleEntryBookkeeping;

/// Transactions are recorded when they occur, not when cash changes hands.
///
/// Source: ASC 606-10-25-1 — Revenue Recognition
#[derive(elicitation::Prop)]
pub struct AccrualBasis;

/// All transactions are measured in a single stable monetary unit (e.g., USD cents).
///
/// Source: ASC 105 — Generally Accepted Accounting Principles
#[derive(elicitation::Prop)]
pub struct MonetaryUnitAssumption;

/// The reporting entity is separate and distinct from its owners.
///
/// Source: ASC 105 — Generally Accepted Accounting Principles
#[derive(elicitation::Prop)]
pub struct EconomicEntityAssumption;

/// Financial statements are prepared for a discrete time period.
///
/// Source: ASC 105, ASC 270 — Interim Reporting
#[derive(elicitation::Prop)]
pub struct TimePeriodAssumption;

/// Financial statements are prepared assuming the entity will continue operating.
///
/// Source: ASC 105, ASC 205-40 — Going Concern
#[derive(elicitation::Prop)]
pub struct GoingConcernAssumption;

// ── Recognition and measurement principles ────────────────────────────────

/// Revenue and associated expenses are recognized in the same accounting period.
///
/// Source: ASC 606-10-25-23 — Revenue Recognition
#[derive(elicitation::Prop)]
pub struct MatchingPrinciple;

/// Assets are recorded at original acquisition cost, not current market value.
///
/// Source: ASC 820-10 — Fair Value Measurement (defines exceptions)
#[derive(elicitation::Prop)]
pub struct HistoricalCostPrinciple;

/// All information that could affect financial decisions is disclosed.
///
/// Source: ASC 235 — Notes to Financial Statements
#[derive(elicitation::Prop)]
pub struct FullDisclosurePrinciple;

/// Revenue is recognized when earned and realizable.
///
/// Source: ASC 606-10-25 — Revenue Recognition
#[derive(elicitation::Prop)]
pub struct RevenueRecognitionPrinciple;

// ── Qualitative principles ────────────────────────────────────────────────

/// When uncertain, choose options resulting in lower income or smaller asset values.
///
/// Source: ASC 250, ASC 450 — Contingencies
#[derive(elicitation::Prop)]
pub struct ConservatismPrinciple;

/// Significant items affecting financial decisions are reported.
///
/// Source: ASC 250 — Accounting Changes and Error Corrections; SEC SAB 99
#[derive(elicitation::Prop)]
pub struct MaterialityPrinciple;

/// Accounting methods are applied consistently across reporting periods.
///
/// Source: ASC 250-10-45 — Accounting Changes and Error Corrections
#[derive(elicitation::Prop)]
pub struct ConsistencyPrinciple;

/// Financial information is free from bias and represents economic reality.
///
/// Source: FASB Concepts Statement No. 8 — Conceptual Framework
#[derive(elicitation::Prop)]
pub struct NeutralityPrinciple;

/// Economic substance governs accounting treatment, not legal form.
///
/// Source: FASB Concepts Statement No. 8 — Conceptual Framework
#[derive(elicitation::Prop)]
pub struct SubstanceOverFormPrinciple;
