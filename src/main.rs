mod tokenizer;
use tokenizer::*;
mod parser;
use parser::Parser;

#[allow(unused_variables)]
fn main() {
    let tokenizer = Tokenizer::init("1.1".to_string());
    let parser = Parser::init(tokenizer.clone());
}
