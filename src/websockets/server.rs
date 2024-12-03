use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug, Clone)]
pub struct WebSocketServer {
    clients: Arc<Mutex<HashMap<usize, tokio::sync::mpsc::UnboundedSender<Message>>>>,
    broadcaster: broadcast::Sender<String>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        let (broadcaster, _) = broadcast::channel(100);
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            broadcaster,
        }
    }

    pub async fn run<F>(&self, addr: &str, handle_message: F)
    where
        F: Fn(
                String,
                usize,
                Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>,
            ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>
            + Send
            + Sync
            + 'static,
    {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let mut id_counter = 0;

        // Wrap the closure in an Arc for safe sharing.
        let handle_message = Arc::new(handle_message);

        while let Ok((stream, _)) = listener.accept().await {
            let ws_stream = accept_async(stream).await.unwrap();
            let (mut ws_tx, mut ws_rx) = ws_stream.split();

            let (tx, mut rx) = mpsc::unbounded_channel();
            let client_id = id_counter;
            id_counter += 1;

            self.clients.lock().await.insert(client_id, tx.clone());

            let clients = Arc::clone(&self.clients);
            let handle_message_clone = Arc::clone(&handle_message);

            // Spawn a task for handling WebSocket messages
            tokio::spawn(async move {
                while let Some(msg) = ws_rx.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        println!("New message: {:?}", text);
                        // Execute the provided closure to handle the message
                        Box::pin(handle_message_clone(text, client_id, clients.clone())).await;
                    }
                }
                clients.lock().await.remove(&client_id);
            });

            // Spawn a task for sending messages to this client
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    let _ = ws_tx.send(msg).await;
                }
            });
        }
    }
    

    pub async fn send(&self, client_id: usize, message: String) {
        if let Some(client) = self.clients.lock().await.get(&client_id) {
            let _ = client.send(Message::Text(message));
        }
    }

    pub async fn broadcast(&self, message: String) {
        for (_, client) in self.clients.lock().await.iter() {
            let _ = client.send(Message::Text(message.clone()));
        }
    }

    pub async fn shutdown(&mut self) {

    }
}
