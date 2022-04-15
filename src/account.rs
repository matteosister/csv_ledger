//! An account is a single client id in the Bank
//!
//! It contains all the data regarding money availabilities and the related transaction
//! It also handles all the possible ledger items and controversy resolution

use rust_decimal::prelude::*;
use serde::Serialize;

use crate::errors::{CsvLedgerResult, Error};
use crate::{ClientId, LedgerItem, TransactionId};

/// a struct that represent an account of the Bank.
#[derive(Debug, Serialize, Default)]
pub struct Account {
    client: ClientId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    #[serde(rename(serialize = "locked"))]
    frozen: bool,
    #[serde(skip)]
    transactions: Vec<Transaction>,
}

impl Account {
    pub fn new(client: ClientId) -> Self {
        Self {
            client,
            ..Self::default()
        }
    }
}

#[derive(Debug, Serialize)]
struct Transaction {
    id: TransactionId,
    amount: Decimal,
    dispute: bool,
}

impl Account {
    /// main api for the Account. Handle a LedgerItem and updates itself accordingly
    pub fn handle_ledger_item(&mut self, ledger_item: LedgerItem) -> CsvLedgerResult<()> {
        if self.frozen {
            return Err(Error::LockedAccount);
        }
        match ledger_item {
            LedgerItem::Deposit(data) => self.deposit(data.tx, data.amount),
            LedgerItem::Withdrawal(data) => self.withdrawal(data.amount)?,
            LedgerItem::Dispute(data) => self.dispute(data.tx)?,
            LedgerItem::Resolve(data) => self.resolve(data.tx)?,
            LedgerItem::Chargeback(data) => self.chargeback(data.tx)?,
        }
        Ok(())
    }

    /// deposit funds into the account
    ///
    /// In this scenario this will never fail. In a real world use case this should probably return
    /// a Result
    fn deposit(&mut self, tx: TransactionId, amount: Decimal) {
        let amount = Self::round(amount);
        self.available += amount;
        self.total += amount;
        self.transactions.push(Transaction {
            id: tx,
            amount,
            dispute: false,
        })
    }

    /// withdraw funds from the account
    /// I'm not saving the transaction here because it's not useful for the purpose of the exercise.
    /// But in a more advanced scenario I should have added this as a negative transaction inside the account.
    ///
    /// Fails if the availability is lower than the amount requested
    fn withdrawal(&mut self, amount: Decimal) -> CsvLedgerResult<()> {
        let amount = Self::round(amount);
        if self.available < amount {
            return Err(Error::InvalidWithdraw);
        }
        self.available -= amount;
        self.total -= amount;
        Ok(())
    }

    /// attempt a dispute for a previous transaction
    ///
    /// Fails if the transaction is not present in the account and if the availability is less than
    /// the transaction amount
    fn dispute(&mut self, tx: TransactionId) -> CsvLedgerResult<()> {
        let transaction = self.transactions.iter_mut().find(|t| t.id == tx);

        match transaction {
            None => Err(Error::InvalidDispute),
            Some(transaction) => {
                let amount = Self::round(transaction.amount);
                transaction.dispute = true;
                if self.available < amount {
                    return Err(Error::InvalidWithdraw);
                }
                self.available -= amount;
                self.held += amount;
                Ok(())
            }
        }
    }

    /// Resolve a previous dispute
    ///
    /// Fails if the transaction is not present in the account, or if the previous transaction
    /// wasn't in a dispute state
    fn resolve(&mut self, tx: TransactionId) -> CsvLedgerResult<()> {
        let transaction = self.transactions.iter().find(|t| t.id == tx);

        match transaction {
            None => Err(Error::InvalidResolve),
            Some(transaction) => {
                let amount = Self::round(transaction.amount);
                if !transaction.dispute {
                    return Err(Error::InvalidResolve);
                }
                self.available += amount;
                self.held -= amount;
                Ok(())
            }
        }
    }

    /// Chargeback a previous dispute
    ///
    /// Fails if the transaction is not present in the account, or if the previous transaction
    /// wasn't in a dispute state
    fn chargeback(&mut self, tx: TransactionId) -> CsvLedgerResult<()> {
        let transaction = self.transactions.iter().find(|t| t.id == tx);

        match transaction {
            None => Err(Error::InvalidChargeback),
            Some(transaction) => {
                let amount = Self::round(transaction.amount);
                if !transaction.dispute {
                    return Err(Error::InvalidChargeback);
                }
                self.total -= amount;
                self.held -= amount;
                self.frozen = true;
                Ok(())
            }
        }
    }

    fn round(amount: Decimal) -> Decimal {
        amount.round_dp(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DepositData;

    #[test]
    fn test_deposit_on_an_new_account() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        assert_eq!(account.available, amount);
        assert_eq!(account.total, amount);
    }

    #[test]
    fn test_double_deposit_should_double_total_and_availability() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        account.deposit(2, amount);
        assert_eq!(account.available, amount + amount);
        assert_eq!(account.total, amount + amount);
    }

    #[test]
    fn test_a_deposit_should_be_rounded_to_4_decimal_digits() {
        let mut account = Account::new(1);
        let amount = Decimal::new(2000001, 6);
        let actual_amount = Decimal::new(20000, 4);
        account.deposit(1, amount);
        assert_eq!(account.available, actual_amount);
        assert_eq!(account.total, actual_amount);
    }

    #[test]
    fn test_withdraw_funds() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        assert_eq!(account.available, amount);
        assert_eq!(account.total, amount);
        let withdraw = account.withdrawal(amount);
        assert!(withdraw.is_ok());
        assert_eq!(account.available, Decimal::zero());
        assert_eq!(account.total, Decimal::zero());
    }

    #[test]
    fn test_withdraw_funds_without_availability() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        let withdraw = account.withdrawal(amount);
        assert!(withdraw.is_err());
        // here I could also check for the error type
    }

    #[test]
    fn test_dispute() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        let dispute = account.dispute(1);
        assert!(dispute.is_ok());
        assert_eq!(account.held, amount);
        assert_eq!(account.total, amount);
        assert_eq!(account.available, Decimal::zero());
    }

    #[test]
    fn test_dispute_for_non_existent_transaction() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        let dispute = account.dispute(2);
        assert!(dispute.is_err());
        // here I could also check for the error type
    }

    #[test]
    fn test_resolve_a_previous_dispute() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        let dispute = account.dispute(1);
        assert!(dispute.is_ok());
        let resolve = account.resolve(1);
        assert!(resolve.is_ok());
        assert_eq!(account.available, amount);
        assert_eq!(account.total, amount);
    }

    #[test]
    fn test_chargeback() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        let dispute = account.dispute(1);
        assert!(dispute.is_ok());
        let chargeback = account.chargeback(1);
        assert!(chargeback.is_ok());
        assert_eq!(account.available, Decimal::zero());
        assert_eq!(account.total, Decimal::zero());
        assert_eq!(account.held, Decimal::zero());
        assert!(account.frozen);
    }

    #[test]
    fn test_a_chargeback_lock_successive_operations() {
        let mut account = Account::new(1);
        let amount = Decimal::new(200, 2);
        account.deposit(1, amount);
        let dispute = account.dispute(1);
        assert!(dispute.is_ok());
        let chargeback = account.chargeback(1);
        assert!(chargeback.is_ok());

        let deposit = account.handle_ledger_item(LedgerItem::Deposit(DepositData {
            client: 1,
            tx: 2,
            amount: Decimal::new(200, 1),
        }));

        assert!(deposit.is_err());
        assert_eq!(deposit, Err(Error::LockedAccount));
    }
}
