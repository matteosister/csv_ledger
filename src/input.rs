use crate::{errors, ClientId, TransactionId};

use crate::errors::Error::InvalidCsvRow;
use csv::StringRecord;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum LedgerItem {
    Deposit(DepositData),
    Withdrawal(WithdrawalData),
    Dispute(DisputeData),
}

impl LedgerItem {
    pub fn client(&self) -> ClientId {
        match self {
            LedgerItem::Deposit(deposit_data) => deposit_data.client,
            LedgerItem::Withdrawal(withdrawal_darta) => withdrawal_darta.client,
            LedgerItem::Dispute(dispute_data) => dispute_data.client,
        }
    }
}

// sadly I need to do this, because it seems like serde Deserialize macros doesn't work with internally tagged enum AND the csv library.
impl TryFrom<StringRecord> for LedgerItem {
    type Error = errors::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let value = match value.get(0) {
            Some("deposit") => Self::Deposit(DepositData {
                client: value.get(1).unwrap().parse()?,
                tx: value.get(2).unwrap().parse()?,
                amount: value.get(3).unwrap().parse()?,
            }),
            Some("withdrawal") => Self::Withdrawal(WithdrawalData {
                client: value.get(1).unwrap().parse()?,
                tx: value.get(2).unwrap().parse()?,
                amount: value.get(3).unwrap().parse()?,
            }),
            Some("dispute") => Self::Dispute(DisputeData {
                client: value.get(1).unwrap().parse()?,
                tx: value.get(2).unwrap().parse()?,
            }),
            _ => return Err(InvalidCsvRow),
        };
        Ok(value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositData {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisputeData {
    pub client: ClientId,
    pub tx: TransactionId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalData {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Decimal,
}
