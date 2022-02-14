use std::any::Any;
use crate::parser::Expression;


pub(crate) struct AdditiveExpression {

}
impl Expression for AdditiveExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        String::from("")
    }

    fn transpile(&self) -> String {
        String::from("")
    }

    fn get_type(&self) -> String {
        todo!()
    }

}