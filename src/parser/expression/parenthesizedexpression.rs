use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct ParenthesizedExpression {
    expr: Box<dyn Expression>,
}

impl ToAny for ParenthesizedExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ParenthesizedExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        self.expr.evaluate(runtime)
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append_no_tabs(String::from("("));
        self.expr.transpile(javascript);
        javascript.append_no_tabs(String::from(")"));
    }

    fn validate(&mut self, st: &SymbolTable) {
        self.expr.validate(st);
    }

    fn debug(&self) -> String {
        String::from("(") + &*self.expr.debug() + &*String::from(")")
    }

    fn get_white_type(&self) -> Type {
        self.expr.get_white_type()
    }

    fn get_expr_type(&self) -> String {
        String::from("ParenthesizedExpression")
    }
}
impl ParenthesizedExpression {
    pub fn new(expr: Box<dyn Expression>) -> ParenthesizedExpression {
        ParenthesizedExpression { expr }
    }

    #[allow(dead_code)]
    pub fn get_expr(&self) -> &Box<dyn Expression> {
        &self.expr
    }
}
