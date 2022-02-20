use crate::parser::whitetypes::*;
use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use std::any::type_name;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::env::var;
use std::iter::Map;
use std::ops::Deref;
use std::ptr::null;
// expressions
pub(crate) mod whitetypes;
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
pub(crate) mod syntaxerrorexpression;
use crate::parser::syntaxerrorexpression::SyntaxErrorExpression;
mod unaryexpression;
use crate::parser::unaryexpression::UnaryExpression;
mod typeliteral;
// statements
mod assignmentstatement;
mod forstatement;
mod functioncallstatement;
mod functiondefinitionstatement;
use crate::parser::functiondefinitionstatement::FunctionDefinitionStatement;
use crate::parser_traits::{Expression, Statement};
mod ifstatement;
mod printstatement;
mod returnstatement;
mod syntaxerrorstatement;
use crate::parser::syntaxerrorstatement::SyntaxErrorStatement;
mod variablestatement;
use crate::parser::variablestatement::VariableStatement;


// Parsing Errors
enum ParserErrorType {
    UnexpectedToken,
    UnterminatedArgList,
    UnterminatedList,
    BadOperator,
    MismatchedTypes,
    SymbolDefinitionError,
    BadReturnType,
    BadVariableType,
}

// The White-lang parser
#[allow(dead_code)]
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

    // requires that a specific token type be at curr_idx,
    // if it matches, it consumes it
    // otherwise pushes an error onto errors
    fn require_token(&mut self, typ: TokenType) {
        use self::ParserErrorType::*;
        if !self.match_and_consume(typ) {
            self.errors.push(UnexpectedToken);
        }
    }

    fn require_one_of(&mut self, types: &Vec<&str>) -> isize {
        let curr_tok = self.get_curr_tok().get_string_value();
        for i in 0..types.len()-1 {
            if types[i] == curr_tok {
                self.consume_token();
                return i as isize;
            }
        }
        self.errors.push(ParserErrorType::BadVariableType);
        -1
    }

    // -------------------------------------------------------------------------- //
    /* Statement Parsing - all the statements that White-Lang accepts for now     */
    // -------------------------------------------------------------------------- //
    fn parse_statement(&mut self) -> Box<dyn Statement> {
        let var_stmt = self.parse_variable_statement();
        if var_stmt.is_some() {
            return Box::new(var_stmt.unwrap());
        }

        Box::new(SyntaxErrorStatement::new())
    }

    fn parse_function_definition_statement(&mut self) -> Option<FunctionDefinitionStatement> {
        unimplemented!();
    }

    fn parse_variable_statement(&mut self) -> Option<VariableStatement> {
        // let _id_ {: type_literal} = expr;
        if self.match_and_consume(Let) {
            let name = self.get_curr_tok().get_string_value();
            self.require_token(Identifier);
            let mut var_stat = VariableStatement::new(name);
            if self.match_and_consume(Colon) {
                let types = vec!["string", "bool", "float", "int",
                                 "list<string>", "list<bool>", "list<float>", "list<int>"];
                let idx = self.require_one_of(&types);
                if idx != -1 {
                    var_stat.set_type(Type::new(types[idx as usize]));
                }
            }
            self.require_token(Equal);
            var_stat.set_expr(self.parse_expression());
            var_stat.set_type(var_stat.get_expr().get_white_type());
            self.require_token(SemiColon);
            return Option::Some(var_stat);
        }
        Option::None
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
            || self.match_token(LessEqual)
        // or <=
        {
            let operator = self.get_curr_tok().get_string_value(); // retrieve op sign
            self.consume_token(); // consume op
            let rhs = self.parse_function_call_expression(); // get the right hand side expression
            let comparison_expr = ComparisonExpression::new(expr, operator.clone(), rhs); // create the expression
            return Box::new(comparison_expr); // return a box wrapper of the expression
        }
        expr // if we didn't parse a comparison expression, return whatever we parsed earlier
    }

    // <expr> (== | !=) <expr>
    fn parse_equality_expression(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_function_call_expression(); // first try to parse a lower level expr
        if self.match_token(EqualEqual) || self.match_token(BangEqual) {
            // if we match either != or ==
            let operator = self.get_curr_tok().get_string_value(); // get the op
            self.consume_token(); // consume the token
            let rhs = self.parse_expression(); // parse some other expression
            let equality_expr = EqualityExpression::new(expr, operator.clone(), rhs);
            return Box::new(equality_expr); // return a box wrapper to the expr
        }
        expr
    }

    fn parse_function_call_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Identifier) && self.peek_next_token(LeftParen) {
            // function_name(
            let mut expr = FunctionCallExpression::new(self.get_curr_tok().get_string_value());
            self.require_token(Identifier); // consume the name and paren
            self.require_token(LeftParen);
            loop {
                if self.match_and_consume(RightParen) {
                    // while the arg list hasn't terminated
                    break;
                }
                let arg = self.parse_expression(); // parse some expression
                expr.add_arg(arg); // add the argument to the argument vector
                self.match_and_consume(Comma); // consume a comma if we have one
                if !self.has_tokens() {
                    // check to see if we've run out of tokens
                    self.errors.push(ParserErrorType::UnterminatedArgList); // add an error if we have
                    break;
                }
            }
            return Box::new(expr); // return whatever we have parsed
        }
        self.parse_list_literal_expression() // otherwise parse a list literal
    }

    fn parse_list_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_and_consume(LeftBracket) {
            // match some [
            let mut lle = ListLiteralExpression::new(); // create a new list literal
            while !self.match_and_consume(RightBracket) {
                // while the list hasn't been terminated
                lle.add_expr(self.parse_expression()); // add some new expression to the list
                self.match_and_consume(Comma); // consume a comma if there is any
                if !self.has_tokens() {
                    // check to see if we have an unterminated list
                    self.errors.push(ParserErrorType::UnterminatedList); // if we do add an error
                    break;
                }
            }
            return Box::new(lle); // return a box wrapper of the lle
        }
        self.parse_parenthesized_expression()
    }

    fn parse_parenthesized_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(LeftParen) {
            self.consume_token();
            let expr = self.parse_expression();
            let pe = ParenthesizedExpression::new(expr);
            return Box::new(pe);
        }
        self.parse_unary_expression()
    }

    fn parse_unary_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Not) || self.match_token(Minus) {
            // match either not or -
            let operator = self.get_curr_tok().get_string_value(); // get the op sign
            self.consume_token(); // consume the token
            let expr = self.parse_integer_literal_expression(); // parse some lower level expression
            let unary_expr = UnaryExpression::new(operator, expr); // create the new expr
            return Box::new(unary_expr); // return a box wrapper
        }
        self.parse_float_literal_expression()
    }

    fn parse_float_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Float) {
            // parse float
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
        if self.match_token(Str) {
            // parse string
            let expr = StringLiteralExpression::new(self.get_curr_tok().get_string_value());
            self.consume_token();
            return Box::new(expr);
        } else {
            return self.parse_integer_literal_expression();
        }
    }

    fn parse_integer_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Int) {
            // parse integers
            let expr = IntegerLiteralExpression::new(
                self.token_list[self.curr_idx]
                    .get_string_value()
                    .parse::<isize>()
                    .unwrap(),
            );
            self.consume_token();
            return Box::new(expr);
        }
        self.parse_identifier_expression()
    }

    fn parse_identifier_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Identifier) {
            let name = self.get_curr_tok().get_string_value();
            self.consume_token();
            let expr = IdentifierExpression::new(name);
            return Box::new(expr);
        }
        return self.parse_boolean_literal_expression();
    }

    fn parse_boolean_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(True) || self.match_token(False) {
            // parse boolean literals
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
        if self.match_token(Null) {
            // parse null literals
            let expr = NullLiteralExpression::new();
            self.consume_token();
            return Box::new(expr);
        }
        Box::new(SyntaxErrorExpression::new())
    }
}

#[cfg(test)]
mod test {
    use crate::parser_traits::{Expression, ToAny};
    use crate::symbol_table::SymbolTable;
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
        assert!(expr.to_any().downcast_ref::<IntegerLiteralExpression>().is_some());
    }

    #[test]
    fn test_parse_string_expression() {
        let mut parser = init_parser("\"Hello World\"".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<StringLiteralExpression>().is_some());
    }

    #[test]
    fn test_parse_float_expression() {
        let mut parser = init_parser("1.1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FloatLiteralExpression>().is_some());
        assert_eq!(expr.debug(), "1.1");
    }

    #[test]
    fn test_null_literal_expression() {
        let mut parser = init_parser("null".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<NullLiteralExpression>().is_some());
    }

    #[test]
    fn test_boolean_literal_expression() {
        let mut parser = init_parser("true false".to_string());
        let mut expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<BooleanLiteralExpression>().is_some());
        assert_eq!(expr.debug(), "true");
        expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<BooleanLiteralExpression>().is_some());
        assert_eq!(expr.debug(), "false");
    }

    #[test]
    fn test_function_call_expression() {
        let mut parser = init_parser("x()".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FunctionCallExpression>().is_some());
        assert_eq!(expr.debug(), "x: ");
    }

    #[test]
    fn test_function_call_args_expression() {
        let mut parser = init_parser("x(1, 2)".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FunctionCallExpression>().is_some());
        assert_eq!(expr.debug(), "x: 1 2 ");
    }

    #[test]
    fn test_fn_unterminated_args() {
        let mut parser = init_parser("x(".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FunctionCallExpression>().is_some());
        assert!(parser.has_errors()); // TODO: FunctionCallExpression has errors instead of the parser
    }

    #[test]
    fn test_parse_comparison_expression() {
        let mut parser = init_parser("2 > 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<ComparisonExpression>().is_some());
        assert_eq!(expr.debug(), "2 > 1");
    }

    #[test]
    fn test_parse_additive_expression() {
        let mut parser = init_parser("1 + 1 1 - 1".to_string());
        let mut expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 + 1");
        expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(expr.debug(), "1 - 1");
    }

    #[test]
    fn additive_expressions_are_associative() {
        let mut parser = init_parser("1 + 1 - 1".to_string());
        let expr = parser.parse_expression();
        let additive_expression = expr.to_any().downcast_ref::<AdditiveExpression>().unwrap();
        let lhs = additive_expression.get_lhs()
            .to_any()
            .downcast_ref::<AdditiveExpression>();
        let rhs = additive_expression.get_rhs()
            .to_any()
            .downcast_ref::<IntegerLiteralExpression>();
        assert!(lhs.is_some());
        assert!(rhs.is_some());
    }

    #[test]
    fn test_parse_factor_expression() {
        let mut parser = init_parser("1 * 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<FactorExpression>().is_some());
        assert_eq!(expr.debug(), "1 * 1");
    }

    #[test]
    fn test_parse_equality_expression() {
        let mut parser = init_parser("1 == 1".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<EqualityExpression>().is_some());
        assert_eq!(expr.debug(), "1 == 1");
    }

    #[test]
    fn test_parse_list_expression() {
        let mut parser = init_parser("[1, 2, 3, 4]".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<ListLiteralExpression>().is_some());
        assert_eq!(expr.debug(), "[1, 2, 3, 4]");
    }

    #[test]
    fn test_parse_identifier_expression() {
        let mut parser = init_parser("x".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<IdentifierExpression>().is_some());
        assert_eq!(expr.debug(), "x");
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let mut parser = init_parser("(1+1)".to_string());
        let expr = parser.parse_expression();
        assert!(expr.to_any().downcast_ref::<ParenthesizedExpression>().is_some());
        let typed_expr = expr.to_any().downcast_ref::<ParenthesizedExpression>().unwrap();
        let interior = typed_expr.get_expr();
        assert!(interior.to_any().downcast_ref::<AdditiveExpression>().is_some());
        assert_eq!(interior.debug(), "1 + 1");
    }

    #[test]
    fn test_symbol_table() {
        let mut st : SymbolTable = SymbolTable::new();
        st.register_symbol(String::from("x"), Type::Integer);
        assert!(st.has_symbol(String::from("x")));
        assert_eq!(
            st.get_symbol_type(String::from("x")).unwrap(),
            Type::Integer
        );
    }

    #[test]
    fn test_parse_variable_statement() {
        let mut parser = init_parser("let x = 10;".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(!variable_statement.has_errors());
        assert!(variable_statement.get_expr().to_any().downcast_ref::<IntegerLiteralExpression>().is_some());
        assert_eq!(variable_statement.get_type(), Type::Integer);
    }

    #[test]
    fn test_parse_variable_statement_explicit_type() {
        let mut parser = init_parser("let x : string = \"Hello World\";".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(!variable_statement.has_errors());
        assert!(variable_statement.get_expr().to_any().downcast_ref::<StringLiteralExpression>().is_some());
        assert_eq!(variable_statement.get_type(), Type::String);
    }

    #[test]
    fn test_parse_variable_statement_bad_assignment_type() {
        let mut parser = init_parser("let x : string = 10;".to_string());
        let stmt = parser.parse_statement();
        assert!(!parser.has_errors());
        let variable_statement = stmt.to_any().downcast_ref::<VariableStatement>().unwrap();
        assert!(variable_statement.has_errors());
    }

}
