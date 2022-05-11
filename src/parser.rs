pub(crate) mod expression;
pub(crate) mod parser_traits;
pub(crate) mod statement;
use crate::parser::whitetypes::*;
use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use std::any::Any;

mod symbol_table;
mod test;
pub(crate) mod whitetypes;

use expression::additiveexpression::AdditiveExpression;
use expression::booleanliteralexpression::BooleanLiteralExpression;
use expression::comparisonexpression::ComparisonExpression;
use expression::equalityexpression::EqualityExpression;
use expression::factorexpression::FactorExpression;
use expression::floatliteralexpression::FloatLiteralExpression;
use expression::functioncallexpression::FunctionCallExpression;
use expression::identifierexpression::IdentifierExpression;
use expression::integerliteralexpression::IntegerLiteralExpression;
use expression::listliteralexpression::ListLiteralExpression;
use expression::logicalexpression::LogicalExpression;
use expression::nullliteralexpression::NullLiteralExpression;
use expression::parenthesizedexpression::ParenthesizedExpression;
use expression::stringliteralexpression::StringLiteralExpression;
use expression::syntaxerrorexpression::SyntaxErrorExpression;
use expression::unaryexpression::UnaryExpression;
use statement::forstatement::ForStatement;
use statement::whilestatement::WhileStatement;

use statement::functiondefinitionstatement::FunctionDefinitionStatement;
use statement::returnstatement::ReturnStatement;

use statement::assignmentstatement::AssignmentStatement;
use statement::functioncallstatement::FunctionCallStatement;
use statement::ifstatement::IfStatement;
use statement::printstatement::PrintStatement;

use crate::config::WhiteLangFloat;
use crate::parser::parser_traits::{Expression, Statement};
use statement::variablestatement::VariableStatement;
use symbol_table::SymbolTable;
use crate::parser::statement::breakstatement::BreakStatement;

// Parsing Errors
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub(crate) enum ParserErrorType {
    UnexpectedToken,       // we've encountered some unexpected token
    UnterminatedArgList,   // function has unterminated argument list
    UnterminatedList,      // list literal is unterminated
    BadOperator,           // calling operator on types that don't make sense
    MismatchedTypes,       // attempting to pass bad types into various facets of whitelang
    SymbolDefinitionError, //
    DuplicateName,         // attempting to redefine a symbol already in the symbol table
    BadReturnType,         // function returns a type that it's not supposed to
    BadVariableType,       // variable has bad type
    UnknownName,           // trying to assign to a variable that whitelang doesn't know about
    ArgMismatch,           //
    IncompatibleTypes,     //
}

// The White-lang parser
#[allow(dead_code)]
pub struct Parser {
    token_list: Vec<Token>,                  // gets the token list
    statement_list: Vec<Box<dyn Statement>>, // generates a list of statements
    st: SymbolTable,                         // has a symbol table
    expr: Box<dyn Expression>,               // generates an expression
    curr_idx: usize,                         // what token it's on
    curr_fn_def: String,
    errors: Vec<ParserErrorType>, // and possible errors
}
#[allow(dead_code)]
impl Parser {
    pub fn new(tokenizer: &mut Tokenizer) -> Parser {
        // the constructor
        if tokenizer.get_token_list().to_vec().is_empty() {
            tokenizer.tokenize();
        }
        Parser {
            token_list: tokenizer.get_token_list().to_vec(),
            statement_list: vec![],
            st: SymbolTable::new(),
            expr: Box::new(SyntaxErrorExpression::new()),
            curr_idx: 0,
            curr_fn_def: String::new(),
            errors: vec![],
        }
    }

    // main loop
    pub fn parse(&mut self) {
        let expr = self.parse_expression(); // try to parse an expression
                                            // check if the parser got a good expression, and if all tokens are consumed
        if expr
            .to_any()
            .downcast_ref::<SyntaxErrorExpression>()
            .is_some()
            || self.has_tokens()
        {
            // if we've got more stuff to do, parse statements
            self.curr_idx = 0;
            while self.has_tokens() {
                let stmt = self.parse_statement();
                self.statement_list.push(stmt);
                self.check_for_parse_errors();
            }
        } else {
            self.expr = expr;
        }
    }

    fn check_for_parse_errors(&self) {
        if !self.errors.is_empty() {
            panic!("Parse error occurred at token `{}`, with error type: {:?}", self.get_curr_tok().get_string_value(), self.errors[0]);
        }
        for statement in &self.statement_list {
            if let Some(_) = statement.to_any().downcast_ref::<SyntaxErrorExpression>() {
                panic!("Parse error occurred at token `{}`", self.get_curr_tok().get_string_value());
            }
        }
    }

    /// Retrieve the expression if the parser has it
    pub fn get_expr(&self) -> Option<&Box<dyn Expression>> {
        if let Some(_) = self.expr.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return Option::None;
        }
        Option::Some(&self.expr)
    }

    /// Get the statement list if the parser has it
    pub fn get_statements(&self) -> Option<&Vec<Box<dyn Statement>>> {
        if let Some(_) = self.expr.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return Option::Some(&self.statement_list);
        }
        Option::None
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
        //println!("-->{}<--", self.get_curr_tok().get_string_value());
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
        if !self.match_token(typ) {
            self.errors.push(UnexpectedToken);
        }
        self.consume_token();
    }

    fn match_str_val(&mut self, strval: String) -> bool {
        if self.get_curr_tok().get_string_value() == strval {
            return true;
        }
        false
    }

    fn require_a_type(&mut self) -> Type {
        let types = vec!["string", "bool", "float", "int", "void"]; // all the types we can assign to so far
        let curr_tok = self.get_curr_tok().get_string_value();
        for i in 0..types.len() - 1 {
            // try to match some type, if we get a good one, return it
            if types[i] == curr_tok {
                self.consume_token();
                return Type::new(types[i]);
            }
        }
        let opt_typ = self.try_parse_list_type(); // try and parse a list<type>
        if opt_typ.is_some() {
            return opt_typ.unwrap();
        }
        self.errors.push(ParserErrorType::BadVariableType); // otherwise we've got some errors
        Type::Error
    }

    fn try_parse_list_type(&mut self) -> Option<Type> {
        if self.match_str_val(String::from("list")) {
            // match list
            self.consume_token();
            self.match_and_consume(Less); // <
            let typ = self.require_a_type().get_list_type(); // make sure we are parsing some type
            self.match_and_consume(Greater); // >
            if typ != Type::Error {
                return Option::Some(typ);
            }
        }
        Option::None
    }

    // -------------------------------------------------------------------------- //
    /* Statement Parsing - all the statements that White-Lang accepts for now     */
    // -------------------------------------------------------------------------- //
    fn parse_statement(&mut self) -> Box<dyn Statement> {
        // pretty readable code, I assume you can read it :-)
        let var_stmt = self.parse_variable_statement();
        if var_stmt.is_some() {
            return Box::new(var_stmt.unwrap());
        }
        let fds = self.parse_function_definition_statement();
        if fds.is_some() {
            return Box::new(fds.unwrap());
        }
        let fcs = self.parse_function_call_statement();
        if fcs.is_some() {
            return Box::new(fcs.unwrap());
        }
        let ret = self.parse_return_statement();
        if ret.is_some() {
            return Box::new(ret.unwrap());
        }
        let for_stmt = self.parse_for_statement();
        if for_stmt.is_some() {
            return Box::new(for_stmt.unwrap());
        }
        let assign_stmt = self.parse_assignment_statement();
        if assign_stmt.is_some() {
            return Box::new(assign_stmt.unwrap());
        }
        let print_stmt = self.parse_print_statement();
        if print_stmt.is_some() {
            return Box::new(print_stmt.unwrap());
        }
        let if_stmt = self.parse_if_statement();
        if if_stmt.is_some() {
            return Box::new(if_stmt.unwrap());
        }
        let while_stmt = self.parse_while_statement();
        if while_stmt.is_some() {
            return Box::new(while_stmt.unwrap());
        }
        let break_stmt = self.parse_break_statement();
        if break_stmt.is_some() {
            return Box::new(break_stmt.unwrap());
        }
        panic!("Parse error occurred at token {}", self.get_curr_tok().get_string_value());
    }

    fn parse_function_definition_statement(&mut self) -> Option<FunctionDefinitionStatement> {
        // fn _name_(arg1 : type1, ... argn typen) [: return] { statements }
        if self.match_and_consume(Function) {
            let name = self.get_curr_tok().get_string_value();
            let mut fds = FunctionDefinitionStatement::new(name.clone());
            self.st.register_function(name.clone(), fds.clone());
            self.consume_token();
            self.require_token(LeftParen);
            while !self.match_and_consume(RightParen) {
                let expr = self.parse_expression();
                fds.add_arg(expr);
                self.require_token(Colon);
                let typ = self.require_a_type();
                fds.add_arg_type(typ);
                if !self.has_tokens() {
                    self.errors.push(ParserErrorType::UnterminatedArgList);
                    break;
                }
            }
            if self.match_and_consume(Colon) {
                fds.set_return_type(self.require_a_type());
            }
            self.require_token(LeftBrace);
            self.curr_fn_def = name.clone();
            while !self.match_and_consume(RightBrace) {
                let stmt = self.parse_statement();
                fds.add_statement(stmt);
            }
            self.curr_fn_def = String::new();
            self.st.register_function(name, fds.clone());
            return Option::Some(fds);
        }
        Option::None
    }

    fn parse_variable_statement(&mut self) -> Option<VariableStatement> {
        // let _id_ {: type_literal} = expr;
        if self.match_and_consume(Let) {
            let name = self.get_curr_tok().get_string_value();
            self.require_token(Identifier);
            let mut var_stat = VariableStatement::new(name);
            if self.match_and_consume(Colon) {
                var_stat.set_type(self.require_a_type());
            }
            self.require_token(Equal);
            var_stat.set_expr(self.parse_expression());
            var_stat.set_type(var_stat.get_expr().get_white_type());
            self.require_token(SemiColon);
            return Option::Some(var_stat);
        }
        Option::None
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        // return expr
        if self.match_token(Return) {
            self.consume_token();
            let rs = ReturnStatement::new(self.parse_expression(), self.curr_fn_def.clone());
            self.require_token(SemiColon);
            return Option::Some(rs);
        }
        Option::None
    }

    fn parse_for_statement(&mut self) -> Option<ForStatement> {
        // for (x in [1, 2, 3]) { statements }
        if self.match_and_consume(For) {
            let mut fs = ForStatement::new();
            self.require_token(LeftParen);
            fs.set_iter_var(self.parse_identifier_expression());
            self.require_token(In);
            fs.set_iter(self.parse_list_literal_expression()); // TODO: more iterators
            self.require_token(RightParen);
            self.require_token(LeftBrace);
            while !self.match_and_consume(RightBrace) {
                fs.add_statement(self.parse_statement());
            }
            return Option::Some(fs);
        }
        Option::None
    }

    fn parse_assignment_statement(&mut self) -> Option<AssignmentStatement> {
        // x = expr;
        if self.match_token(Identifier) && self.peek_next_token(Equal) {
            let mut assignmentstatement = AssignmentStatement::new();
            assignmentstatement.set_variable(self.parse_expression());
            self.require_token(Equal);
            assignmentstatement.set_expr(self.parse_expression());
            self.require_token(SemiColon);
            return Option::Some(assignmentstatement);
        }
        Option::None
    }

    fn parse_print_statement(&mut self) -> Option<PrintStatement> {
        // print(expr);
        if self.match_token(Print) {
            self.require_token(Print);
            self.require_token(LeftParen);
            let expr = self.parse_expression();
            let print_stmt = PrintStatement::new(expr);
            self.require_token(RightParen);
            self.require_token(SemiColon);
            return Option::Some(print_stmt);
        }
        Option::None
    }

    fn parse_if_statement(&mut self) -> Option<IfStatement> {
        // if ( <expr> ) { stmts } [else { stmts }]
        if self.match_token(If) {
            let mut if_stmt = IfStatement::new();
            self.require_token(If);
            self.require_token(LeftParen);
            let expr = self.parse_expression();
            if_stmt.set_expr(expr);
            self.require_token(RightParen);
            self.require_token(LeftBrace);
            while !self.match_and_consume(RightBrace) && self.has_tokens() {
                if_stmt.add_true_statement(self.parse_statement());
                if !self.has_tokens() {
                    self.errors.push(ParserErrorType::UnexpectedToken);
                    break;
                }
            }
            if self.match_and_consume(Else) {
                self.require_token(LeftBrace);
                while !self.match_and_consume(RightBrace) && self.has_tokens() {
                    if_stmt.add_false_statement(self.parse_statement());
                    if !self.has_tokens() {
                        self.errors.push(ParserErrorType::UnexpectedToken);
                        break;
                    }
                }
            }
            return Option::Some(if_stmt);
        }
        Option::None
    }

    fn parse_function_call_statement(&mut self) -> Option<FunctionCallStatement> {
        // x(args);
        if self.match_token(Identifier) && self.peek_next_token(LeftParen) {
            let name = self.token_list[self.curr_idx.clone()].get_string_value();
            let expr = self.parse_expression(); // retrieve the function call expression
            self.require_token(TokenType::SemiColon);
            let fcs = FunctionCallStatement::new(expr, name.clone());
            return Option::Some(fcs);
        }
        Option::None
    }

    fn parse_while_statement(&mut self) -> Option<WhileStatement> {
        if self.match_token(TokenType::While) {
            self.consume_token();
            self.require_token(LeftParen);
            let expr = self.parse_expression(); // condition we will loop on
            self.require_token(RightParen);
            self.require_token(LeftBrace);
            let mut while_statement = WhileStatement::new();
            while_statement.set_expr(expr);
            while !self.match_and_consume(RightBrace) && self.has_tokens() {
                while_statement.add_body_statement(self.parse_statement());
                if !self.has_tokens() {
                    self.errors.push(ParserErrorType::UnexpectedToken);
                    break;
                }
            }
            return Option::Some(while_statement);
        }

        None
    }

    fn parse_break_statement(&mut self) -> Option<BreakStatement> {
        if self.match_and_consume(TokenType::Break) {
            self.require_token(TokenType::SemiColon);
            return Option::Some(BreakStatement::new());
        }
        None
    }

    // -------------------------------------------------------------------------- //
    /* Expression Parsing - all lexemes that can be evaluated to a specific value */
    // -------------------------------------------------------------------------- //

    /*
    This is a pretty cool algorithm. I was taught this in my compilers class at
    Montana State University. It is called recursive descent.
    https://en.wikipedia.org/wiki/Recursive_descent_parser#:~:text=In%20computer%20science%2C%20a%20recursive,the%20nonterminals%20of%20the%20grammar.
     */

    fn parse_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_additive_expression();
        expr.validate(&self.st);
        expr
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
        // similar to additive, but deeper in the grammar so we evaluate factor expressions
        // before we evaluate additive expressions
        let mut expr = self.parse_logical_expression();
        while self.match_token(Star) || self.match_token(Slash) {
            let operator = self.get_curr_tok().get_string_value();
            self.consume_token();
            let rhs = self.parse_logical_expression();
            let factor_expr = FactorExpression::new(expr, operator.clone(), rhs);
            expr = Box::new(factor_expr);
        }
        expr
    }

    fn parse_logical_expression(&mut self) -> Box<dyn Expression> {
        let mut expr = self.parse_comparison_expression();
        while self.match_token(Land) || self.match_token(Lor) {
            let operator = self.get_curr_tok().get_string_value();
            self.consume_token();
            let rhs = self.parse_comparison_expression();
            let mut logical_expr = LogicalExpression::new(expr, rhs);
            logical_expr.set_operator(operator);
            expr = Box::new(logical_expr);
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

    // (expr)
    fn parse_parenthesized_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(LeftParen) {
            self.require_token(LeftParen);
            let expr = self.parse_expression();
            let pe = ParenthesizedExpression::new(expr);
            self.require_token(RightParen);
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
        return if self.match_token(Float) {
            // parse float
            let expr = FloatLiteralExpression::new(
                self.token_list[self.curr_idx]
                    .get_string_value()
                    .parse::<WhiteLangFloat>()
                    .unwrap(),
            );
            self.consume_token();
            Box::new(expr)
        } else {
            self.parse_string_literal_expression()
        };
    }

    fn parse_string_literal_expression(&mut self) -> Box<dyn Expression> {
        return if self.match_token(Str) {
            // parse string
            let expr = StringLiteralExpression::new(self.get_curr_tok().get_string_value());
            self.consume_token();
            Box::new(expr)
        } else {
            self.parse_integer_literal_expression()
        };
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
