use std::{error::Error};

use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::comms;

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    sender: mpsc::Sender<String>,
}

impl WebSocketClient {
    pub async fn connect<F>(address: String, receiver_handler: F) -> Result<Self, Box<dyn Error>>
    where
        F: Fn(String) -> () + Send + Sync + 'static + Clone,
    {
        let url_string = format!("ws://{address}");

        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(url_string).await?;
        println!("{}", "[Client] Connected to the server".blue());

        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = mpsc::channel::<String>(32);

        // Spawn a task to process received messages using the handler
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                if let Ok(Message::Text(message)) = message {
                    receiver_handler(message);
                }
            }
        });

        // Writer
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = write.send(Message::Text(message)).await {
                    eprintln!("Send error: {:?}", e);
                    break;
                }
            }
        });

        Ok(Self { sender: tx })
    }

    pub async fn send_message<T: serde::Serialize>(
        &mut self,
        message: comms::Message<T>,
    ) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&message)?;
        let _ = self.sender.send(serialized).await?;
        Ok(())
    }
}
