-- Create ledger tables for GAAP-native accounting system

-- Accounts table
CREATE TABLE IF NOT EXISTS ledger_accounts (
    account_id UUID PRIMARY KEY,
    entity_id UUID NOT NULL,
    account_number VARCHAR(20) NOT NULL,
    name VARCHAR(255) NOT NULL,
    account_class VARCHAR(50) NOT NULL, -- Asset, Liability, Equity, Revenue, Expense
    account_type_json JSONB NOT NULL, -- Specific type (CurrentAsset::Cash, etc.)
    parent_account_number VARCHAR(20),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    normal_balance VARCHAR(10) NOT NULL, -- Debit or Credit
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (entity_id, account_number)
);

CREATE INDEX IF NOT EXISTS idx_ledger_accounts_entity ON ledger_accounts(entity_id);
CREATE INDEX IF NOT EXISTS idx_ledger_accounts_class ON ledger_accounts(account_class);
CREATE INDEX IF NOT EXISTS idx_ledger_accounts_number ON ledger_accounts(account_number);

-- Journal entries table
CREATE TABLE IF NOT EXISTS ledger_journal_entries (
    entry_id UUID PRIMARY KEY,
    entity_id UUID NOT NULL,
    entry_date DATE NOT NULL,
    description TEXT NOT NULL,
    state VARCHAR(20) NOT NULL, -- Balanced, Posted, Closed
    gaap_proof_json JSONB NOT NULL, -- Established propositions
    state_data_json JSONB NOT NULL, -- State-specific metadata
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    posted_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    CONSTRAINT valid_state CHECK (state IN ('Balanced', 'Posted', 'Closed'))
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_entity ON ledger_journal_entries(entity_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_date ON ledger_journal_entries(entry_date);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_state ON ledger_journal_entries(state);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_created_at ON ledger_journal_entries(created_at);

-- Journal lines table
CREATE TABLE IF NOT EXISTS ledger_journal_lines (
    line_id UUID PRIMARY KEY,
    entry_id UUID NOT NULL REFERENCES ledger_journal_entries(entry_id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES ledger_accounts(account_id),
    debit_cents BIGINT, -- Amount in cents, NULL if credit
    credit_cents BIGINT, -- Amount in cents, NULL if debit
    memo TEXT NOT NULL,
    line_order INTEGER NOT NULL,
    CONSTRAINT debit_or_credit CHECK (
        (debit_cents IS NOT NULL AND credit_cents IS NULL) OR
        (debit_cents IS NULL AND credit_cents IS NOT NULL)
    ),
    CONSTRAINT positive_amounts CHECK (
        (debit_cents IS NULL OR debit_cents >= 0) AND
        (credit_cents IS NULL OR credit_cents >= 0)
    )
);

CREATE INDEX IF NOT EXISTS idx_ledger_lines_entry ON ledger_journal_lines(entry_id);
CREATE INDEX IF NOT EXISTS idx_ledger_lines_account ON ledger_journal_lines(account_id);

-- Account balances materialized view (optional - can be computed from journal_lines)
CREATE TABLE IF NOT EXISTS ledger_account_balances (
    balance_id UUID PRIMARY KEY,
    account_id UUID NOT NULL REFERENCES ledger_accounts(account_id) ON DELETE CASCADE,
    as_of_date DATE NOT NULL,
    debit_total_cents BIGINT NOT NULL,
    credit_total_cents BIGINT NOT NULL,
    net_balance_cents BIGINT NOT NULL,
    entry_count INTEGER NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (account_id, as_of_date)
);

CREATE INDEX IF NOT EXISTS idx_ledger_balances_account_date ON ledger_account_balances(account_id, as_of_date DESC);
