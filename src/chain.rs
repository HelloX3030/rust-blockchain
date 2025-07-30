use std::path;
use anyhow::{Result, anyhow};
use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;
use crate::block::{Block, TRANSACTIONS_PER_BLOCK};

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
    pub mempool: Vec<Transaction>,
    pub next_index: u64,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Chain { blocks: Vec::with_capacity(10_000), mempool: Vec::with_capacity(1_000), next_index: 1 };
        chain.blocks.push(Block::new_genesis()); 
        return chain;
    }
    
    pub fn store(&self, path: &str) -> Result<()> {
        let file_path = path::Path::new(path);
        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    pub fn load(path: &str) -> Option<Self>
    {
        let file_path = path::Path::new(path);

        if !file_path.exists() {
            return None;
        }

        let file = File::open(file_path).unwrap_or_else(|_| {
            panic!("Failed to open file: {}", file_path.display())
        });

        let chain: Self = serde_json::from_reader(file).unwrap_or_else(|_| {
            panic!("Failed to deserialize chain from file: {}", file_path.display())
        });

        Some(chain)
    }

    pub fn mine(&mut self) -> Result<()> {
        while !self.mempool.is_empty(){
            let tx_count = std::cmp::min(TRANSACTIONS_PER_BLOCK, self.mempool.len());
            let handle_transactions: Vec<Transaction> = self.mempool.drain(..tx_count).collect();
            let previous_hash = self.blocks.last().ok_or_else(|| anyhow!("Blockchain is empty"))?.hash.clone();
            let mut block = Block::new(self.next_index, previous_hash, handle_transactions);
            self.next_index += 1;
            block.mine().expect("Failed to mine block");
            println!("Mined block {:?}", block);
            self.blocks.push(block);
        }
        Ok(())
    }

    pub fn add_example_transactions(&mut self, amount: u64) -> Result<()> {
        for i in 0..amount {
            let tx = Transaction {
                sender: format!("sender_{}", i),
                recipient: format!("recipient_{}", i),
                amount: (i + 1) as f64 * 10.0,
            };
            self.mempool.push(tx);
        }
        Ok(())
    }
}
