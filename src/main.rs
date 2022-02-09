
mod tokenizer;
use tokenizer::Tokenizer;
mod parser;
use parser::Parser;

fn main() {
    let mut tokenizer = Tokenizer::init("1.1".to_string());
    //let mut parser = Parser::init(tokenizer);
    tokenizer.tokenize();
}
