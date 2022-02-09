
#[path = "tokenizer/tokenizer.rs"] mod tokenizer;
use tokenizer::Tokenizer;
#[path = "parser/parser.rs"] mod parser;
use parser::Parser;
//#[path = "parser/expressions/expression.rs"] mod expression; // <- correct import

fn main() {
    let mut tokenizer = Tokenizer::init("1.1".to_string());
    //let mut parser = Parser::init(tokenizer); // TODO: passing tokenizer into parser is a bug
    tokenizer.tokenize();
}
