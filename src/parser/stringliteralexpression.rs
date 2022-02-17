use crate::parser::{Expression, SymbolTable};
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct StringLiteralExpression {
    string_value: String,
}
impl Expression for StringLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        Box::new(self.string_value.clone())
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) { }

    fn debug(&self) -> String {
        self.string_value.clone()
    }

    fn get_white_type(&self) -> Type {
        Type::String
    }

    fn has_errors(&self) -> bool {
        false
    }

    fn get_expr_type(&self) -> String {
        String::from("StringLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl StringLiteralExpression {
    pub fn new(string_value: String) -> StringLiteralExpression {
        StringLiteralExpression { string_value }
    }
}
