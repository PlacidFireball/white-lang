use crate::parser::whitetypes::Type;
use crate::symbol_table::SymbolTable;
use std::any::Any;
use crate::parser_traits::{Expression, ToAny};

pub(crate) struct IntegerLiteralExpression {
    value: isize,
}

impl ToAny for IntegerLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
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

    fn validate(&mut self, st: &SymbolTable) {}

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
}
impl IntegerLiteralExpression {
    pub fn new(value: isize) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }
}
