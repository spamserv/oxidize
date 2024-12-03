use std::sync::Arc;

use crate::{
    wallet::{MessagePayload, WalletMessage, WalletMessageDirection, WalletMessagePayload},
    websockets::{SubscriptionManager, SubscriptionMessage, SubscriptionTopic, WebSocketServer},
};
use anyhow::{Error, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

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
    subscription_manager: Arc<SubscriptionManager>,
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
                                self_locked
                                    .subscription_manager
                                    .subscribe(client_id, SubscriptionTopic::Transactions)
                                    .await;
                            }
                            SubscriptionTopic::WalletBalance => {
                                println!("Client {} subscribed to Wallet Balance", client_id);
                                let self_locked = self_ref.lock().await;
                                self_locked
                                    .subscription_manager
                                    .subscribe(client_id, SubscriptionTopic::WalletBalance)
                                    .await;

                                let payload = MessagePayload::Balance { balance: 65 };
                                let message = WalletMessage::new(
                                    "request_id".to_string(),
                                    "account_id".to_string(),
                                    WalletMessageDirection::ServerToClient,
                                    payload,
                                );

                                match self_locked.send(client_id, message).await {
                                    Err(_) => println!(
                                        "{} {}",
                                        "Message not sent successfully to".red(),
                                        client_id.to_string().red()
                                    ),
                                    Ok(_) => {}
                                };

                                // Send immediatelly message about balance to that user
                                //let _ = this.broadcast_to_topic(SubscriptionTopic::WalletBalance, message).await;
                            }
                            SubscriptionTopic::BlockchainStatus => {
                                println!("Client {} subscribed to Blockchain Status", client_id);
                                let self_locked = self_ref.lock().await;
                                self_locked
                                    .subscription_manager
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

    pub async fn send<T>(&self, client_id: usize, message: WalletMessage<T>) -> Result<(), Error>
    where
        T: WalletMessagePayload,
    {
        let serialized_message = serde_json::to_string(&message)?;
        self.server.send(client_id, serialized_message).await;
        Ok(())
    }

    pub async fn broadcast(&self, message: String) {
        self.server.broadcast(message).await;
    }
}