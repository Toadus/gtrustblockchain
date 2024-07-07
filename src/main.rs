mod block;
mod blockchain;
mod solana_api;

use blockchain::Blockchain;

use std::{io::Read, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() {
    println!("Hello, gtrustblockchain!");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server running on port 8080");

    tokio::spawn(async {
        run_blockchain_tasks().await;
    });

    loop {
        for stream in listener.incoming(){
            tokio::spawn(async move {
                handle_connection(stream.unwrap()).await;
            });
        }
    }
 
}

async fn run_blockchain_tasks() {
    let mut blockchain = Blockchain::new().await;

    blockchain.add_block("First block after genesis".to_string()).await;
    blockchain.add_block("Second block after genesis".to_string()).await;

    for block in blockchain.blocks {
        println!("{:?}", block);
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                println!("Connection closed");
            } else {
                println!("Received: {:?}", &buffer[..n]);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}
