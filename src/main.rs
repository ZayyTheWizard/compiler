use std::env;
use std::fs;
use std::process::ExitCode;
mod tokenizer;
mod parser;

fn main() -> ExitCode {
    // Taking in source code as arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("No source file included\n");
        return ExitCode::FAILURE;
    }

    let file_path = &args[1];
    if file_path != "main.wz" {
        eprintln!("main file not found, found {} instead\n", file_path);
        return ExitCode::FAILURE;
    }

    let contents = fs::read_to_string(file_path).expect("Could Not Read");

    parser::print_ok(&contents);
    /* Converting to tokens
    let _tokens_list = tokenizer::tokenize(&contents);

    for it in &_tokens_list {
        println!("{:?}", it)
    }
    */
    return ExitCode::SUCCESS;
}