use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct FloatLiteralExpression {
    value: f64,
}

impl ToAny for FloatLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FloatLiteralExpression {
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
        Type::Float
    }

    fn has_errors(&self) -> bool {
        false
    }

    fn get_expr_type(&self) -> String {
        String::from("FloatLiteralExpression")
    }
}
impl FloatLiteralExpression {
    pub(crate) fn new(value: f64) -> FloatLiteralExpression {
        FloatLiteralExpression { value }
    }
}
