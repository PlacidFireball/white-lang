
mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::init("1.1".to_string());
    tokenizer.tokenize();
}
