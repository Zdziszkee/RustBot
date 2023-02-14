use std::env;
use std::fs::{File};
use std::io::{Write};
use std::time::Instant;
use serde_json::{Value};
use tokio::fs;
use std::time::SystemTime;
use dashmap::DashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //map holding most recentTransaction for each wallet
    let cache: DashMap<String, String> = DashMap::new();

    let start = Instant::now();

    let mut config_path = env::current_dir()?;
    config_path.push("botconfig.json");

    match File::open(&config_path) {
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
    let wallets: Vec<String> = serde_json::from_str(&contents)?;
    for wallet in wallets {
       // cache.insert(wallet, String::from(""));
    }


    let response: Value = reqwest::get("https://api.etherscan.io/api?module=account&action=txlist&address=0xa7b5ca022774bd02842932e4358ddcbea0ccaade&startblock=0&endblock=99999999&page=1&offset=10&sort=desc&apikey=NYA5ZPRQDECFZBINXU3GE6BTQQVWZN3FHG")
        .await?
        .json()
        .await?;
    let transactions: &Vec<Value> = response["result"].as_array().unwrap();
    for transaction in transactions {
        let time_stamp = &transaction["timeStamp"].as_str();
        let date_time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(time_stamp.unwrap().parse().unwrap());
        let formatted = format!("{}", chrono::DateTime::<chrono::Local>::from(date_time));
        println!("{}", formatted);
    }
    let duration = start.elapsed();
    println!("Operation took {} miliseconds to complete", duration.as_millis());

    Ok(())
}