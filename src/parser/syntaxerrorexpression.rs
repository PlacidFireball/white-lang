use crate::parser::Expression;
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct SyntaxErrorExpression {}
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

    fn validate(&mut self) {
        todo!()
    }

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

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression {}
    }
}
