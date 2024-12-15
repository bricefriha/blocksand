use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String,
}

impl Block {
    // Block constructor
    pub fn new(index: u32, data: String, previous_hash: String, validator: String) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
            validator,
        }
    }

    pub fn calculate_hash(index: u32, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, data, previous_hash));
        format!("{:x}", hasher.finalize())
    }
}
