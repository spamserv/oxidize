use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use super::SubscriptionTopic;

pub type ClientId = String;

#[derive(Debug, Clone)]
pub struct SubscriptionManager {
    subscribers: Arc<RwLock<HashMap<SubscriptionTopic, Vec<ClientId>>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn subscribe(&self, client_id: ClientId, topic: SubscriptionTopic) {
        let mut subscribers = self.subscribers.write().await;
        subscribers.entry(topic).or_default().push(client_id);
    }

    pub async fn unsubscribe(&self, client_id: &ClientId, topic: &SubscriptionTopic) {
        let mut subscribers = self.subscribers.write().await;
        if let Some(subscribers_list) = subscribers.get_mut(topic) {
            subscribers_list.retain(|id| id != client_id);
        }
    }

    pub async fn get_subscribers(&self, topic: &SubscriptionTopic) -> Vec<ClientId> {
        let subscribers = self.subscribers.read().await;
        subscribers.get(topic).cloned().unwrap_or_default()
    }
}