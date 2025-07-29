use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

/// Represents a single block in the blockchain.
#[derive(Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Creates the genesis (first) block in the blockchain.
    pub fn new_genesis() -> Self {
        let mut block = Block {
            index: 0,
            timestamp: 0,
            transactions: Vec::new(),
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: String::new(),
        };
        block.update_hash();
        block
    }

    /// Calculates the hash of the block by serializing its data (excluding `hash`)
    /// and passing it through SHA256.
    pub fn calculate_hash(&self) -> String {
        let mut block_copy = self.clone();
        block_copy.hash = String::new(); // Exclude the hash field

        let bytes = to_vec(&block_copy).expect("Failed to serialize block for hashing");
        let hash = Sha256::digest(&bytes);
        format!("{:x}", hash)
    }

    /// Updates the block's hash field using its current contents.
    pub fn update_hash(&mut self) {
        self.hash = self.calculate_hash();
    }
}
