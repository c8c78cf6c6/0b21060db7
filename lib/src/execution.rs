use crate::transaction::{LedgerBookEntry, Transaction};

#[derive(Debug)]
pub enum ExecutionError {
    TransactionNotDisputed,
    InsufficientBalance,
    // should only happen at dispute & chargeback
    InvalidAccount,
    InvalidTransactionType,
    InvalidTransaction,
    AccountLocked,
}

#[derive(Debug)]
pub enum ExecutionResult {
    Ok,
    BookEntry(LedgerBookEntry),
    NewAvailableBalance(i64),
}
