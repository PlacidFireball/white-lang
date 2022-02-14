use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ParenthesizedExpression {
    expr: dyn Expression,
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

    fn get_type(&self) -> String {
        String::from("ParenthesizedExpression")
    }
}
