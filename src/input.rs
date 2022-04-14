//! All the types needed to deserialize the input csv

use crate::{errors, ClientId, TransactionId};

use crate::errors::Error::InvalidCsvRow;
use csv::StringRecord;
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum LedgerItem {
    Deposit(DepositData),
    Withdrawal(WithdrawalData),
    Dispute(DisputeData),
    Resolve(ResolveData),
    Chargeback(ChargebackData),
}

impl LedgerItem {
    pub fn client(&self) -> ClientId {
        match self {
            LedgerItem::Deposit(data) => data.client,
            LedgerItem::Withdrawal(data) => data.client,
            LedgerItem::Dispute(data) => data.client,
            LedgerItem::Resolve(data) => data.client,
            LedgerItem::Chargeback(data) => data.client,
        }
    }
}

// sadly I need to do this, because it seems like serde Deserialize macros doesn't work with internally tagged enum AND the csv library.
// On a json input it works fine. Probably it's because of the internal representation of a csv row, with looks like a tuple, more than a record,
// but I didn't investigate a lot on this problem.
// I prefer having this complexity here, at the boundary of the application as an adapter, while having a better and more expressive internal model.
impl TryFrom<StringRecord> for LedgerItem {
    type Error = errors::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let value = match value.get(0) {
            Some("deposit") => Self::Deposit(value.try_into()?),
            Some("withdrawal") => Self::Withdrawal(value.try_into()?),
            Some("dispute") => Self::Dispute(value.try_into()?),
            Some("resolve") => Self::Resolve(value.try_into()?),
            Some("chargeback") => Self::Chargeback(value.try_into()?),
            _ => return Err(InvalidCsvRow),
        };
        Ok(value)
    }
}

#[derive(Debug)]
pub struct DepositData {
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Decimal,
}

impl TryFrom<StringRecord> for DepositData {
    type Error = errors::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            client: value.get(1).unwrap().parse()?,
            tx: value.get(2).unwrap().parse()?,
            amount: value.get(3).unwrap().parse()?,
        })
    }
}

pub type WithdrawalData = DepositData;

#[derive(Debug)]
pub struct DisputeData {
    pub client: ClientId,
    pub tx: TransactionId,
}

impl TryFrom<StringRecord> for DisputeData {
    type Error = errors::Error;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            client: value.get(1).unwrap().parse()?,
            tx: value.get(2).unwrap().parse()?,
        })
    }
}

pub type ResolveData = DisputeData;
pub type ChargebackData = DisputeData;
