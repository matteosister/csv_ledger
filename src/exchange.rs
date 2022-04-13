use std::collections::HashMap;

use crate::account::Account;
use crate::errors::CsvLedgerResult;
use crate::{ClientId, LedgerItem};

#[derive(Debug)]
pub struct Exchange(HashMap<ClientId, Account>);

impl Exchange {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn handle_ledger_item(&mut self, ledger_item: LedgerItem) -> CsvLedgerResult<()> {
        self.get_or_create_bank_account(ledger_item.client())
            .handle_ledger_item(ledger_item)?;
        Ok(())
    }

    fn get_or_create_bank_account(&mut self, client_id: ClientId) -> &mut Account {
        self.0.entry(client_id).or_insert(Account::default())
    }
}
