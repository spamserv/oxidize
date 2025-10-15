use futures_util::{SinkExt, StreamExt};
use std::{error::Error, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;

use crate::websockets::{SubscriptionMessage, WebSocketClient};

#[derive(Debug, Clone)]
pub struct WalletClient {
    address: String,
    ws: Arc<Mutex<Option<WebSocketClient>>>,
}

impl WalletClient {
    pub fn new(address: String) -> WalletClient {
        Self {
            address,
            ws: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let ws = WebSocketClient::new(self.address.to_string()).await?;
        self.ws = Arc::new(Mutex::new(Some(ws)));
        Ok(())
    }

    pub async fn send_message(&mut self, message: SubscriptionMessage) -> Result<(), Box<dyn Error>> {

        let ws = self
            .ws
            .as_ref()
            .lock()
            .await
            .clone()
            .ok_or("Websocket not connected")?;

        let serialized = serde_json::to_string(&message)?;
        let mut ws_stream = ws.ws_stream.lock().await;
        // Send the message
        ws_stream.send(Message::Text(serialized)).await?;

        Ok(())
    }

    pub async fn start_receiving<F>(&mut self, receiver_handler: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(String) -> () + Send + Sync + 'static + Clone,
    {
        // Ensure WebSocket is connected
        let ws = self
            .ws
            .as_ref()
            .lock()
            .await
            .clone()
            .ok_or("Websocket not connected")?;

        // Create a channel to receive messages
        let (tx, mut rx) = mpsc::channel(1);

        // Clone the handler for the spawned task
        //let handler_clone = receiver_handler.clone();

        // Spawn a task to receive and process messages
        tokio::spawn(async move {
            let mut ws_stream = ws.ws_stream.lock().await;

            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        // Send the message to the channel
                        if tx.send(text).await.is_err() {
                            eprintln!("Failed to send message to channel");
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {} // Handle other message types as needed
                }
            }
        });

        // Spawn a task to process received messages using the handler
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                receiver_handler(message);
            }
        });

        Ok(())
    }
}
