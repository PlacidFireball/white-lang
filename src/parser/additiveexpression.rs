use crate::parser::Expression;
use std::any::Any;

pub(crate) struct AdditiveExpression {}
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

    fn debug(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }
}
