#[macro_use]
extern crate serde_derive;

mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_transaction::BlockchainTransaction;
use dotenv::dotenv;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    dotenv().ok(); // Load environment variables
    println!("enter your wallet address");
    let mut wallet_address = String::new();
    io::stdin()
        .read_line(&mut wallet_address)
        .expect("Failed to read line");

    blockchain_info_app(&wallet_address);
}

fn blockchain_info_app(address: &str) {
    let blockchain_status = blockchain_info::blockchain_status_request();
    println!(
        "\n\nQuerying {} - chain: {}\n\n",
        blockchain_status.blockbook.coin, blockchain_status.backend.chain
    );

    let blockchain_address = blockchain_info::blockchain_address_request(address);
    println!(
        "\n\nAnalyzing transactions for Bitcoin address {}\n",
        blockchain_address.address
    );

    thread::sleep(Duration::from_millis(2500));

    println!(
        "You have a total of {} transactions!\n",
        blockchain_address.txids.len()
    );
    println!("Do you want to query these transactions? (y/n)\n");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    if command.trim().eq("y") {
        process_transactions(address, &blockchain_address.txids);
    }
}

fn process_transactions(address: &str, txids: &[String]) {
    let sleep_time = Duration::from_millis(2500);
    println!("\nWe will look up the following transactions:\n");
    println!("{:#?}", txids);

    thread::sleep(sleep_time);

    let mut balance: i32 = 0;
    for tx_id in txids {
        let blockchain_transaction = blockchain_info::blockchain_transaction_request(&tx_id);

        let match_address = String::from(address);

        let subtotal_vin: i32 = blockchain_transaction
            .vin
            .iter()
            .filter(|tx| tx.addresses.contains(&match_address))
            .map(|tx| tx.value.parse::<i32>().unwrap_or(0))
            .sum();

        let subtotal_vout: i32 = blockchain_transaction
            .vout
            .iter()
            .filter(|tx| tx.addresses.contains(&match_address))
            .map(|tx| tx.value.parse::<i32>().unwrap_or(0))
            .sum();

        balance += subtotal_vout - subtotal_vin;

        println!("-----------------------------------------------------");
        println!("TX ID:           {}", tx_id);
        println!("SATOSHIS IN:     {}", subtotal_vout);
        println!("SATOSHIS OUT:    {}", subtotal_vin);
        println!("BALANCE:         {}", balance);
        println!("-----------------------------------------------------");
    }

    println!("CURRENT BALANCE:     {}", balance);
    println!("         IN BTC:     {}\n\n", balance as f32 * 0.00000001);
}
