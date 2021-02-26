use simledger::account::Account;
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

type TestCasesBalances = Vec<(Vec<Transaction>, (i64, i64, i64, bool))>;

fn run_test_balances(cases: TestCasesBalances) {
    for case in cases.iter() {
        let (
            txs,
            (
                exp_total,
                exp_avail,
                exp_held,
                exp_locked,
            )
        ) = case;

        let mut account = Account::new(
            CLIENT_ID_A,
        );

        for tx in txs.iter() {
            dbg!(account.execute_transaction(dbg!(&tx)));
            dbg!(&account.book);
            dbg!(&account.book_disputed);
            dbg!(&account.book_chargeback);
            dbg!(
                &account.amount_available(),
                &account.amount_held(),
                &account.amount_total(),
            );
        }

        assert_eq!(
            &account.amount_total(),
            exp_total,
        );

        assert_eq!(
            &account.amount_available(),
            exp_avail,
        );

        assert_eq!(
            &account.amount_held(),
            exp_held,
        );

        assert_eq!(
            &account.locked(),
            exp_locked,
        );
    }
}

#[test]
fn account_basic_balance_credit_debit_check() {
    run_test_balances(
        // basic balance credit / debit check
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Withdrawal(10000)),
                ),
                (5000, 5000, 0, false),
            ),
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Withdrawal(10000)),
                    fake_tx(3, TransactionTag::Withdrawal(10000)),
                    fake_tx(4, TransactionTag::Withdrawal(10000)),
                ),
                (5000, 5000, 0, false),
            ),
        ),
    );
}

#[test]
fn account_verify_tx_reverse_hijack_by_reusing_id_impossible() {
    run_test_balances(
        // basic balance credit / debit check
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Deposit(1)),
                    fake_tx(1, TransactionTag::Dispute),
                ),
                (15000, 0, 15000, false),
            ),
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Deposit(1)),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(2, TransactionTag::Withdrawal(15000)),
                    fake_tx(1, TransactionTag::Chargeback),
                ),
                (0, 0, 0, true),
            ),
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    // will fail because tx with id 1 exists
                    fake_tx(1, TransactionTag::Deposit(1)),
                    fake_tx(1, TransactionTag::Dispute),
                    // will fail because tx is disputed
                    fake_tx(2, TransactionTag::Withdrawal(15000)),
                    fake_tx(1, TransactionTag::Resolve),
                    // will fail because tx with id 1 exists
                    fake_tx(1, TransactionTag::Deposit(1)),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(1, TransactionTag::Chargeback),
                    // everything below will fail because account is locked
                    fake_tx(3, TransactionTag::Deposit(20000)),
                    fake_tx(4, TransactionTag::Deposit(20000)),
                    fake_tx(5, TransactionTag::Withdrawal(2)),
                ),
                (0, 0, 0, true),
            ),
        ),
    );
}

#[test]
fn account_verify_user_cannot_overdraw() {
    run_test_balances(
        // basic balance credit / debit check
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Withdrawal(10000)),
                    fake_tx(3, TransactionTag::Withdrawal(2500)),
                    fake_tx(4, TransactionTag::Withdrawal(2500)),
                    fake_tx(5, TransactionTag::Withdrawal(2500)),
                ),
                (0, 0, 0, false),
            ),
        ),
    );
}

#[test]
fn account_basic_dispute_referencing_older_tx_sanity_check() {
    run_test_balances(
        // basic dispute referencing older tx sanity check
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Deposit(15000)),
                    fake_tx(3, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Dispute),
                ),
                (45000, 30000, 15000, false),
            ),
        ),
    );
}

#[test]
fn account_verify_chargeback_execution_order_respect() {
    run_test_balances(
        // verify chargeback can't just randomly happen
        // but requires dispute beforehand
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Chargeback),
                ),
                (15000, 15000, 0, false),
            ),
        )
    );
}

#[test]
fn account_verify_resolve_dispute_sanity() {
    run_test_balances(
        // verify resolve-dispute sanity
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(1, TransactionTag::Resolve),
                ),
                (30000, 30000, 0, false),
            ),
        ),
    );
}

#[test]
fn account_dispute_fuzz() {
    run_test_balances(
        // verify resolve-dispute sanity
        vec!(
            (
                vec!(
                    fake_tx(1, TransactionTag::Deposit(15000)),
                    fake_tx(2, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Resolve),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(1, TransactionTag::Resolve),
                    fake_tx(1, TransactionTag::Resolve),
                    fake_tx(3, TransactionTag::Deposit(15000)),
                    fake_tx(1, TransactionTag::Dispute),
                    fake_tx(1, TransactionTag::Resolve),
                ),
                (45000, 45000, 0, false),
            )
        )
    );
}
