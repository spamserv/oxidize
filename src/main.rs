use std::error::Error;

use colored::Colorize;
use oxidize::{
    blockchain::{Blockchain, BlockchainConfig}, config::WEBSOCKET_URI, logger::init_logging, wallet::Wallet
};
use tracing::info;

const NUMBER_OF_BLOCKS: u16 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _guard = init_logging();
    
    let config = BlockchainConfig::new(false);

    let mut node = Blockchain::build(config)
        .await
        .expect("Cannot create blockchain.");
    info!(
        "{}",
        "Initiated blockchain, with genesis block".bold().green()
    );
    info!("{:?}", node.config());

    // let ws_port = node.config().

    info!("Creating {} blocks", NUMBER_OF_BLOCKS);
    for _ in 1..=NUMBER_OF_BLOCKS {
        node.add_block().await;
    }

    let blocks = node.blocks().clone();
    let block_1 = blocks.get(1).unwrap().clone();
    let block_2 = blocks.get(3).unwrap().clone();

    match node.validate_single_block(block_1.header().current_hash()) {
        Err(e) => info!(
            "Error on validating the block {} with error {}",
            block_1.header().current_hash(),
            e
        ),
        Ok(_) => info!("{}", "Block validated".bold().green()),
    }

    match node.validate_full_chain() {
        Err(e) => info!("Error validating full chain with error {}", e),
        Ok(_) => info!("{}", "Full chain validated".bold().green()),
    }

    match node.validate_range_chain(
        block_1.header().current_hash(),
        block_2.header().current_hash(),
    ) {
        Err(e) => info!("Error validating chain range with error {}", e),
        Ok(_) => info!(
            "{} from: {} to {}",
            "Chain range validated".bold().green(),
            block_1.header().current_hash().yellow(),
            block_2.header().current_hash().yellow()
        ),
    }

    info!("{}", "Creating 2 wallets".bold());
    let mut wallet1 = Wallet::new("Wallet#1".to_string(), WEBSOCKET_URI.to_string()).await;
    wallet1.create_new_account("MainAccount");
    wallet1.create_new_account("SecondAccount");

    let mut wallet2 = Wallet::new("Wallet#2".to_string(), WEBSOCKET_URI.to_string()).await;
    wallet2.create_new_account("MiceAccount");
    wallet2.create_new_account("CheeseAccount");
    let wallet2_account = wallet2.find_account("MiceAccount")?.address().to_string();
    
    wallet1.initiate_payment("MainAccount", &wallet2_account, 5).await?;
    wallet1.initiate_payment("SecondAccount", &wallet2_account, 25).await?;
    wallet1.initiate_payment("SecondAccount", &wallet2_account, 15).await?;
    // dbg!(node);
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
