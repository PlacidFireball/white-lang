mod tokenizer;
use tokenizer::*;
mod parser;
mod parser_traits;
mod symbol_table;
mod program;

use parser::Parser;

#[allow(unused_variables)]
fn main() {
    let tokenizer = Tokenizer::init("1.1".to_string());
    let parser = Parser::init(&mut tokenizer.clone());
}
