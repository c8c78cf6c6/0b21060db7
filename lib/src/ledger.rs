use hashbrown;
use hashbrown::HashMap;

use crate::account::Account;
use crate::execution::{ExecutionError, ExecutionResult};
use crate::traits::execution::TransactionExecution;
use crate::transaction::{LedgerBookEntry, Transaction, TransactionTag};

/*
> The client has a single asset account.
> All transactions are to and from this single asset account;

> There are multiple clients. Transactions reference clients.
> If a client doesn’t exist create a new record;

> Clients are represented by u16 integers.
> No names, addresses, or complex client profile info;

---

# deposit

> A deposit is a credit to the client’s asset account,
> meaning it should increase the available and total
> funds of the client account.

# withdraw

> A withdraw is a debit to the client’s asset account,
> meaning it should decrease the available and total
> funds of the client account.

> If a client does not have sufficient available funds
> the withdrawal should fail and the total amount of
> funds should not change.

# dispute

> A dispute represents a client’s claim that a transaction
> was erroneous and should be reverse. The transaction
> shouldn’t be reversed yet but the associated funds should
> be held. This means that the clients available funds should
> decrease by the amount disputed, their held funds should
> increase by the amount disputed, while their total funds
> should remain the same.

> Notice that a dispute does not state the amount disputed.
> Instead a dispute references the transaction that is
> disputed by ID. If the tx specified by the dispute doesn’t
> exist you can ignore it and assume this is an error on our
> partners side.

# resolve

> A resolve represents a resolution to a dispute, releasing
> the associated held funds. Funds that were previously
> disputed are no longer disputed. This means that the
> clients held funds should decrease by the amount no
> longer disputed, their available funds should increase
> by the amount no longer disputed, and their total funds
> should remain the same.

> Like dispute s, resolve s do not specify an amount.
> Instead they refer to a transaction that was under dispute
> by ID. If the tx specified doesn’t exist, or the tx isn’t
> under dispute, you can ignore the resolve and assume this
> is an error on our partner’s side.

# chargeback

> A chargeback is the final state of a dispute and represents
> the client reversing a transaction. Funds that were held
> have now been withdrawn. This means that the clients held
> funds and total funds should decrease by the amount
> previously disputed. If a chargeback occurs the client’s
> account should be immediately frozen.

> Like a dispute and a resolve a chargeback refers to the
> transaction by ID ( tx ) and does not specify an amount.
> Like a resolve, if the tx specified doesn’t exist, or the
> tx isn’t under dispute, you can ignore chargeback and
> assume this is an error on our partner’s side.
 */

pub struct Ledger {
    accounts: HashMap<u16, Account>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            accounts: HashMap::new(),
        }
    }

    pub fn accounts(&self) -> &HashMap<u16, Account> {
        &self.accounts
    }
}

impl TransactionExecution for Ledger {
    fn execute_transaction(
        &mut self,
        tx: Transaction,
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
