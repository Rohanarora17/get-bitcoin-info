use crate::blockchain_address::*;
use crate::blockchain_status::*;
use crate::blockchain_transaction::*;
use dotenv;
use reqwest;
use serde_json::Result;
use tokio;
const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header(
            "api-key",
            dotenv::var("API_KEY").expect("could not find api key"),
        )
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    serde_json::from_str(&response).expect("failed the parse json")
}
