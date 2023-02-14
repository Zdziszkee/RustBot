use std::env;
use std::fs::{File};
use std::io::{Write};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use tokio::fs;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
struct Wallet<> {
    id: String,
    transactions: Vec<Transaction>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let mut config_path = env::current_dir()?;
    config_path.push("botconfig.json");

    let mut config = match File::open(&config_path) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(&config_path).unwrap();
            let wallets = vec!["walletid1", "walletid2", "walletid3"];
            let serialized = serde_json::to_string(&wallets).unwrap();
            new_file.write_all(serialized.as_ref())?;
            new_file
        }
    };
    let contents: String = fs::read_to_string(config_path).await?;
    let array: Vec<String> = serde_json::from_str(&contents)?;
    for x in array {
        println!("{}", x)
    }
    let response: Value = reqwest::get("https://api.etherscan.io/api?module=account&action=txlist&address=0xc5102fE9359FD9a28f877a67E36B0F050d81a3CC&startblock=0&endblock=99999999&page=1&offset=10&sort=asc&apikey=NYA5ZPRQDECFZBINXU3GE6BTQQVWZN3FHG")
        .await?
        .json()
        .await?;

    let transactions: &Vec<Value> = response["result"].as_array().unwrap();
    for transaction in transactions {
        println!("{}", transaction)
    }
    let duration = start.elapsed();
    println!("Operation took {} miliseconds to complete", duration.as_millis());
    Ok(())
}