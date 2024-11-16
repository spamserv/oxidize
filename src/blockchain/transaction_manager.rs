//! This module handles:
//! - creating and signing the transaction
//! - calculating transaction fee
//! - building the transaction from scratch (inputs, outputs, validation)
//! 

#[derive(Debug, Clone)]
pub struct Transaction {
    inputs: Vec<TransactionInput>,
    output: Vec<TransactionOutput>,
    metadata: TransactionMetadata,
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
    id: String,
    timestamp: String,
    value: String,
    state: TransactionStatus
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Mined,
    Rejected,
}

pub enum TransactionType {
    Coinbase, // For mining a new block, does not get put into UTXO
    Fee, // For transaction fees
    Regular // P2P
}

pub enum TransactionError {
    NotEnoughFunds,
}

struct TransactionManager {}

struct TransactionBuilder {}

struct Signer {
    
}

impl TransactionManager {
    pub fn create_transaction(&self, recipient: &str, amount: u64) -> Transaction {
        todo!()
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
        TransactionInput {
            previous_tx_hash: tx_id.to_string(),
            index,
            amount
        }
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
