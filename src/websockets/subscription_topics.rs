use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SubscriptionTopic {
    Transactions,
    WalletBalance,
    BlockchainStatus,
}

impl fmt::Display for SubscriptionTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubscriptionTopic::Transactions => write!(f, "transactions"),
            SubscriptionTopic::WalletBalance => write!(f, "wallet_balance"),
            SubscriptionTopic::BlockchainStatus => write!(f, "blockchain_status"),
        }
    }
}