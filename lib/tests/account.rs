use simledger::account::Account;
use simledger::transaction::{Transaction, TransactionTag};
use simledger::traits::execution::TransactionExecution;

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

#[test]
fn account_deposit_withdrawal() {
    let cases: Vec<(Vec<Transaction>, (i64, i64, i64))> = vec!(
        (
            vec!(
                fake_tx(1, TransactionTag::Deposit(15000)),
                fake_tx(2, TransactionTag::Withdrawal(10000)),
            ),
            (5000, 5000, 0),
        ),
        (
            vec!(
                fake_tx(1, TransactionTag::Deposit(15000)),
                fake_tx(2, TransactionTag::Deposit(15000)),
                fake_tx(3, TransactionTag::Deposit(15000)),
                fake_tx(1, TransactionTag::Dispute),
            ),
            (45000, 30000, 15000),
        )
    );

    for (
        txs,
        (exp_total, exp_avail, exp_held)
    ) in cases.iter() {
        let mut account = Account::new(
            CLIENT_ID_A,
        );

        for tx in txs.iter() {
            dbg!(&account.book);
            dbg!(&account.book_chargeback);
            dbg!(&account.book_disputed);
            dbg!(account.execute_transaction(&tx));
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
    }
}
