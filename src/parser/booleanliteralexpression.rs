use std::any::Any;
use crate::parser::Expression;

pub(crate) struct BooleanLiteralExpression {
    boolean: bool
}
impl Expression for BooleanLiteralExpression {

    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.boolean)
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