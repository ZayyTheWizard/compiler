use std::env;
use std::fs;

fn main() {
    println!("Hello, world!\n");

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Could Not Read");

    println!("contents:\n {}", contents); 

    iterate(&contents);
}

fn iterate(s: &String) {
    let mut i = 0;
    while i < s.chars().count() {
        let this = s.chars().nth(i);
        println!("{}th char: {:?}\n", i, this);
        i += 1;
    }
}
