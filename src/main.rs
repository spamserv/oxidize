use blockchain::blockchain::Blockchain;
use blockchain::wallet::Wallet;

fn main() {
    println!("Hello, world!");
    let mut node = Blockchain::build();
    node.add_block();

    let mut wallet1 = Wallet::new("Wallet#1".to_string());
    wallet1.create_new_address();
    wallet1.create_new_address();
    
    let mut wallet2 = Wallet::new("Wallet#2".to_string());
    wallet2.create_new_address();
    wallet2.create_new_address();

    dbg!({}, node);
    dbg!({}, wallet1);
    dbg!({}, wallet2);
}
