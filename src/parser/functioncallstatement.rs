use std::any::Any;
use crate::parser_traits::{Expression, Statement, ToAny};
use crate::symbol_table::SymbolTable;

pub struct FunctionCallStatement {
    expr: Box<dyn Expression>
}

impl ToAny for FunctionCallStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for FunctionCallStatement {
    fn execute(&self) -> String {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        todo!()
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_statement_type(&self) -> String {
        String::from("FunctionCallStatement")
    }

    fn has_errors(&self) -> bool {
        todo!()
    }
}

impl FunctionCallStatement {
    pub fn new() -> Self {
        todo!()
    }
}
