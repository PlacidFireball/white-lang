use crate::parser::parser_traits::{Expression, Statement, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct SyntaxErrorStatement {}

impl ToAny for SyntaxErrorStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for SyntaxErrorStatement {
    fn execute(&self, _runtime: &mut Runtime) {
        todo!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

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
#[allow(dead_code)]
impl SyntaxErrorStatement {
    pub fn new() -> SyntaxErrorStatement {
        SyntaxErrorStatement {}
    }
}
