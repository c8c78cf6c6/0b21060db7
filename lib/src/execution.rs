use crate::transaction::{LedgerBookEntry, Transaction};

#[derive(Debug, Eq, PartialEq)]
pub enum ExecutionError {
    InsufficientBalance,
    InvalidTransactionType,
    InvalidTransaction,
    TransactionExists,
    TransactionDisputed,
    AccountLocked,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ExecutionResult {
    Ok,
    BookEntry(LedgerBookEntry),
    NewAvailableBalance(i64),
}
