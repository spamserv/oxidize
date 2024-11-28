use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletMessageType {
    TransactionSend { transactions: Transaction }, // Sends a new transaction to the server "node"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeMessageType {
    Balance { balance: u64 }, // Sends balance from server "node" to client
    TransactionHistory { transactions: Vec<Transaction> }, // Sends transaction history from server "node" to client
}

pub trait WalletMessagePayload: Serialize {}

impl WalletMessagePayload for WalletMessageType {}
impl WalletMessagePayload for NodeMessageType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletMessage<T>
where
    T: WalletMessagePayload,
{
    request_id: String,
    account_id: String,
    direction: WalletMessageDirection,
    pub payload: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletMessageDirection {
    ServerToClient,
    ClientToServer,
}
