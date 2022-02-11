use std::ptr::null;
use crate::tokenizer::*;
// expressions
mod booleanliteralexpression;
mod integerliteralexpression;
mod floatliteralexpression;
mod additiveexpression;
mod comparisonexpression;
mod equalityexpression;
mod factorexpression;
mod functioncallexpression;
mod identifierexpression;
mod listliteralexpression;
mod nullliteralexpression;
mod parenthesizedexpression;
mod stringliteralexpression;
mod syntaxerrorexpression;
mod unaryexpression;
mod typeliteral;
// statements
mod assignmentstatement;
mod forstatement;
mod functioncallstatement;
mod functiondefinitionstatement;
mod ifstatement;
mod printstatement;
mod returnstatement;
mod variablestatement;
mod syntaxerrorstatement;



pub trait Expression {
    fn evaluate<T>(&self) -> T;
    fn compile(&self) -> String;
    fn transpile(&self) -> String;
}

pub trait Statement {
    fn execute<T>() -> T;
}

enum ParserErrorType {
    UnexpectedToken,
}

// The White-lang parser
pub struct Parser {
    token_list: Vec<Token>,       // gets the token list
    //statement_list: Vec<dyn Statement>,
    //expression: dyn Expression<T>,
    curr_idx: usize,              // what token it's on
    errors: Vec<ParserErrorType>, // and possible errors
}
#[allow(dead_code)]
impl Parser {
    pub fn init(tokenizer: &mut Tokenizer) -> Parser {
        // the constructor
        if tokenizer.get_token_list().to_vec().is_empty() {
            tokenizer.tokenize();
        }
        Parser {
            token_list: tokenizer.get_token_list().to_vec(),
            //statement_list: vec![],
            //expression: null(),
            curr_idx: 0,
            errors: vec![],
        }
    }

    // main loop (eventually)
    pub fn parse(&self) {
        while self.has_tokens() {
            break;
        }
    }

    // 
    fn has_tokens(&self) -> bool {
        !self.token_list.len() <= self.curr_idx
    }

    // tells us if we have errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    // consumes the token unconditionally
    fn consume_token(&mut self) {
        self.curr_idx += 1;
    }

    // will match and a token at token_list[curr_idx] if its type = typ
    fn match_token(&self, typ: TokenType) -> bool {
        self.token_list[self.curr_idx].get_type() == typ
    }
    
    // will match and consume a token at token_list[curr_idx] if type = typ
    fn match_and_consume(&mut self, typ: TokenType) -> bool {
        if !self.has_tokens() {
            return false;
        }
        if self.match_token(typ) {
            self.consume_token();
            return true;
        }
        false
    }

    // requires that a specific tokentype be at curr_idx, 
    // otherwise pushes an error onto errors
    fn require_token(&mut self, typ: TokenType) {
        use self::ParserErrorType::*;
        if !self.match_and_consume(typ) {
            self.errors.push(UnexpectedToken);
        }
    }
}

#[cfg(test)]
mod test {
    
    use super::*;

    fn init_parser(src: String) -> Parser {
        let tokenizer: Tokenizer = Tokenizer::init(src);
        Parser::init(&mut tokenizer.clone())
    }

    #[test] 
    fn test_match_and_consume() {
        let mut parser = init_parser(String::from("1"));
        assert_eq!(parser.curr_idx, 0);
        assert_eq!(parser.match_and_consume(TokenType::Int), true);
        assert_eq!(parser.curr_idx, 1);
    }

    #[test]
    fn test_require() {
        let mut parser = init_parser("\"\"".to_string());
        parser.require_token(TokenType::Int);
        assert_eq!(parser.has_errors(), true);
    }
}