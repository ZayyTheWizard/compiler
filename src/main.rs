use std::env;
use std::fs;
use std::process::ExitCode;
mod tokenizer;

fn main() -> ExitCode {
    // Taking in source code as arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No source file included");
        return ExitCode::FAILURE;
    }
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Could Not Read");

    println!("contents:\n {}", contents); 

    // Converting to tokens
    let _tokens_list = tokenizer::tokenize(&contents);

    for it in &_tokens_list {
        println!("{:?}", it)
    }

    return ExitCode::SUCCESS;
}