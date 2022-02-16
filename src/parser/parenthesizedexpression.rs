use crate::parser::Expression;
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct ParenthesizedExpression {
    expr: Box<dyn Expression>,
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

    fn validate(&mut self) {
        todo!()
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

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl ParenthesizedExpression {
    pub fn new(expr: Box<dyn Expression>) -> ParenthesizedExpression {
        ParenthesizedExpression {
            expr
        }
    }
}
