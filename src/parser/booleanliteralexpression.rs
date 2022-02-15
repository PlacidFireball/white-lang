use crate::parser::Expression;
use std::any::Any;

pub(crate) struct BooleanLiteralExpression {
    boolean: bool,
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

    fn validate(&self) { }

    fn debug(&self) -> String {
        String::from(self.boolean.to_string())
    }

    fn get_type(&self) -> String {
        String::from("BooleanLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl BooleanLiteralExpression {
    pub fn new(boolean: bool) -> BooleanLiteralExpression {
        BooleanLiteralExpression { boolean }
    }
}
