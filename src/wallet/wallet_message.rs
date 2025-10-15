use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    TransactionSend { transaction: Transaction }, // Sends a transaction from client (Wallet) to server (Node)
    TransactionHistory { transactions: Vec<Transaction> }, // Sends transaction history from server (Node) to client (Wallet)
    Balance { balance: u64 }, // Sends balance from server (Node) to client (Wallet)
    
}

pub trait WalletMessagePayload: Serialize {}

impl WalletMessagePayload for MessagePayload {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletMessageDirection {
    ServerToClient,
    ClientToServer,
}

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

impl<T> WalletMessage<T>
where
    T: WalletMessagePayload,
{
    pub fn new(
        request_id: String,
        account_id: String,
        direction: WalletMessageDirection,
        payload: T,
    ) -> Self {
        WalletMessage {
            request_id,
            account_id,
            direction,
            payload,
        }
    }
}
