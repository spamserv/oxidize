use blockchain::blockchain::Blockchain;

fn main() {
    println!("Hello, world!");
    let mut node = Blockchain::build();
    node.add_block();
    dbg!({}, node);
}
