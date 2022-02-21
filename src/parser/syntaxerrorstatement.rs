use crate::parser_traits::{Expression, Statement, ToAny};
use crate::symbol_table::SymbolTable;
use std::any::Any;

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

    fn validate(&mut self, st: &mut SymbolTable) -> String {
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorStatement {
    pub fn new() -> SyntaxErrorStatement {
        SyntaxErrorStatement {}
    }
}
