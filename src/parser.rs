use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use crate::parser::whitetypes::*;
use std::any::type_name;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::Map;
use std::ptr::null;
// expressions
mod whitetypes;
use crate::parser::whitetypes::*;
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
mod syntaxerrorstatement;
mod variablestatement;

/* Generic Expression type, implemented by all Expressions in white */
pub trait Expression {
    fn evaluate(&self) -> Box<dyn Any>; // evaluate the expression
    fn compile(&self) -> String;        // compile the expression to nasm
    fn transpile(&self) -> String;      // transpile the expression to javascript
    fn validate(&mut self);             // validate the expression
    fn debug(&self) -> String;          // for retrieving information about the expression
    fn get_white_type(&self) -> Type;   // getting the type of the expression
    fn has_errors(&self) -> bool;       // check if the expression has errors
    // fn get_errors(&self) -> Vec<ParserErrorType>; // potentially implement this in the future
    fn get_expr_type(&self) -> String;  // get the rust type of the expression
    fn get_lhs(&self) -> &Box<dyn Expression>;  // get the left hand expresssion
    fn get_rhs(&self) -> &Box<dyn Expression>;  // get the right hand expression
}

pub trait Statement {
    fn execute() -> String;
    fn get_expr() -> &Box<dyn Expression>;
}

//fn instance_of<T>(_: T, ) -> bool {
//    false
//}

pub struct SymbolTable {
    symbol_stack: Vec<HashMap<String, Type>>
}
impl SymbolTable {
    pub fn has_symbol(&self, name: String) -> bool {
        self.get_symbol(name) != Option::None
    }
    pub fn get_symbol(&self, name: String) -> Option<Type> {
        for next in self.symbol_stack {
            match next.get(&name) {
                Some(S) => return Option::Some(S.clone()),
                None => { continue; }
            }
        }
        Option::None
    }
    pub fn get_symbol_type(&self, name: String) -> Option<Type> {
        let thing = match self.get_symbol(name) {
            Some(T) => Option::Some(T),
            _ => {Option::None}
        };
        thing
    }
}

enum ParserErrorType {
    UnexpectedToken,
    UnterminatedArgList,
    UnterminatedList,
    BadOperator,
    MismatchedTypes
}

// The White-lang parser
pub struct Parser {
    token_list: Vec<Token>, // gets the token list
    statement_list: Vec<Box<dyn Statement>>,
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
            statement_list: vec![],
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
        !(self.get_curr_tok().get_type() == Eof)
    }

    // tells us if we have errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_curr_tok(&self) -> &Token {
        &self.token_list[self.curr_idx]
    }

    // consumes the token unconditionally
    fn consume_token(&mut self) {
        self.curr_idx += 1;
    }

    // peeks at the next token and sees if it matches typ
    fn peek_next_token(&self, typ: TokenType) -> bool {
        self.token_list[self.curr_idx + 1].get_type() == typ
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
    // -------------------------------------------------------------------------- //
    /* Expression Parsing - all lexemes that can be evaluated to a specific value */
    // -------------------------------------------------------------------------- //
    fn parse_expression(&mut self) -> Box<dyn Expression> {
        self.parse_additive_expression()
    }

    // <expr> + <expr>
    fn parse_additive_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_factor_expression();
        while self.match_token(Plus) || self.match_token(Minus) {
            let operator = self.get_curr_tok().get_string_value(); // get the operator value
            self.consume_token();
            let rhs = self.parse_factor_expression(); // get the right hand side
            let additive_expr = AdditiveExpression::new(expr, operator.clone(), rhs);
            expr = Box::new(additive_expr);
        }
        expr
    }

    // <expr> * <expr>
    fn parse_factor_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_comparison_expression();
        while self.match_token(Star) || self.match_token(Slash) {
            let operator = self.get_curr_tok().get_string_value();
            self.consume_token();
            let rhs = self.parse_function_call_expression();
            let factor_expr = FactorExpression::new(expr, operator.clone(), rhs);
            expr = Box::new(factor_expr);
        }
        expr
    }

    // <expr> (> | >= | < | <=) <expr>
    fn parse_comparison_expression(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_equality_expression(); // try to parse a lower level expression first
        if self.match_token(Greater)                // If we match either >
            || self.match_token(GreaterEqual)       // >=,
            || self.match_token(Less)               // <,
            || self.match_token(LessEqual)          // or <=
        {
            let operator = self.get_curr_tok().get_string_value();  // retrieve op sign
            self.consume_token();       // consume op
            let rhs = self.parse_function_call_expression();        // get the right hand side expression
            let comparison_expr = ComparisonExpression::new(expr, operator.clone(), rhs); // create the expression
            return Box::new(comparison_expr); // return a box wrapper of the expression
        }
        expr // if we didn't parse a comparison expression, return whatever we parsed earlier
    }

    fn parse_parenthesized_expression(&mut self) -> Box<dyn Expression> {
        todo!()
    }

    // <expr> (== | !=) <expr>
    fn parse_equality_expression(&mut  self) -> Box<dyn Expression> {
        let expr = self.parse_function_call_expression();            // first try to parse a lower level expr
        if self.match_token(EqualEqual) || self.match_token(BangEqual) {      // if we match either != or ==
            let operator = self.get_curr_tok().get_string_value();              // get the op
            self.consume_token();       // consume the token
            let rhs = self.parse_expression();          // parse some other expression
            let equality_expr = EqualityExpression::new(expr, operator.clone(), rhs);
            return Box::new(equality_expr);                           // return a box wrapper to the expr
        }
        expr
    }

    fn parse_function_call_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Identifier) && self.peek_next_token(LeftParen) { // function_name(
            let mut expr = FunctionCallExpression::new(self.get_curr_tok().get_string_value());
            self.require_token(Identifier);             // consume the name and paren
            self.require_token(LeftParen);
            loop {
                if self.match_and_consume(RightParen) { // while the arg list hasn't terminated
                    break;
                }
                let arg = self.parse_expression(); // parse some expression
                expr.add_arg(arg);                          // add the argument to the argument vector
                self.match_and_consume(Comma);          // consume a comma if we have one
                if !self.has_tokens() {                     // check to see if we've run out of tokens
                    self.errors.push(ParserErrorType::UnterminatedArgList); // add an error if we have
                    break;
                }
            }
            return Box::new(expr); // return whatever we have parsed
        }
        self.parse_list_literal_expression() // otherwise parse a list literal
    }

    fn parse_list_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_and_consume(LeftBracket) {            // match some [
            let mut lle = ListLiteralExpression::new(); // create a new list literal
            while !self.match_and_consume(RightBracket) {   // while the list hasn't been terminated
                lle.add_expr(self.parse_expression());     // add some new expression to the list
                self.match_and_consume(Comma);              // consume a comma if there is any
                if !self.has_tokens() {                         // check to see if we have an unterminated list
                    self.errors.push(ParserErrorType::UnterminatedList); // if we do add an error
                    break;
                }
            }
            return Box::new(lle);                           // return a box wrapper of the lle
        }
        self.parse_unary_expression()                           // try to parse a unary expression
    }

    fn parse_unary_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Not) || self.match_token(Minus) {         // match either not or -
            let operator = self.get_curr_tok().get_string_value();      // get the op sign
            self.consume_token();                   // consume the token
            let expr = self.parse_integer_literal_expression(); // parse some lower level expression
            let unary_expr = UnaryExpression::new(operator, expr); // create the new expr
            return Box::new(unary_expr);        // return a box wrapper
        }
        self.parse_float_literal_expression()
    }

    fn parse_float_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Float) {    // parse float
            let expr = FloatLiteralExpression::new(
                self.token_list[self.curr_idx]
                    .get_string_value()
                    .parse::<f64>()
                    .unwrap(),
            );
            self.consume_token();
            return Box::new(expr);
        } else {
            return self.parse_string_literal_expression();
        }
    }

    fn parse_string_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Str) {      // parse string
            let expr = StringLiteralExpression::new(self.get_curr_tok().get_string_value());
            self.consume_token();
            return Box::new(expr);
        } else {
            return self.parse_integer_literal_expression();
        }
    }

    fn parse_integer_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Int) {      // parse integers
            let expr = IntegerLiteralExpression::new(
                self.token_list[self.curr_idx]
                    .get_string_value()
                    .parse::<isize>()
                    .unwrap(),
            );
            self.consume_token();
            return Box::new(expr);
        }
        self.parse_boolean_literal_expression()
    }

    fn parse_boolean_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(True) || self.match_token(False) { // parse boolean literals
            let expr = BooleanLiteralExpression::new(
                self.get_curr_tok()
                    .get_string_value()
                    .parse::<bool>()
                    .unwrap(),
            );
            self.consume_token();
            return Box::new(expr);
        }
        self.parse_null_literal_expression()
    }

    fn parse_null_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Null) {     // parse null literals
            let expr = NullLiteralExpression::new();
            self.consume_token();
            return Box::new(expr);
        }
        Box::new(SyntaxErrorExpression::new())
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
        assert_eq!(expr.get_expr_type(), "IntegerLiteralExpression");
    }

    #[test]
    fn test_parse_string_expression() {
        let mut parser = init_parser("\"Hello World\"".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "StringLiteralExpression");
    }

    #[test]
    fn test_parse_float_expression() {
        let mut parser = init_parser("1.1".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "FloatLiteralExpression");
        assert_eq!(expr.debug(), "1.1");
    }

    #[test]
    fn test_null_literal_expression() {
        let mut parser = init_parser("null".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "NullLiteralExpression");
    }

    #[test]
    fn test_boolean_literal_expression() {
        let mut parser = init_parser("true false".to_string());
        let mut expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "BooleanLiteralExpression");
        assert_eq!(expr.debug(), "true");
        expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "BooleanLiteralExpression");
        assert_eq!(expr.debug(), "false");
    }

    #[test]
    fn test_function_call_expression() {
        let mut parser = init_parser("x()".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "FunctionCallExpression");
        assert_eq!(expr.debug(), "x: ");
    }

    #[test]
    fn test_function_call_args_expression() {
        let mut parser = init_parser("x(1, 2)".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "FunctionCallExpression");
        assert_eq!(expr.debug(), "x: 1 2 ");
    }

    #[test]
    fn test_fn_unterminated_args() {
        let mut parser = init_parser("x(".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "FunctionCallExpression");
        assert!(parser.has_errors());
    }

    #[test]
    fn test_parse_comparison_expression() {
        let mut parser = init_parser("2 > 1".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "ComparisonExpression");
        assert_eq!(expr.debug(), "2 > 1");
    }

    #[test]
    fn test_parse_additive_expression() {
        let mut parser = init_parser("1 + 1 1 - 1".to_string());
        let mut expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "AdditiveExpression");
        assert_eq!(expr.debug(), "1 + 1");
        expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "AdditiveExpression");
        assert_eq!(expr.debug(), "1 - 1");
    }

    #[test]
    fn additive_expressions_are_associative() {
        let mut parser = init_parser("1 + 1 - 1".to_string());
        let mut expr = parser.parse_expression();
        let x = expr.debug();
        assert_eq!(expr.get_lhs().get_expr_type(), "AdditiveExpression");
        assert_eq!(expr.get_rhs().get_expr_type(), "IntegerLiteralExpression");
    }

    #[test]
    fn test_parse_factor_expression() {
        let mut parser = init_parser("1 * 1".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "FactorExpression");
        assert_eq!(expr.debug(), "1 * 1");
    }

    #[test]
    fn test_parse_equality_expression() {
        let mut parser = init_parser("1 == 1".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "EqualityExpression");
        assert_eq!(expr.debug(), "1 == 1");
    }

    #[test]
    fn test_parse_list_expression() {
        let mut parser = init_parser("[1, 2, 3, 4]".to_string());
        let expr = parser.parse_expression();
        assert_eq!(expr.get_expr_type(), "ListLiteralExpression");
        assert_eq!(expr.debug(), "[1, 2, 3, 4]");
    }
}
