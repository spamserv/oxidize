use std::error::Error;

use colored::Colorize;
use oxidize::{
    blockchain::{Blockchain, BlockchainConfig},
    config::WEBSOCKET_URI,
    wallet::Wallet,
};

const NUMBER_OF_BLOCKS: u16 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let config = BlockchainConfig::new(false);

    let mut node = Blockchain::build(config)
        .await
        .expect("Cannot create blockchain.");
    println!(
        "{}",
        "Initiated blockchain, with genesis block".bold().green()
    );
    dbg!(node.config());

    // let ws_port = node.config().

    println!("Creating {} blocks", NUMBER_OF_BLOCKS);
    for _ in 1..=NUMBER_OF_BLOCKS {
        node.add_block().await;
    }

    let blocks = node.blocks().clone();
    let block_1 = blocks.get(1).unwrap().clone();
    let block_2 = blocks.get(3).unwrap().clone();

    match node.validate_single_block(block_1.header().current_hash()) {
        Err(e) => println!(
            "Error on validating the block {} with error {}",
            block_1.header().current_hash(),
            e
        ),
        Ok(_) => println!("{}", "Block validated".bold().green()),
    }

    match node.validate_full_chain() {
        Err(e) => println!("Error validating full chain with error {}", e),
        Ok(_) => println!("{}", "Full chain validated".bold().green()),
    }

    match node.validate_range_chain(
        block_1.header().current_hash(),
        block_2.header().current_hash(),
    ) {
        Err(e) => println!("Error validating chain range with error {}", e),
        Ok(_) => println!(
            "{} from: {} to {}",
            "Chain range validated".bold().green(),
            block_1.header().current_hash().yellow(),
            block_2.header().current_hash().yellow()
        ),
    }

    println!("{}", "Creating 2 wallets".bold());
    let mut wallet1 = Wallet::new("Wallet#1".to_string(), WEBSOCKET_URI.to_string());
    wallet1.connect().await?;
    wallet1.create_new_account();
    wallet1.create_new_account();

    let mut wallet2 = Wallet::new("Wallet#2".to_string(), WEBSOCKET_URI.to_string());

    wallet2.connect().await?;
    wallet2.create_new_account();
    wallet2.create_new_account();

    //dbg!(node);
    // dbg!(
    //     wallet1.id,
    //     wallet1.name,
    //     wallet1.created_at,
    //     wallet1.accounts
    // );
    // dbg!(
    //     wallet2.id,
    //     wallet2.name,
    //     wallet2.created_at,
    //     wallet2.accounts
    // );

    loop {
        // DO STUFF
    }
}
