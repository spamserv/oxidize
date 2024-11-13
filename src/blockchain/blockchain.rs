// Imports
use chrono::Utc;
use thiserror::Error;

// Modules/Crates
use crate::helpers::HashHelper;

// Consts
const BLOCKCHAIN_INITIAL_DIFFICULTY: u8 = 4;
const BLOCKCHAIN_INITIAL_NONCE: u64 = 0;
#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>
}

#[derive(Debug, Clone)]
pub struct Block {
    header: BlockHeader,
    body: BlockBody
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    timestamp: String,
    previous_hash: String,
    current_hash: String,
    nonce: u64,
    difficulty: u8,
}

#[derive(Debug, Clone)]
pub struct BlockBody {
    transactions: Vec<BlockTransaction>
}

#[derive(Debug, Clone)]
pub struct BlockTransaction {
    inputs: String,
    outputs: String,
    metadata: TransactionMetadata,
}

#[derive(Debug, Clone)]
struct TransactionMetadata {
    sender: String,
    receiver: String,
    value: String
}

#[derive(Error, Debug)]
pub enum BlockValidationError {
    #[error("Block not found with the specified hash")]
    BlockNotFound,
    #[error("Blockchain must have at least 2 blocks")]
    InsufficientBlocks,
    #[error("Invalid block hash format or value")]
    InvalidHash,
    #[error("Previous block not found in chain")]
    PreviousBlockNotFound,
    #[error("Previous hash mismatch")]
    PreviousHashMismatch,
    #[error("Block timestamp must be greater than previous block")]
    InvalidTimestamp,
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

    pub fn validate_single_block(&mut self, hash: &String) -> Result<&Block, BlockValidationError> {
        let block= self.blocks
            .iter()
            .find(|b| b.header.current_hash == *hash)
            .ok_or(BlockValidationError::BlockNotFound)?;

        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks)
        }

        if !HashHelper::is_valid_hash(&block) {
            return Err(BlockValidationError::InvalidHash)
        }

        let prev_block = self.blocks
            .iter()
            .find(|b| b.header.current_hash == block.header.previous_hash)
            .ok_or(BlockValidationError::PreviousBlockNotFound)?;
        
        if prev_block.header.current_hash == block.header.previous_hash {
            return Err(BlockValidationError::PreviousHashMismatch)
        }

        if block.header.timestamp <= prev_block.header.timestamp {
            return Err(BlockValidationError::InvalidTimestamp)
        }

        Ok(block)

    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
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

    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    pub fn body(&self) -> &BlockBody {
        &self.body
    }

    pub fn create_genesis_block() -> Self {
        let previous_hash = "0".repeat(64);
        let timestamp = Utc::now().to_rfc3339();
        let transactions = Vec::new();
        let mut nonce = BLOCKCHAIN_INITIAL_NONCE;
        let mut hash_result = String::new();
        let blockchain_difficulty_str = "0".repeat(BLOCKCHAIN_INITIAL_DIFFICULTY as usize);
        
        loop {
            hash_result = HashHelper::generate_hash(&previous_hash, BLOCKCHAIN_INITIAL_DIFFICULTY, &timestamp, &transactions, nonce + 1);
            if hash_result.starts_with(&blockchain_difficulty_str) {
                break;
            }
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
            transactions
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
        let transactions = Vec::new();
        let blockchain_difficulty_str = "0".repeat(blockchain_difficulty as usize);
        
        while !hash_result.starts_with(&blockchain_difficulty_str){
            hash_result = HashHelper::generate_hash(&previous_hash, blockchain_difficulty, &timestamp, &transactions, nonce + 1);
            nonce += 1
        }

        let header = BlockHeader {
            previous_hash: previous_hash.to_string(),
            difficulty: blockchain_difficulty,
            nonce,
            timestamp: Utc::now().to_rfc3339(),
            current_hash: hash_result
        };

        let body = BlockBody {
            transactions
        };

        Self {
            header,
            body
        }
    }
}

impl BlockHeader {
    pub fn current_hash(&self) -> &String {
        &self.current_hash
    }

    pub fn previous_hash(&self) -> &String {
        &self.previous_hash
    }

    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn timestamp(&self) -> &String {
        &self.timestamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }
}

impl BlockBody {
    pub fn transactions(&self) -> &Vec<BlockTransaction> {
        &self.transactions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Works!")
    }
}
