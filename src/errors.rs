//! The error module

use std::num::ParseIntError;
use thiserror::Error;

pub type CsvLedgerResult<T> = Result<T, Error>;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("insufficient fund to complete a withdraw")]
    InvalidWithdraw,

    #[error("the csv contains an invalid row")]
    InvalidCsvRow,

    #[error("invalid dispute")]
    InvalidDispute,

    #[error("invalid resolve")]
    InvalidResolve,

    #[error("invalid chargeback")]
    InvalidChargeback,

    #[error("attempted operation on a locked account")]
    LockedAccount,

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    DecimalConversion(#[from] rust_decimal::Error),
}
