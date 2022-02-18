use crate::parser::whitetypes::Type;
use crate::parser::{Expression, SymbolTable, ToAny};
use std::any::Any;

pub(crate) struct NullLiteralExpression {}

impl ToAny for NullLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

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

    fn validate(&mut self, st: &SymbolTable) {}

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
}
impl NullLiteralExpression {
    pub fn new() -> NullLiteralExpression {
        NullLiteralExpression {}
    }
}
