use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WalletMessageType {
    TransactionSend {
        transactions: Transaction
    }, // Sends a new transaction to the server "node"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeMessageType {
    Balance {
        balance: u64
    }, // Sends balance from server "node" to client
    TransactionHistory {
        transactions: Vec<Transaction>
    }, // Sends transaction history from server "node" to client
}

pub trait WalletMessagePayload: Serialize {}

impl WalletMessagePayload for WalletMessageType {}
impl WalletMessagePayload for NodeMessageType {}

pub struct WalletMessage<T> {
    request_id: String,
    account_id: String,
    direction: WalletMessageDirection,
    pub payload: T,
}

#[derive(Debug, Clone)]
pub enum WalletMessageDirection {
    ServerToClient,
    ClientToServer,
}
