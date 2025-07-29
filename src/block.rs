use serde::{Deserialize, Serialize};
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use sha2::{Sha256, Digest};
use crate::transaction::{Transaction};
use anyhow::{bail, Result};
use std::time::{SystemTime, UNIX_EPOCH};

// number of leading zeros required
pub const DIFFICULTY: usize = 4;
pub const TRANSACTIONS_PER_BLOCK: usize = 10;


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
    pub fn new_genesis() -> Self {
        let mut block = Block {
            index: 0,
            timestamp: 0,
            transactions: Vec::new(),
            previous_hash: "0".to_string(),
            nonce: 0,
            hash: String::new(),
        };
        block
    }

    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            index,
            timestamp: 0,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut block_copy = self.clone();
        block_copy.hash = String::new(); // Exclude the hash field

        let bytes = to_vec(&block_copy).expect("Failed to serialize block for hashing");
        let hash = Sha256::digest(&bytes);
        format!("{:x}", hash)
    }

    pub fn update_hash(&mut self) {
        self.hash = self.calculate_hash();
    }

    pub fn mine(&mut self) -> Result<()> {
        if self.transactions.len() < TRANSACTIONS_PER_BLOCK {
            bail!("Not enough transactions to mine a block. Required: {}, Found: {}", 
                  TRANSACTIONS_PER_BLOCK, self.transactions.len());
        }
        let start = SystemTime::now();
        self.timestamp =  start.duration_since(UNIX_EPOCH).expect("Time went backwards???").as_secs();
        let target_prefix = "0".repeat(DIFFICULTY);
        while !self.calculate_hash().starts_with(&target_prefix) {
            self.nonce += 1;
            self.update_hash();
        }
        Ok(())
    }
}
