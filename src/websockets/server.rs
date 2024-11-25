use std::sync::{Arc};

use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::{mpsc, oneshot, Mutex}};
use tokio_tungstenite::{accept_async, tungstenite::Message};

pub struct WebSocketServer {
    clients: Arc<Mutex<Vec<mpsc::Sender<String>>>>,
}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run<F, Fut>(
        &self, 
        addr: &str, 
        on_client_message: F,
        mut shutdown_rx: oneshot::Receiver<()>
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind(addr).await?;
        println!("[Server] Listening on {}", addr);

        let on_client_message = Arc::new(on_client_message);
        let clients = Arc::clone(&self.clients);

        tokio::select! {
            result = self.accept_connections(listener, on_client_message, clients) => {
                if let Err(e) = result {
                    eprintln!("Server error: {}", e);
                }
            }
            _ = &mut shutdown_rx => {
                println!("Received shutdown signal");
            }
        }

        Ok(())
    }

    async fn accept_connections<F, Fut>(
        &self,
        listener: TcpListener,
        on_client_message: Arc<F>,
        clients: Arc<Mutex<Vec<mpsc::Sender<String>>>>
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        loop {
            let (stream, addr) = listener.accept().await?;
            println!("{} {}", "[Server] New connection from: ".blue().bold(), addr.to_string().yellow());
            
            let ws_stream = accept_async(stream).await?;
            let (mut write, mut read) = ws_stream.split();

            // Create a channel for sending messages to the client
            let (tx, mut rx) = mpsc::channel(10);
            clients.lock().await.push(tx);

            // Spawn a task to handle incoming messages from the client
            let on_client_message = Arc::clone(&on_client_message);
            tokio::spawn(async move {
                while let Some(Ok(Message::Text(msg))) = read.next().await {
                    println!("{} {}: {}", "[Server] Received from: ".blue().bold(), addr.to_string().yellow(), msg.yellow());
                    (on_client_message)(msg).await;
                }
            });
        }
    }
}