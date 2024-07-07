use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::solana_api::fetch_block_height;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub data: String,
}

impl Block {
    pub async fn new(index: u64, previous_hash: String, mut data: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let nonce = 0;

        let mut sol_block_height:u64 = 0;
        
        match fetch_block_height().await {
            Ok(rpc_response) => {
                println!("Current Solana Block Height: {}", rpc_response.result);
                sol_block_height = rpc_response.result;
            }
            Err(e) => {
                eprintln!("Error fetching block height: {}", e);
            }
        }

        data += ":: Solana block height: ";
        data += &sol_block_height.to_string();

        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            nonce,
            data,
        };

        block.mine();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string());
        hasher.update(&self.data);
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self) {
        let mut iterations:u64 = 0;
        let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        while !self.hash.starts_with("00000") {
            self.nonce += 1;
            self.hash = self.calculate_hash();
            iterations += 1;
        }
        timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - timestamp;
        println!("Mined block {}. {} iterations completed in {} millis", self.index, iterations, timestamp);
    }
}