use std::ptr::null;
use crate::parser::Expression;

pub(crate) struct NullLiteralExpression {

}
impl Expression for NullLiteralExpression {
    fn evaluate(&self) -> std::ptr::null {
        null()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }
}