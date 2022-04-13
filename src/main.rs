use std::env;
use std::error::Error;

use crate::exchange::Exchange;
pub use crate::input::*;

mod account;
mod errors;
mod exchange;
mod input;

pub type ClientId = u16;
pub type TransactionId = u32;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _binary_name = args.next();
    let csv_file = args
        .next()
        .expect("You need to pass a csv file name argument");
    let mut file_reader = csv::Reader::from_path(csv_file).expect("unable to read the file");
    let mut exchange = Exchange::new();
    for record in file_reader.records() {
        let ledger_item: LedgerItem = record.expect("unable to deserialize row").try_into()?;
        exchange.handle_ledger_item(ledger_item)?;
    }

    dbg!(exchange);

    Ok(())
}
