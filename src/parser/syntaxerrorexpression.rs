use crate::parser::whitetypes::Type;
use crate::parser::{Expression, SymbolTable, ToAny};
use std::any::Any;

pub(crate) struct SyntaxErrorExpression {}

impl ToAny for SyntaxErrorExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for SyntaxErrorExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {}

    fn debug(&self) -> String {
        String::from("Syntax Error")
    }

    fn get_white_type(&self) -> Type {
        Type::Error
    }

    fn has_errors(&self) -> bool {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression {}
    }
}
