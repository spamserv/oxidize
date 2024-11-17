//! This module contains the core logic for the blockchain.
//!
//! It defines the structure of the blockchain, how blocks are created, and
//! the validation of transactions within the blockchain.
//! 
use core::time;
use std::{collections::HashMap, vec};
use super::{wallet, Address, Transaction, TransactionInput, TransactionManager, TransactionOutput, Wallet};

// Imports
use chrono::Utc;
use colored::Colorize;
use thiserror::Error;

// Modules/Crates
use crate::helpers::HashHelper;
use crate::config::{BLOCKCHAIN_COINBASE_FEE, BLOCKCHAIN_INITIAL_DIFFICULTY, BLOCKCHAIN_INITIAL_NONCE};

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>, // Mined blocks
    mempool: Vec<TransactionInput>, // Pending transactions
    utxo: HashMap<String, Vec<TransactionOutput>>, // Unspent transaction outputs used for inputs into other transactions
    ledger: Vec<Transaction>, // The blockchain ledger keeps track of every transaction and the issuance of new coins through coinbase transactions.
    config: BlockchainConfig,
    wallet: Wallet, // Mining wallet to collect coinbase block fees
}

#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    difficulty: u8,
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
    transactions: Vec<Transaction>
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

/// Blockchain structure, consisting of vector of blocks and its configuration
impl Blockchain {
    
    /// Builds a blockchain from scratch
    /// Creates genesis block based on the BLOCKCHAIN_INITIAL_DIFFICULTY
    pub fn build() -> Self {
        let config = BlockchainConfig {
            difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY
        };

        let mut wallet = Wallet::new("MiningFeeWallet#1".to_string());
        wallet.create_new_account();
        let coinbase_account = wallet.accounts()
            .get(0)
            .expect("No coinbase error available.");

        let coinbase_address = coinbase_account.address();

        let coinbase_transaction = TransactionManager::create_coinbase_transaction(&coinbase_address.id(), BLOCKCHAIN_COINBASE_FEE);

        let genesis_block = Block::create_genesis_block(coinbase_transaction);

        let blocks = vec![genesis_block.clone()]; // Clone it because it has to be borrowed to reward_block_finder
        let mempool  = vec![];
        let utxo = HashMap::new();
        let ledger = vec![];
        
        let mut blockchain = Self {
            blocks,
            config,
            mempool,
            utxo,
            ledger,
            wallet
        };

        blockchain.reward_block_finder(&genesis_block);

        blockchain
    }

    /// Returns blockchain configuration
    pub fn config(&self) -> &BlockchainConfig {
        &self.config
    }

    /// Mines a new block
    /// Based on the previous block hash and transactions that will go inside the block
    pub fn add_block(&mut self){
        let last_block_header = &self.blocks.last().unwrap().header;
        let new_block = Block::new(&last_block_header.current_hash, &vec![], last_block_header.difficulty);
        self.blocks.push(new_block);
    }

    /// Validates a single block by checking several factors
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_single_block(&mut self, hash: &String) -> Result<(), BlockValidationError> {
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
        
        if prev_block.header.current_hash != block.header.previous_hash {
            return Err(BlockValidationError::PreviousHashMismatch)
        }

        if block.header.timestamp <= prev_block.header.timestamp {
            return Err(BlockValidationError::InvalidTimestamp)
        }

        Ok(())

    }

    /// Validates the full chain by looping through every block
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_full_chain(&mut self) -> Result<(), BlockValidationError> {
        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks)
        }

        for (idx, block) in self.blocks.iter().enumerate() {
            if !HashHelper::is_valid_hash(&block) {
                return Err(BlockValidationError::InvalidHash)
            }
            
            if idx != 0 {
                let prev_block = self.blocks
                .iter()
                .find(|b| b.header.current_hash == block.header.previous_hash)
                .ok_or(BlockValidationError::PreviousBlockNotFound)?;

                if prev_block.header.current_hash != block.header.previous_hash {
                    return Err(BlockValidationError::PreviousHashMismatch)
                }

                if block.header.timestamp <= prev_block.header.timestamp {
                    return Err(BlockValidationError::InvalidTimestamp)
                }
            }
        }
        Ok(())
    }

    /// Validates a range between `from_hash` and `to_hash`
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_range_chain(&mut self, from_hash: &String, to_hash: &String) -> Result<(), BlockValidationError> {
        let (from_index, to_index) = match self.find_hash_indices(from_hash, to_hash) {
            None => return Err(BlockValidationError::RangeIndexFault),
            Some((from_index, to_index)) => (from_index, to_index),
        };

        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks)
        }

        for idx in from_index..=to_index {
            
            let block = self.blocks
                .get(idx)
                .ok_or(BlockValidationError::BlockNotFound)?;
    
            if !HashHelper::is_valid_hash(&block) {
                return Err(BlockValidationError::InvalidHash)
            }
            
            if idx != 0 {
                let prev_block = self.blocks
                .iter()
                .find(|b| b.header.current_hash == block.header.previous_hash)
                .ok_or(BlockValidationError::PreviousBlockNotFound)?;

                if prev_block.header.current_hash != block.header.previous_hash {
                    return Err(BlockValidationError::PreviousHashMismatch)
                }

                if block.header.timestamp <= prev_block.header.timestamp {
                    return Err(BlockValidationError::InvalidTimestamp)
                }
            }
        }
        Ok(())

    }

    /// Finds indexes of:
    /// 1) the hash from where `validate_range_chain` should start validation
    /// 2) the hash to where `validate_range_chain` should end validation, inclusive
    pub fn find_hash_indices(&self, from_hash: &str, to_hash: &str) -> Option<(usize, usize)> {
        let mut hash_to_index: HashMap<&str, usize> = HashMap::new();

        for (i, block) in self.blocks.iter().enumerate() {
            hash_to_index.entry(&block.header.current_hash).or_insert(i);
        }

        let from_index = hash_to_index.get(from_hash);
        let to_index = hash_to_index.get(to_hash);

        match (from_index, to_index) {
            (Some(from_idx), Some(to_idx)) => Some((*from_idx, *to_idx)),
            _ => None,
        }
    }

    /// Returns copy of the blocks
    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    fn reward_block_finder(&mut self, block: &Block) {
        let coinbase_transaction = block.body()
            .transactions()
            .get(0)
            .expect(&"No (coinbase) Transactions in the Transactions vector of a Block.".red())
            .clone();

        let coinbase_transaction_output = coinbase_transaction
            .outputs()
            .get(0)
            .expect(&"No (coinbase) TransactionOutput in the TransactionOutputs vector.".red())
            .clone();

        let recipient_address = coinbase_transaction_output.recipient_address.to_string();

        // Push the whole transaction to ledger
        self.ledger.push(coinbase_transaction);

        // Update UTXO hash map for that address with a new transaction output
        self.utxo.entry(recipient_address)
            .or_default()
            .push(coinbase_transaction_output);
    }


}

impl Block {
    /// Generates a new block based on previous block hash, transactions that are meant to go into the block
    /// and current blockchain difficulty 
    fn new(previous_hash: &String, transactions: &Vec<Transaction>, blockchain_difficulty: u8) -> Self {
        if transactions.is_empty() {
            let genesis_block = Block::create_data_block(previous_hash, transactions, blockchain_difficulty);
            genesis_block
        } else {
            let genesis_block = Block::create_data_block(previous_hash, transactions, blockchain_difficulty);
            genesis_block
        }
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
        let mut hash_result = String::new();
        let blockchain_difficulty_str = "0".repeat(BLOCKCHAIN_INITIAL_DIFFICULTY as usize);
        
        loop {
            hash_result = HashHelper::generate_hash(&previous_hash, BLOCKCHAIN_INITIAL_DIFFICULTY, &timestamp, &transactions, nonce);
            if hash_result.starts_with(&blockchain_difficulty_str) {
                break;
            }
            nonce += 1
        }

        transactions.push(coinbase_transaction);

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
        let mut hash_result = String::new();
        let blockchain_difficulty_str = "0".repeat(blockchain_difficulty as usize);
        
        loop {
            hash_result = HashHelper::generate_hash(&previous_hash, blockchain_difficulty, &timestamp, &transactions, nonce);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Works!")
    }
}
