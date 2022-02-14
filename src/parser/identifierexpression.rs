use crate::parser::Expression;
use std::any::Any;

pub(crate) struct IdentifierExpression {
    name: String,
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

    fn debug(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> String {
        todo!()
    }
}
