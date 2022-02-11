use std::any::Any;
use crate::parser::Expression;

pub(crate) struct IntegerLiteralExpression {
    value: isize
}
impl Expression for IntegerLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        String::from("IntegerLiteralExpression")
    }

    fn set_type(&self) {
        todo!()
    }
}
impl IntegerLiteralExpression {
    pub fn new(value: isize) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }
}