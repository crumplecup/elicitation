//! ASC 500 series — Equity.
//!
//! Covers ASC 505: stockholders' equity presentation, treasury stock, stock splits,
//! retained earnings, OCI, noncontrolling interests, dividends, and preferred stock.
//!
//! Source: FASB ASC 505 — <https://asc.fasb.org/>

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // ── ASC 505 — Equity ──────────────────────────────────────────────────────

    /// Stockholders' equity section is complete and clearly presented.
    ///
    /// Source: ASC 505-10-45 — Stockholders' Equity Presentation
    pub struct StockholdersEquityPresented;

    /// Par value, shares authorized, issued, and outstanding are disclosed for each class.
    ///
    /// Source: ASC 505-10-50-2 — Capital Structure Disclosures
    pub struct CommonStockParValueDisclosed;

    /// Treasury stock is accounted for using the cost method or par value method, disclosed consistently.
    ///
    /// Source: ASC 505-30-30 — Treasury Stock
    pub struct TreasuryStockAccountedFor;

    /// Stock split or stock dividend is accounted for and all per-share amounts are retroactively adjusted.
    ///
    /// Source: ASC 505-20-25 — Stock Dividends and Stock Splits
    pub struct StockSplitAccountedFor;

    /// Retained earnings are reconciled from the beginning to the end of the period.
    ///
    /// Source: ASC 505-10-50-3 — Retained Earnings Rollforward
    pub struct RetainedEarningsReconciled;

    /// Accumulated other comprehensive income (AOCI) is presented as a separate component of equity.
    ///
    /// Source: ASC 220-10-45-14 — AOCI in Stockholders' Equity
    pub struct OciAccumulatedSeparately;

    /// Noncontrolling interest is presented within equity (not as a liability or mezzanine).
    ///
    /// Source: ASC 810-10-45-16 — Noncontrolling Interest in Consolidated Balance Sheet
    pub struct NoncontrollingInterestPresented;

    /// Dividends declared are recorded in the period of declaration.
    ///
    /// Source: ASC 505-10-25-1 — Dividends
    pub struct DividendsDeclaredRecorded;

    /// Preferred stock liquidation preference, dividend rate, and conversion rights are disclosed.
    ///
    /// Source: ASC 505-10-50-4 — Preferred Stock Terms
    pub struct PreferredStockTermsDisclosed;

    /// Subscription receivable from stockholders is presented as a contra-equity item, not an asset.
    ///
    /// Source: ASC 505-10-45-2 — Stock Subscriptions Receivable
    pub struct StockSubscriptionReceivableAsContraEquity;

    /// Appropriations of retained earnings are separately identified and disclosed.
    ///
    /// Source: ASC 505-10-45-4 — Appropriated Retained Earnings
    pub struct RetainedEarningsAppropriationDisclosed;

    structural_prop!(StockholdersEquityPresented, "StockholdersEquityPresented");
    structural_prop!(CommonStockParValueDisclosed, "CommonStockParValueDisclosed");
    structural_prop!(TreasuryStockAccountedFor, "TreasuryStockAccountedFor");
    structural_prop!(StockSplitAccountedFor, "StockSplitAccountedFor");
    structural_prop!(RetainedEarningsReconciled, "RetainedEarningsReconciled");
    structural_prop!(OciAccumulatedSeparately, "OciAccumulatedSeparately");
    structural_prop!(
        NoncontrollingInterestPresented,
        "NoncontrollingInterestPresented"
    );
    structural_prop!(DividendsDeclaredRecorded, "DividendsDeclaredRecorded");
    structural_prop!(PreferredStockTermsDisclosed, "PreferredStockTermsDisclosed");
    structural_prop!(
        StockSubscriptionReceivableAsContraEquity,
        "StockSubscriptionReceivableAsContraEquity"
    );
    structural_prop!(
        RetainedEarningsAppropriationDisclosed,
        "RetainedEarningsAppropriationDisclosed"
    );
}

pub use emit_impls::{
    CommonStockParValueDisclosed, DividendsDeclaredRecorded, NoncontrollingInterestPresented,
    OciAccumulatedSeparately, PreferredStockTermsDisclosed, RetainedEarningsAppropriationDisclosed,
    RetainedEarningsReconciled, StockSplitAccountedFor, StockSubscriptionReceivableAsContraEquity,
    StockholdersEquityPresented, TreasuryStockAccountedFor,
};
