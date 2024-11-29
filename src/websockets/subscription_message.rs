use serde::{Deserialize, Serialize};

use super::SubscriptionTopic;

#[derive(Deserialize, Serialize)]
pub struct SubscriptionMessage {
    pub action: String,
    pub topic: SubscriptionTopic, // Enum for type-safe matching
}
