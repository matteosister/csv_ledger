use std::env;
use std::error::Error;

use crate::bank::Bank;
pub(crate) use crate::input::*;

mod account;
mod bank;
mod errors;
mod input;

/// A client id
pub type ClientId = u16;

/// A transaction id
pub type TransactionId = u32;

fn main() -> Result<(), Box<dyn Error>> {
    // parse arguments. I could have used clap here, but it's overkill for just a parameter :)
    let csv_file = env::args()
        .nth(1)
        .expect("You need to pass a csv file name argument");

    // create a bank instance from the given csv file
    let bank = Bank::from_file(&csv_file)?;

    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for account in bank.accounts() {
        writer.serialize(account)?;
    }

    Ok(())
}
