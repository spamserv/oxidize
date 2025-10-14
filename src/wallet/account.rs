use chrono::Utc;
use hdwallet::secp256k1::PublicKey;
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

use super::Address;

/// Account struct, used to store transaction history, address.
#[derive(Debug, Clone, Default)]
pub struct Account {
    address: Address,
    balance: u64,
    created_at: String,
    transaction_history: Vec<Transaction>,
}

impl Account {
    /// Creates new account
    pub fn new(public_key: &PublicKey) -> Self {
        let created_at = Utc::now().to_rfc3339();
        //let address = Self::generate_address(&public_key);
        let address = Address {
            id: Self::generate_address(public_key),
            transactions: vec![],
        };
        let transaction_history = vec![];

        Self {
            created_at,
            address,
            transaction_history,
            balance: 0
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

    pub fn next_nonce(&self) -> u64 {
        self.transaction_history.len() as u64
    }

    pub fn get_balance() -> u64 {
        todo!()
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}
