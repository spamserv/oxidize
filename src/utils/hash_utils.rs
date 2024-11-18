//! This module handles:
//! - HashHelper functions to help with all hash related things, such as generating new hashes and validating block hashes

use sha2::{Digest, Sha256};

use crate::blockchain::Block;
use crate::transaction::Transaction;

/// HashHelper struct storing various helper methods related to hashing
pub struct HashHelper;

impl HashHelper {
    /// Generates hash based on previous block hash, difficulty, timestamp, transactions vector and nonce
    pub fn generate_hash(previous_hash: &String, difficulty: u8, timestamp: &String, transactions: &Vec<Transaction>, nonce: u64) -> String {
        let combined_string = format!("{}{}{}{:?}{}", previous_hash, difficulty, timestamp, transactions, nonce);
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let hash_result = hasher.finalize();
        let hash_result = format!("{:x}", hash_result);
        hash_result
    }

    /// Checks if current block hash valid hash
    /// by recalculating the hash using block data and comparing it to the currently stored hash
    pub fn is_valid_hash(block: &Block) -> bool{
        let hash = Self::generate_hash(block.header().previous_hash(), block.header().difficulty(), block.header().timestamp(), block.body().transactions(), block.header().nonce());
        &hash == block.header().current_hash()
    }
}


