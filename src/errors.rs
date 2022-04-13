use std::num::ParseIntError;
use thiserror::Error;

pub type CsvLedgerResult<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("insufficient fund to complete a withdraw")]
    InvalidWithdraw,

    #[error("the csv contains an invalid row")]
    InvalidCsvRow,

    #[error("invalid dispute")]
    InvalidDispute,

    #[error("invalid resolve")]
    InvalidResolve,

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    DecimalConversionError(#[from] rust_decimal::Error),
}
