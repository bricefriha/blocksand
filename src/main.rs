mod structs;
use structs::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    // Add blocks
    blockchain.add_block("First Block".to_string());

    for block in &blockchain.chain {
        println!("{:?}", block);
    }
}
