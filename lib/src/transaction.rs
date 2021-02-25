use crate::execution::ExecutionError;
use crate::traits::transaction::{TagConstraints, BookEntryExt};

// amounts are in tenth of cent precision
#[derive(Debug, Copy, Clone)]
pub enum TransactionTag {
    // balance flows

    Deposit(i64),
    Withdrawal(i64),

    // administrative

    Dispute,
    Resolve,
    Chargeback,
}

impl TagConstraints for TransactionTag {
    fn is_deposit(&self) -> bool {
        if let TransactionTag::Deposit(_) = self {
            true
        } else {
            false
        }
    }

    fn is_withdrawal(&self) -> bool {
        if let TransactionTag::Withdrawal(_) = self {
            true
        } else {
            false
        }
    }
}

#[derive(Copy, Clone)]
pub struct Transaction {
    pub id: u32,
    pub client_id: u16,
    pub tag: TransactionTag,
}

#[derive(Debug, Copy, Clone)]
pub struct LedgerBookEntry(pub TransactionTag);

impl BookEntryExt for LedgerBookEntry {
    fn deposit_amount(&self) -> Result<i64, ExecutionError> {
        match self.0 {
            TransactionTag::Deposit(amount) => Ok(amount),
            _ => Err(ExecutionError::InvalidTransactionType),
        }
    }

    fn withdrawal_amount(&self) -> Result<i64, ExecutionError>{
        match self.0 {
            TransactionTag::Withdrawal(amount) => Ok(amount),
            _ => Err(ExecutionError::InvalidTransactionType),
        }
    }
}

impl TagConstraints for LedgerBookEntry {
    fn is_deposit(&self) -> bool {
        let LedgerBookEntry(ref tag) = self;

        tag.is_deposit()
    }

    fn is_withdrawal(&self) -> bool {
        let LedgerBookEntry(ref tag) = self;

        tag.is_withdrawal()
    }
}

impl Into<LedgerBookEntry> for Transaction {
    fn into(self) -> LedgerBookEntry {
        LedgerBookEntry(
            self.tag.clone(),
        )
    }
}
