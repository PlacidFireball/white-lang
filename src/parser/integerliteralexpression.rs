use crate::parser::Expression;
use std::any::Any;

pub(crate) struct IntegerLiteralExpression {
    value: isize,
}
impl Expression for IntegerLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn debug(&self) -> String {
        String::from(self.value.to_string())
    }

    fn get_type(&self) -> String {
        String::from("IntegerLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &Box::new(self)
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &Box::new(Self)
    }
}
impl IntegerLiteralExpression {
    pub fn new(value: isize) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }
}
