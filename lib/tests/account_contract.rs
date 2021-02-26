use simledger::account::Account;
use simledger::execution::{ExecutionError, ExecutionResult};
use simledger::traits::execution::TransactionExecution;
use simledger::transaction::{Transaction, TransactionTag};

static CLIENT_ID_A: u16 = 12;
static CLIENT_ID_B: u16 = 21;

fn fake_tx(
    id: u32,
    tag: TransactionTag,
) -> Transaction {
    Transaction {
        id,
        client_id: CLIENT_ID_A,
        tag,
    }
}

type TestCases = Vec<(Transaction, Result<ExecutionResult, ExecutionError>)>;

fn run_test_contract(cases: TestCases) {
    let mut account = Account::new(
        CLIENT_ID_A,
    );

    for (tx, exp_result) in cases.iter() {
        assert_eq!(
            &account.execute_transaction(&tx),
            exp_result,
        );
    }
}

#[test]
fn account_ensure_no_tx_overwrite() {
    run_test_contract(
        // basic balance credit / debit check
        vec!(
            (
                fake_tx(1, TransactionTag::Deposit(15000)),
                Ok(ExecutionResult::NewAvailableBalance(15000)),
            ),
            (
                fake_tx(1, TransactionTag::Deposit(1)),
                Err(ExecutionError::TransactionExists),
            ),
            (
                fake_tx(1, TransactionTag::Withdrawal(1)),
                Err(ExecutionError::TransactionExists),
            ),
            (
                fake_tx(2, TransactionTag::Withdrawal(15000)),
                Ok(ExecutionResult::NewAvailableBalance(0)),
            ),
        ),
    );
}

#[test]
fn account_verify_dispute_contract() {
    run_test_contract(
        // basic balance credit / debit check
        vec!(
            (
                fake_tx(1, TransactionTag::Deposit(15000)),
                Ok(ExecutionResult::NewAvailableBalance(15000)),
            ),
            (
                fake_tx(1, TransactionTag::Dispute),
                Ok(ExecutionResult::Ok),
            ),
            (
                fake_tx(1, TransactionTag::Dispute),
                Err(ExecutionError::InvalidTransaction),
            ),
            (
                fake_tx(1, TransactionTag::Resolve),
                Ok(ExecutionResult::Ok),
            ),
            (
                fake_tx(1, TransactionTag::Dispute),
                Ok(ExecutionResult::Ok),
            ),
            (
                fake_tx(1, TransactionTag::Chargeback),
                Ok(ExecutionResult::Ok),
            ),
        ),
    );
}

#[test]
fn account_verify_dispute_resolve_chargeback_reject_withdrawal_tx() {
    run_test_contract(
        // basic balance credit / debit check
        vec!(
            (
                fake_tx(1, TransactionTag::Deposit(15000)),
                Ok(ExecutionResult::NewAvailableBalance(15000)),
            ),
            (
                fake_tx(2, TransactionTag::Withdrawal(15000)),
                Ok(ExecutionResult::NewAvailableBalance(0)),
            ),
            (
                fake_tx(2, TransactionTag::Dispute),
                Err(ExecutionError::InvalidTransactionType),
            ),
            (
                fake_tx(2, TransactionTag::Resolve),
                // can't reach invalid type because tx needs
                // to be in disputed book
                Err(ExecutionError::InvalidTransaction),
            ),
            (
                fake_tx(2, TransactionTag::Chargeback),
                // can't reach invalid type because tx needs
                // to be in chargeback book
                Err(ExecutionError::InvalidTransaction),
            ),
        ),
    );
}
