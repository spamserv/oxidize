//! # Constants
//!
//! Global blockchain configuration values used throughout Oxidize.
//! These are static, deterministic, and compile-time fixed.
//! 

/// Initial proof-of-work difficulty.
pub const BLOCKCHAIN_INITIAL_DIFFICULTY: u8 = 2;

/// Starting nonce for mining.
pub const BLOCKCHAIN_INITIAL_NONCE: u64 = 0;

/// Total supply of Oxcoin.
pub const BLOCKCHAIN_OXCOIN_SUPPLY: u64 = 1_000_000;

/// Reward for mining a standard block.
pub const BLOCKCHAIN_COINBASE_BLOCK_FEE: u64 = 20;

/// Reward for mining the genesis block.
pub const BLOCKCHAIN_COINBASE_GENESIS_BLOCK_FEE: u64 = 1_000;

/// Transaction fee for standard transactions.
pub const BLOCKCHAIN_TRANSACTION_FEE: u8 = 1;

/// WebSocket URI for blockchain network communication.
pub const WEBSOCKET_URI: &str = "localhost:8080";

