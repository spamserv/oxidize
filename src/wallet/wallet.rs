//! This module handles:
//! - wallet creation
//! - account creation
//! - address generation and validation

use std::error::Error;

use bip39::{Language, Mnemonic};
use chrono::Utc;
use hdwallet::{
    secp256k1::{PublicKey, SecretKey},
    ExtendedPrivKey, ExtendedPubKey,
};

use crate::websockets::{SubscriptionMessage, SubscriptionTopic};

use super::{Account, WalletClient, WalletMessage};

/// Wallet struct, used for storing accounts and key pair
#[derive(Debug, Clone)]
pub struct Wallet {
    pub id: String, // Derived from public key
    pub name: String,
    pub created_at: String,
    pub accounts: Vec<Account>,
    public_key: PublicKey,
    private_key: SecretKey,
    ws: WalletClient, // Used for testing, idea is not to store it in the future
}

impl Wallet {
    /// Creates new wallet & generates key pair
    pub fn new(name: String, ws_uri: String) -> Self {
        let created_at = Utc::now().to_rfc3339();
        let accounts = vec![];
        let (public_key, private_key) = Wallet::generate_key_pair().unwrap();
        let id = "".to_string();
        let ws = WalletClient::new(ws_uri.to_string());
        // Lets wait for the full blockchain init before sending messages.
        // ws.send_message(NodeMessageType::Balance { balance: 24 }).await?;

        Self {
            id,
            public_key,
            private_key,
            created_at,
            accounts,
            name,
            ws,
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.ws.connect().await?;

        // Subscribe to WalletBalance changes
        let message = SubscriptionMessage {
            action: "subscribe".to_string(),
            topic: SubscriptionTopic::WalletBalance
        };

        self.ws.send_message(message).await?;

        self.ws.start_receiving(|message| {
            println!("Received message: {}", message);
        }).await?;
        
        Ok(())
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
        let seed = mnemonic.to_seed(""); // Create the seed from the mnemonic

        // Attempt to create an ExtendedPrivKey from the seed
        let master_key = ExtendedPrivKey::with_seed(&seed);

        // Check if the master key creation succeeded
        if let Ok(master_key) = master_key {
            // Derive child private and public keys if master key is created successfully
            let child_priv_key = master_key
                .derive_private_key(hdwallet::KeyIndex::Normal(0))
                .unwrap();
            let child_pub_key = ExtendedPubKey::from_private_key(&child_priv_key);

            // Return the derived public key
            return Ok((child_pub_key.public_key, master_key.private_key));
        }

        // If the master key creation failed, return an error
        Err("Cannot create master key from seed!".to_string())
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
}
