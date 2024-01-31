use std::env;
use std::fs;
pub mod tokenizer;

fn main() {
    println!("Hello, world!\n");

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Could Not Read");

    println!("contents:\n {}", contents); 

    tokenizer::iterate(&contents);
}