use std::ptr::null;
use std::any::{Any, TypeId};
use std::any::type_name;
use crate::tokenizer::*;
use crate::tokenizer::TokenType::*;
// expressions
mod booleanliteralexpression;
use crate::parser::booleanliteralexpression::BooleanLiteralExpression;
mod integerliteralexpression;
use crate::parser::integerliteralexpression::IntegerLiteralExpression;
mod floatliteralexpression;
use crate::parser::floatliteralexpression::FloatLiteralExpression;
mod additiveexpression;
use crate::parser::additiveexpression::AdditiveExpression;
mod comparisonexpression;
use crate::parser::comparisonexpression::ComparisonExpression;
mod equalityexpression;
use crate::parser::equalityexpression::EqualityExpression;
mod factorexpression;
use crate::parser::factorexpression::FactorExpression;
mod functioncallexpression;
use crate::parser::functioncallexpression::FunctionCallExpression;
mod identifierexpression;
use crate::parser::identifierexpression::IdentifierExpression;
mod listliteralexpression;
use crate::parser::listliteralexpression::ListLiteralExpression;
mod nullliteralexpression;
use crate::parser::nullliteralexpression::NullLiteralExpression;
mod parenthesizedexpression;
use crate::parser::parenthesizedexpression::ParenthesizedExpression;
mod stringliteralexpression;
use crate::parser::stringliteralexpression::StringLiteralExpression;
mod syntaxerrorexpression;
use crate::parser::syntaxerrorexpression::SyntaxErrorExpression;
mod unaryexpression;
use crate::parser::unaryexpression::UnaryExpression;
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

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub trait Expression {
    fn evaluate(&self) -> Box<dyn Any>;
    fn compile(&self) -> String;
    fn transpile(&self) -> String;

    fn get_type(&self) -> String;
    fn set_type(&self, s: String);
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

    // tells us if parsing is done or not
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
    // if it matches, it consumes it
    // otherwise pushes an error onto errors
    fn require_token(&mut self, typ: TokenType) {
        use self::ParserErrorType::*;
        if !self.match_and_consume(typ) {
            self.errors.push(UnexpectedToken);
        }
    }

    fn parse_expression(&mut self) -> Box<dyn Expression>{
        self.parse_additive_expression()
    }

    fn parse_additive_expression(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_integer_literal_expression();
        while self.match_token(Plus) || self.match_token(Minus) {
            self.consume_token()
        }
        expr
    }

    fn parse_integer_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Int) {
            let expr = IntegerLiteralExpression::new(
                self.token_list[self.curr_idx].get_string_value().parse::<isize>().unwrap()
            );
            return Box::new(expr);
        }
        Box::new(SyntaxErrorExpression::new())
    }




}

#[cfg(test)]
mod test {
    
    use super::*;
    use crate::parser::type_of;

    fn init_parser(src: String) -> Parser {
        let tokenizer: Tokenizer = Tokenizer::init(src);
        Parser::init(&mut tokenizer.clone())
    }

    #[test] 
    fn test_match_and_consume() {
        let mut parser = init_parser(String::from("1"));
        assert_eq!(parser.curr_idx, 0);
        assert_eq!(parser.match_and_consume(Int), true);
        assert_eq!(parser.curr_idx, 1);
    }

    #[test]
    fn test_require() {
        let mut parser = init_parser("\"\"".to_string());
        parser.require_token(Int);
        assert_eq!(parser.has_errors(), true);
    }

    #[test]
    fn test_parse_integer_expression() {
        let mut parser = init_parser("1".to_string());
        let expr = parser.parse_expression();
        assert_eq!(type_of(expr), "IntegerLiteralExpression");
    }
}