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

use anyhow::{Result};

use crate::{
    comms::{EventTopic, Message, RequestType},
    transaction::{TransactionInput, TransactionManager, TransactionOutput},
    websockets::{SubscriptionMessage, SubscriptionTopic},
};

use super::{Account, WalletClient};

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
        // let message = SubscriptionMessage {
        //     action: "subscribe".to_string(),
        //     topic: SubscriptionTopic::WalletBalance,
        // };

        let message = Message::Event { id: uuid::Uuid::new_v4().to_string(), topic: EventTopic::BlockchainPing, data: () };

        self.ws.send_message(message).await?;

        self.ws
            .start_receiving(|message| {
                println!("Received message: {}", message);
            })
            .await?;

        Ok(())
    }

    /// Initiate payment by creating a transaction from the UTXOs and broadcasting it to the network
    /// Note: This function does not yet handle UTXO selection, fees, or change addresses
    pub async fn initiate_payment(
        &mut self,
        account_name: &str,
        recipient_addr: &str,
        amount: u64,
    ) -> Result<(), Box<dyn Error>> {
        let account = self.find_account(account_name)?;

        let previous_tx_hash = account
            .transaction_history()
            .last()
            .map(|tx| tx.metadata().transaction_hash)
            .unwrap_or([0u8; 32]);

        let tx_input = TransactionInput {
            previous_tx_hash, // Placeholder, should be set to actual previous transaction hash
            index: 0,         // Placeholder, should be set to actual index
            signature: String::from("PLACEHOLDER_SIGNATURE"), // Placeholder, should be set to actual signature
            public_key: self.public_key.clone(),
            amount,
            nonce: account.next_nonce(),
        };

        let tx_output = TransactionOutput {
            amount,
            recipient_address: recipient_addr.to_string(),
        };

        let tx = TransactionManager::create_transaction(
            vec![tx_input],
            vec![tx_output],
            self.private_key.clone(),
        );

        // TODO: Broadcast the transaction to the network
        println!("Created transaction: {:?}", tx);
        // let message = SubscriptionMessage {
        //     action: "subscribe".to_string(),
        //     topic: SubscriptionTopic::InitiateTransaction
        // };

        let message = Message::Request {
            id: uuid::Uuid::new_v4().to_string(),
            r#type: RequestType::SubmitTransaction,
            payload: tx,
        };

        self.ws.send_message(message).await?;

        Ok(())
    }

    pub fn find_account(&self, name: &str) -> Result<Account, String> {
        let result = self.accounts.iter().find(|acc| acc.name() == name);
        match result {
            Some(account) => Ok(account.clone()),
            _ => Err(format!("Account with name {} not found", name)),
        }
    }

    /// Create new account for the wallet, based on the `public_key`
    pub fn create_new_account(&mut self, name: &str) {
        let account = Account::new(&self.public_key, name);
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

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn private_key(&self) -> &SecretKey {
        &self.private_key
    }
}
