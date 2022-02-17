use crate::parser::{Expression, SymbolTable};
use std::any::Any;
use crate::parser::whitetypes::Type;

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

    fn validate(&mut self, st: &SymbolTable) { }

    fn debug(&self) -> String {
        String::from(self.boolean.to_string())
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
    }

    fn has_errors(&self) -> bool {
        false
    }

    fn get_expr_type(&self) -> String {
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
