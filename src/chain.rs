use std::path;
use anyhow::Result;
use std::fs::File;
use serde::{Deserialize, Serialize};

use crate::block::Block;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Chain { blocks: Vec::with_capacity(10_000) };
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

    // TODO: add block
    //  - accepts a block
    //  - validates the block
    //  - adds the block to the chain
}
