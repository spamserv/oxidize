use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};


pub struct WebSocketServer {
    clients: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run<F, Fut>(&self, addr: &str, on_client_message: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind(addr).await?;
        println!("[Server] Listening on {}", addr);

        let on_client_message = Arc::new(on_client_message);

        // Main server loop to accept connections
        while let Ok((stream, addr)) = listener.accept().await {
            println!("[Server] New connection from: {}", addr);
            let ws_stream = accept_async(stream).await?;
            let (mut write, mut read) = ws_stream.split();

            // Create a channel for sending messages to the client
            let (tx, mut rx) = mpsc::channel(10);
            self.clients.lock().unwrap().push(tx);

            // Spawn a task to handle incoming messages from the client
            let on_client_message = Arc::clone(&on_client_message);
            tokio::spawn({
                let on_client_message = Arc::clone(&on_client_message);
                async move {
                    while let Some(Ok(Message::Text(msg))) = read.next().await {
                        println!("[Server] Received from {}: {}", addr, msg);
                        (on_client_message)(msg).await;
                    }
                }
            });

            // Spawn a task to handle outgoing messages to the client
            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if write.send(Message::Text(msg)).await.is_err() {
                        break;
                    }
                }
            });
        }

        Ok(())
    }

    /// Broadcasts a message to all connected clients
    pub async fn broadcast(&self, message: String) {
        let mut clients = self.clients.lock().expect("Failed to lock clients list");
        clients.retain(|tx| {
            match tx.try_send(message.clone()) {
                Ok(_) => true,  // Keep the client if sending succeeds
                Err(_) => {
                    println!("[Server] Removing a client due to failed message sending.");
                    false // Remove the client if sending fails
                }
            }
        });
    }
}

pub struct BlockchainListener {}

impl BlockchainListener {
    pub async fn run(address: String) {

        let server = WebSocketServer::new();

        // Run the WebSocket server as a separate task
        tokio::spawn(async move {
            if let Err(e) = server.run(&address, Self::on_client_message).await {
                eprintln!("Error running server: {}", e);
            }
        });

        // Continue with other tasks that need to run concurrently
        println!("Websocket initialized...");

        // Keep the server running indefinitely
        // tokio::signal::ctrl_c()
        //     .await
        //     .expect("Failed to listen for ctrl-c signal");
        
    }

    async fn on_client_message(msg: String) {
        println!("Handling message: {}", msg);
        // Process the message (e.g., log, respond, etc.)
    }
}
