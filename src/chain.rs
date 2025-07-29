use crate::block::Block;

#[derive(Debug)]
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn new() -> Self {
        let mut chain = Chain { blocks: Vec::with_capacity(10_000) };
        chain.blocks.push(Block::new_genesis());
        return chain;
    }
    // TODO: store to file
    // TODO: add block
    //  - accepts a block
    //  - validates the block
    //  - adds the block to the chain
}
