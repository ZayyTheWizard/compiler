use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../Docs/grammar.pest"]
pub struct MyParser;

pub fn print_ok(source: &String) {
    let successful_parse = MyParser::parse(Rule::program, source);
    println!("{:?}", successful_parse);
}