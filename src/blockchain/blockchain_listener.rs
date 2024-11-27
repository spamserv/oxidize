use std::sync::Arc;

use crate::websockets::WebSocketServer;
use tokio::sync::{oneshot, Mutex};

#[derive(Debug, Clone)]
pub struct BlockchainListener {
    server: WebSocketServer,
    shutdown_tx: oneshot::Sender<()>,
    shutdown_rx: oneshot::Receiver<()>,
    server_handle: tokio::task::JoinHandle<()>
}

impl BlockchainListener {
    pub async fn run(address: String) -> Self {
        let server = WebSocketServer::new(&address).await;

        let mut server = match server {
            Ok(server) => server,
            Err(e) => panic!("Error running TcpClient"),
        };

        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        // Run the WebSocket server as a separate task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.clone().run().await {
                eprintln!("Error running server: {}", e);
            }
        });

        println!("Websocket initialized...");

        Self {
            server.clone(),
            shutdown_tx,
            shutdown_rx,
            server_handle,
        }
    }

    pub async fn shutdown(&self) {
        self.server.
    }
    

    async fn on_client_message(msg: String) {
        // Process the message from here if needed, will be passed back all the way up
    }
}
