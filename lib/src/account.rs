use hashbrown::HashMap;

use crate::execution::{ExecutionError, ExecutionResult};
use crate::traits::account::{AccountBookActions, AccountBookEntry, AccountDebitCredit};
use crate::traits::execution::TransactionExecution;
use crate::traits::transaction::{TagConstraints, BookEntryExt};
use crate::transaction::{LedgerBookEntry, Transaction, TransactionTag};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Account {
    id: u16,

    is_locked: bool,

    amount_available: i64,

    pub book: BTreeMap<u32, LedgerBookEntry>,
    pub book_disputed: BTreeMap<u32, LedgerBookEntry>,
    pub book_chargeback: BTreeMap<u32, LedgerBookEntry>,
}

impl Account {
    pub fn new(id: u16) -> Account {
        Account {
            id,

            is_locked: false,

            amount_available: 0,

            book: BTreeMap::new(),
            book_disputed: BTreeMap::new(),
            book_chargeback: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> u16 { self.id }
    pub fn locked(&self) -> bool { self.is_locked }

    pub fn lock(&mut self) {
        self.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.is_locked = true;
    }

    pub fn assert_is_not_locked(
        &self,
    ) -> Result<ExecutionResult, ExecutionError> {
        if self.locked() {
            Err(ExecutionError::AccountLocked)
        } else {
            Ok(ExecutionResult::Ok)
        }
    }

    pub fn amount_available(&self) -> i64 {
        self.amount_available
    }

    pub fn amount_held(&self) -> i64 {
        let mut amount = 0i64;

        for (_, LedgerBookEntry(tag)) in self.book_disputed.iter() {
            if let TransactionTag::Deposit(tx_amount) = tag {
                amount += tx_amount;
            }
        };

        amount
    }

    pub fn amount_total(&self) -> i64 {
        self.amount_available + self.amount_held()
    }
}

impl AccountDebitCredit for Account {
    fn debit(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        self.assert_is_not_locked()?;

        if let TransactionTag::Withdrawal(amount) = tx.tag {
            if self.amount_available < amount {
                return Err(ExecutionError::InsufficientBalance);
            }

            self.book
                .insert(
                    tx.id,
                    tx.clone().into(),
                );

            self.amount_available -= amount;

            Ok(
                ExecutionResult::NewAvailableBalance(
                    self.amount_available,
                )
            )
        } else {
            Err(ExecutionError::InvalidTransactionType)
        }
    }

    fn credit(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        self.assert_is_not_locked()?;

        if let TransactionTag::Deposit(amount) = tx.tag {
            self.book
                .insert(
                    tx.id,
                    tx.clone().into(),
                );

            self.amount_available += amount;

            Ok(
                ExecutionResult::NewAvailableBalance(
                    self.amount_available,
                )
            )
        } else {
            Err(ExecutionError::InvalidTransactionType)
        }
    }
}

impl AccountBookEntry for Account {
    fn find_book_entry(
        &self,
        tx: &Transaction,
    ) -> Result<&LedgerBookEntry, ExecutionError> {
        match self.book.get(&tx.id) {
            None => Err(ExecutionError::InvalidTransaction),
            Some(tx) => Ok(tx),
        }
    }

    fn find_disputed_book_entry(
        &self,
        tx: &Transaction,
    ) -> Result<&LedgerBookEntry, ExecutionError> {
        match self.book.get(&tx.id) {
            None => Err(ExecutionError::InvalidTransaction),
            Some(tx) => Ok(tx),
        }
    }
}

impl AccountBookActions for Account {
    fn dispute_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        let subject_tx =
            self.find_book_entry(&tx)?
                .clone();

        if !subject_tx.is_deposit() {
            return Err(ExecutionError::InvalidTransactionType);
        }

        self.amount_available -= subject_tx.deposit_amount()?;

        self.book.remove(&tx.id);
        self.book_disputed.insert(tx.id, subject_tx);

        Ok(ExecutionResult::Ok)
    }

    fn resolve_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        let subject_tx =
            self.find_disputed_book_entry(&tx)?
                .clone();

        if !subject_tx.is_deposit() {
            return Err(ExecutionError::InvalidTransactionType);
        }

        self.amount_available += subject_tx.deposit_amount()?;

        self.book_disputed.remove(&tx.id);
        self.book.insert(tx.id, subject_tx);

        Ok(ExecutionResult::Ok)
    }

    fn chargeback_book_entry(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        let subject_tx =
            self.find_disputed_book_entry(&tx)?
                .clone();

        if !subject_tx.is_deposit() {
            return Err(ExecutionError::InvalidTransactionType);
        }

        self.book_disputed.remove(&tx.id);
        self.book_chargeback.insert(tx.id, subject_tx);

        self.lock();

        Ok(ExecutionResult::Ok)
    }
}

impl TransactionExecution for Account {
    fn execute_transaction(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError> {
        match tx.tag {
            // balance flow

            TransactionTag::Deposit(_) => {
                self.credit(tx)?;
            }

            TransactionTag::Withdrawal(_) => {
                self.debit(tx)?;
            }

            // administrative

            TransactionTag::Dispute => {
                self.dispute_book_entry(tx)?;
            }

            TransactionTag::Resolve => {
                self.resolve_book_entry(tx)?;
            }

            TransactionTag::Chargeback => {
                self.chargeback_book_entry(tx)?;
            }
        };

        Ok(ExecutionResult::Ok)
    }
}
