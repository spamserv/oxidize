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
            let genesis_block = Block::new("".to_string());
            let blocks = vec![genesis_block];
            return Self {
                blocks
            }
        }

    }

    impl Block {
        fn new(stuff: String) -> Self {
            if stuff.is_empty() {
                let genesis_block = Block::create_genesis_block();
                genesis_block
            } else {
                let genesis_block = Block::create_genesis_block();
                genesis_block
            }

        }
        fn create_genesis_block() -> Self {
            let previous_hash = "0".repeat(64);
            let difficulty = BLOCKCHAIN_INITIAL_DIFFICULTY;
            let timestamp = Utc::now().to_rfc3339();
            let transactions = Vec::new();

            let mut nonce = BLOCKCHAIN_INITIAL_NONCE;

            let blockchain_initial_hash = "0".repeat(6);
            let mut hash_result = String::new();
            
            while !hash_result.starts_with(&blockchain_initial_hash){
                hash_result = Block::generate_hash(&previous_hash, difficulty, &timestamp, &transactions, nonce + 1);
                nonce += 1
            }
            

            let header = BlockHeader {
                previous_hash: blockchain_initial_hash,
                difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY,
                nonce: nonce,
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
                let combined_string = format!("{}{}{}{:?}{}", previous_hash, difficulty, timestamp, transactions, nonce);
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
