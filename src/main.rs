mod block;
mod chain;
mod transaction;

use chain::Chain;

fn main() {
    let mut chain = Chain::new();
    if let Some(loaded_chain) = Chain::load("chain.json") {
        chain = loaded_chain;
    }
    println!("{:?}", chain);
    chain.store("chain.json").expect("Failed to store chain");
}
