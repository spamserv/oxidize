use hdwallet::secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use crate::transaction::{TransactionInput, TransactionOutput, TransactionStatus};
pub struct TransactionHelper {}

impl TransactionHelper {
    // Transaction Hash
    pub fn generate_transaction_hash(
        inputs: &Vec<TransactionInput>,
        outputs: &Vec<TransactionOutput>,
        timestamp: &String,
        status: &TransactionStatus,
    ) -> [u8; 32] {
        // 1. Serialize the transaction deterministically
        let tx_bytes = bincode::encode_to_vec(
            &(inputs, outputs, timestamp, status),
            bincode::config::standard(),
        )
        .expect("Serialization failed");

        // 2. Hash the bytes
        let mut hasher = Sha256::new();
        hasher.update(&tx_bytes);
        let result = hasher.finalize();

        // 3. Return raw bytes (32 bytes)
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        hash_bytes
    }

    /// Signs transaction using ECDSA with the given Wallet secret key
    pub fn sign_transaction(secret_key: &SecretKey, tx_hash: [u8; 32]) -> Signature {
        let secp = Secp256k1::new();
        let msg = Message::from_slice(&tx_hash).unwrap();
        secp.sign_ecdsa(&msg, secret_key)
    }

    /// Verifies the transaction signature using ECDSA with the given Wallet public key
    pub fn verify_signature(pubkey: &PublicKey, tx_hash: [u8; 32], signature: &Signature) -> bool {
        let secp = Secp256k1::new();
        let msg = Message::from_slice(&tx_hash).unwrap();
        secp.verify_ecdsa(&msg, signature, pubkey).is_ok()
    }
}
