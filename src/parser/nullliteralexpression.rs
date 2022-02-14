use std::any::Any;
use std::ptr::null;
use crate::parser::Expression;

pub(crate) struct NullLiteralExpression {

}
impl Expression for NullLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any>{
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