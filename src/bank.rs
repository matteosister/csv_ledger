//! A representation of a bank, that contains multiple accounts and is able to handle items from the input csv

use std::collections::HashMap;

use crate::account::Account;
use crate::errors::CsvLedgerResult;
use crate::{ClientId, LedgerItem};

#[derive(Debug)]
pub struct Bank(HashMap<ClientId, Account>);

impl Bank {
    /// returns an iterator over accounts
    pub fn accounts(&self) -> impl Iterator<Item = &Account> {
        self.0.iter().map(|map| map.1)
    }

    /// create a Bank instance from a csv file
    pub fn from_file(path: &str) -> CsvLedgerResult<Self> {
        let mut file_reader = csv::Reader::from_path(path).expect("unable to read the file");
        let mut bank = Self::new();
        for record in file_reader.records() {
            let ledger_item: LedgerItem = record.expect("unable to deserialize row").try_into()?;
            bank.handle_ledger_item(ledger_item)?;
        }
        Ok(bank)
    }

    /// creates a new Bank struct
    /// this is private so that a Bank can be created only with a csv
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// handle an item from the ledger csv
    fn handle_ledger_item(&mut self, ledger_item: LedgerItem) -> CsvLedgerResult<()> {
        self.get_or_create_bank_account(ledger_item.client())
            .handle_ledger_item(ledger_item)?;
        Ok(())
    }

    /// If the Bank do not have an account with the given client_id, then create it.
    /// In any case return the account as a mutable reference
    fn get_or_create_bank_account(&mut self, client_id: ClientId) -> &mut Account {
        self.0
            .entry(client_id)
            .or_insert_with(|| Account::new(client_id))
    }
}
