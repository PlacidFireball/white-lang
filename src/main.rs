
mod tokenizer;
use tokenizer::Tokenizer;
//#[path = /parser/expressions] mod expression; 

fn main() {
    let mut tokenizer = Tokenizer::init("1.1".to_string());
    tokenizer.tokenize();
}
