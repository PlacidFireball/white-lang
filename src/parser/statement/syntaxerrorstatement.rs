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
    fn execute(&self, runtime: &mut Runtime) {
        panic!("Evaluated a syntax error")
    }

    fn compile(&self) {
        panic!("Compiled a syntax error")
    }

    fn transpile(&self) -> String {
        panic!("Transpiled a syntax error")
    }

    fn validate(&mut self, _st: &mut SymbolTable) {
        panic!("Validated a syntax error")
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        panic!("Syntax errors don't have expressions")
    }

    fn get_statement_type(&self) -> String {
        String::from("SyntaxErrorStatement")
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
