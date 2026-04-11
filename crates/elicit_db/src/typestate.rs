//! Typestate markers for transaction and query lifecycle.
//!
//! Mirrors the `Layout<Pending/Verified/Rendered>` pattern from `elicit_ui`.

use crate::IsolationLevel;
use std::marker::PhantomData;

/// Typestate marker: transaction is open, awaiting commit or rollback.
///
/// Source: ISO/IEC 9075-2 §17.1 — Start transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Open;

/// Typestate marker: transaction has been durably committed.
///
/// Source: ISO/IEC 9075-2 §17.3 — Commit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Committed;

/// Typestate marker: transaction has been rolled back; changes discarded.
///
/// Source: ISO/IEC 9075-2 §17.4 — Rollback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RolledBack;

/// Typestate marker: query is built but not yet executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prepared;

/// Typestate marker: query has been executed; results are available.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Executed;

/// A typed transaction marker carrying isolation level information.
///
/// State transitions:
/// - `TxMarker<Open>` — active transaction
/// - `TxMarker<Committed>` — durably committed
/// - `TxMarker<RolledBack>` — rolled back
///
/// Actual connection state lives in the [`crate::DbTransactor`] implementation.
/// This marker is proof that a transaction reached the given state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxMarker<S> {
    /// The isolation level this transaction was started with.
    pub isolation: IsolationLevel,
    _state: PhantomData<S>,
}

impl TxMarker<Open> {
    /// Create an open transaction marker.
    pub fn open(isolation: IsolationLevel) -> Self {
        Self {
            isolation,
            _state: PhantomData,
        }
    }

    /// Transition to committed state.
    pub fn commit(self) -> TxMarker<Committed> {
        TxMarker {
            isolation: self.isolation,
            _state: PhantomData,
        }
    }

    /// Transition to rolled-back state.
    pub fn rollback(self) -> TxMarker<RolledBack> {
        TxMarker {
            isolation: self.isolation,
            _state: PhantomData,
        }
    }
}
