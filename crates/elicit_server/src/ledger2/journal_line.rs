//! Journal line - a single line in a journal entry (debit or credit).

use serde::{Deserialize, Serialize};

use crate::ledger2::{Account, DebitCredit};

// ─────────────────────────────────────────────────────────────
//  Amount Type
// ─────────────────────────────────────────────────────────────

/// Monetary amount in cents (to avoid floating-point precision issues).
///
/// All financial amounts are stored as signed 64-bit integers representing
/// cents. This ensures exact arithmetic without rounding errors.
///
/// # Example
///
/// ```rust,ignore
/// let amount = Amount::from_dollars(100); // $100.00 = 10000 cents
/// let amount = Amount::from_cents(12345); // $123.45
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Amount(i64);

impl Amount {
    /// Creates an amount from cents.
    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    /// Creates an amount from dollars (converts to cents).
    pub fn from_dollars(dollars: i64) -> Self {
        Self(dollars * 100)
    }

    /// Returns the amount in cents.
    pub fn cents(&self) -> i64 {
        self.0
    }

    /// Returns the amount in dollars (truncates fractional cents).
    pub fn dollars(&self) -> i64 {
        self.0 / 100
    }

    /// Returns the cents component (0-99).
    pub fn cents_component(&self) -> i64 {
        self.0 % 100
    }

    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Returns the absolute value.
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }
}

impl std::ops::Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs_cents = self.0.abs();
        let dollars = abs_cents / 100;
        let cents = abs_cents % 100;
        write!(f, "{}${}.{:02}", sign, dollars, cents)
    }
}

// ─────────────────────────────────────────────────────────────
//  Journal Line
// ─────────────────────────────────────────────────────────────

/// A single line in a journal entry.
///
/// Each journal line contains either a debit or a credit (never both) to a
/// specific account. Multiple lines make up a complete journal entry, which
/// must balance (total debits = total credits).
///
/// # GAAP Requirements
///
/// - Each line must affect exactly one account
/// - Amount must be either debit OR credit (mutually exclusive)
/// - All amounts must be positive (sign encoded by debit/credit distinction)
/// - Memo provides audit trail documentation
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{JournalLine, Amount};
///
/// // Debit line: Dr. Cash $1000.00
/// let debit_line = JournalLine::debit(
///     cash_account,
///     Amount::from_dollars(1000),
///     "Payment received from customer"
/// );
///
/// // Credit line: Cr. Accounts Receivable $1000.00
/// let credit_line = JournalLine::credit(
///     ar_account,
///     Amount::from_dollars(1000),
///     "Payment received from customer"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalLine {
    /// The account affected by this line.
    account: Account,

    /// Debit amount (if this is a debit line).
    debit: Option<Amount>,

    /// Credit amount (if this is a credit line).
    credit: Option<Amount>,

    /// Memo explaining this line's purpose (audit trail).
    memo: String,
}

impl JournalLine {
    /// Creates a debit line.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to debit
    /// * `amount` - The debit amount (must be positive)
    /// * `memo` - Description for audit trail
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let line = JournalLine::debit(
    ///     cash_account,
    ///     Amount::from_dollars(500),
    ///     "Cash sale"
    /// );
    /// ```
    pub fn debit(account: Account, amount: Amount, memo: impl Into<String>) -> Self {
        Self {
            account,
            debit: Some(amount),
            credit: None,
            memo: memo.into(),
        }
    }

    /// Creates a credit line.
    ///
    /// # Arguments
    ///
    /// * `account` - The account to credit
    /// * `amount` - The credit amount (must be positive)
    /// * `memo` - Description for audit trail
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let line = JournalLine::credit(
    ///     revenue_account,
    ///     Amount::from_dollars(500),
    ///     "Cash sale"
    /// );
    /// ```
    pub fn credit(account: Account, amount: Amount, memo: impl Into<String>) -> Self {
        Self {
            account,
            debit: None,
            credit: Some(amount),
            memo: memo.into(),
        }
    }

    /// Returns the account affected by this line.
    pub fn account(&self) -> &Account {
        &self.account
    }

    /// Returns the debit amount, if this is a debit line.
    pub fn debit_amount(&self) -> Option<Amount> {
        self.debit
    }

    /// Returns the credit amount, if this is a credit line.
    pub fn credit_amount(&self) -> Option<Amount> {
        self.credit
    }

    /// Returns the memo.
    pub fn memo(&self) -> &str {
        &self.memo
    }

    /// Returns the amount (debit or credit).
    pub fn amount(&self) -> Amount {
        self.debit
            .or(self.credit)
            .expect("Line must have debit or credit")
    }

    /// Returns whether this is a debit line.
    pub fn is_debit(&self) -> bool {
        self.debit.is_some()
    }

    /// Returns whether this is a credit line.
    pub fn is_credit(&self) -> bool {
        self.credit.is_some()
    }

    /// Returns the side (debit or credit).
    pub fn side(&self) -> DebitCredit {
        if self.is_debit() {
            DebitCredit::Debit
        } else {
            DebitCredit::Credit
        }
    }

    /// Returns the signed amount (positive for debit, negative for credit).
    ///
    /// This is useful for calculating account balances where debits increase
    /// and credits decrease (or vice versa depending on account type).
    pub fn signed_amount(&self) -> Amount {
        match self.side() {
            DebitCredit::Debit => self.amount(),
            DebitCredit::Credit => -self.amount(),
        }
    }
}

impl std::fmt::Display for JournalLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.side() {
            DebitCredit::Debit => write!(
                f,
                "Dr. {} {} - {}",
                self.account.name(),
                self.amount(),
                self.memo
            ),
            DebitCredit::Credit => write!(
                f,
                "Cr. {} {} - {}",
                self.account.name(),
                self.amount(),
                self.memo
            ),
        }
    }
}
