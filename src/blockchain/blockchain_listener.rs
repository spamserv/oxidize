use std::sync::Arc;

use crate::{wallet::Wallet, websockets::WebSocketServer};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::{oneshot, Mutex}, task::JoinHandle};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockchainWebsocketMessage {
    NewTransaction,
    QueryUtxo,
    WalletBalance,
    Ping,
    Other
}

#[derive(Debug, Clone)]
pub struct BlockchainListener {
    address: String,
    websocket_server: WebSocketServer,
    wallet: Arc<Mutex<Wallet>>
}

impl BlockchainListener {
    pub async fn new(address: String, wallet: Arc<Mutex<Wallet>>) -> Self {
        let server = WebSocketServer::new(&address);

        println!("Websocket initialized...");

        Self {
            address,
            websocket_server: server.clone(),
            wallet
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        // Start WebSocket server with connection handler
        let this = self.clone();
        self.websocket_server.start( move |stream| {
            // Use service listener to handle each connection
            this.handle_connection(stream);
        }).await?;

        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) -> JoinHandle<()> {
        // Clone wallets for use in async block
        let wallets = Arc::clone(&self.wallet);

        // Spawn task for each connection
        tokio::spawn(async move {
            // Upgrade to WebSocket
            let mut websocket = match accept_async(stream).await {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("WebSocket accept error: {}", e);
                    return;
                }
            };

            while let Some(msg) = websocket.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Deserialize the balance request
                        println!("{}", text);
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {} // Ignore non-text messages
                }
            }
        })
    }

    pub async fn shutdown(&mut self) {
        self.websocket_server.shutdown().await;
    }

}
