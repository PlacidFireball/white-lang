use crate::javascript::JavaScript;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::program::Program;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct PrintStatement {
    expr: Box<dyn Expression>,
}

impl ToAny for PrintStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for PrintStatement {
    fn execute(&mut self, runtime: &mut Runtime) {
        let eval = self.expr.evaluate(runtime);
        runtime.push_output(String::from(Program::try_print_output(&eval)));
        runtime.push_output(String::from("\n"));
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append(String::from("console.log("));
        self.expr.transpile(javascript);
        javascript.append_no_tabs(String::from(");")).newline();
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
}

impl PrintStatement {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        PrintStatement { expr }
    }
}
