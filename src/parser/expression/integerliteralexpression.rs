use crate::config::*;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct IntegerLiteralExpression {
    value: WhiteLangInt,
}

impl ToAny for IntegerLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IntegerLiteralExpression {
    fn evaluate(&self, _runtime: &mut Runtime) -> Box<dyn Any> {
        Box::new(self.value)
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, _st: &SymbolTable) {}

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
    pub fn new(value: WhiteLangInt) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }
}
