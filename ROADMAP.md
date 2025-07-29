Mini Blockchain Simulator — Project Roadmap
Phase 1: Basic Blockchain Structure
Goals:

    Define the core Block struct.

    Implement a basic Blockchain as a linked list of blocks.

    Add transaction data storage inside blocks.

Tasks:

    Design Block struct with:

        index or height

        timestamp

        transactions (can start as simple Vec<String> or a custom struct)

        previous_hash (String or fixed-size array)

        nonce (for proof-of-work)

        hash (current block hash)

    Design Blockchain struct that stores blocks in a vector or linked structure.

    Implement a method to calculate hash of a block (using sha2 or blake3).

    Implement a method to add a block to the chain (reference previous hash).

Phase 2: Proof-of-Work (Mining)
Goals:

    Implement simple proof-of-work algorithm.

    Control mining difficulty by requiring hash to have leading zeros.

Tasks:

    Add difficulty parameter to blockchain.

    Implement mine_block function:

        Increment nonce until hash meets difficulty target.

    Validate block hash before adding to chain.

    Print mining progress or stats.

Phase 3: Transaction Handling
Goals:

    Replace simple strings with structured transactions.

    Validate transactions inside blocks.

Tasks:

    Create a Transaction struct with fields like sender, receiver, amount.

    Add basic transaction validation (non-empty fields, positive amounts).

    Add method to collect transactions into blocks.

    Optionally implement a transaction pool (unconfirmed transactions).

Phase 4: Blockchain Validation and Integrity
Goals:

    Validate entire blockchain integrity.

    Detect tampering or invalid blocks.

Tasks:

    Implement a method to iterate over the chain and verify:

        Each block’s previous_hash matches the hash of the previous block.

        Each block’s hash is valid (meets difficulty, matches content).

    Write tests to simulate tampering and detect invalid chains.

Phase 5: Persistence and CLI Interface (Optional)
Goals:

    Save/load blockchain state from file.

    Interact with blockchain from command line.

Tasks:

    Serialize blockchain using serde + serde_json or binary formats.

    Add CLI commands for:

        Creating blockchain,

        Adding transactions,

        Mining blocks,

        Validating chain,

        Printing blockchain state.

    Use clap crate to build CLI.

Bonus Phase: Networking (Optional)
Goals:

    Add simple peer-to-peer syncing.

    Broadcast blocks/transactions over network.

Tasks:

    Use Tokio + TCP or UDP to send/receive blocks.

    Merge chains, handle forks.

    Implement simple consensus rules (longest chain wins).

Suggested Timeline
Phase	Time Estimate
Phase 1	1–2 days
Phase 2	2 days
Phase 3	2–3 days
Phase 4	1–2 days
Phase 5 (optional)	2–3 days
Bonus Phase	3+ days (networking is complex)
Tools & Crates to Use

    Hashing: sha2 or blake3

    Serialization: serde + serde_json

    CLI: clap

    Async/network (optional): tokio
