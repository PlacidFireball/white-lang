use std::any::Any;
use crate::parser::Expression;

pub(crate) struct IdentifierExpression {
    name: String
}
impl Expression for IdentifierExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.name.clone())
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