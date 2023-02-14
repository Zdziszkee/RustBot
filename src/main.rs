use std::env;
use std::fs::{File};
use std::io::{Write};
use std::time::Instant;
use serde_json::{Value};
use tokio::{fs, task, time};
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

        loop {
            let copy = wallet.clone();
            // Spawn a new task to run the repeating code asynchronously.
            let handle = task::spawn(async move {
                let request = format!("https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=10&sort=desc&apikey=NYA5ZPRQDECFZBINXU3GE6BTQQVWZN3FHG", copy);
                let response: Value = reqwest::get(request)
                    .await.unwrap()
                    .json()
                    .await.unwrap();

                let transactions: &Vec<Value> = response["result"].as_array().unwrap();

                for transaction in transactions {
                    let time_stamp = &transaction["timeStamp"].as_str();
                    let date_time = SystemTime::UNIX_EPOCH + time::Duration::from_secs(time_stamp.unwrap().parse().unwrap());
                    let formatted = format!("{}", chrono::DateTime::<chrono::Local>::from(date_time));
                    println!("{}", formatted);
                }

                time::sleep(time::Duration::from_secs(180)).await;
            });

            // Wait for the task to complete.
            handle.await.unwrap();
        }
    }

    Ok(())
}