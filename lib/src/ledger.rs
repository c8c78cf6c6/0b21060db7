use std::collections::BTreeMap;

use crate::account::Account;
use crate::execution::{ExecutionError, ExecutionResult};
use crate::traits::execution::TransactionExecution;
use crate::transaction::{LedgerBookEntry, Transaction, TransactionTag};

pub struct Ledger {
    accounts: BTreeMap<u16, Account>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            accounts: BTreeMap::new(),
        }
    }

    pub fn accounts(&self) -> &BTreeMap<u16, Account> {
        &self.accounts
    }
}

impl TransactionExecution for Ledger {
    fn execute_transaction(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        self.accounts
            .entry(tx.client_id)
            .or_insert(
                Account::new(
                    tx.client_id,
                ),
            )
            .execute_transaction(tx)
    }
}
