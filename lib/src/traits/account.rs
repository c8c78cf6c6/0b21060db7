use crate::execution::{ExecutionError, ExecutionResult};
use crate::transaction::{LedgerBookEntry, Transaction};

pub trait AccountDebitCredit {
    fn debit(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;

    fn credit(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;
}

pub trait AccountBookEntry {
    fn find_book_entry(
        &self,
        tx: &Transaction,
    ) -> Result<&LedgerBookEntry, ExecutionError>;

    fn find_disputed_book_entry(
        &self,
        tx: &Transaction,
    ) -> Result<&LedgerBookEntry, ExecutionError>;
}

pub trait AccountBookActions {
    fn dispute_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;

    fn resolve_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;

    fn chargeback_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;
}
