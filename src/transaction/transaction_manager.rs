//! This module handles:
//! - creating and signing the transaction
//! - calculating transaction fee
//! - building the transaction from scratch (inputs, outputs, validation)
//!

use bincode::{Decode, Encode};
use chrono::Utc;
use hdwallet::secp256k1::{PublicKey, SecretKey};
use serde::{Deserialize, Serialize};

use crate::utils::TransactionHelper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    metadata: TransactionMetadata,
}

impl Transaction {
    pub fn outputs(&self) -> &Vec<TransactionOutput> {
        &self.outputs
    }

    pub fn inputs(&self) -> &Vec<TransactionInput> {
        &self.inputs
    }

    pub fn metadata(&self) -> &TransactionMetadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInput {
    pub previous_tx_hash: [u8; 32], // Hash of the previous transaction
    pub index: u32,               // Index of the output being used
    pub signature: String,
    #[serde(default="default_public_key", skip_serializing, skip_deserializing)]
    pub public_key: PublicKey,        // Signature for authorization
    pub amount: u64,
    pub nonce: u64,
}

fn default_public_key() -> PublicKey {
    // Replace this with a real default if needed
    PublicKey::from_slice(&[0u8; 33]).unwrap()
}

impl bincode::Encode for TransactionInput {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.signature, encoder)?;
        bincode::Encode::encode(&self.previous_tx_hash, encoder)?;
        bincode::Encode::encode(&self.amount, encoder)?;
        bincode::Encode::encode(&self.nonce, encoder)?;
        bincode::Encode::encode(&self.index, encoder)?;

        Ok(())
    }
}

impl bincode::Encode for TransactionOutput {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.recipient_address, encoder)?;
        bincode::Encode::encode(&self.amount, encoder)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionOutput {
    pub recipient_address: String, // The address of the recipient
    pub amount: u64,               // The amount of currency being sent
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TransactionMetadata {
    pub transaction_hash: [u8; 32],
    timestamp: String,
    status: TransactionStatus,
    r#type: TransactionType,
    signature: Vec<u8>, // Store signature as bytes for serialization
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub enum TransactionStatus {
    Pending,
    Mined,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Coinbase, // For mining a new block, does not get put into UTXO
    Fee,      // For transaction fees
    Regular,  // P2P
}

pub enum TransactionError {
    NotEnoughFunds,
}

pub struct TransactionManager {}

struct TransactionBuilder {}

struct Signer {}

impl TransactionManager {
    pub fn create_transaction(
        inputs: Vec<TransactionInput>,
        outputs: Vec<TransactionOutput>,
        private_key: SecretKey,
    ) -> Transaction {
        let timestamp = Utc::now().to_rfc3339();
        let status = TransactionStatus::Pending;

        // Creates transaction hash
        let transaction_hash =
            TransactionHelper::generate_transaction_hash(&inputs, &outputs, &timestamp, &status);
        let r#type = TransactionType::Coinbase;

        // Signs the transaction hash using Wallet private key
        let signature_obj = TransactionHelper::sign_transaction(&private_key, transaction_hash);
        let signature = signature_obj.serialize_compact().to_vec(); // Convert signature to Vec<u8>

        let metadata = TransactionMetadata {
            timestamp,
            status,
            transaction_hash,
            r#type,
            signature,
        };

        Transaction {
            inputs,
            outputs,
            metadata,
        }
    }

    pub fn create_coinbase_transaction(
        private_key: &SecretKey,
        public_key: &PublicKey,
        recipient_addr: &str,
        amount: u64,
        nonce: u64
    ) -> Transaction {
        let transaction_input = TransactionInput {
            previous_tx_hash: [0u8; 32],
            index: 0,
            signature: String::from("INITIAL_COINBASE_SIGNATURE"),
            public_key: public_key.clone(),
            amount: amount,
            nonce,
        };

        let inputs = vec![transaction_input];

        let transaction_output = TransactionOutput {
            amount,
            recipient_address: recipient_addr.to_string(),
        };
        let outputs = vec![transaction_output];

        let tx = TransactionManager::create_transaction(inputs, outputs, private_key.clone());

        tx
    }

    pub fn broadcast_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        // Broadcast the transaction to the blockchain network
        todo!()
    }

    pub fn calculate_fee(&self, transaction: &Transaction) -> u64 {
        // Calculate transaction fee
        todo!()
    }

    pub fn track_transaction(&self, tx_id: &str) -> Result<TransactionStatus, String> {
        // Track the status of the transaction
        todo!()
    }
}

impl TransactionBuilder {
    pub fn add_input(&mut self, tx_id: &str, index: u32, amount: u64) -> TransactionInput {
        // Add an input to the transaction
        todo!()
    }

    pub fn add_output(&mut self, recipient: &str, amount: u64) -> TransactionOutput {
        // Add an output to the transaction
        todo!()
    }

    pub fn create_change_output(&mut self, amount: u64) -> Result<(), String> {
        // Create a change output if the inputs exceed the transaction amount
        todo!()
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validate the transaction, ensuring inputs and outputs match
        todo!()
    }

    pub fn calculate_fee(&self) -> u64 {
        // Calculate the fee based on the size and inputs/outputs
        todo!()
    }
}
