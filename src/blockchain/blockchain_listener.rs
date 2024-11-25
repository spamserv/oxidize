use tokio::sync::oneshot;
use crate::websockets::WebSocketServer;

#[derive(Debug)]
pub struct BlockchainListener {
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
}

impl BlockchainListener {
    pub fn new() -> Self {
        Self {
            shutdown_tx: None.into(),
            server_handle: None.into(),
        }
    }

    pub fn run(address: String) -> Self {
        let server = WebSocketServer::new();
        
        // Create shutdown channel
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        // Run the WebSocket server as a separate task
        let server_handle = tokio::spawn(async move {
            if let Err(e) = server.run(&address, Self::on_client_message, shutdown_rx).await {
                eprintln!("Error running server: {}", e);
            }
        });

        println!("Websocket initialized...");

        Self {
            shutdown_tx: Some(shutdown_tx).into(),
            server_handle: Some(server_handle).into(),
        }
    }

    pub async fn shutdown(&mut self) {
        // Send shutdown signal
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        // Wait for server task to complete
        if let Some(handle) = self.server_handle.take() {
            let _ = handle.await;
        }
    }

    async fn on_client_message(msg: String) {
        // Process the message
    }
}
