use crate::parser::Expression;
use std::any::Any;

pub(crate) struct FloatLiteralExpression {
    value: f64,
}
impl Expression for FloatLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&self) { }

    fn debug(&self) -> String {
        String::from(self.value.to_string())
    }

    fn get_type(&self) -> String {
        String::from("FloatLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl FloatLiteralExpression {
    pub(crate) fn new(value: f64) -> FloatLiteralExpression {
        FloatLiteralExpression { value }
    }
}
