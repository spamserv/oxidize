//! This module contains the core logic for the blockchain.
//!
//! It defines the structure of the blockchain, how blocks are created, and
//! the validation of transactions within the blockchain.
//!

// Imports
use colored::Colorize;
use std::error::Error;
use std::sync::Arc;
use std::{collections::HashMap, vec};
use tokio::sync::Mutex;

// Modules/Crates
use super::{Block, BlockValidationError, BlockchainListener};
use crate::transaction::{Transaction, TransactionInput, TransactionManager, TransactionOutput};
use crate::wallet::{Account, Address, Wallet};
use crate::{
    config::{
        BLOCKCHAIN_COINBASE_BLOCK_FEE, BLOCKCHAIN_COINBASE_GENESIS_BLOCK_FEE,
        BLOCKCHAIN_INITIAL_DIFFICULTY, WEBSOCKET_URI,
    },
    utils::HashHelper,
};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,                            // Mined blocks
    mempool: Vec<TransactionInput>,                // Pending transactions
    utxo: HashMap<String, Vec<TransactionOutput>>, // Unspent transaction outputs used for inputs into other transactions
    ledger: Vec<Transaction>, // The blockchain ledger keeps track of every transaction and the issuance of new coins through coinbase transactions.
    config: Arc<BlockchainConfig>,
    wallet: Arc<Mutex<Wallet>>,
    pub listener: Arc<Mutex<BlockchainListener>>,
}

impl Clone for Blockchain {
    fn clone(&self) -> Self {
        Self {
            blocks: self.blocks.clone(),   // Mined blocks
            mempool: self.mempool.clone(), // Pending transactions
            utxo: self.utxo.clone(), // Unspent transaction outputs used for inputs into other transactions
            ledger: self.ledger.clone(), // The blockchain ledger keeps track of every transaction and the issuance of new coins through coinbase transactions.
            config: self.config.clone(),
            wallet: self.wallet.clone(),
            listener: self.listener.clone(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BlockchainConfig {
    difficulty: u8,
    pub addr: String,
}

impl BlockchainConfig {
    pub fn new(dynamic: bool) -> Self {
        let difficulty = BLOCKCHAIN_INITIAL_DIFFICULTY;

        let addr = if dynamic {
            "127.0.0.1:0".to_string()
        } else {
            WEBSOCKET_URI.to_string()
        };

        BlockchainConfig { addr, difficulty }
    }
}

/// Blockchain structure, consisting of vector of blocks and its configuration
impl Blockchain {
    /// Builds a blockchain from scratch
    /// Creates genesis block based on the BLOCKCHAIN_INITIAL_DIFFICULTY
    pub async fn build(config: BlockchainConfig) -> Result<Self, Box<dyn Error>> {
        let wallet = Arc::new(Mutex::new(Wallet::new(
            "MiningFeeWallet#1".to_string(),
            config.addr.to_string(),
        )));

        let config = Arc::new(config);
        let config_clone = Arc::clone(&config);

        // Websocket server for wallets to connect
        let listener = Arc::new(Mutex::new(BlockchainListener::new()));
        let listener_clone = Arc::clone(&listener);

        tokio::spawn(async move {
            listener_clone.lock().await.run(&config_clone.addr).await;
        });

        let mut wallet_mutex = wallet.lock().await;
        wallet_mutex.connect().await?;
        wallet_mutex.create_new_account();

        let coinbase_account: &Account;
        let coinbase_address: &Address;
        let coinbase_transaction: Transaction;

        // Scoped lock for mutex
        {
            let wallet_lock = wallet_mutex;

            coinbase_account = wallet_lock
                .accounts()
                .first()
                .expect("No coinbase error available.");

            coinbase_address = coinbase_account.address();

            coinbase_transaction = TransactionManager::create_coinbase_transaction(
                wallet_lock.private_key(),
                wallet_lock.public_key(),
                coinbase_address.id.as_str(),
                BLOCKCHAIN_COINBASE_GENESIS_BLOCK_FEE,
                coinbase_account.next_nonce(),
            );
        }

        let genesis_block = Block::create_genesis_block(coinbase_transaction);

        let blocks = vec![genesis_block.clone()]; // Clone it because it has to be borrowed to reward_block_finder
        let mempool = vec![];
        let utxo = HashMap::new();
        let ledger = vec![];

        let mut blockchain = Self {
            blocks,
            config,
            mempool,
            utxo,
            ledger,
            wallet,
            listener,
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
    pub async fn add_block(&mut self) {
        let last_block_header = &self.blocks.last().unwrap().header;

        let coinbase_account: &Account;
        let coinbase_address: &Address;
        let coinbase_transaction: Transaction;

        {
            let wallet_lock = self.wallet.lock().await;
            coinbase_account = wallet_lock
                .accounts()
                .first()
                .expect("No coinbase error available.");
            coinbase_address = coinbase_account.address();
            coinbase_transaction = TransactionManager::create_coinbase_transaction(
                wallet_lock.private_key(),
                wallet_lock.public_key(),
                coinbase_address.id.as_str(),
                BLOCKCHAIN_COINBASE_BLOCK_FEE,
                coinbase_account.next_nonce(),
            );
        }

        // Get all transactions for the block
        let transactions = vec![coinbase_transaction];
        let new_block = Block::new(
            &last_block_header.current_hash,
            &transactions,
            last_block_header.difficulty,
        );

        self.reward_block_finder(&new_block);
        self.push_new_block(new_block);
    }

    /// Validates a single block by checking several factors
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_single_block(&mut self, hash: &String) -> Result<(), BlockValidationError> {
        let block = self
            .blocks
            .iter()
            .find(|b| b.header.current_hash == *hash)
            .ok_or(BlockValidationError::BlockNotFound)?;

        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks);
        }

        if !HashHelper::is_valid_hash(block) {
            println!("Block hash: {}", block.header.current_hash);
            return Err(BlockValidationError::InvalidHash);
        }

        let prev_block = self
            .blocks
            .iter()
            .find(|b| b.header.current_hash == block.header.previous_hash)
            .ok_or(BlockValidationError::PreviousBlockNotFound)?;

        if prev_block.header.current_hash != block.header.previous_hash {
            return Err(BlockValidationError::PreviousHashMismatch);
        }

        if block.header.timestamp <= prev_block.header.timestamp {
            return Err(BlockValidationError::InvalidTimestamp);
        }

        Ok(())
    }

    /// Validates the full chain by looping through every block
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_full_chain(&mut self) -> Result<(), BlockValidationError> {
        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks);
        }

        for (idx, block) in self.blocks.iter().enumerate() {
            if !HashHelper::is_valid_hash(block) {
                println!(
                    "Failed on idx: {}, block hash: {}",
                    idx, block.header.current_hash
                );
                return Err(BlockValidationError::InvalidHash);
            }

            if idx != 0 {
                let prev_block = self
                    .blocks
                    .iter()
                    .find(|b| b.header.current_hash == block.header.previous_hash)
                    .ok_or(BlockValidationError::PreviousBlockNotFound)?;

                if prev_block.header.current_hash != block.header.previous_hash {
                    return Err(BlockValidationError::PreviousHashMismatch);
                }

                if block.header.timestamp <= prev_block.header.timestamp {
                    return Err(BlockValidationError::InvalidTimestamp);
                }
            }
        }
        Ok(())
    }

    /// Validates a range between `from_hash` and `to_hash`
    /// Returns a Result<(), BlockValidationError>
    pub fn validate_range_chain(
        &mut self,
        from_hash: &str,
        to_hash: &str,
    ) -> Result<(), BlockValidationError> {
        let (from_index, to_index) = match self.find_hash_indices(from_hash, to_hash) {
            none => return Err(BlockValidationError::RangeIndexFault),
            Some((from_index, to_index)) => (from_index, to_index),
        };

        if self.blocks.len() <= 1 {
            return Err(BlockValidationError::InsufficientBlocks);
        }

        for idx in from_index..=to_index {
            let block = self
                .blocks
                .get(idx)
                .ok_or(BlockValidationError::BlockNotFound)?;

            if !HashHelper::is_valid_hash(block) {
                println!(
                    "Failed on idx: {}, block hash: {}",
                    idx, block.header.current_hash
                );
                return Err(BlockValidationError::InvalidHash);
            }

            if idx != 0 {
                let prev_block = self
                    .blocks
                    .iter()
                    .find(|b| b.header.current_hash == block.header.previous_hash)
                    .ok_or(BlockValidationError::PreviousBlockNotFound)?;

                if prev_block.header.current_hash != block.header.previous_hash {
                    return Err(BlockValidationError::PreviousHashMismatch);
                }

                if block.header.timestamp <= prev_block.header.timestamp {
                    return Err(BlockValidationError::InvalidTimestamp);
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
        let coinbase_transaction = block
            .body()
            .transactions()
            .first()
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    "No (coinbase) Transactions in the Transactions vector of a Block."
                        .red()
                        .to_string()
                )
            })
            .clone();

        let coinbase_transaction_output = coinbase_transaction
            .outputs()
            .first()
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    "No (coinbase) TransactionOutput in the TransactionOutputs vector."
                        .red()
                        .to_string()
                )
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
    fn update_utxo_with_transaction(
        &mut self,
        address: String,
        transaction_output: TransactionOutput,
    ) {
        self.utxo
            .entry(address)
            .or_default()
            .push(transaction_output);
    }

    /// Push newly mined block to the blocks vector of a blockchain
    fn push_new_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub async fn shutdown(&mut self) {
        //self.listener.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn build_blockchain() -> Blockchain {
        let config = BlockchainConfig::new(true);

        match Blockchain::build(config).await {
            Ok(node) => node,
            Err(e) => {
                panic!("Failed to build blockchain: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn it_builds_a_blockchain() {
        let node = build_blockchain().await;
    }

    #[tokio::test]
    async fn it_creates_blockchain_blocks() {
        let mut node = build_blockchain().await;

        for _ in 1..=3 {
            node.add_block().await;
        }

        assert_eq!(node.blocks.len(), 3);

        node.shutdown().await
    }

    #[tokio::test]
    async fn it_validates_single_blockchain_block() {
        let mut node = build_blockchain().await;

        node.add_block().await;

        let blocks = node.blocks().clone();
        let block_1 = blocks.get(0).unwrap().clone();

        let validation = node.validate_single_block(block_1.header().current_hash());

        assert!(validation.is_ok());

        node.shutdown().await
    }

    #[tokio::test]
    async fn it_validates_blockchain_range() {
        let mut node = build_blockchain().await;

        for _ in 1..=5 {
            node.add_block().await;
        }

        let blocks = node.blocks().clone();
        let block_1 = blocks.get(1).unwrap().clone();
        let block_2 = blocks.get(3).unwrap().clone();

        let validation = node.validate_range_chain(
            block_1.header().current_hash(),
            block_2.header().current_hash(),
        );

        assert!(validation.is_ok());

        node.shutdown().await
    }

    #[tokio::test]
    async fn it_validates_full_blockchain() {
        let mut node = build_blockchain().await;

        for _ in 1..=5 {
            node.add_block().await;
        }

        let validation = node.validate_full_chain();

        assert!(validation.is_ok());

        node.shutdown().await
    }
}
