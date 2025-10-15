use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SubscriptionTopic {
    Transactions,
    InitiateTransaction,
    WalletBalance,
    BlockchainStatus,
}

impl fmt::Display for SubscriptionTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionTopic::Transactions => write!(f, "transactions"),
            SubscriptionTopic::InitiateTransaction => write!(f, "initiate_transaction"),
            SubscriptionTopic::WalletBalance => write!(f, "wallet_balance"),
            SubscriptionTopic::BlockchainStatus => write!(f, "blockchain_status"),
        }
    }
}
