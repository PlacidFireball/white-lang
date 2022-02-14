use crate::parser::Expression;
use std::any::Any;

pub struct EqualityExpression {}
impl Expression for EqualityExpression {
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
