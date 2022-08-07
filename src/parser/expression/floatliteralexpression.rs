use crate::config::WhiteLangFloat;
use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

use super::integerliteralexpression::IntegerLiteralExpression;
use super::stringliteralexpression::StringLiteralExpression;

#[derive(Clone, Debug)]
pub struct FloatLiteralExpression {
    value: WhiteLangFloat,
}

impl ToAny for FloatLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FloatLiteralExpression {
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
        Type::Float
    }

    fn get_expr_type(&self) -> String {
        String::from("FloatLiteralExpression")
    }
}
#[allow(dead_code)]
impl FloatLiteralExpression {
    pub(crate) fn new(value: WhiteLangFloat) -> Self {
        FloatLiteralExpression { value }
    }

    pub(crate) fn to_integer_literal(&self) -> IntegerLiteralExpression {
        IntegerLiteralExpression::new(self.value.round() as isize)
    }

    pub(crate) fn to_string_literal(&self) -> StringLiteralExpression {
        StringLiteralExpression::new(self.value.to_string())
    }
}
