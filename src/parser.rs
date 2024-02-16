use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../Docs/grammar.pest"]
pub struct MyParser;

pub fn print_ok(source: &String) {
    let successful_parse = MyParser::parse(Rule::Program, source);
    println!("{:?}", successful_parse);

    let unsuccessful_parse = MyParser::parse(Rule::Program, source);
    println!("{:?}", unsuccessful_parse);
}