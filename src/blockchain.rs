

use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));
        blockchain.blocks.push(genesis_block);
        blockchain
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);
        self.blocks.push(new_block);
    }
}