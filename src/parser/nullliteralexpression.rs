use crate::parser::{Expression, SymbolTable};
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct NullLiteralExpression {}
impl Expression for NullLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(String::from("null"))
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) { }

    fn debug(&self) -> String {
        String::from("null")
    }

    fn get_white_type(&self) -> Type {
        Type::Null
    }

    fn has_errors(&self) -> bool {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        String::from("NullLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl NullLiteralExpression {
    pub fn new() -> NullLiteralExpression {
        NullLiteralExpression {}
    }
}
