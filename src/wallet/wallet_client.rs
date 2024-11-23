use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub struct WalletWsClient {
    receiver_handle: JoinHandle<()>,
    sender_handle: JoinHandle<()>,
}

impl WalletWsClient {
    pub async fn new(address: String) -> Self {
        println!("Connecting to {}", &address);

        let (mut ws_stream, _) = connect_async(&address).await.expect("Failed to connect");
        println!("Connected to {}", &address);

        // Wrap ws_stream in an Arc<Mutex> to allow shared mutable access
        let ws_stream = Arc::new(Mutex::new(ws_stream));

        // Create an mpsc channel for sending messages to the WebSocket task
        let (tx, mut rx) = mpsc::channel::<String>(10);

        let ws_stream_clone = ws_stream.clone();

        // Spawn a task to listen for incoming WebSocket messages
        let receiver_handle = tokio::spawn(async move {
            loop {
                let mut ws_stream = ws_stream_clone.lock().await;
                match ws_stream.next().await {
                    Some(Ok(Message::Text(text))) => {
                        println!("[WebSocket] Received: {}", text);
                    }
                    Some(Ok(_)) => {} // Ignore other message types
                    Some(Err(e)) => {
                        eprintln!("[WebSocket] Error: {}", e);
                        break;
                    }
                    None => {
                        println!("[WebSocket] Connection closed.");
                        break;
                    }
                }
            }
        });

        // Spawn a task to send messages from Wallet to the WebSocket server
        let ws_stream_clone = ws_stream.clone();
        let sender_handle = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let mut ws_stream = ws_stream_clone.lock().await;

                if let Err(e) = ws_stream.send(Message::Text(message)).await {
                    eprintln!("[WebSocket] Failed to send message: {}", e);
                }
            }
        });

        Self {
            receiver_handle,
            sender_handle,
        }
    }
}
