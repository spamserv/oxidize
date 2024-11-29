use std::{collections::HashMap, sync::Arc};

use crate::{wallet::{MessagePayload, Wallet, WalletMessage, WalletMessageDirection, WalletMessagePayload}, websockets::{subscription_manager::ClientId, SubscriptionManager, SubscriptionMessage, SubscriptionTopic, WebSocketServer}};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, sync::{oneshot, Mutex, RwLock}, task::JoinHandle};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use anyhow::{Error, Result};

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
    websocket_server: Arc<Mutex<WebSocketServer>>,
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
            websocket_server: Arc::new(Mutex::new(server)),
            wallet,
            subscription_manager: Arc::new(SubscriptionManager::new()),
            websocket_clients: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn start( &self) -> Result<()> {
        // Start WebSocket server with connection handler
        let this = Arc::new(self.clone());
        let mut websocket_server = self.websocket_server.lock().await;
        websocket_server.start( move |stream| {
            // Use service listener to handle each connection
            let this_clone = Arc::clone(&this);
            this_clone.handle_connection(stream);
        }).await?;

        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) -> JoinHandle<()> {
        // Clone wallets for use in async block
        let wallets = Arc::clone(&self.wallet);
        let subscription_manager = Arc::clone(&self.subscription_manager);
        
        /*
            You have to create new atomically reference counted instance because it is used 
            by tokio tasks and consumed by it, so lifetimes aren't matched otherwise.
        */  
        let this = Arc::new(self.clone());

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
                println!("{:?}",msg);
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

                                    let payload = MessagePayload::Balance { balance: 65 };
                                    let message = WalletMessage::new("request_id".to_string(), "account_id".to_string(), WalletMessageDirection::ServerToClient, payload);

                                    let _ = this.broadcast_to_topic(SubscriptionTopic::WalletBalance, message).await;
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

    async fn broadcast_to_topic<T>(&self, topic: SubscriptionTopic, message: WalletMessage<T>) -> Result<(), Error> where T: WalletMessagePayload{ 
        let subscribers = self.subscription_manager.get_subscribers(&topic).await;
        let websocket_clients = &mut self.websocket_clients.write().await;
        let serialized_message = serde_json::to_string(&message)?;
        println!("{:?}", websocket_clients);
        // TODO: Check why websocket_clients aren't stored properly
        for subscriber in subscribers {
            if let Some(websocket_client) = websocket_clients.get_mut(&subscriber) {
                websocket_client.send(Message::Text(serialized_message.clone())).await?;
                println!("Sent!");
            }
            
            
        }   
        Ok(())
    }

    pub async fn shutdown(&mut self) {
        let mut websocket_server_mutex = self.websocket_server.lock().await;
        websocket_server_mutex.shutdown().await;
    }

}
