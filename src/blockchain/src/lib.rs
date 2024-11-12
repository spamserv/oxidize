pub mod blockchain {
    const BLOCKCHAIN_INITIAL_DIFFICULTY: u8 = 4;
    const BLOCKCHAIN_INITIAL_NONCE: u64 = 0;

    use sha2::{Digest, Sha256};
    use chrono::Utc;

    #[derive(Debug)]
    pub struct Blockchain {
        blocks: Vec<Block>
    }

    #[derive(Debug)]
    struct Block {
       header: BlockHeader,
       body: BlockBody
    }

    #[derive(Debug)]
    struct BlockHeader {
        timestamp: String,
        previous_hash: String,
        current_hash: String,
        nonce: u64,
        difficulty: u8,
    }

    #[derive(Debug)]
    struct BlockBody {
        transactions: Vec<Option<BlockTransaction>>
    }

    #[derive(Debug)]
    struct BlockTransaction {
        inputs: String,
        outputs: String,
        metadata: TransactionMetadata,
    }

    #[derive(Debug)]
    struct TransactionMetadata {
        sender: String,
        receiver: String,
        value: String
    }

    impl Blockchain {
        pub fn build() -> Self {
            let genesis_block = Block::create_genesis_block();
            let blocks = vec![genesis_block];
            return Self {
                blocks
            }
        }

        pub fn add_block(&mut self){
            let last_block_header = &self.blocks.last().unwrap().header;
            let new_block = Block::new(&last_block_header.current_hash, &vec![], last_block_header.difficulty);
            self.blocks.push(new_block);
        }

    }

    impl Block {
        fn new(previous_hash: &String, transactions: &Vec<BlockTransaction>, blockchain_difficulty: u8) -> Self {
            if transactions.is_empty() {
                let genesis_block = Block::create_data_block(previous_hash, transactions, blockchain_difficulty);
                genesis_block
            } else {
                let genesis_block = Block::create_data_block(previous_hash, transactions, blockchain_difficulty);
                genesis_block
            }
        }

        pub fn create_genesis_block() -> Self {
            let previous_hash = "0".repeat(64);
            let timestamp = Utc::now().to_rfc3339();
            let transactions = Vec::new();
            let mut nonce = BLOCKCHAIN_INITIAL_NONCE;
            let mut hash_result = String::new();
            let blockchain_difficulty_str = "0".repeat(BLOCKCHAIN_INITIAL_DIFFICULTY as usize);
            
            
            while !hash_result.starts_with(&blockchain_difficulty_str){
                hash_result = Block::generate_hash(&previous_hash, BLOCKCHAIN_INITIAL_DIFFICULTY, &timestamp, &transactions, nonce + 1);
                nonce += 1
            }

            let header = BlockHeader {
                previous_hash: previous_hash.to_string(),
                difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY,
                nonce,
                timestamp: Utc::now().to_rfc3339(),
                current_hash: hash_result
            };

            let body = BlockBody {
                transactions: Vec::new()
            };

            Self {
                header,
                body
            }
        }

        pub fn create_data_block(previous_hash: &String, transactions: &Vec<BlockTransaction>, blockchain_difficulty: u8) -> Self {
            let timestamp = Utc::now().to_rfc3339();
            let mut nonce = BLOCKCHAIN_INITIAL_NONCE;
            let mut hash_result = String::new();
            let blockchain_difficulty_str = "0".repeat(blockchain_difficulty as usize);
            
            while !hash_result.starts_with(&blockchain_difficulty_str){
                hash_result = Block::generate_hash(&previous_hash, blockchain_difficulty, &timestamp, &transactions, nonce + 1);
                nonce += 1
            }

            let header = BlockHeader {
                previous_hash: previous_hash.to_string(),
                difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY,
                nonce,
                timestamp: Utc::now().to_rfc3339(),
                current_hash: hash_result
            };

            let body = BlockBody {
                transactions: Vec::new()
            };

            Self {
                header,
                body
            }
        }

        fn generate_hash(previous_hash: &String, difficulty: u8, timestamp: &String, transactions: &Vec<BlockTransaction>, nonce: u64) -> String {
                let combined_string = format!("{}{}{}{:?}{}", previous_hash, &difficulty, timestamp, transactions, nonce);
                let mut hasher = Sha256::new();
                hasher.update(combined_string);
                let hash_result = hasher.finalize();
                let hash_result = format!("{:x}", hash_result);
                hash_result
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
