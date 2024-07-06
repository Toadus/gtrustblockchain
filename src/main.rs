

mod block;
mod blockchain;
mod solana_api;

use blockchain::Blockchain;
fn main() {
    println!("Hello, gtrustblockchain!");

    let mut blockchain = Blockchain::new();


    //https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD,EUR

    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Second block after genesis".to_string());

    for block in blockchain.blocks {
        println!("{:?}", block);
    }
}
