use std::{collections::HashMap, sync::Arc};

use crate::{wallet::Wallet, websockets::{subscription_manager::ClientId, SubscriptionManager, SubscriptionMessage, SubscriptionTopic, WebSocketServer}};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::{oneshot, Mutex, RwLock}, task::JoinHandle};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
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
    wallet: Arc<Mutex<Wallet>>,
    subscription_manager: Arc<SubscriptionManager>,
    websocket_clients: Arc<RwLock<HashMap<ClientId, WebSocketStream<TcpStream>>>>
}

impl BlockchainListener {
    pub async fn new(address: String, wallet: Arc<Mutex<Wallet>>) -> Self {
        let server = WebSocketServer::new(&address);

        println!("Websocket initialized...");

        Self {
            address,
            websocket_server: server.clone(),
            wallet,
            subscription_manager: Arc::new(SubscriptionManager::new()),
            websocket_clients: Arc::new(RwLock::new(HashMap::new()))
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
        let subscription_manager = Arc::clone(&self.subscription_manager);

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

            let client_id = uuid::Uuid::new_v4().to_string();

            while let Some(msg) = websocket.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Attempt to deserialize the text into a SubscriptionMessage
                        match serde_json::from_str::<SubscriptionMessage>(&text) {
                            Ok(subscription_message) => match subscription_message.topic {
                                SubscriptionTopic::Transactions => {
                                    println!("Client {} subscribed to Transactions", client_id);
                                    subscription_manager
                                        .subscribe(client_id.clone(), SubscriptionTopic::Transactions)
                                        .await;
                                }
                                SubscriptionTopic::WalletBalance => {
                                    println!("Client {} subscribed to Wallet Balance", client_id);
                                    subscription_manager
                                        .subscribe(client_id.clone(), SubscriptionTopic::WalletBalance)
                                        .await;
                                }
                                SubscriptionTopic::BlockchainStatus => {
                                    println!("Client {} subscribed to Blockchain Status", client_id);
                                    subscription_manager
                                        .subscribe(client_id.clone(), SubscriptionTopic::BlockchainStatus)
                                        .await;
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to parse subscription message: {}", e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Client {} disconnected", client_id);
                    }
                    Ok(Message::Ping(data)) => {
                        println!("Received ping from client {}: {:?}", client_id, data);
                    }
                    Ok(Message::Pong(data)) => {
                        println!("Received pong from client {}: {:?}", client_id, data);
                    }
                    Ok(Message::Binary(_)) => {
                        // Handle other message types if needed
                        println!("Received an unsupported message type from client {}", client_id);
                    },
                    Err(e) => eprintln!("Failed to parse the tungstenite Message: {}", e),
                    _ => eprintln!("Not even sure what could happen here... {}", client_id)
                }
            }
        })
    }

    pub async fn shutdown(&mut self) {
        self.websocket_server.shutdown().await;
    }

}
