use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, BorshSerialize, BorshDeserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f64,
}
