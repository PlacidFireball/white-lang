use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::symbol_table::SymbolTable;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct SyntaxErrorStatement {}

impl ToAny for SyntaxErrorStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for SyntaxErrorStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) { }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        todo!()
    }

    fn has_errors(&self) -> bool {
        true
    }
}
impl SyntaxErrorStatement {
    pub fn new() -> SyntaxErrorStatement {
        SyntaxErrorStatement {}
    }
}
