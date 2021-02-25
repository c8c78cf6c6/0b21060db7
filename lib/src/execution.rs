use crate::transaction::{LedgerBookEntry, Transaction};

#[derive(Debug)]
pub enum ExecutionError {
    InsufficientBalance,
    InvalidTransactionType,
    InvalidTransaction,
    TransactionExists,
    AccountLocked,
}

#[derive(Debug)]
pub enum ExecutionResult {
    Ok,
    BookEntry(LedgerBookEntry),
    NewAvailableBalance(i64),
}
