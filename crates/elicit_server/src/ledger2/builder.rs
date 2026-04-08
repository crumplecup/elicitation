//! Journal entry builder with double-entry enforcement.

use chrono::{NaiveDate, Utc};

use crate::ledger2::{
    Account, Amount, Balanced, EntityId, EntryId, GaapProof, JournalEntry, JournalEntryError,
    JournalEntryResult, JournalLine, StateData,
};

/// Builder for constructing balanced journal entries.
///
/// The builder enforces double-entry bookkeeping at construction time:
/// - Tracks total debits and credits as lines are added
/// - Validates balance when `build()` is called
/// - Ensures all accounts belong to same entity
/// - Ensures at least two lines (debit + credit minimum)
///
/// # Example
///
/// ```rust,ignore
/// use elicit_server::ledger2::{JournalEntryBuilder, Amount};
/// use chrono::NaiveDate;
///
/// let entry = JournalEntryBuilder::new(entity_id, NaiveDate::from_ymd(2024, 1, 15))
///     .description("Cash sale")
///     .debit(cash_account, Amount::from_dollars(100), "Payment received")
///     .credit(revenue_account, Amount::from_dollars(100), "Sale of goods")
///     .build()?;
/// ```
#[derive(Debug)]
pub struct JournalEntryBuilder {
    entity_id: EntityId,
    date: NaiveDate,
    description: Option<String>,
    lines: Vec<JournalLine>,
    total_debits: Amount,
    total_credits: Amount,
    gaap_proof: GaapProof,
}

impl JournalEntryBuilder {
    /// Creates a new journal entry builder.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The entity this entry belongs to
    /// * `date` - The date the transaction occurred
    pub fn new(entity_id: EntityId, date: NaiveDate) -> Self {
        Self {
            entity_id,
            date,
            description: None,
            lines: Vec::new(),
            total_debits: Amount::from_cents(0),
            total_credits: Amount::from_cents(0),
            gaap_proof: GaapProof::default(),
        }
    }

    /// Sets the transaction description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a debit line.
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
    /// builder.debit(cash_account, Amount::from_dollars(500), "Cash received");
    /// ```
    pub fn debit(mut self, account: Account, amount: Amount, memo: impl Into<String>) -> Self {
        self.total_debits = self.total_debits + amount;
        self.lines.push(JournalLine::debit(account, amount, memo));
        self
    }

    /// Adds a credit line.
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
    /// builder.credit(revenue_account, Amount::from_dollars(500), "Sale of goods");
    /// ```
    pub fn credit(mut self, account: Account, amount: Amount, memo: impl Into<String>) -> Self {
        self.total_credits = self.total_credits + amount;
        self.lines.push(JournalLine::credit(account, amount, memo));
        self
    }

    /// Adds a GAAP proof.
    pub fn with_gaap_proof(mut self, proof: GaapProof) -> Self {
        self.gaap_proof = proof;
        self
    }

    /// Builds the journal entry, validating balance and constraints.
    ///
    /// # Validation
    ///
    /// - At least two lines (double-entry minimum)
    /// - Debits equal credits (balanced)
    /// - All accounts belong to same entity
    /// - All accounts are active
    /// - Description is provided
    ///
    /// # Errors
    ///
    /// Returns error if validation fails.
    pub fn build(self) -> JournalEntryResult<JournalEntry<Balanced>> {
        // Validate description
        let description = self
            .description
            .ok_or_else(|| JournalEntryError::gaap_validation("Description is required"))?;

        // Validate minimum lines
        if self.lines.is_empty() {
            return Err(JournalEntryError::empty_entry());
        }

        if self.lines.len() == 1 {
            return Err(JournalEntryError::single_line());
        }

        // Validate balance
        if self.total_debits != self.total_credits {
            return Err(JournalEntryError::imbalance(
                self.total_debits,
                self.total_credits,
            ));
        }

        // Validate all accounts belong to same entity
        for line in &self.lines {
            if line.account().entity_id() != &self.entity_id {
                return Err(JournalEntryError::entity_mismatch());
            }
        }

        // Validate all accounts are active
        for line in &self.lines {
            if !line.account().active() {
                return Err(JournalEntryError::inactive_account(
                    line.account().number().to_string(),
                ));
            }
        }

        // Create balanced entry
        Ok(JournalEntry {
            entry_id: EntryId::new_v4(),
            entity_id: self.entity_id,
            date: self.date,
            description,
            lines: self.lines,
            gaap_proof: self.gaap_proof,
            state_data: StateData::Balanced {
                total: self.total_debits,
                balanced_at: Utc::now(),
            },
            created_at: Utc::now(),
            _state: std::marker::PhantomData,
        })
    }
}

impl JournalEntry<Balanced> {
    /// Creates a builder for constructing a journal entry.
    pub fn builder(entity_id: EntityId, date: NaiveDate) -> JournalEntryBuilder {
        JournalEntryBuilder::new(entity_id, date)
    }
}
