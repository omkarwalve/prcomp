// Main.rs
use std::env;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("GENRE: {}\nQuery: {:?}", &arguments[1], &arguments[2..]);
}
