use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "r#type", rename_all = "snake_case")]
pub enum Message<T> {
    Request {
        id: String,
        r#type: RequestType,
        payload: T,
    },
    Response {
        id: String,
        status: String,
        data: Option<T>,
        error: Option<String>,
    },
    Event {
        id: String,
        topic: EventTopic,
        data: T,
    },
}

pub enum Direction {
    ClientToServer,
    ServerToClient,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum EventTopic {
    BlockchainStatus,
    BlockchainPing,
    NewBlock,
    TxConfirmed,
    MempoolTxAdded,
    MempoolTxRemoved
}

impl fmt::Display for EventTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventTopic::BlockchainStatus => write!(f, "blockchain_status"),
            EventTopic::BlockchainPing => write!(f, "blockchain_ping"),
            EventTopic::NewBlock => write!(f, "new_block"), 
            EventTopic::TxConfirmed => write!(f, "tx_confirmed"),
            EventTopic::MempoolTxAdded => write!(f, "mempool_tx_added"),
            EventTopic::MempoolTxRemoved => write!(f, "mempool_tx_removed"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestType {
    GetBalance,
    SubmitTransaction,
    GetMempool
}
   
