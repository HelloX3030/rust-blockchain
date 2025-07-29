use crate::transaction::Transaction;

/// Represents a single block in the blockchain.
///
/// Each block contains a list of transactions and links to the previous block via its hash.
/// It also includes a nonce used for proof-of-work mining and the block's own hash for integrity.
///
/// # Fields
///
/// - `index`: The position of the block in the blockchain (starting from 0 for the genesis block).
/// - `timestamp`: The time when the block was created, represented as UNIX epoch seconds.
/// - `transactions`: A list of transactions included in this block.
/// - `previous_hash`: The hash of the previous block in the chain, ensuring immutability and order.
/// - `nonce`: A number adjusted by miners to find a hash that satisfies the proof-of-work difficulty.
/// - `hash`: The cryptographic hash of this block, calculated over its contents including the nonce.
/// 
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}
