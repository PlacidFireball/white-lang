use crate::parser::Expression;
use std::any::Any;

pub(crate) struct SyntaxErrorExpression {}
impl Expression for SyntaxErrorExpression {
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
        String::from("Syntax Error")
    }

    fn get_type(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression {}
    }
}
