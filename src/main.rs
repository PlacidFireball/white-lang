mod tokenizer;
use tokenizer::*;
mod parser;
use parser::Parser;
mod parser_traits;
mod program;

mod symbol_table;

#[allow(unused_variables)]
fn main() {
    let tokenizer = Tokenizer::init("1.1".to_string());
    let parser = Parser::init(&mut tokenizer.clone());
}
