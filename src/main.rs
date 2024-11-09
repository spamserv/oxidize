use blockchain::blockchain::Blockchain;

fn main() {
    println!("Hello, world!");
    let node = Blockchain::build();

    dbg!({}, node);
}
