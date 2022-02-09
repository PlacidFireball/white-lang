use crate::tokenizer::*;

enum ParserErrorType {
    UnexpectedToken,
}

// The White-lang parser
pub struct Parser {
    tokenizer: Tokenizer,         // gets the tokenizer
    curr_idx: usize,              // what token its on
    errors: Vec<ParserErrorType>, // and possible errors
}
#[allow(dead_code)]
impl Parser {
    pub fn init(tokenizer: Tokenizer) -> Parser {
        // the constructor
        Parser {
            tokenizer: tokenizer,
            curr_idx: 0,
            errors: vec![],
        }
    }

    // main loop (eventually)
    pub fn parse(&self) {
        let token_list: Vec<Token> = self.tokenizer.get_token_list().to_vec();
        while self.tokenizer.has_tokens() {
            break;
        }
    }

    // tells us if we have errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    // will match and consume a token at token_list[curr_idx] if its type = typ
    fn match_token(typ: TokenType) -> bool {
        false
    }
}
