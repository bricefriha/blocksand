use crate::structs::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::build_genesis_block()],
        }
    }

    pub fn build_genesis_block() -> Block {
        Block::new(0, "Genesis Block".to_string(), "0".to_string())
    }

    pub fn get_latest_block(&mut self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, data: String) {
        let latest_block = self.get_latest_block();
        let new_block = Block::new(latest_block.index + 1, data, latest_block.hash.clone());
        self.chain.push(new_block);
    }
}
