
use serde::Deserialize;
use reqwest::blocking::Client;
use std::error::Error;


#[derive(Deserialize, Debug)]
pub struct RpcResponse {
    pub result: u64,
    // You can add more fields if needed, such as error, id, jsonrpc, etc.
}

pub fn fetch_block_height() -> Result<RpcResponse, Box<dyn Error>> {
    let client = Client::new();
    let response = client.post("https://api.mainnet-beta.solana.com")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlockHeight",
            "params": []
        }))
        .send()?
        .json::<RpcResponse>()?;
    Ok(response)
}