use std::any::Any;
use crate::parser::Expression;

pub(crate) struct StringLiteralExpression {
    string_value: String
}
impl Expression for StringLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.string_value.clone())
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