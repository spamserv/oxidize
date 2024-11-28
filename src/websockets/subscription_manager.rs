use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

type Topic = String;
type ClientId = String;

pub struct SubscriptionManager {
    subscribers: Arc<RwLock<HashMap<Topic, Vec<ClientId>>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn subscribe(&self, client_id: ClientId, topic: Topic) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.entry(topic).or_default().push(client_id);
    }

    pub async fn unsubscribe(&self, client_id: &ClientId, topic: &Topic) {
        let mut subscribers = self.subscribers.write().await;
        if let Some(subscribers_list) = subscribers.get_mut(topic) {
            subscribers_list.retain(|id| id != client_id);
        }
    }

    pub async fn get_subscribers(&self, topic: &Topic) -> Vec<ClientId> {
        let subscribers = self.subscribers.read().await;
        subscribers.get(topic).cloned().unwrap_or_default()
    }
}