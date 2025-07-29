use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            sender: "default_sender".to_string(),
            recipient: "default_recipient".to_string(),
            amount: 0.0,
        }
    }
}
