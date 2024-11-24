//! This module contains the core logic for the blockchain.
//!
//! It defines the structure of the blockchain, how blocks are created, and
//! the validation of transactions within the blockchain.
//! 

// Imports
use std::{collections::HashMap, vec};
use colored::Colorize;
use std::error::Error;

// Modules/Crates
use crate::{
    utils::HashHelper, 
    config::{
        BLOCKCHAIN_COINBASE_BLOCK_FEE, 
        BLOCKCHAIN_COINBASE_GENESIS_BLOCK_FEE, 
        BLOCKCHAIN_INITIAL_DIFFICULTY,
        WEBSOCKET_URI
    }};
use crate::transaction::{Transaction, TransactionInput, TransactionManager, TransactionOutput};
use crate::wallet::Wallet;
use super::{Block, BlockValidationError, BlockchainListener};

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>, // Mined blocks
    mempool: Vec<TransactionInput>, // Pending transactions
    utxo: HashMap<String, Vec<TransactionOutput>>, // Unspent transaction outputs used for inputs into other transactions
    ledger: Vec<Transaction>, // The blockchain ledger keeps track of every transaction and the issuance of new coins through coinbase transactions.
    config: BlockchainConfig,
    wallet: Wallet,
}

#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    difficulty: u8,
}

/// Blockchain structure, consisting of vector of blocks and its configuration
impl Blockchain {
    
    /// Builds a blockchain from scratch
    /// Creates genesis block based on the BLOCKCHAIN_INITIAL_DIFFICULTY
    pub async fn build() -> Result<Self, Box <dyn Error>> {
        let config = BlockchainConfig {
            difficulty: BLOCKCHAIN_INITIAL_DIFFICULTY
        };

        // Websocket server for wallets to connect
        BlockchainListener::run(WEBSOCKET_URI.to_string()).await;

        let mut wallet = Wallet::new("MiningFeeWallet#1".to_string()).await?;

        wallet.create_new_account();
        let coinbase_account = wallet.accounts()
            .first()
            .expect("No coinbase error available.");

        let coinbase_address = coinbase_account.address();

        let coinbase_transaction = TransactionManager::create_coinbase_transaction(coinbase_address.id(), BLOCKCHAIN_COINBASE_GENESIS_BLOCK_FEE);

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

        Ok(blockchain)
    }

    /// Returns blockchain configuration
    pub fn config(&self) -> &BlockchainConfig {
        &self.config
    }

    /// Mines a new block
    /// Based on the previous block hash and transactions that will go inside the block
    pub fn add_block(&mut self){
        let last_block_header = &self.blocks.last().unwrap().header;

        let coinbase_account = self.wallet.accounts()
            .first()
            .expect("No coinbase error available.");
        let coinbase_address = coinbase_account.address();
        let coinbase_transaction = TransactionManager::create_coinbase_transaction(coinbase_address.id(), BLOCKCHAIN_COINBASE_BLOCK_FEE);

        // Get all transactions for the block
        let transactions = vec![coinbase_transaction];
        let new_block = Block::new(&last_block_header.current_hash, &transactions, last_block_header.difficulty);
        
        self.reward_block_finder(&new_block);
        self.push_new_block(new_block);
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

        if !HashHelper::is_valid_hash(block) {
            println!("Block hash: {}", block.header.current_hash);
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
            if !HashHelper::is_valid_hash(block) {
                println!("Failed on idx: {}, block hash: {}", idx, block.header.current_hash);
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
    pub fn validate_range_chain(&mut self, from_hash: &str, to_hash: &str) -> Result<(), BlockValidationError> {
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
    
            if !HashHelper::is_valid_hash(block) {
                println!("Failed on idx: {}, block hash: {}", idx, block.header.current_hash);
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
            .first()
            .unwrap_or_else( ||  { 
                panic!("{}", "No (coinbase) Transactions in the Transactions vector of a Block.".red().to_string()) 
            })
            .clone();

        let coinbase_transaction_output = coinbase_transaction
            .outputs()
            .first()
            .unwrap_or_else( ||  { 
                panic!("{}", "No (coinbase) TransactionOutput in the TransactionOutputs vector.".red().to_string())
            })
            .clone();

        let recipient_address = coinbase_transaction_output.recipient_address.to_string();

        self.push_transaction_to_ledger(coinbase_transaction);

        self.update_utxo_with_transaction(recipient_address, coinbase_transaction_output);
    }

    /// Push the whole transaction to ledger
    fn push_transaction_to_ledger(&mut self, transaction: Transaction) {
        self.ledger.push(transaction);
    }

    /// Update UTXO hash map for that address with a new transaction output
    fn update_utxo_with_transaction(&mut self, address: String, transaction_output: TransactionOutput) {
        self.utxo.entry(address)
            .or_default()
            .push(transaction_output);
    }

    /// Push newly mined block to the blocks vector of a blockchain
    fn push_new_block(&mut self, block: Block) {
        self.blocks.push(block);
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
