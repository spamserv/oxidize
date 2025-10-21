//! # Transaction
//!
//! Handles creation, signing, validation, and management of blockchain transactions.
//!
//! ```rust
//! use oxidize::transaction::TransactionManager;
//! // Example usage to create a coinbase transaction
//! ```
//!
//! ## Exports
//! - [`transaction_manager`]: Core transaction logic.
//! 

mod transaction_manager;
pub use transaction_manager::*;