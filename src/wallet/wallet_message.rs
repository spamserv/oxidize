pub enum WalletMessageType {
    Balance,
    TransactionHistory,
    TransactionSend,
}

pub enum WalletMessageDirection {
    ServerToClient,
    ClientToServer,
}

impl WalletMessageType {
    fn as_str(&self) -> &str {
        match &self {
            WalletMessageType::Balance => "get_balance",
            WalletMessageType::TransactionHistory => "get_transaction_history",
            WalletMessageType::TransactionSend => "send_transaction"
        }
    }
}

pub struct WebsocketMessage {
    message_type: WalletMessageType,
    direction: WalletMessageDirection,

}