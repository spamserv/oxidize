//! # WalletClient
//!
//! Provides a WebSocket client for interacting with the blockchain network,
//! sending transactions, and receiving events.
//! 

use std::error::Error;

use crate::{comms, websockets::WebSocketClient};

#[derive(Debug, Clone)]
pub struct WalletClient {
    address: String,
    ws: WebSocketClient,
}

impl WalletClient {
    /// Connects to a blockchain node via WebSocket.
    pub async fn connect<F>(
        address: String,
        receiver_handler: F,
    ) -> Result<WalletClient, Box<dyn Error>>
    where
        F: Fn(String) -> () + Send + Sync + 'static + Clone,
    {
        let ws = WebSocketClient::connect(address.to_string(), receiver_handler).await?;
        let wc = WalletClient { address, ws };

        Ok(wc)
    }

    /// Sends a custom message to the blockchain network.
    pub async fn send_message<T: serde::Serialize>(
        &mut self,
        message: comms::Message<T>,
    ) -> Result<(), Box<dyn Error>> {

        self.ws.send_message(message).await?;

        Ok(())
    }

    /// Sends a ping message to the blockchain node.
    pub async fn ping(&mut self) -> Result<(), Box<dyn Error>> {
        let message = comms::Message::Event {
            id: uuid::Uuid::new_v4().to_string(),
            topic: comms::EventTopic::BlockchainPing,
            data: (),
        };

        self.ws.send_message(message).await?;

        Ok(())
    }
}
