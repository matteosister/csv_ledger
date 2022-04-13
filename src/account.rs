use rust_decimal::prelude::*;
use serde::Serialize;

use crate::errors::{CsvLedgerResult, Error};
use crate::input::*;
use crate::{LedgerItem, TransactionId};

#[derive(Debug, Serialize, Default)]
pub struct Account {
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
    #[serde(skip)]
    transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
struct Transaction {
    id: TransactionId,
    amount: Decimal,
}

impl Account {
    pub fn handle_ledger_item(&mut self, ledger_item: LedgerItem) -> CsvLedgerResult<()> {
        match ledger_item {
            LedgerItem::Deposit(deposit_data) => self.deposit(&deposit_data),
            LedgerItem::Withdrawal(withdrawal_data) => self.withdrawal(&withdrawal_data)?,
            LedgerItem::Dispute(dispute_data) => self.dispute(&dispute_data)?,
        }
        Ok(())
    }

    fn deposit(&mut self, deposit_data: &DepositData) {
        self.available += deposit_data.amount.round_dp(4);
        self.total += deposit_data.amount.round_dp(4);
        self.transactions.push(Transaction {
            id: deposit_data.tx,
            amount: deposit_data.amount,
        })
    }

    fn withdrawal(&mut self, withdrawal_data: &WithdrawalData) -> CsvLedgerResult<()> {
        if self.available < withdrawal_data.amount {
            return Err(Error::InvalidWithdraw);
        }
        self.available -= withdrawal_data.amount.round_dp(4);
        self.total -= withdrawal_data.amount.round_dp(4);
        Ok(())
    }

    fn dispute(&mut self, dispute_data: &DisputeData) -> CsvLedgerResult<()> {
        let transaction = self.transactions.iter().find(|t| t.id == dispute_data.tx);

        match transaction {
            None => Err(Error::InvalidDispute),
            Some(transaction) => {
                if self.available < transaction.amount {
                    return Err(Error::InvalidWithdraw);
                }
                self.available -= transaction.amount;
                self.held += transaction.amount;
                Ok(())
            }
        }
    }
}
