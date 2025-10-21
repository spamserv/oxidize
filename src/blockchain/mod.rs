//! # Blockchain Module
//!
//! This module encapsulates all core blockchain functionality for the Oxidize project.
//! It serves as the foundational layer of the system, responsible for creating, managing,
//! validating, and synchronizing the blockchain ledger, blocks, and transactions.
//!
//! ## Overview
//!
//! The blockchain implementation here follows a **proof-of-work**-inspired mechanism,
//! designed primarily for educational and research purposes. It demonstrates the
//! fundamental building blocks of a decentralized ledger without external dependencies
//! or complex consensus protocols.
//!
//! ## Core Responsibilities
//!
//! - **Block Management:**  
//!   Creation and validation of blocks through proof-of-work–style hashing.
//!
//! - **Transaction Handling:**  
//!   Integration with the [`transaction_manager`] module for adding, processing,
//!   and validating transactions before they are included in a block.
//!
//! - **Wallet Integration:**  
//!   Each node includes a wallet and can act as a miner, earning rewards through
//!   coinbase transactions for each mined block.
//!
//! - **Network Communication:**  
//!   The [`blockchain_listener`] module provides WebSocket-based connectivity for
//!   real-time blockchain event propagation (new blocks, transactions, etc.).
//!
//! ## Module Composition
//!
//! | Submodule | Description |
//! |------------|-------------|
//! | [`block`] | Defines the [`Block`], [`BlockHeader`], and [`BlockBody`] data structures, along with genesis and data block creation logic. |
//! | [`blockchain`] | Implements the [`Blockchain`] struct — the core chain management logic including block addition, validation, and reward assignment. |
//! | [`blockchain_listener`] | Provides asynchronous WebSocket-based event listening and broadcasting for blockchain-related messages. |
//!
//! ## Example
//!
//! ```rust,no_run
//! use oxidize::blockchain::{Blockchain, BlockchainConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Initialize blockchain with static configuration
//!     let config = BlockchainConfig::new(false);
//!     let mut blockchain = Blockchain::build(config).await.unwrap();
//!
//!     // Add a new block to the chain
//!     blockchain.add_block().await;
//!
//!     // Validate entire chain
//!     blockchain.validate_full_chain().unwrap();
//! }
//! ```
//!
//! ## Notes
//! - This implementation is **not production-ready** and is designed for
//!   learning, experimentation, and demonstration purposes.
//! - Networking and consensus layers are simplified to ensure code clarity.
//!
//! ## Future Improvements
//! - Peer-to-peer network synchronization  
//! - Dynamic difficulty adjustment  
//! - Persistent storage backend (RocksDB or SQLite)  
//! - Smart contract execution layer  
//!
//! ---
//! **Modules**
//!
//! - [`blockchain`]: Blockchain structure, validation, and lifecycle management  
//! - [`block`]: Block and block header definitions  
//! - [`blockchain_listener`]: Real-time blockchain event server  
//!
//! ---



mod blockchain;
mod blockchain_listener;
mod block;

pub use blockchain::*;
pub use blockchain_listener::*;
pub use block::*;