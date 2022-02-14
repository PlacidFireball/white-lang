use std::any::Any;
use crate::parser::Expression;

pub struct FactorExpression {

}
impl Expression for FactorExpression {

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