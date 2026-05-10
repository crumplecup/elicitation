//! ASC 400 series — Liabilities.
//!
//! Covers ASC 405–480: general liabilities, asset retirement and environmental
//! obligations, exit activities, deferred revenue, contingencies, guarantees,
//! debt, and equity classification.
//!
//! Source: FASB ASC 400 series — <https://asc.fasb.org/>
// ── ASC 405 — Liabilities ─────────────────────────────────────────────────

/// Liability is recognized when the obligation is probable and the amount is measurable.
///
/// Source: ASC 405-20-40 — Derecognition of Liabilities; ASC 450-20-25 — Loss Contingencies
#[derive(elicitation::Prop)]
pub struct LiabilityRecognitionCriteriaMet;

/// Trade accounts payable are accrued through period end.
///
/// Source: ASC 405-20-25 — Accounts Payable Recognition
#[derive(elicitation::Prop)]
pub struct TradeAccountsPayableAccrued;

// ── ASC 410 — Asset Retirement and Environmental Obligations ─────────────

/// Asset retirement obligation is recognized at fair value when the legal obligation arises.
///
/// Source: ASC 410-20-25-1 — Asset Retirement Obligations Recognition
#[derive(elicitation::Prop)]
pub struct AssetRetirementObligationRecognized;

/// ARO accretion expense is recognized each period to reflect the passage of time.
///
/// Source: ASC 410-20-35-2 — ARO Subsequent Measurement
#[derive(elicitation::Prop)]
pub struct AroAccretionExpenseRecognized;

/// Environmental remediation liability is recognized when probable and estimable.
///
/// Source: ASC 410-30-25-1 — Environmental Obligations
#[derive(elicitation::Prop)]
pub struct EnvironmentalLiabilityRecognized;

// ── ASC 420 — Exit or Disposal Cost Obligations ───────────────────────────

/// Exit and restructuring costs are recognized when the liability is incurred, not when the plan is approved.
///
/// Source: ASC 420-10-25-1 — Exit or Disposal Activities
#[derive(elicitation::Prop)]
pub struct ExitCostRecognizedWhenLiabilityIncurred;

/// Employee termination benefits under a one-time plan are recognized when the plan is communicated.
///
/// Source: ASC 420-10-25-10 — One-Time Termination Benefits
#[derive(elicitation::Prop)]
pub struct SeveranceLiabilityMeasured;

// ── ASC 430 — Deferred Revenue ────────────────────────────────────────────

/// Advance payments from customers are deferred until the associated performance obligation is satisfied.
///
/// Source: ASC 430-10-25 — Deferred Revenue; ASC 606-10-45-2 — Contract Liabilities
#[derive(elicitation::Prop)]
pub struct DeferredRevenueRecordedUntilEarned;

// ── ASC 450 — Contingencies ───────────────────────────────────────────────

/// Loss contingency is evaluated for accrual at each reporting date.
///
/// Source: ASC 450-20-25-1 — Loss Contingency Evaluation
#[derive(elicitation::Prop)]
pub struct LossContingencyAssessed;

/// Probable and reasonably estimable loss contingency is accrued.
///
/// Source: ASC 450-20-25-2 — Accrual of Loss Contingencies
#[derive(elicitation::Prop)]
pub struct ProbableLossAccrued;

/// Reasonably possible loss contingency is disclosed with a range or best estimate.
///
/// Source: ASC 450-20-50-3 — Disclosure of Reasonably Possible Losses
#[derive(elicitation::Prop)]
pub struct ReasonablyPossibleLossDisclosed;

/// Gain contingency is not recognized in earnings until realized.
///
/// Source: ASC 450-30-25-1 — Gain Contingencies
#[derive(elicitation::Prop)]
pub struct GainContingencyNotRecorded;

/// Contingencies are reassessed for changes in probability or estimate each period.
///
/// Source: ASC 450-20-25-8 — Subsequent Changes to Contingencies
#[derive(elicitation::Prop)]
pub struct ContingentLiabilityReassessed;

// ── ASC 460 — Guarantees ──────────────────────────────────────────────────

/// Guarantee obligation is recognized at fair value at the inception of the guarantee.
///
/// Source: ASC 460-10-25-1 — Guarantees — Recognition
#[derive(elicitation::Prop)]
pub struct GuaranteeObligationRecognized;

/// Guarantee terms, maximum exposure, and carrying value are disclosed in the notes.
///
/// Source: ASC 460-10-50-4 — Guarantees — Disclosure
#[derive(elicitation::Prop)]
pub struct GuaranteeDisclosed;

// ── ASC 470 — Debt ────────────────────────────────────────────────────────

/// Debt is correctly classified as current or noncurrent on the balance sheet.
///
/// Source: ASC 470-10-45 — Debt Classification
#[derive(elicitation::Prop)]
pub struct DebtClassifiedCorrectly;

/// Debt issuance costs are presented as a direct deduction from the carrying value of the debt.
///
/// Source: ASC 835-30-45-1A — Debt Issuance Costs Presentation
#[derive(elicitation::Prop)]
pub struct DebtIssuanceCostsDeferred;

/// The effective interest method is used to amortize debt discount, premium, and issuance costs.
///
/// Source: ASC 835-30-35-2 — Effective Interest Method
#[derive(elicitation::Prop)]
pub struct EffectiveInterestMethodUsed;

/// Debt covenant compliance status is evaluated and disclosed when material.
///
/// Source: ASC 470-10-50-1 — Debt Covenant Disclosures
#[derive(elicitation::Prop)]
pub struct DebtCovenantComplianceDisclosed;

/// Troubled debt restructuring accounting is applied when the creditor grants concessions.
///
/// Source: ASC 470-60-15 — Troubled Debt Restructuring by Debtors
#[derive(elicitation::Prop)]
pub struct DebtorTroubledDebtRestructuringAccounted;

/// Short-term debt expected to be refinanced on a long-term basis is classified as noncurrent when criteria met.
///
/// Source: ASC 470-10-45-14 — Short-Term Debt Refinancing
#[derive(elicitation::Prop)]
pub struct ShortTermDebtRefinancingClassified;

// ── ASC 480 — Equity Classification ──────────────────────────────────────

/// Mandatorily redeemable financial instrument is classified as a liability.
///
/// Source: ASC 480-10-25-14 — Mandatorily Redeemable Instruments
#[derive(elicitation::Prop)]
pub struct MandatorilyRedeemableInstrumentInLiabilities;

/// Freestanding financial instrument is correctly classified as liability or equity.
///
/// Source: ASC 480-10-25 — Equity Classification
#[derive(elicitation::Prop)]
pub struct FreestandingInstrumentClassified;

/// Mandatory redemption features and settlement alternatives are disclosed.
///
/// Source: ASC 480-10-50-2 — Mezzanine Equity Disclosures
#[derive(elicitation::Prop)]
pub struct MandatoryRedemptionDisclosed;
