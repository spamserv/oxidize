//! # Account
//!
//! Represents a wallet account, including its address, balance, and transaction history.
//!

use chrono::Utc;
use hdwallet::secp256k1::PublicKey;
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

/// Stores account data including address, balance, and transaction history.
#[derive(Debug, Clone, Default)]
pub struct Account {
    address: String,
    name: String,
    balance: u64,
    created_at: String,
    transaction_history: Vec<Transaction>, // local mempool
}

impl Account {
    /// Creates a new account from a public key and name.
    pub fn new(public_key: &PublicKey, name: &str) -> Self {
        let created_at = Utc::now().to_rfc3339();
        let address = Self::generate_address(public_key);
        let transaction_history = vec![];
        let name = String::from(name);

        Self {
            address,
            name,
            created_at,
            balance: 0,
            transaction_history,
        }
    }

    /// Generate Account address based on the public key
    fn generate_address(public_key: &PublicKey) -> String {
        let combined_string = format!("{}", public_key);
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let hash_result = hasher.finalize();
        let hash_result = format!("{:x}", hash_result);
        hash_result
    }

    /// Returns the next nonce based on transaction history length.
    pub fn next_nonce(&self) -> u64 {
        self.transaction_history.len() as u64
    }

    pub fn get_balance() -> u64 {
        todo!()
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn transaction_history(&self) -> &Vec<Transaction> {
        &self.transaction_history
    }

    pub fn address(&self) -> &String {
        &self.address
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }
}
