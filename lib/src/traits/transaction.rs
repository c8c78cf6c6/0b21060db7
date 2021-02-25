use crate::execution::ExecutionError;

pub trait TagConstraints {
    fn is_deposit(&self) -> bool;
    fn is_withdrawal(&self) -> bool;
}

pub trait BookEntryExt {
    fn deposit_amount(&self) -> Result<i64, ExecutionError>;
    fn withdrawal_amount(&self) -> Result<i64, ExecutionError>;
}
