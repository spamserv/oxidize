use sha2::{Digest, Sha256};

use crate::blockchain::{Block, Transaction};

pub struct HashHelper;

impl HashHelper {
    pub fn generate_hash(previous_hash: &String, difficulty: u8, timestamp: &String, transactions: &Vec<Transaction>, nonce: u64) -> String {
        let combined_string = format!("{}{}{}{:?}{}", previous_hash, difficulty, timestamp, transactions, nonce);
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let hash_result = hasher.finalize();
        let hash_result = format!("{:x}", hash_result);
        hash_result
    }

    pub fn is_valid_hash(block: &Block) -> bool{
        let hash = Self::generate_hash(block.header().previous_hash(), block.header().difficulty(), block.header().timestamp(), block.body().transactions(), block.header().nonce());
        &hash == block.header().current_hash()
    }
}

