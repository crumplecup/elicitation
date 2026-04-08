//! Database persistence for ledger data.
//!
//! Provides PostgreSQL storage for accounts, journal entries, and balances.
//! Supports both in-memory and database-backed ledgers.

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::ledger2::{
    Account, AccountNumber, Amount, EntityId, EntryId, GaapProof, JournalEntry, JournalLine,
    Ledger, NormalBalance, Posted, StateData,
};

// ─────────────────────────────────────────────────────────────
//  Error Types
// ─────────────────────────────────────────────────────────────

/// Errors that can occur during database persistence operations.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum PersistenceError {
    /// Database error.
    #[display("Database error: {}", _0)]
    Database(sqlx::Error),

    /// Serialization error.
    #[display("Serialization error: {}", _0)]
    Serialization(serde_json::Error),

    /// Account not found.
    #[display("Account not found: {}", _0)]
    AccountNotFound(#[error(not(source))] AccountNumber),

    /// Entry not found.
    #[display("Entry not found: {}", _0)]
    EntryNotFound(#[error(not(source))] EntryId),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(err: sqlx::Error) -> Self {
        PersistenceError::Database(err)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(err: serde_json::Error) -> Self {
        PersistenceError::Serialization(err)
    }
}

/// Result type for persistence operations.
pub type PersistenceResult<T> = Result<T, PersistenceError>;

// ─────────────────────────────────────────────────────────────
//  Account Persistence
// ─────────────────────────────────────────────────────────────

/// Saves an account to the database.
pub async fn save_account(pool: &PgPool, account: &Account) -> PersistenceResult<()> {
    let account_type_json = serde_json::to_value(account.class())?;
    let normal_balance_str = match account.normal_balance() {
        NormalBalance::Debit => "Debit",
        NormalBalance::Credit => "Credit",
    };

    sqlx::query!(
        r#"
        INSERT INTO ledger_accounts
            (account_id, entity_id, account_number, name, account_class, account_type_json,
             parent_account_number, active, normal_balance)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (entity_id, account_number)
        DO UPDATE SET
            name = EXCLUDED.name,
            account_class = EXCLUDED.account_class,
            account_type_json = EXCLUDED.account_type_json,
            parent_account_number = EXCLUDED.parent_account_number,
            active = EXCLUDED.active,
            normal_balance = EXCLUDED.normal_balance,
            updated_at = NOW()
        "#,
        Uuid::new_v4(),
        account.entity_id().0,
        account.number().0,
        account.name(),
        account.class().class_name(),
        account_type_json,
        account.parent().map(|p| p.0.as_str()),
        account.active(),
        normal_balance_str,
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Loads an account from the database.
pub async fn load_account(
    pool: &PgPool,
    entity_id: EntityId,
    account_number: &AccountNumber,
) -> PersistenceResult<Account> {
    let row = sqlx::query!(
        r#"
        SELECT account_number, name, account_class, account_type_json,
               parent_account_number, active
        FROM ledger_accounts
        WHERE entity_id = $1 AND account_number = $2
        "#,
        entity_id.0,
        account_number.0,
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| PersistenceError::AccountNotFound(account_number.clone()))?;

    let account_class = serde_json::from_value(row.account_type_json)?;

    let mut builder = Account::builder()
        .number(row.account_number)
        .name(row.name)
        .class(account_class)
        .entity_id(entity_id)
        .active(row.active);

    if let Some(parent) = row.parent_account_number {
        builder = builder.parent(parent);
    }

    Ok(builder.build().expect("Valid account from database"))
}

// ─────────────────────────────────────────────────────────────
//  Journal Entry Persistence
// ─────────────────────────────────────────────────────────────

/// Saves a posted journal entry to the database.
pub async fn save_entry(pool: &PgPool, entry: &JournalEntry<Posted>) -> PersistenceResult<()> {
    let mut tx = pool.begin().await?;

    // Save entry
    let gaap_proof_json = serde_json::to_value(entry.gaap_proof())?;
    let state_data_json = serde_json::to_value(entry.state_data())?;

    sqlx::query!(
        r#"
        INSERT INTO ledger_journal_entries
            (entry_id, entity_id, entry_date, description, state, gaap_proof_json,
             state_data_json, created_at, posted_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (entry_id) DO NOTHING
        "#,
        entry.entry_id().0,
        entry.entity_id().0,
        entry.date(),
        entry.description(),
        "Posted",
        gaap_proof_json,
        state_data_json,
        entry.created_at(),
        entry.posted_at(),
    )
    .execute(&mut *tx)
    .await?;

    // Save lines
    for (i, line) in entry.lines().iter().enumerate() {
        // First ensure the account exists
        save_account_if_not_exists(&mut tx, line.account()).await?;

        let (debit_cents, credit_cents) = if line.is_debit() {
            (Some(line.amount().cents()), None)
        } else {
            (None, Some(line.amount().cents()))
        };

        sqlx::query!(
            r#"
            INSERT INTO ledger_journal_lines
                (line_id, entry_id, account_id, debit_cents, credit_cents, memo, line_order)
            VALUES ($1, $2, (SELECT account_id FROM ledger_accounts WHERE entity_id = $3 AND account_number = $4),
                    $5, $6, $7, $8)
            ON CONFLICT (line_id) DO NOTHING
            "#,
            Uuid::new_v4(),
            entry.entry_id().0,
            entry.entity_id().0,
            line.account().number().0,
            debit_cents,
            credit_cents,
            line.memo(),
            i as i32,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// Helper to save account if it doesn't exist.
async fn save_account_if_not_exists(
    tx: &mut Transaction<'_, Postgres>,
    account: &Account,
) -> PersistenceResult<()> {
    let account_type_json = serde_json::to_value(account.class())?;
    let normal_balance_str = match account.normal_balance() {
        NormalBalance::Debit => "Debit",
        NormalBalance::Credit => "Credit",
    };

    sqlx::query!(
        r#"
        INSERT INTO ledger_accounts
            (account_id, entity_id, account_number, name, account_class, account_type_json,
             parent_account_number, active, normal_balance)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (entity_id, account_number) DO NOTHING
        "#,
        Uuid::new_v4(),
        account.entity_id().0,
        account.number().0,
        account.name(),
        account.class().class_name(),
        account_type_json,
        account.parent().map(|p| p.0.as_str()),
        account.active(),
        normal_balance_str,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

// ─────────────────────────────────────────────────────────────
//  Ledger Persistence
// ─────────────────────────────────────────────────────────────

/// Saves an entire ledger to the database.
pub async fn save_ledger(pool: &PgPool, ledger: &Ledger) -> PersistenceResult<()> {
    // Save all entries (which will also save accounts)
    for entry in ledger.entries() {
        save_entry(pool, entry).await?;
    }

    Ok(())
}

/// Loads all entries for an entity from the database into a new ledger.
///
/// Note: This loads Posted entries. The ledger will be populated with
/// all historical entries for the entity.
pub async fn load_ledger(pool: &PgPool, entity_id: EntityId) -> PersistenceResult<Ledger> {
    let mut ledger = Ledger::new(entity_id);

    // Load all posted entries for this entity
    let entry_rows = sqlx::query!(
        r#"
        SELECT entry_id, entry_date, description, gaap_proof_json, state_data_json, created_at
        FROM ledger_journal_entries
        WHERE entity_id = $1 AND state = 'Posted'
        ORDER BY created_at ASC
        "#,
        entity_id.0,
    )
    .fetch_all(pool)
    .await?;

    for entry_row in entry_rows {
        let entry_id = EntryId::from_uuid(entry_row.entry_id);

        // Load lines for this entry
        let line_rows = sqlx::query!(
            r#"
            SELECT l.debit_cents, l.credit_cents, l.memo, l.line_order,
                   a.account_number, a.name, a.account_class, a.account_type_json,
                   a.parent_account_number, a.active
            FROM ledger_journal_lines l
            JOIN ledger_accounts a ON l.account_id = a.account_id
            WHERE l.entry_id = $1
            ORDER BY l.line_order ASC
            "#,
            entry_row.entry_id,
        )
        .fetch_all(pool)
        .await?;

        let mut lines = Vec::new();
        for line_row in line_rows {
            // Reconstruct account
            let account_class = serde_json::from_value(line_row.account_type_json)?;
            let mut builder = Account::builder()
                .number(line_row.account_number)
                .name(line_row.name)
                .class(account_class)
                .entity_id(entity_id)
                .active(line_row.active);

            if let Some(parent) = line_row.parent_account_number {
                builder = builder.parent(parent);
            }

            let account = builder.build().expect("Valid account from database");

            // Create journal line
            let line = if let Some(debit_cents) = line_row.debit_cents {
                JournalLine::debit(account, Amount::from_cents(debit_cents), line_row.memo)
            } else if let Some(credit_cents) = line_row.credit_cents {
                JournalLine::credit(account, Amount::from_cents(credit_cents), line_row.memo)
            } else {
                continue; // Skip invalid lines
            };

            lines.push(line);
        }

        // Reconstruct journal entry
        // Note: We're reconstructing Posted entries, so we need to create a Balanced entry first
        // and then transition it to Posted. However, since we're loading from DB, we can
        // use internal fields directly.
        let gaap_proof: GaapProof = serde_json::from_value(entry_row.gaap_proof_json)?;
        let state_data: StateData = serde_json::from_value(entry_row.state_data_json)?;

        // Create Posted entry directly (bypassing builder)
        let posted_entry = JournalEntry {
            entry_id,
            entity_id,
            date: entry_row.entry_date,
            description: entry_row.description,
            lines,
            gaap_proof,
            state_data,
            created_at: entry_row.created_at,
            _state: std::marker::PhantomData,
        };

        // Add to ledger
        ledger
            .add_posted_entry(posted_entry)
            .expect("Valid entry from database");
    }

    Ok(ledger)
}
