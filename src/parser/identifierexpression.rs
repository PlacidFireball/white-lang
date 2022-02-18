use crate::parser::whitetypes::Type;
use crate::parser::{Expression, SymbolTable, ToAny};
use std::any::Any;

pub(crate) struct IdentifierExpression {
    name: String,
}

impl ToAny for IdentifierExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
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
}
impl IdentifierExpression {
    pub fn new(name: String) -> IdentifierExpression {
        IdentifierExpression { name }
    }
}
