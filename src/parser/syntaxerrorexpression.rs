use std::any::Any;
use crate::parser::Expression;

pub(crate) struct SyntaxErrorExpression {

}
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

    fn get_type(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression { }
    }
}