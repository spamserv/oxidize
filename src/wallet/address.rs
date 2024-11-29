use crate::transaction::Transaction;

#[derive(Debug, Clone, Default)]
pub struct Address {
    pub id: String,
    pub transactions: Vec<Transaction>,
}

impl Address {
    pub fn id(&self) -> &String {
        &self.id
    }
}
