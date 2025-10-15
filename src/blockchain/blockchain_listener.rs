use std::sync::Arc;

use crate::{
    comms,
    websockets::{SubscriptionManager, SubscriptionMessage, SubscriptionTopic, WebSocketServer},
};
use anyhow::{Error, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
            .run(addr, move |message, client_id, _clients| {
                let self_ref = Arc::clone(&self_ref);
                Box::pin(async move {
                    println!("Message from client {}: {}", client_id, message);
                    let raw_message: comms::Message<Value> = match serde_json::from_str(&message) {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("Failed to parse message from client {}: {}", client_id, e);
                            return;
                        }
                    };
                    // Add custom blockchain-related message handling logic here
                    match &raw_message {
                        comms::Message::Request {
                            id,
                            r#type,
                            payload,
                        } => {
                            println!(
                                "Received request: id={}, type={:?}, payload={:?}",
                                id, r#type, payload
                            );
                        }
                        comms::Message::Response {
                            id,
                            status,
                            data,
                            error,
                        } => {
                            println!(
                                "Received response: id={}, status={:?}, data={:?}, error={:?}",
                                id, status, data, error
                            );
                        }
                        comms::Message::Event { id, topic, data } => {
                            println!(
                                "Received event: id={}, topic={:?}, data={:?}",
                                id, topic, data
                            );
                            let self_locked = self_ref.lock().await;

                            self_locked
                                .subscription_manager
                                .subscribe(client_id.clone(), topic.clone())
                                .await;
                        }
                    }
                })
            })
            .await;
    }

    pub async fn send<T>(&self, client_id: usize, message: comms::Message<T>) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        let serialized_message = serde_json::to_string(&message)?;
        self.server.send(client_id, serialized_message).await;
        Ok(())
    }

    pub async fn broadcast(&self, message: String) {
        self.server.broadcast(message).await;
    }

}
