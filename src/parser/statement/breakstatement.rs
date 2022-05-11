use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
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
        false
    }
}

impl BreakStatement {
    pub fn new() -> Self {
        BreakStatement { }
    }
}
