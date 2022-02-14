use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ComparisonExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
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
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_type(&self) -> String {
        String::from("ComparisonExpression")
    }
}
impl ComparisonExpression {
    pub fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> ComparisonExpression {
        ComparisonExpression { lhs, operator, rhs }
    }
}
