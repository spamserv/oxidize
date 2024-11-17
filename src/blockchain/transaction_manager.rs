//! This module handles:
//! - creating and signing the transaction
//! - calculating transaction fee
//! - building the transaction from scratch (inputs, outputs, validation)
//! 

use chrono::Utc;

use crate::{config::BLOCKCHAIN_COINBASE_FEE, helpers::TransactionHelper};

#[derive(Debug, Clone)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    metadata: TransactionMetadata,
}

impl Transaction {
    pub fn outputs(&self) -> &Vec<TransactionOutput> {
        &self.outputs
    }
}

#[derive(Debug, Clone)]
pub struct TransactionInput {
    pub previous_tx_hash: String,  // Hash of the previous transaction
    pub index: u32,                // Index of the output being used
    pub signature: String,         // Signature for authorization
    pub amount: u64
}

#[derive(Debug, Clone)]
pub struct TransactionOutput {
    pub recipient_address: String,  // The address of the recipient
    pub amount: u64,                // The amount of currency being sent
}

#[derive(Debug, Clone)]
struct TransactionMetadata {
    transaction_id: String,
    timestamp: String,
    status: TransactionStatus, 
    r#type: TransactionType
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Mined,
    Rejected,
}

#[derive(Debug, Clone)]
pub enum TransactionType {
    Coinbase, // For mining a new block, does not get put into UTXO
    Fee, // For transaction fees
    Regular // P2P
}

pub enum TransactionError {
    NotEnoughFunds,
}

pub struct TransactionManager {}

struct TransactionBuilder {}

struct Signer {}

impl TransactionManager {
    pub fn create_transaction(&self, recipient: &String, amount: u64) -> Transaction {
        todo!()
    }
    
    pub fn create_coinbase_transaction(recipient: &String, amount: u64) -> Transaction {
        let inputs = vec![];
        let transaction_output = TransactionOutput{
            amount: BLOCKCHAIN_COINBASE_FEE,
            recipient_address: recipient.to_string(),
        };
        let outputs = vec![transaction_output];

        let timestamp = Utc::now().to_rfc3339();
        let status = TransactionStatus::Pending;

        let transaction_id = TransactionHelper::generate_transaction_id(&inputs, &outputs, &timestamp, &status);
        let r#type = TransactionType::Coinbase;

        let metadata = TransactionMetadata {
            timestamp,
            status,
            transaction_id,
            r#type
        };

        Transaction {
            inputs,
            outputs,
            metadata
        }
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) -> Result<(), String> {
        // Sign the transaction with private key
        todo!()
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
    pub fn add_input(&mut self, tx_id: &String, index: u32, amount: u64) -> TransactionInput {
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
