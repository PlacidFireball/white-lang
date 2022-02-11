use std::any::Any;
use crate::parser::Expression;

pub struct EqualityExpression {

}
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

    fn get_type(&self) -> String {
        todo!()
    }

    fn set_type(&self) {
        todo!()
    }
}