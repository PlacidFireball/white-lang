use crate::config::*;
use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

use super::floatliteralexpression::FloatLiteralExpression;
use super::stringliteralexpression::StringLiteralExpression;

#[derive(Clone, Debug)]
pub struct IntegerLiteralExpression {
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

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append_no_tabs(self.value.to_string());
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

    fn debug(&self) -> String {
        String::from(self.value.to_string())
    }

    fn get_white_type(&self) -> Type {
        Type::Integer
    }

    fn get_expr_type(&self) -> String {
        String::from("IntegerLiteralExpression")
    }
}
#[allow(dead_code)]
impl IntegerLiteralExpression {
    pub fn new(value: WhiteLangInt) -> IntegerLiteralExpression {
        IntegerLiteralExpression { value }
    }

    pub(crate) fn to_float_literal(&self) -> FloatLiteralExpression {
        FloatLiteralExpression::new(self.value as f64)
    }

    pub(crate) fn to_string_literal(&self) -> StringLiteralExpression {
        StringLiteralExpression::new(self.value.to_string())
    }
}
