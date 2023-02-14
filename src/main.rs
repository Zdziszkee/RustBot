use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wallet {
    id: String,
    transactions: Vec<Transaction>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = env::current_dir()?;
    config_path.push("botconfig.json");

    let mut config = match File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(&config_path).unwrap();
            let example_transaction = Transaction { id: String::from("exampleTransactionId") };
            let example_wallet = Wallet { id: String::from("examplewalletid"), transactions: vec![example_transaction] };
            let serialized = serde_json::to_string(&example_wallet).unwrap();
            new_file.write_all(serialized.as_ref())?;
            new_file
        }
    };

    let mut contents = String::new();
    config.read_to_string(&mut contents).unwrap();


    let response = reqwest::get("https://api.etherscan.io/api?module=account&action=balance&address=0xde0b295669a9fd93d5f28d9ec85e40f4cb697bae&tag=latest&apikey=NYA5ZPRQDECFZBINXU3GE6BTQQVWZN3FHG")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", response);
    Ok(())
}