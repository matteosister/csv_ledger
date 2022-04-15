use csv::Writer;
use rust_decimal::Decimal;
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
struct Row<'a> {
    r#type: &'a str,
    client: &'a str,
    tx: u32,
    amount: Decimal,
}

fn main() {
    let file = File::create("data/big.csv").expect("unable to create file");
    let mut wtr = Writer::from_writer(file);

    for i in 1..=10000 {
        wtr.serialize(Row {
            r#type: "deposit",
            client: &i.to_string(),
            tx: i as u32,
            amount: Decimal::new(100, 1),
        });
    }
}
