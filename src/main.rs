mod blockchain_address;
mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;

extern crate serde_derive;
extern crate serde_json;
use crate::blockchain_address::*;
use crate::blockchain_info::*;
use crate::blockchain_status::*;
use crate::blockchain_transaction::*;

fn main() {
    let blockchain_status = blockchain_info::blockchain_status_request();
    println!(
        "\n\n querying {} - chain : {}\n\n",
        &blockchain_status.blockbook.coin, &blockchain_status.backend.chain
    )
}
