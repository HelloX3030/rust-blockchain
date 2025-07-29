mod block;
mod chain;
mod transaction;

use chain::Chain;

fn main() {
    let chain = Chain::new();
    println!("{:?}", chain)
}
