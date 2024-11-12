pub mod blockchain {
    const BLOCKCHAIN_INITIAL_DIFFICULTY: u8 = 4;
    const BLOCKCHAIN_INITIAL_NONCE: u64 = 0;

    use sha2::{Digest, Sha256};
    use chrono::Utc;

    #[derive(Debug)]
    pub struct Blockchain {
        pub blockchain_difficulty: String,
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
        transactions: Vec<BlockTransaction>
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
            let blockchain_difficulty= "0".repeat(BLOCKCHAIN_INITIAL_DIFFICULTY as usize);
            let genesis_block = Block::new(&blockchain_difficulty, "".to_string());
            let blocks = vec![genesis_block];
            return Self {
                blockchain_difficulty,
                blocks
            }
        }

        pub fn blockchain_difficulty(&self) -> &String {
            &self.blockchain_difficulty
        }

    }

    impl Block {
        fn new(blockchain_difficulty: &String, stuff: String) -> Self {
            if stuff.is_empty() {
                let genesis_block = Block::create_genesis_block(&blockchain_difficulty);
                genesis_block
            } else {
                let genesis_block = Block::create_genesis_block(&blockchain_difficulty);
                genesis_block
            }

        }

        fn create_genesis_block(blockchain_difficulty: &String) -> Self {
            let previous_hash = "0".repeat(64);
            let timestamp = Utc::now().to_rfc3339();
            let transactions = Vec::new();

            let mut nonce = BLOCKCHAIN_INITIAL_NONCE;

            let mut hash_result = String::new();
            
            while !hash_result.starts_with(blockchain_difficulty){
                hash_result = Block::generate_hash(&previous_hash, blockchain_difficulty, &timestamp, &transactions, nonce + 1);
                nonce += 1
            }
            

            let header = BlockHeader {
                previous_hash: blockchain_difficulty.to_string(),
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

        fn generate_hash(previous_hash: &String, difficulty: &String, timestamp: &String, transactions: &Vec<BlockTransaction>, nonce: u64) -> String {
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
