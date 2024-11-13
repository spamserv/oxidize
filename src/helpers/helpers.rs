use crate::blockchain::Block;

pub struct HashHelper;

impl HashHelper {
    pub fn generate_hash(previous_hash: &String, difficulty: u8, timestamp: &String, transactions: &Vec<BlockTransaction>, nonce: u64) -> String {
        let combined_string = format!("{}{}{}{:?}{}", previous_hash, &difficulty, timestamp, transactions, nonce);
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let hash_result = hasher.finalize();
        let hash_result = format!("{:x}", hash_result);
        hash_result
    }

    pub fn is_valid_hash(block: &Block) {
        let hash = Self::generate_hash(block.previous_hash, block.blockchain_difficulty, block.timestamp, block.transactions, block.nonce);
        hash == block.header.current_hash;
    }
}

