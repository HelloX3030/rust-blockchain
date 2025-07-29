mod block;
mod chain;
mod transaction;

use chain::Chain;

fn main() {
    println!("Creating a new blockchain or loading existing one...");
    let mut chain = Chain::new();
    if let Some(loaded_chain) = Chain::load("chain.json") {
        chain = loaded_chain;
    }
    println!("{:?}", chain);
    println!("Adding example transactions to the chain...");
    chain.add_example_transactions(20).expect("Failed to add example transactions");
    println!("Mining blocks...");
    chain.mine().expect("Failed to mine block");
    println!("Chain after mining: {:?}", chain);
    println!("Storing the chain to 'chain.json'...");
    chain.store("chain.json").expect("Failed to store chain");
}
