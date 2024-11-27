use futures_util::SinkExt;
use std::error::Error;
use tokio_tungstenite::tungstenite::Message;

use super::wallet_message::WalletMessagePayload;
use crate::websockets::WebSocketClient;

#[derive(Debug, Clone)]
pub struct WalletClient {
    address: String,
    ws: Option<WebSocketClient>,
}

impl WalletClient {
    pub fn new(address: String) -> WalletClient {
        Self { address, ws: None }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let ws = WebSocketClient::new(self.address.to_string()).await?;
        self.ws = Some(ws);
        Ok(())
    }

    pub async fn send_message<T: WalletMessagePayload>(
        &mut self,
        message: T,
    ) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&message)?;
        let mut ws_stream = self.ws.as_ref().unwrap().ws_stream.lock().await;
        // Send the message
        ws_stream.send(Message::Text(serialized)).await?;

        Ok(())
    }
}
