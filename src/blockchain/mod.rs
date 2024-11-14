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
mod address;
mod wallet;
mod transaction_manager;

pub use blockchain::*;
pub use address::*;
pub use wallet::*;
pub use transaction_manager::*;