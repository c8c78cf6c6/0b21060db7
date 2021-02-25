use crate::execution::{ExecutionError, ExecutionResult};
use crate::transaction::Transaction;

pub trait TransactionExecution {
    fn execute_transaction(
        &mut self,
        tx: &Transaction,
    ) -> Result<ExecutionResult, ExecutionError>;
}
