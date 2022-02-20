use std::any::Any;
use crate::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::syntaxerrorexpression::SyntaxErrorExpression;

pub(crate) trait ToAny: 'static {
    fn to_any(&self) -> &dyn Any;
}

pub(crate) fn default_expr() -> Box<dyn Expression> {
    Box::new(SyntaxErrorExpression::new())
}

#[allow(dead_code)]
pub(crate) trait Expression : ToAny {
    fn evaluate(&self) -> Box<dyn Any>; // evaluate the expression
    fn compile(&self) -> String; // compile the expression to nasm
    fn transpile(&self) -> String; // transpile the expression to javascript
    fn validate(&mut self, st: &SymbolTable); // validate the expression via the symbol table
    fn debug(&self) -> String; // for retrieving information about the expression
    fn get_white_type(&self) -> Type; // getting the type of the expression
    fn has_errors(&self) -> bool; // check if the expression has errors
    fn get_expr_type(&self) -> String; // get the rust type of the expression
}

#[allow(dead_code)]
pub(crate) trait Statement : ToAny {
    fn execute(&self) -> String; // execute the statement
    fn compile(&self) -> String; // compile the statement to nasm
    fn transpile(&self) -> String; // transpile the statement to Javascript
    fn validate(&mut self, st: &SymbolTable) -> String; // validate the statement via the symbol table
    fn get_expr(&self) -> &Box<dyn Expression>; // retrieve the expression if the statement has one
    fn get_statement_type(&self) -> String;
}
