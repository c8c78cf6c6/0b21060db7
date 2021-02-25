use crate::execution::ExecutionError;
use crate::traits::transaction::TagConstraints;

// amounts are in tenth of cent precision
#[derive(Copy, Clone)]
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
    fn is_balance_flow_tx(&self) -> bool {
        if let TransactionTag::Deposit(_)
        | TransactionTag::Withdrawal(_) = self {
            true
        } else {
            false
        }
    }
}

pub struct Transaction {
    pub id: u32,
    pub client_id: u16,
    pub tag: TransactionTag,
}

#[derive(Copy, Clone)]
pub struct LedgerBookEntry(pub TransactionTag);

impl TagConstraints for LedgerBookEntry {
    fn is_balance_flow_tx(&self) -> bool {
        let LedgerBookEntry(ref tag) = self;

        tag.is_balance_flow_tx()
    }
}

impl Into<LedgerBookEntry> for Transaction {
    fn into(self) -> LedgerBookEntry {
        LedgerBookEntry(
            self.tag.clone(),
        )
    }
}
