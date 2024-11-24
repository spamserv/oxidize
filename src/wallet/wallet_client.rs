use std::error::Error;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::Message;

use crate::websockets::WebSocketClient;
use super::wallet_message::WalletMessagePayload;

#[derive(Debug, Clone)]
pub struct WalletClient {
    ws: WebSocketClient,
}

impl WalletClient {
    pub async fn new(address: String) -> Result<Self, Box<dyn Error>> {
        let ws = WebSocketClient::new(address).await?;
        Ok(Self { ws })
    }

    pub async fn send_message<T: WalletMessagePayload>(&self, message: T) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&message)?;
        let mut ws_stream = self.ws.ws_stream.lock().await;
        // Send the message
        ws_stream.send(Message::Text(serialized)).await?;
        Ok(())
    }
}
