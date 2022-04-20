use crate::parser::expression::identifierexpression::IdentifierExpression;
use crate::parser::expression::syntaxerrorexpression::SyntaxErrorExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;

use std::any::Any;

#[derive(Clone)]
pub(crate) struct BreakStatement {

}

impl ToAny for BreakStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for BreakStatement {
    fn execute(&self, runtime: &mut Runtime) {
        // TODO: make something like a return exception, and refactor return statement as well
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, _st: &mut SymbolTable) { }

    fn get_expr(&self) -> &Box<dyn Expression> {
        panic!("Break statement does not have an expression");
    }

    fn get_statement_type(&self) -> String {
        String::from("BreakStatement")
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl BreakStatement {
    pub fn new() -> Self {
        BreakStatement { }
    }
}
