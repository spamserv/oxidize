mod blockchain;
mod helpers;

use blockchain::*;

use colored::Colorize;

const NUMBER_OF_BLOCKS: u16 = 12;

fn main() {
    println!("Hello, world!");

    let mut node = Blockchain::build();
    println!("{}", "Initiated blockchain, with genesis block".bold().green());
    dbg!(node.config());

    println!("Creating {} blocks", NUMBER_OF_BLOCKS);
    for _ in 1..=NUMBER_OF_BLOCKS {
        node.add_block();
    }

    let blocks = node.blocks().clone(); 
    let third_block = blocks.get(2).unwrap().clone();
    let seventh_block = blocks.get(6).unwrap().clone();
    match node.validate_single_block(third_block.header().current_hash()) {
        Err(e) => println!("Error on validating the block {} with error {}", third_block.header().current_hash(), e),
        Ok(_) => println!("{}", "Block validated".bold().green())
    }

    match node.validate_full_chain() {
        Err(e) => println!("Error validating full chain with error {}", e),
        Ok(_) => println!("{}", "Full chain validated".bold().green())
    }

    match node.validate_range_chain(third_block.header().current_hash(), seventh_block.header().current_hash()) {
        Err(e) => println!("Error validating chain range with error {}", e),
        Ok(_) => println!("{} from: {} to {}", 
            "Chain range validated".bold().green(), 
            third_block.header().current_hash().yellow(), 
            seventh_block.header().current_hash().yellow()
        )
    }

    println!("{}", "Creating 2 wallets".bold());
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
