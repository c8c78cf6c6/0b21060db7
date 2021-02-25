use crate::transaction::{LedgerBookEntry, Transaction};

pub enum ExecutionError {
    TransactionNotDisputed,
    InsufficientBalance,
    // should only happen at dispute & chargeback
    InvalidAccount,
    InvalidTransactionType,
    InvalidTransaction,
    AccountLocked,
}

pub enum ExecutionResult {
    Ok,
    BookEntry(LedgerBookEntry),
    NewAvailableBalance(i64),
}
