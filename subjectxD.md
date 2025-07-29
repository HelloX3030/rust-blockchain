Mini Blockchain Simulator — 42-Point Subject
Data Structures & Basics (1–10)

    Define a Block struct with these fields:

        index (u64)

        timestamp (u64 or chrono DateTime)

        transactions (Vec of a Transaction struct or strings)

        previous_hash (String or fixed byte array)

        nonce (u64)

        hash (String or fixed byte array)

    Define a Transaction struct with fields:

        sender (String)

        receiver (String)

        amount (f64 or u64)

    Define a Blockchain struct holding a chain of blocks (Vec or linked list).

    Implement Block::calculate_hash() method:

        Computes hash from all block fields except hash.

    Implement a method to create the genesis block (index=0, no previous hash).

    Implement Blockchain::new() creating a chain with the genesis block.

    Implement Blockchain::add_block() that:

        Accepts a block,

        Validates it,

        Adds to chain.

    Use a cryptographic hash function like SHA-256 or BLAKE3.

    Implement a simple way to print block info for debugging.

    Use serde to serialize/deserialize blocks and blockchain to/from JSON.

Proof of Work (11–20)

    Add difficulty field to Blockchain.

    Implement Block::mine(difficulty):

        Increment nonce until hash starts with difficulty number of zeros.

    Ensure Blockchain::add_block() validates proof of work before adding.

    Show mining progress (e.g., print nonce every N tries).

    Make mining interruptible or cancelable (optional).

    Benchmark mining speed for different difficulty levels.

    Validate that mined block’s hash meets difficulty target.

    Handle timestamp correctly during mining (update timestamp as needed).

    Ensure genesis block matches difficulty or set difficulty=0 for it.

    Write unit tests verifying proof-of-work correctness.

Transactions & Validation (21–30)

    Implement basic transaction validation:

        Sender and receiver not empty,

        Amount > 0.

    Add method to add transactions to a transaction pool (Vec).

    Implement adding transactions from the pool into new blocks.

    Validate that transactions in blocks are valid before mining.

    Implement block validation:

        Correct previous hash,

        Correct proof of work,

        Valid transactions.

    Add a method to verify the whole blockchain integrity.

    Write tests simulating tampering with block data or transactions.

    Implement balance tracking per user (optional).

    Reject blocks containing invalid transactions or bad proof of work.

    Log/print validation errors clearly.

Persistence & Interface (31–36)

    Save blockchain state to a JSON file.

    Load blockchain from JSON file at startup.

    Implement a simple CLI or REPL interface to:

        Show blockchain,

        Add transactions,

        Mine blocks,

        Validate chain.

    Allow exporting/importing blocks or transactions.

    Provide clear error handling and user feedback.

    Document usage and commands clearly.

Bonus / Networking (37–42)

    Use Tokio to implement a TCP or UDP peer-to-peer server.

    Broadcast newly mined blocks to peers.

    Receive blocks from peers and validate chain.

    Resolve forks using the longest valid chain.

    Synchronize transaction pools among peers.

    Secure communication (optional, e.g., encrypt or sign messages).
    