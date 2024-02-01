use std::env;
use std::fs;
mod tokenizer;

fn main() {
    // Taking in source code as arguments
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Could Not Read");

    println!("contents:\n {}", contents); 

    // Converting to tokens
    let _tokens_list = tokenizer::tokenize(&contents);

    for it in &_tokens_list {
        println!("{:?}", it)
    }
}