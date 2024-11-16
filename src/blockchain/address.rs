use super::Transaction;

#[derive(Debug, Clone)]
pub struct Address {
    pub id: String,
    pub transactions: Vec<Transaction>
}

impl Address {
    pub fn id(&self) -> &String {
        &self.id
    }
}