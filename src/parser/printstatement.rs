use std::any::Any;
use crate::parser_traits::{Expression, Statement, ToAny};
use crate::symbol_table::SymbolTable;

#[derive(Clone)]
pub(crate) struct PrintStatement {
    expr: Box<dyn Expression>
}

impl ToAny for PrintStatement {
    fn to_any(&self) -> &dyn Any { self }
}

impl Statement for PrintStatement {
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
        self.expr.validate(st);
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("PrintStatement")
    }

    fn has_errors(&self) -> bool {
        self.expr.has_errors()
    }
}

impl PrintStatement {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        PrintStatement {
            expr
        }
    }
}