use crate::structs::block::Block;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub stakes: HashMap<String, u32>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::build_genesis_block()],
            stakes: HashMap::new(),
        }
    }

    pub fn build_genesis_block() -> Block {
        Block::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
            "None".to_string(),
        )
    }

    pub fn get_latest_block(&mut self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, data: String) {
        let latest_block = self.get_latest_block();
        let new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
            self.select_validator(),
        );
        self.chain.push(new_block);
    }

    pub fn add_stake(&mut self, validator: String, stake: u32) {
        let entry = self.stakes.entry(validator).or_insert(0);

        *entry += stake;
    }

    pub fn select_validator(&self) -> String {
        let total_stake: u32 = self.stakes.values().sum();
        let mut rng = rand::thread_rng();
        let mut choice = rng.gen_range(0..total_stake);

        if total_stake == 0 {
            panic!("failed: no validators available");
        }

        for (validator, &stake) in &self.stakes {
            if choice < stake {
                return validator.clone();
            }
            choice -= stake;
        }
        panic!("Validator selection failed")
    }
}
