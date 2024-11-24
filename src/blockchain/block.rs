use chrono::Utc;
use thiserror::Error;
use crate::{config::{BLOCKCHAIN_INITIAL_DIFFICULTY, BLOCKCHAIN_INITIAL_NONCE}, transaction::Transaction, utils::HashHelper};


#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub timestamp: String,
    pub previous_hash: String,
    pub current_hash: String,
    pub nonce: u64,
    pub difficulty: u8,
}

#[derive(Debug, Clone)]
pub struct BlockBody {
    pub transactions: Vec<Transaction>
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
    #[error("Hash `from_hash` index needs to be lower than `to_hash`")]
    RangeIndexFault
}


impl Block {
    /// Generates a new block based on previous block hash, transactions that are meant to go into the block
    /// and current blockchain difficulty 
    pub fn new(previous_hash: &String, transactions: &Vec<Transaction>, blockchain_difficulty: u8) -> Self {
        Block::create_data_block(previous_hash, transactions, blockchain_difficulty)
    }

    /// Returns BlockHeader
    pub fn header(&self) -> &BlockHeader {
        &self.header
    }

    /// Returns BlockBody
    pub fn body(&self) -> &BlockBody {
        &self.body
    }

    /// Crates genesis block, where previoush hash is `"0".repeat(64)`, based on:
    /// - previous hash
    /// - current timestamp
    /// - blockchain difficulty
    /// - transactions (empty)
    /// - nonce that is iterated until the blockchain difficulty is met
    pub fn create_genesis_block(coinbase_transaction: Transaction) -> Self {
        let previous_hash = "0".repeat(64);
        let timestamp = Utc::now().to_rfc3339();
        let mut transactions = Vec::new();
        let mut nonce = BLOCKCHAIN_INITIAL_NONCE;
        let mut hash_result;
        let blockchain_difficulty_str = "0".repeat(BLOCKCHAIN_INITIAL_DIFFICULTY as usize);
        
        transactions.push(coinbase_transaction);

        loop {
            hash_result = HashHelper::generate_hash(&previous_hash, BLOCKCHAIN_INITIAL_DIFFICULTY, &timestamp, &transactions, nonce);
            if hash_result.starts_with(&blockchain_difficulty_str) {
                break;
            }
            nonce += 1
        }

        let header = BlockHeader {
            previous_hash: previous_hash.to_string(),
            difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY,
            nonce,
            timestamp,
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

    /// Crates data block, based on:
    /// - previous block hash
    /// - current timestamp
    /// - blockchain difficulty
    /// - transactions included in the block
    /// - nonce that is iterated until the blockchain difficulty is met
    pub fn create_data_block(previous_hash: &String, transactions: &Vec<Transaction>, blockchain_difficulty: u8) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut nonce = BLOCKCHAIN_INITIAL_NONCE;
        let mut hash_result;
        let blockchain_difficulty_str = "0".repeat(blockchain_difficulty as usize);
        
        loop {
            hash_result = HashHelper::generate_hash(previous_hash, blockchain_difficulty, &timestamp, transactions, nonce);
            if hash_result.starts_with(&blockchain_difficulty_str) {
                break;
            }
            nonce += 1
        }    

        let header = BlockHeader {
            previous_hash: previous_hash.to_string(),
            difficulty: blockchain_difficulty,
            nonce,
            timestamp,
            current_hash: hash_result
        };

        let body = BlockBody {
            transactions: transactions.to_vec()
        };

        Self {
            header,
            body
        }
    }

}

/// BlockHeader structure
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

/// BlockBody structure
impl BlockBody {
    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}