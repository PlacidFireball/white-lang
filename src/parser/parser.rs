
#[path = "../tokenizer/tokenizer.rs"] mod tokenizer;
use tokenizer::Tokenizer;
use tokenizer::Token;
use tokenizer::TokenType;

enum ParserErrorType {
    UnexpectedToken,

}

pub struct Parser {
    tokenizer: Tokenizer,
    curr_token_idx: usize,
    errors: Vec<ParserErrorType>
}
#[allow(dead_code)]
impl Parser {
    pub fn init(tokenizer: Tokenizer) -> Parser {
        Parser {
            tokenizer: tokenizer.clone(),
            curr_token_idx: 0,
            errors: vec![]
        }
    }

    pub fn parse(&self) {
        let token_list: Vec<Token> = self.tokenizer.get_token_list().to_vec();
        while self.tokenizer.has_tokens() {
            break;
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn match_token(typ: TokenType) -> bool {
        
        false
    }
}
