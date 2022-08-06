use crate::javascript::JavaScript;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct SyntaxErrorStatement {}

impl ToAny for SyntaxErrorStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for SyntaxErrorStatement {
    fn execute(&mut self, _: &mut Runtime) {
        panic!("Evaluated a syntax error")
    }

    fn compile(&self) {
        panic!("Compiled a syntax error")
    }

    fn transpile(&self, _: &mut JavaScript) {
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
}
#[allow(dead_code)]
impl SyntaxErrorStatement {
    pub fn new() -> SyntaxErrorStatement {
        SyntaxErrorStatement {}
    }
}
