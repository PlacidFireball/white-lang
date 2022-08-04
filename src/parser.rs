pub(crate) mod expression;
pub(crate) mod parser_traits;
pub(crate) mod statement;
use crate::parser::whitetypes::*;
use crate::tokenizer::TokenType::*;
use crate::tokenizer::*;
use std::any::Any;
use std::fmt::format;

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
use statement::functiondefinitionstatement::FunctionDefinitionStatement;
use statement::returnstatement::ReturnStatement;
use statement::structdefinitionstatement::StructDefinitionStatement;
use statement::whilestatement::WhileStatement;

use statement::assignmentstatement::AssignmentStatement;
use statement::functioncallstatement::FunctionCallStatement;
use statement::ifstatement::IfStatement;
use statement::printstatement::PrintStatement;

use crate::config::WhiteLangFloat;
use crate::parser::parser_traits::{add_parser_error, Expression, Statement};
use crate::parser::statement::breakstatement::BreakStatement;
use crate::parser::statement::syntaxerrorstatement::SyntaxErrorStatement;
use crate::parser::ParserErrorType::{EmptyStructVariable, UnexpectedToken, UnknownName, UnterminatedArgList};
use statement::variablestatement::VariableStatement;
use symbol_table::SymbolTable;

use crate::LOGGER;
use crate::parser::expression::structexpression::StructExpression;

// Parsing Errors
#[derive(Clone, Debug)]
pub enum ParserErrorType {
    UnexpectedToken(Token),        // we've encountered some unexpected token
    UnterminatedArgList(Token),    // function has unterminated argument list
    UnterminatedList(Token),       // list literal is unterminated
    BadOperator(String),           // calling operator on types that don't make sense
    MismatchedTypes(Type, Type),   // attempting to pass bad types into various facets of whitelang
    SymbolDefinitionError,         //
    DuplicateName(String, Type),   // attempting to redefine a symbol already in the symbol table
    BadReturnType,                 // function returns a type that it's not supposed to
    BadVariableType,               // variable has bad type
    UnknownName(String), // trying to assign to a variable that whitelang doesn't know about
    ArgMismatch,         //
    IncompatibleTypes(Type, Type), //
    UnexpectedExpression(Box<dyn Expression>),
    BadType(Type),
    EmptyStructVariable(String)
}
impl ParserErrorType {
    fn to_error_msg(&self) -> String {
        use ParserErrorType::*;
        match self {
            UnexpectedToken(tok) => format!("Unexpected token: {}", tok),
            UnterminatedArgList(tok) => format!("Unterminated argument list: {}", tok),
            UnterminatedList(tok) => format!("Unterminated list: {}", tok),
            BadOperator(op) => format!("Bad operator: {}", op),
            MismatchedTypes(t1, t2) => format!("Mismatched types: {:?} + {:?}", t1, t2),
            SymbolDefinitionError => "Symbol definition error: {}".to_string(),
            DuplicateName(name, typ) => format!(
                "Duplicate name: {}\n| It was earlier defined as: {0} -> {:?}",
                name, typ
            ),
            BadReturnType => "Bad return type: {}".to_string(),
            BadVariableType => "Bad variable name: {}".to_string(),
            UnknownName(name) => format!("Unknown name: {}", name),
            ArgMismatch => "Argument mismatch: {}".to_string(),
            IncompatibleTypes(t1, t2) => format!("Incompatible types: {:?} + {:?}", t1, t2),
            UnexpectedExpression(expr) => format!("Unexpected expression: {:?}", expr),
            BadType(typ) => format!("Bad type: {:?}", typ),
            EmptyStructVariable(name) => format!("You didn't populate {} on this struct", name),
        }
    }
}

/// The White-lang parser
/// Turns tokens from crate::Tokenizer into an AST
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

    pub fn new_uninit() -> Parser {
        Parser {
            token_list: vec![],
            statement_list: vec![],
            st: SymbolTable::new(),
            expr: Box::new(SyntaxErrorExpression::new()),
            curr_idx: 0,
            curr_fn_def: "".to_string(),
            errors: vec![],
        }
    }

    pub fn set_token_list(&mut self, token_list: &Vec<Token>) {
        if self.token_list.is_empty() {
            self.token_list = token_list.clone();
        } else {
            panic!("Do not set token list if the token list is already init!");
        }
    }

    // main loop
    pub fn parse(&mut self) {
        if !self.statement_list.is_empty() || !self.expr.get_white_type().eq(&Type::Error) {
            return;
        }
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
                let mut stmt = self.parse_statement();
                stmt.validate(&mut self.st);
                self.statement_list.push(stmt);
                self.check_for_parse_errors();
            }
        } else {
            self.expr = expr;
        }
    }

    fn check_for_parse_errors(&self) {
        if !self.errors.is_empty() {
            panic!(
                "Parse error occurred at token `{}`, with error type: {:?}",
                self.get_curr_tok().get_string_value(),
                self.errors[0].to_error_msg()
            );
        }
        for statement in &self.statement_list {
            if let Some(_) = statement.to_any().downcast_ref::<SyntaxErrorStatement>() {
                panic!(
                    "Parse error occurred at token `{}`",
                    self.get_curr_tok().get_string_value()
                );
            }
        }
    }

    /// Retrieve the expression if the parser has it
    pub fn get_expr(&self) -> Option<&Box<dyn Expression>> {
        if let Some(_) = self.expr.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return None;
        }
        Some(&self.expr)
    }

    /// Get the statement list if the parser has it
    pub fn get_statements(&self) -> Option<&Vec<Box<dyn Statement>>> {
        if let Some(_) = self.expr.to_any().downcast_ref::<SyntaxErrorExpression>() {
            return Some(&self.statement_list);
        }
        None
    }

    // tells us if parsing is done or not
    fn has_tokens(&self) -> bool {
        //println!("current index: {} has_tokens: {}", self.curr_idx, self.get_curr_tok().get_type().ne(&Eof));
        self.get_curr_tok().get_type().ne(&Eof)
    }

    pub fn error_panic(&self, error: ParserErrorType) {
        println!("[PARSE ERROR][FATAL]: {}", error.to_error_msg());
        panic!(
            "[PARSE ERROR][FATAL]: {:?} occurred during validation",
            error
        );
    }

    // tells us if we have errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_curr_tok(&self) -> Token {
        self.token_list[self.curr_idx].clone()
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

    fn token_list_like(&self, expected: Vec<TokenType>) -> bool {
        let len = expected.len();
        let mut i = 0;
        if self.curr_idx + len > self.token_list.len() - 1 {
            LOGGER.debug(format!("Tried to check token list like {:?} but doing the check will cause an error, returning false.", expected), false);
            return false;

        }
        for typ in expected.iter() {
            if !self.token_list[self.curr_idx + i].get_type().eq(typ) {
                return false;
            }
            i += 1;
        }
        true
    }

    // will match and a token at token_list[curr_idx] if its type = typ
    fn match_token(&self, typ: TokenType) -> bool {
        // println!("{} == {} -> {}",
        //     self.token_list[self.curr_idx].get_type(),
        //     typ,
        //     self.token_list[self.curr_idx].get_type().eq(&typ)
        // );
        self.token_list[self.curr_idx].get_type().eq(&typ)
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
            LOGGER.warn(format!(
                "Got unexpected token during parse: {:?}",
                self.get_curr_tok()
            ));
            self.errors.push(UnexpectedToken(self.get_curr_tok()));
            self.check_for_parse_errors();
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
        let types = vec!["string", "bool", "float", "int", "void"]; // all the primitive types we can assign to so far
        // custom struct types

        let curr_tok = self.get_curr_tok().get_string_value();
        if self.st.has_symbol(curr_tok.clone()) {
            if let Some(obj) = self.st.get_struct(curr_tok.clone()) {
                LOGGER.debug(format!("Found a custom type: {:?}", obj.get_type()), false);
                return obj.get_type();
            }
        }
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
                                             //LOGGER.debug(format!("parsed a type: {:?}", typ));
            if typ != Type::Error {
                return Some(typ);
            }
        }
        None
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
        let struct_def_stmt = self.parse_struct_definition_statement();
        if struct_def_stmt.is_some() {
            return Box::new(struct_def_stmt.unwrap());
        }
        panic!(
            "Parse error occurred at token {}",
            self.get_curr_tok().get_string_value()
        );
    }

    fn parse_function_definition_statement(&mut self) -> Option<FunctionDefinitionStatement> {
        // fn _name_(arg1 : type1, ... argn typen) [: return] { statements }
        if self.match_and_consume(Function) {
            let name = self.get_curr_tok().get_string_value();
            let mut fds = FunctionDefinitionStatement::new(name.clone());
            self.consume_token();
            self.require_token(LeftParen);
            while !self.match_and_consume(RightParen) {
                let expr = self.parse_expression();
                fds.add_arg(expr);
                self.require_token(Colon);
                let typ = self.require_a_type();
                fds.add_arg_type(typ);
                if !self.has_tokens() {
                    self.errors.push(UnterminatedArgList(self.get_curr_tok()));
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
            self.st.register_function(name.clone(), fds.clone());
            LOGGER.debug(
                format!("Parsed a function definition statement: {:?}", fds),
                false,
            );
            return Some(fds);
        }
        None
    }

    fn parse_variable_statement(&mut self) -> Option<VariableStatement> {
        // let _id_ {: type_literal} = expr;
        if self.match_and_consume(Let) {
            let name = self.get_curr_tok().get_string_value();
            self.require_token(Identifier);
            let mut var_stmt = VariableStatement::new(name);
            if self.match_and_consume(Colon) {
                var_stmt.set_type(self.require_a_type());
            }
            self.require_token(Equal);
            var_stmt.set_expr(self.parse_expression());
            var_stmt.set_type(var_stmt.get_expr().get_white_type());
            self.require_token(SemiColon);
            LOGGER.debug(
                format!("Parsed a variable statement: {:?}", var_stmt),
                false,
            );
            return Some(var_stmt);
        }
        None
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        // return expr
        if self.match_token(Return) {
            self.consume_token();
            let rs = ReturnStatement::new(self.parse_expression(), self.curr_fn_def.clone());
            self.require_token(SemiColon);
            LOGGER.debug(format!("Parsed a return statement: {:?}", rs), false);
            return Some(rs);
        }
        None
    }

    fn parse_for_statement(&mut self) -> Option<ForStatement> {
        // for (x in [1, 2, 3]) { statements }
        if self.match_and_consume(For) {
            let mut fs = ForStatement::new();
            self.require_token(LeftParen);
            fs.set_iter_var(self.parse_expression());
            self.require_token(In);
            fs.set_iter(self.parse_expression()); // TODO: more iterators
            self.require_token(RightParen);
            self.require_token(LeftBrace);
            while !self.match_and_consume(RightBrace) {
                fs.add_statement(self.parse_statement());
            }
            LOGGER.debug(format!("Parsed a for statement: {:?}", fs), false);
            return Some(fs);
        }
        None
    }

    fn parse_assignment_statement(&mut self) -> Option<AssignmentStatement> {
        // x = expr;
        if self.match_token(Identifier) && self.peek_next_token(Equal) {
            let mut assign_stmt = AssignmentStatement::new();
            assign_stmt.set_variable(self.parse_expression());
            self.require_token(Equal);
            assign_stmt.set_expr(self.parse_expression());
            self.require_token(SemiColon);
            LOGGER.debug(
                format!("Parsed an assignment statement: {:?}", assign_stmt),
                false,
            );
            return Some(assign_stmt);
        }
        None
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
            LOGGER.debug(format!("Parsed a print statement: {:?}", print_stmt), false);
            return Some(print_stmt);
        }
        None
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
                    self.errors
                        .push(UnexpectedToken(self.get_curr_tok()));
                    break;
                }
            }
            if self.match_and_consume(Else) {
                self.require_token(LeftBrace);
                while !self.match_and_consume(RightBrace) && self.has_tokens() {
                    if_stmt.add_false_statement(self.parse_statement());
                    if !self.has_tokens() {
                        self.errors
                            .push(UnexpectedToken(self.get_curr_tok()));
                        break;
                    }
                }
            }
            LOGGER.debug(format!("Parsed an if statement: {:?}", if_stmt), false);
            return Some(if_stmt);
        }
        None
    }

    fn parse_function_call_statement(&mut self) -> Option<FunctionCallStatement> {
        // x(args);
        if self.match_token(Identifier) && self.peek_next_token(LeftParen) {
            self.do_parse_function_call()
        } else if self.token_list_like(vec![Identifier, Dot, Identifier, LeftParen]) {
            self.do_parse_function_call()
        } else {
            None
        }
    }

    fn do_parse_function_call(&mut self) -> Option<FunctionCallStatement> {
        let name = self.token_list[self.curr_idx.clone()].get_string_value();
        let expr = self.parse_expression(); // retrieve the function call expression
        self.require_token(SemiColon);
        let fcs = FunctionCallStatement::new(expr, name.clone());
        LOGGER.debug(
            format!("Parsed a function call statement: {:?}", fcs),
            false,
        );
        return Some(fcs);
    }

    fn parse_while_statement(&mut self) -> Option<WhileStatement> {
        if self.match_token(While) {
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
                    self.errors
                        .push(UnexpectedToken(self.get_curr_tok()));
                    break;
                }
            }
            LOGGER.debug(
                format!("Parsed a while statement: {:?}", while_statement),
                false,
            );
            return Some(while_statement);
        }

        None
    }

    fn parse_break_statement(&mut self) -> Option<BreakStatement> {
        if self.match_and_consume(Break) {
            self.require_token(SemiColon);
            return Some(BreakStatement::new());
        }
        None
    }

    fn parse_struct_definition_statement(&mut self) -> Option<StructDefinitionStatement> {
        if self.match_and_consume(Struct) {
            let name = self.get_curr_tok().get_string_value();
            let mut sds = StructDefinitionStatement::new(name.clone());
            self.consume_token();
            self.require_token(LeftBrace);
            while !self.match_and_consume(RightBrace) {
                let expr = self.parse_identifier_expression();
                self.require_token(Colon);
                let typ = self.require_a_type();
                sds.add_field(expr.debug(), typ);
                self.match_and_consume(Comma);
                if !self.has_tokens() {
                    LOGGER.warn(format!(
                        "Unexpected token: {:?} while parsing struct definition.",
                        self.get_curr_tok()
                    ));
                    self.errors
                        .push(UnexpectedToken(self.get_curr_tok()));
                }
            }
            if self.match_and_consume(Implement) {
                if self.match_str_val(name.clone()) {
                    self.consume_token();
                    self.require_token(LeftBrace);
                    while !self.match_and_consume(RightBrace) {
                        let method_opt = self.parse_function_definition_statement();
                        if method_opt.is_some() {
                            let method = method_opt.unwrap();
                            sds.add_method(method.name.clone(), method.clone());
                        } else {
                            self.errors
                                .push(UnexpectedToken(self.get_curr_tok()));
                            self.check_for_parse_errors();
                        }
                        if !self.has_tokens() {
                            LOGGER.warn(format!(
                                "Unexpected token: {:?} while parsing struct definition.",
                                self.get_curr_tok()
                            ));
                            self.errors
                                .push(UnexpectedToken(self.get_curr_tok()));
                            self.check_for_parse_errors();
                        }
                    }
                } else {
                    self.errors
                        .push(UnexpectedToken(self.get_curr_tok()));
                    self.check_for_parse_errors();
                }
            }
            self.require_token(SemiColon);
            return Some(sds);
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
        let expr = self.parse_additive_expression();
        //if !IS_TESTING.with(|t| t.get()) {    // not sure if commenting this out is correct, we want to ensure that
        //    expr.validate(&self.st);          // all expressions/statements are validated, but having it here causes
        //}                                     // for statement variables to break.
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
            LOGGER.debug(
                format!("Parsed an additive expression: {:?}", additive_expr),
                false,
            );
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
            LOGGER.debug(
                format!("Parsed a factor expression: {:?}", factor_expr),
                false,
            );
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
            LOGGER.debug(
                format!("Parsed a logical expression: {:?}", logical_expr),
                false,
            );
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
            LOGGER.debug(
                format!("Parsed a comparison expression: {:?}", comparison_expr),
                false,
            );
            return Box::new(comparison_expr); // return a box wrapper of the expression
        }
        expr // if we didn't parse a comparison expression, return whatever we parsed earlier
    }

    // <expr> (== | !=) <expr>
    fn parse_equality_expression(&mut self) -> Box<dyn Expression> {
        let expr = self.parse_struct_expression(); // first try to parse a lower level expr
        if self.match_token(EqualEqual) || self.match_token(BangEqual) {
            // if we match either != or ==
            let operator = self.get_curr_tok().get_string_value(); // get the op
            self.consume_token(); // consume the token
            let rhs = self.parse_expression(); // parse some other expression
            let equality_expr = EqualityExpression::new(expr, operator.clone(), rhs);
            LOGGER.debug(
                format!("Parsed an equality expression: {:?}", equality_expr),
                false,
            );
            return Box::new(equality_expr); // return a box wrapper to the expr
        }
        expr
    }

    fn parse_struct_expression(&mut self) -> Box<dyn Expression> {
        let str_val = self.get_curr_tok().get_string_value();
        if self.st.has_symbol(str_val.clone()) {
            if let Some(obj) = self.st.get_struct(str_val.clone()) {
                // now we're in business
                let mut struct_expr = StructExpression::new(str_val.clone(), obj.get_type());
                self.consume_token(); // consume the init token
                self.require_token(LeftParen);
                // fields
                let fields = obj.fields.clone();
                let fields_size = fields.keys().len();
                let mut handled_fields = vec![];
                while self.has_tokens() {
                    let field_name = self.get_curr_tok().get_string_value();
                    if fields.contains_key(&field_name) {
                        self.consume_token();
                        self.require_token(Equal);
                        let expr = self.parse_expression();
                        struct_expr.add_field(field_name.clone(), expr);
                        handled_fields.push(field_name);
                        if !self.match_and_consume(Comma) {
                            if fields_size != handled_fields.len() {
                                for field in handled_fields.iter() {
                                    if !fields.contains_key(field) {
                                        add_parser_error(EmptyStructVariable(field.clone()), "Empty variable".to_string());
                                    }
                                }
                                add_parser_error(UnexpectedToken(self.get_curr_tok()), "idk".to_string());
                            }
                            self.require_token(RightParen);
                            break;
                        }
                    } else if self.match_and_consume(RightParen) {
                        break;
                    } else if !self.has_tokens(){
                        add_parser_error(UnexpectedToken(self.get_curr_tok()), format!("You probably didn't close the paren on your struct :)"));
                    } else {
                        add_parser_error(UnknownName(field_name.clone()), format!("No such field: {}", field_name));
                    }
                }
                return Box::new(struct_expr)
            }
        }
        self.parse_function_call_expression()
    }

    fn parse_function_call_expression(&mut self) -> Box<dyn Expression> {
        if self.token_list_like(vec![Identifier, Dot, Identifier, LeftParen]){
            let namespace = self.get_curr_tok().get_string_value();
            self.consume_token(); // namespace
            self.consume_token(); // dot
            let subname = self.get_curr_tok().get_string_value();
            self.consume_token(); // subname
            let mut expr = FunctionCallExpression::new(format!("{}.{}", namespace, subname));
            self.require_token(LeftParen);
            expr = self.decorate_function_call(expr);
            LOGGER.debug(
                format!("Parsed a function call expression: {:?}", expr),
                false,
            );
            return Box::new(expr); // return whatever we have parsed
        }
        else if self.match_token(Identifier) && self.peek_next_token(LeftParen) {
            // function_name(
            let mut expr = FunctionCallExpression::new(self.get_curr_tok().get_string_value());
            self.require_token(Identifier); // consume the name and paren
            self.require_token(LeftParen);
            expr = self.decorate_function_call(expr);
            LOGGER.debug(
                format!("Parsed a function call expression: {:?}", expr),
                false,
            );
            return Box::new(expr); // return whatever we have parsed
        }
        self.parse_list_literal_expression() // otherwise parse a list literal
    }

    fn decorate_function_call(&mut self, mut expr: FunctionCallExpression) -> FunctionCallExpression {
        loop {
            if self.match_and_consume(RightParen) {
                break;
            }
            let arg = self.parse_expression(); // parse some expression
            expr.add_arg(arg); // add the argument to the argument vector
            // parse commas until the end of the arg list
            if !self.match_token(Comma) {
                self.require_token(RightParen);
                break;
            }
            if !self.has_tokens() {
                self.errors.push(UnterminatedArgList(self.get_curr_tok()));
                break;
            }
        }
        expr
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
                    self.errors
                        .push(ParserErrorType::UnterminatedList(self.get_curr_tok())); // if we do add an error
                    break;
                }
            }
            LOGGER.debug(format!("Parsed a list literal: {:?}", lle), false);
            lle.validate(&mut self.st);
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
            LOGGER.debug(
                format!("Parsed a parenthesized expression: {:?}", pe),
                false,
            );
            return Box::new(pe);
        }
        self.parse_unary_expression()
    }

    fn parse_unary_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Not) || self.match_token(Minus) {
            // match either not or -
            let operator = self.get_curr_tok().get_string_value(); // get the op sign
            self.consume_token(); // consume the token
            let expr = self.parse_float_literal_expression(); // parse some other expression
            let unary_expr = UnaryExpression::new(operator, expr); // create the new expr
            LOGGER.debug(
                format!("Parsed a unary expression: {:?}", unary_expr),
                false,
            );
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
            LOGGER.debug(format!("Parsed a float literal: {:?}", expr), false);
            self.consume_token();
            Box::new(expr)
        } else {
            self.parse_string_literal_expression()
        };
    }

    fn parse_string_literal_expression(&mut self) -> Box<dyn Expression> {
        // println!(
        //     "curr_idx: {}: {}",
        //     self.curr_idx,
        //     self.token_list[self.curr_idx].get_type()
        // );
        // println!("will match: {}", self.match_token(Str));
        return if self.match_token(Str) {
            // parse string
            let expr = StringLiteralExpression::new(self.get_curr_tok().get_string_value());
            self.consume_token();
            LOGGER.debug(format!("Parsed a string literal: {:?}", expr), false);
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
            LOGGER.debug(format!("Parsed an integer literal: {:?}", expr), false);
            return Box::new(expr);
        }
        self.parse_identifier_expression()
    }

    fn parse_identifier_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Identifier) && self.peek_next_token(Dot) {
            let name = self.get_curr_tok().get_string_value();
            self.consume_token(); // name
            self.consume_token(); // dot
            let subname = if self.match_token(Identifier) {
                let tmp = self.get_curr_tok().get_string_value();
                self.consume_token(); // name
                tmp
            } else {
                add_parser_error(UnexpectedToken(self.get_curr_tok()), "".to_string());
                "".to_string() // unreachable
            };
            let expression = IdentifierExpression::new(format!("{}.{}", name, subname));
            return Box::new(expression);
        } else if self.match_token(Identifier) {
            let name = self.get_curr_tok().get_string_value();
            self.consume_token();
            let expr = IdentifierExpression::new(name);
            LOGGER.debug(format!("Parsed an identifier: {:?}", expr), false);
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
            LOGGER.debug(format!("Parsed a boolean literal: {:?}", expr), false);
            return Box::new(expr);
        }
        self.parse_null_literal_expression()
    }

    fn parse_null_literal_expression(&mut self) -> Box<dyn Expression> {
        if self.match_token(Null) {
            // parse null literals
            let expr = NullLiteralExpression::new();
            self.consume_token();
            LOGGER.debug(format!("Parsed a null literal: {:?}", expr), false);
            return Box::new(expr);
        }
        if !self.has_tokens() {
            LOGGER.warn(format!(
                "Couldn't parse an expression. Token: {}",
                self.get_curr_tok()
            ));
        } else {
            LOGGER.debug(format!("Couldn't parse an expression: this is likely because you've got a set of statements, like a normal human being, at the beginning of your file, who'da thunk"), true)
        }
        Box::new(SyntaxErrorExpression::new())
    }
}
