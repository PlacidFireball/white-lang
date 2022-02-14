use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ComparisonExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>
}
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

    fn debug(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }
}
impl ComparisonExpression {
    pub fn new(lhs: Box<dyn Expression>, operator: String, rhs: Box<dyn Expression>) -> ComparisonExpression {
        ComparisonExpression {
            lhs,
            operator,
            rhs
        }
    }
}
