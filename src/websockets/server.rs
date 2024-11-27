use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex as AsyncMutex};
use tokio::task::JoinHandle;

// WebSocket Server - Responsible only for creating and managing the listener
struct WebSocketServer {
    shutdown_tx: mpsc::Sender<()>,
    shutdown_rx: Arc<AsyncMutex<mpsc::Receiver<()>>>,
    server_handle: Option<JoinHandle<()>>,
}

impl WebSocketServer {
    fn new() -> Self {
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

        Self {
            shutdown_tx,
            shutdown_rx: Arc::new(AsyncMutex::new(shutdown_rx)),
            server_handle: None,
        }
    }

    // Create listener and pass it to the service listener
    async fn start<F>(&mut self, addr: &str, connection_handler: F) -> Result<()>
    where
        F: Fn(TcpStream) -> () + Send + Sync + 'static + Clone,
    {
        let listener = TcpListener::bind(addr)
            .await
            .context("Failed to bind WebSocket server")?;

        let shutdown_tx = self.shutdown_tx.clone();
        let shutdown_rx = self.shutdown_rx.clone();

        let handler = connection_handler.clone();

        let server_handle = tokio::spawn(async move {
            /*
                What I learnt here:
                - The problem here is that shutdown_rx is borrowed across an asynchronous boundary 
                (inside tokio::spawn), but its lifetime cannot satisfy the 'static requirement. 
                This occurs because tokio::spawn requires all captured variables to have a 'static lifetime,
                meaning they must either be owned or not reference local variables directly.

                To fix this, you can clone the shutdown_rx before passing it into the tokio::spawn closure, 
                ensuring that the borrowed value isn't tied to the local scope.
                Also, declare the shutdown_rx_locked to ensure it is owned by the thread and lives long enough. 
             */
            let mut shutdown_rx_locked = shutdown_rx.lock().await;

            tokio::select! {
                _ = async {
                    loop {
                        match listener.accept().await {
                            Ok((stream, _)) => {
                                // Call the connection handler with the stream
                                handler(stream);
                            },
                            Err(e) => {
                                eprintln!("Accept error: {}", e);
                            }
                        }
                    }
                } => {},
                _ = shutdown_rx_locked.recv() => {
                    println!("WebSocket server shutting down");
                }
            }
        });

        self.server_handle = Some(server_handle);
        Ok(())
    }

    async fn shutdown(&mut self) {
        let _ = self.shutdown_tx.send(()).await;

        if let Some(handle) = self.server_handle.take() {
            let _ = handle.await;
        }
    }
}
