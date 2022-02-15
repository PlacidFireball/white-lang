use crate::parser::Expression;
use std::any::Any;

pub(crate) struct StringLiteralExpression {
    string_value: String,
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

    fn validate(&self) {
        todo!()
    }

    fn debug(&self) -> String {
        self.string_value.clone()
    }

    fn get_type(&self) -> String {
        String::from("StringLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl StringLiteralExpression {
    pub fn new(string_value: String) -> StringLiteralExpression {
        StringLiteralExpression { string_value }
    }
}
