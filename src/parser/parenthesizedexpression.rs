use crate::parser::whitetypes::Type;
use crate::parser::{Expression, SymbolTable, ToAny};
use std::any::Any;

pub(crate) struct ParenthesizedExpression {
    expr: Box<dyn Expression>,
}

impl ToAny for ParenthesizedExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ParenthesizedExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        self.expr.evaluate()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
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

    fn has_errors(&self) -> bool {
        self.expr.has_errors()
    }

    fn get_expr_type(&self) -> String {
        String::from("ParenthesizedExpression")
    }
}
impl ParenthesizedExpression {
    pub fn new(expr: Box<dyn Expression>) -> ParenthesizedExpression {
        ParenthesizedExpression { expr }
    }
}
