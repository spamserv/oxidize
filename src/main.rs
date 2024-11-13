mod blockchain;
mod helpers;

use blockchain::*;

fn main() {
    println!("Hello, world!");
    let mut node = Blockchain::build();
    node.add_block();
    node.add_block();
    node.add_block();
    node.add_block();
    node.add_block();

    let blocks = node.blocks().clone(); 
    let third_block = blocks.get(2).unwrap().clone();
    match node.validate_single_block(third_block.header().current_hash()) {
        Err(e) => println!("Error on validating the block {} with error {}", third_block.header().current_hash(), e),
        Ok(_) => println!("Block validated")
    }

    let mut wallet1 = Wallet::new("Wallet#1".to_string());
    wallet1.create_new_address();
    wallet1.create_new_address();
    
    let mut wallet2 = Wallet::new("Wallet#2".to_string());
    wallet2.create_new_address();
    wallet2.create_new_address();

    dbg!(node);
    dbg!(wallet1);
    dbg!(wallet2);
}
