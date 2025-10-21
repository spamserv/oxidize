//! # Wallet Module
//!
//! Provides wallet-related functionality, including account management, 
//! wallet creation, and communication with the blockchain network.
//!
//! ## Exports
//! - [`Wallet`]: Core wallet struct for managing accounts and transactions.
//! - [`Account`]: Individual account structure with balance, address, and transaction history.
//! - [`WalletClient`]: WebSocket client to interact with blockchain nodes.
//! 

mod wallet;
mod wallet_client;
mod account;

pub use wallet::Wallet;
pub use account::Account;
pub use wallet_client::WalletClient;