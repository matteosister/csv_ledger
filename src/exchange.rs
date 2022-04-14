//! A representation of an exchange, that contains multiple accounts and is able to handle items from the input csv

use std::collections::HashMap;

use crate::account::Account;
use crate::errors::CsvLedgerResult;
use crate::{ClientId, LedgerItem};

#[derive(Debug)]
pub struct Exchange(HashMap<ClientId, Account>);

impl Exchange {
    /// creates a new Exchange struct
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// handle an item from the ledger csv
    pub fn handle_ledger_item(&mut self, ledger_item: LedgerItem) -> CsvLedgerResult<()> {
        self.get_or_create_bank_account(ledger_item.client())
            .handle_ledger_item(ledger_item)?;
        Ok(())
    }

    /// If the Exchange do not have an account with the given client_id, then create it.
    /// In any case return the account as a mutable reference
    fn get_or_create_bank_account(&mut self, client_id: ClientId) -> &mut Account {
        self.0.entry(client_id).or_default()
    }
}
