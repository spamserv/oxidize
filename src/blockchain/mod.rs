//! This module handles blockchain-related functionality, including:
//! - Wallet management
//! - Transaction processing
//! - Blockchain structure and operations
//!
//! Modules:
//! - [`wallet`]: For wallet creation and management
//! - [`transaction_manager`]: For managing blockchain transactions
//! - [`blockchain`]: For blockchain data structure and logic

mod blockchain;
mod blockchain_listener;
mod transaction_manager;

pub use blockchain::*;
pub use blockchain_listener::*;
pub use transaction_manager::*;