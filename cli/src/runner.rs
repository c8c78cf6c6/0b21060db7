use std::convert::{TryFrom, TryInto};
use std::error::Error;

use csv_async::{AsyncDeserializer, AsyncSerializer};
use serde_derive::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::Stdout;
use tokio_stream::StreamExt;

use simledger::account::Account;
use simledger::ledger::Ledger;
use simledger::traits::execution::TransactionExecution;
use simledger::transaction::{Transaction, TransactionTag};

use crate::unwrap_or_err;
use crate::util::convert_csv_tx_to_transaction;

#[derive(Debug)]
pub enum RunnerError {
    InvalidCsvRow,
    FileOpenFailed,
    InvalidColumn(String),
    InternalError(&'static str),
}

pub struct Runner {
    pub ledger: Ledger,

    pub csv_reader: AsyncDeserializer<File>,
    pub csv_stdout_writer: AsyncSerializer<Stdout>,
}

impl Runner {
    async fn new(
        file_name: String,
    ) -> Result<Runner, RunnerError> {
        let ledger = Ledger::new();

        let source_file =
            unwrap_or_err!(
            File::open(file_name).await,
            RunnerError::FileOpenFailed
        );

        let csv_reader =
            csv_async
            ::AsyncDeserializer
            ::from_reader(
                source_file,
            );

        let csv_stdout_writer =
            csv_async
            ::AsyncSerializer
            ::from_writer(
                tokio::io::stdout(),
            );

        Ok(
            Runner {
                ledger,

                csv_reader,
                csv_stdout_writer,
            }
        )
    }

    pub async fn process_csv(&mut self) -> Result<(), RunnerError> {
        let mut records =
            self.csv_reader
                .deserialize::<CsvTransaction>();

        // first line should be header
        let mut line = 2;

        while let Some(record) = records.next().await {
            let record = unwrap_or_err!(
                record,
                RunnerError::InvalidCsvRow
            );

            let tx: Transaction =
                match record.clone().try_into() {
                    Err(err) => {
                        eprintln!(
                            "OUTPUT MAY BE INVALID -- Error while parsing line {}: {:?}",
                            line,
                            err,
                        );

                        continue;
                    },
                    Ok(tx) => tx,
                };

            self.ledger
                .execute_transaction(
                    &tx,
                );

            line += 1;
        }

        for (_, account) in self.ledger.accounts().iter() {
            let account_summary: CsvLedgerSummary =
                account.clone().into();

            self.csv_stdout_writer
                .serialize(&account_summary)
                .await;
        }

        Ok(())
    }

    pub async fn ignition(
        file_name: String,
    ) -> Result<(), RunnerError> {
        let mut runner =
            Runner::new(file_name)
                .await?;

        runner
            .process_csv()
            .await
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CsvTransaction {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: String,
    pub tx: String,
    pub amount: String,
}

impl TryInto<Transaction> for CsvTransaction {
    type Error = RunnerError;

    fn try_into(self) -> Result<Transaction, Self::Error> {
        convert_csv_tx_to_transaction(
            &self,
        )
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CsvLedgerSummary {
    pub client: String,
    pub available: String,
    pub held: String,
    pub total: String,
    pub locked: String,
}

impl Into<CsvLedgerSummary> for Account {
    fn into(self) -> CsvLedgerSummary {
        let amount_available =
            self.amount_available() as f32 / 10_000f32;

        let amount_held =
            self.amount_held() as f32 / 10_000f32;

        let amount_total =
            self.amount_total() as f32 / 10_000f32;

        CsvLedgerSummary {
            client: self.id().to_string(),
            available: amount_available.to_string(),
            held: amount_held.to_string(),
            total: amount_total.to_string(),
            locked: format!("{:?}", self.locked()),
        }
    }
}
