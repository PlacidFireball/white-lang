use crate::parser::Expression;
use std::any::Any;
use std::ptr::null;

pub(crate) struct NullLiteralExpression {}
impl Expression for NullLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(String::from("null"))
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn debug(&self) -> String {
        String::from("null")
    }

    fn get_type(&self) -> String {
        String::from("NullLiteralExpression")
    }
}
impl NullLiteralExpression {
    pub fn new() -> NullLiteralExpression {
        NullLiteralExpression {}
    }
}
