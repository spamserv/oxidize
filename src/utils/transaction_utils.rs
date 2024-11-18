use sha2::{Sha256, Digest};

use crate::transaction::{TransactionInput, TransactionOutput, TransactionStatus};

pub struct TransactionHelper {}

impl TransactionHelper {
    pub fn generate_transaction_id(inputs: &Vec<TransactionInput>, outputs: &Vec<TransactionOutput>, timestamp: &String, status: &TransactionStatus) -> String {
        let combined_string = format!("{:?}{:?}{}{:?}", inputs, outputs, timestamp, status);
        let mut hasher = Sha256::new();
        hasher.update(combined_string);
        let transaction_id = hasher.finalize();
        let transaction_id = format!("{:x}", transaction_id);
        transaction_id
    }
}