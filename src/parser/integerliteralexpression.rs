use crate::parser::Expression;
use std::any::Any;
use crate::parser::whitetypes::Type;

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

    fn validate(&mut self) { }

    fn debug(&self) -> String {
        String::from(self.value.to_string())
    }

    fn get_white_type(&self) -> Type {
        Type::Integer
    }

    fn has_errors(&self) -> bool {
        false
    }

    fn get_expr_type(&self) -> String {
        String::from("IntegerLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl IntegerLiteralExpression {
    pub fn new(value: isize) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }
}
