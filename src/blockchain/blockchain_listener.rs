use std::{collections::HashMap, sync::Arc};

use crate::{
    wallet::{MessagePayload, Wallet, WalletMessage, WalletMessageDirection, WalletMessagePayload},
    websockets::{
        subscription_manager::ClientId, SubscriptionManager, SubscriptionMessage,
        SubscriptionTopic, WebSocketServer,
    },
};
use anyhow::{Error, Result};
use async_trait::async_trait;
use colored::Colorize;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpStream,
    sync::{broadcast, Mutex, RwLock},
    task::JoinHandle,
};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

#[derive(Debug, Serialize, Deserialize)]
pub enum BlockchainWebsocketMessage {
    NewTransaction,
    QueryUtxo,
    WalletBalance,
    Ping,
    Other,
}

#[derive(Debug, Clone)]
pub struct BlockchainListener {
    server: WebSocketServer,
    subscription_manager: Arc<SubscriptionManager>
}

impl BlockchainListener {
    pub fn new() -> Self {
        Self {
            server: WebSocketServer::new(),
            subscription_manager: Arc::new(SubscriptionManager::new()),
        }
    }

    pub async fn run(&self, addr: &str) {
        let self_ref = Arc::new(Mutex::new(self.clone()));
        
        self.server
            .run(addr, move |message, client_id, clients| {
                let self_ref = Arc::clone(&self_ref);
                Box::pin(async move {
                    println!("Message from client {}: {}", client_id, message);
                    // Add custom blockchain-related message handling logic here
                    match serde_json::from_str::<SubscriptionMessage>(&message) {
                        Ok(subscription_message) => match subscription_message.topic {
                            SubscriptionTopic::Transactions => {
                                println!("Client {} subscribed to Transactions", client_id);
                                let self_locked = self_ref.lock().await;
                                self_locked.subscription_manager
                                    .subscribe(
                                        client_id,
                                        SubscriptionTopic::Transactions,
                                    )
                                    .await;
                            }
                            SubscriptionTopic::WalletBalance => {
                                println!("Client {} subscribed to Wallet Balance", client_id);
                                let self_locked = self_ref.lock().await;
                                self_locked.subscription_manager
                                    .subscribe(
                                        client_id,
                                        SubscriptionTopic::WalletBalance,
                                    )
                                    .await;

                                let payload = MessagePayload::Balance { balance: 65 };
                                let message = WalletMessage::new(
                                    "request_id".to_string(),
                                    "account_id".to_string(),
                                    WalletMessageDirection::ServerToClient,
                                    payload,
                                );

                                match self_locked.send(client_id, message).await {
                                    Err(_) => println!("{} {}", "Message not sent successfully to".red(), client_id.to_string().red()),
                                    Ok(_) => {}
                                };

                                // Send immediatelly message about balance to that user
                                //let _ = this.broadcast_to_topic(SubscriptionTopic::WalletBalance, message).await;
                            }
                            SubscriptionTopic::BlockchainStatus => {
                                println!(
                                    "Client {} subscribed to Blockchain Status",
                                    client_id
                                );
                                let self_locked = self_ref.lock().await;
                                self_locked.subscription_manager
                                    .subscribe(
                                        client_id.clone(),
                                        SubscriptionTopic::BlockchainStatus,
                                    )
                                    .await;
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed to parse subscription message: {}", e);
                        }
                    }
                })
            })
            .await;
    }

    pub async fn send<T>(&self, client_id: usize, message: WalletMessage<T>) -> Result<(), Error> where T:WalletMessagePayload {
        let serialized_message = serde_json::to_string(&message)?;
        self.server.send(client_id, serialized_message).await;
        Ok(())
    }

    pub async fn broadcast(&self, message: String) {
        self.server.broadcast(message).await;
    }
}

/*
async fn handle_connection(&self, stream: TcpStream) -> Result<JoinHandle<()>, Error> {
        // Clone wallets for use in async block
        let wallets = Arc::clone(&self.wallet);
        let subscription_manager = Arc::clone(&self.subscription_manager);

        /*
            You have to create new atomically reference counted instance because it is used
            by tokio tasks and consumed by it, so lifetimes aren't matched otherwise.
        */
        let this = Arc::new(self.clone());

        // Upgrade to WebSocket
        let websocket_stream = accept_async(stream).await?;

        let (mut ws_sender, mut ws_receiver) = websocket_stream.split();

        let mut rx
        // Spawn task for each connection
        let thread_handle = tokio::spawn(async move {

            let client_id = uuid::Uuid::new_v4().to_string();
            let websocket_cloned = Arc::new(Mutex::new(websocket));

            this.add_client(client_id.clone(), Arc::clone(&websocket_cloned))
                .await;

            while let Some(msg) = websocket_cloned.lock().await.next().await {
                println!("{:?}", msg);
                match msg {
                    Ok(Message::Text(text)) => {
                        // Attempt to deserialize the text into a SubscriptionMessage
                        match serde_json::from_str::<SubscriptionMessage>(&text) {
                            Ok(subscription_message) => match subscription_message.topic {
                                SubscriptionTopic::Transactions => {
                                    println!("Client {} subscribed to Transactions", client_id);
                                    subscription_manager
                                        .subscribe(
                                            client_id.clone(),
                                            SubscriptionTopic::Transactions,
                                        )
                                        .await;
                                }
                                SubscriptionTopic::WalletBalance => {
                                    println!("Client {} subscribed to Wallet Balance", client_id);
                                    subscription_manager
                                        .subscribe(
                                            client_id.clone(),
                                            SubscriptionTopic::WalletBalance,
                                        )
                                        .await;

                                    let payload = MessagePayload::Balance { balance: 65 };
                                    let message = WalletMessage::new(
                                        "request_id".to_string(),
                                        "account_id".to_string(),
                                        WalletMessageDirection::ServerToClient,
                                        payload,
                                    );

                                    // Send immediatelly message about balance to that user
                                    //let _ = this.broadcast_to_topic(SubscriptionTopic::WalletBalance, message).await;
                                }
                                SubscriptionTopic::BlockchainStatus => {
                                    println!(
                                        "Client {} subscribed to Blockchain Status",
                                        client_id
                                    );
                                    subscription_manager
                                        .subscribe(
                                            client_id.clone(),
                                            SubscriptionTopic::BlockchainStatus,
                                        )
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
                        println!(
                            "Received an unsupported message type from client {}",
                            client_id
                        );
                    }
                    Err(e) => eprintln!("Failed to parse the tungstenite Message: {}", e),
                    _ => eprintln!("Not even sure what could happen here... {}", client_id),
                }
            }
        })

        Ok(thread_handle)
    }
*/
