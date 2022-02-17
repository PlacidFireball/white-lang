use crate::parser::{Expression, SymbolTable};
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct IdentifierExpression {
    name: String,
}
impl Expression for IdentifierExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.name.clone())
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        todo!()
    }

    fn debug(&self) -> String {
        self.name.clone()
    }

    fn get_white_type(&self) -> Type {
        todo!()
    }

    fn has_errors(&self) -> bool {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        String::from("IdentifierExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl IdentifierExpression {
    pub fn new(name: String) -> IdentifierExpression {
        IdentifierExpression { name }
    }
}
