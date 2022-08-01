use crate::parser::expression::functioncallexpression::FunctionCallExpression;
use crate::parser::parser_traits::*;
use crate::parser::symbol_table::SymbolTable;
use crate::runtime::Runtime;
use std::any::Any;
use crate::javascript::JavaScript;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FunctionCallStatement {
    name: String,
    expr: Box<dyn Expression>,
    args: Vec<Box<dyn Expression>>,
}

impl ToAny for FunctionCallStatement {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for FunctionCallStatement {
    fn execute(&self, runtime: &mut Runtime) {
        self.expr.evaluate(runtime);
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        self.expr.transpile(javascript);
        javascript.semicolon();
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        self.expr.validate(st);
    }

    fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }

    fn get_statement_type(&self) -> String {
        String::from("FunctionCallStatement")
    }
}

impl FunctionCallStatement {
    pub(crate) fn new(expr: Box<dyn Expression>, name: String) -> Self {
        let fce = expr
            .to_any()
            .downcast_ref::<FunctionCallExpression>()
            .unwrap();
        FunctionCallStatement {
            name,
            expr: expr.clone(),
            args: fce.get_args().clone(),
        }
    }
}
