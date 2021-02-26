use simledger::transaction::{Transaction, TransactionTag};

use crate::runner::{CsvTransaction, RunnerError};
use crate::unwrap_or_err;

pub fn convert_csv_tx_to_transaction(
    csv_tx: &CsvTransaction,
) -> Result<Transaction, RunnerError> {
    let amount = || -> Result<i64, RunnerError> {
        let amnt: f32 =
            unwrap_or_err!(
                csv_tx.amount.parse(),
                RunnerError::InvalidColumn(
                    format!(
                        "{:?} could not be parsed (amount)",
                        csv_tx.amount,
                    ),
                )
            );

        // lock precision at 10'000th of a dollar
        Ok((amnt * 10_000.0) as i64)
    };

    let tx_tag =
        match &*csv_tx.tx_type {
            "deposit" => TransactionTag::Deposit(amount()?),
            "withdrawal" => TransactionTag::Withdrawal(amount()?),

            "dispute" => TransactionTag::Dispute,
            "resolve" => TransactionTag::Resolve,
            "chargeback" => TransactionTag::Chargeback,

            val => {
                return Err(
                    RunnerError::InvalidColumn(
                        format!(
                            "{} is not a valid transaction type",
                            val,
                        ),
                    ),
                );
            }
        };

    let client_id: u16 =
        unwrap_or_err!(
            csv_tx.client.parse(),
            RunnerError::InvalidColumn(
                format!(
                    "{:?} could not be parsed (client)",
                    csv_tx.amount,
                ),
            )
        );

    let tx_id: u32 =
        unwrap_or_err!(
            csv_tx.tx.parse(),
            RunnerError::InvalidColumn(
                format!(
                    "{:?} could not be parsed (tx)",
                    csv_tx.amount,
                ),
            )
        );

    Ok(
        Transaction {
            id: tx_id,
            client_id,
            tag: tx_tag,
        },
    )
}
