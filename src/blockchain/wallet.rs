//! This module handles:
//! - wallet creation
//! - account creation
//! - address generation and validation

use std::ops::Add;

use hdwallet::{secp256k1::{PublicKey, SecretKey}, ExtendedPrivKey, ExtendedPubKey};
use sha2::{Digest, Sha256};
use chrono::Utc;
use bip39::{Mnemonic, Language};

use super::{Transaction, Address};

/// Wallet struct, used for storing accounts and key pair
#[derive(Debug, Clone)]
pub struct Wallet {
    id: String, // Derived from public key
    name: String,
    created_at: String,
    accounts: Vec<Account>,
    public_key: PublicKey,
    private_key: SecretKey // Used for testing, idea is not to store it in the future
}

/// Account struct, used to store transaction history, address.
#[derive(Debug, Clone)]
pub struct Account {
    address: Address,
    created_at: String,
    transaction_history: Vec<Transaction>
}

impl Wallet {
    /// Creates new wallet & generates key pair
    pub fn new(name: String) -> Self {
        let created_at = Utc::now().to_rfc3339();
        let accounts = vec![];
        let (public_key, private_key) = Wallet::generate_key_pair().unwrap();
        let id = "".to_string();
        
        Self {
            id,
            public_key,
            private_key,
            created_at,
            accounts,
            name,
        }
    }

    /// Create new account for the wallet, based on the `public_key`
    pub fn create_new_account(&mut self) {
        let account = Account::new(&self.public_key);
        self.accounts.push(account);
    }

    /// Generates keypair for the Wallet (PublicKey, SecretKey)
    /// Based on the mnemonic 24 word english
    fn generate_key_pair() -> Result<(PublicKey, SecretKey), String> {
        // Generate a mnemonic and seed
        let mut rng = bip39::rand::thread_rng();
        let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, 24).unwrap();
        let seed = mnemonic.to_seed("");  // Create the seed from the mnemonic
    
        // Attempt to create an ExtendedPrivKey from the seed
        let master_key = ExtendedPrivKey::with_seed(&seed);
    
        // Check if the master key creation succeeded
        if let Ok(master_key) = master_key {
            // Derive child private and public keys if master key is created successfully
            let child_priv_key = master_key.derive_private_key(hdwallet::KeyIndex::Normal(0)).unwrap();
            let child_pub_key = ExtendedPubKey::from_private_key(&child_priv_key);
    
            // Debug and print
            //dbg!("Master private key: ", master_key);
            //dbg!("Child public key: ", &child_pub_key);
            //println!("{}", mnemonic);
    
            // Return the derived public key
            return Ok((child_pub_key.public_key, master_key.private_key));
        }
    
        // If the master key creation failed, return an error
        Err("Cannot create master key from seed!".to_string())
    }
}

impl Account {
    /// Creates new account
    pub fn new(public_key: &PublicKey) -> Self {
        let created_at = Utc::now().to_rfc3339();
        //let address = Self::generate_address(&public_key);
        let address = Address {
            id: "TO_DO".to_string(),
            transactions: vec![],
        };
        let transaction_history = vec![];

        Self {
            created_at,
            address,
            transaction_history
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

    pub fn get_balance() -> u64 {
        todo!()
    }
}
