use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ComparisonExpression {}
impl Expression for ComparisonExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }
}
