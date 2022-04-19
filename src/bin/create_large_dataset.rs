use csv::Writer;
use rand::Rng;
use rust_decimal::Decimal;
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
struct Row<'a> {
    r#type: &'a str,
    client: &'a str,
    tx: u32,
    amount: Option<Decimal>,
}

fn main() {
    let file = File::create("data/big.csv").expect("unable to create file");
    let mut wtr = Writer::from_writer(file);
    let mut rng = rand::thread_rng();

    for i in 1..=65535 {
        let _ = wtr.serialize(Row {
            r#type: "deposit",
            client: &i.to_string(),
            tx: i as u32,
            amount: Some(Decimal::new(100, 1)),
        });
    }
    for i in 1..=65535 {
        let rnd_bool: bool = rng.gen();
        if rnd_bool {
            let _ = wtr.serialize(Row {
                r#type: "dispute",
                client: &i.to_string(),
                tx: i as u32,
                amount: None,
            });
            let _ = wtr.serialize(Row {
                r#type: "chargeback",
                client: &i.to_string(),
                tx: i as u32,
                amount: Some(Decimal::new(100, 1)),
            });
        } else {
            let _ = wtr.serialize(Row {
                r#type: "withdrawal",
                client: &i.to_string(),
                tx: i as u32,
                amount: Some(Decimal::new(100, 1)),
            });
        }
    }
}
