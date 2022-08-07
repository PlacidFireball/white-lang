use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

use super::stringliteralexpression::StringLiteralExpression;

#[derive(Clone, Debug)]
pub struct BooleanLiteralExpression {
    boolean: bool,
}
#[allow(unused_variables)]
impl ToAny for BooleanLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for BooleanLiteralExpression {
    fn evaluate(&self, _runtime: &mut Runtime) -> Box<dyn Any> {
        Box::new(self.boolean)
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append_no_tabs(self.boolean.to_string());
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

    fn debug(&self) -> String {
        String::from(self.boolean.to_string())
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
    }

    fn get_expr_type(&self) -> String {
        String::from("BooleanLiteralExpression")
    }
}
impl BooleanLiteralExpression {
    pub fn new(boolean: bool) -> BooleanLiteralExpression {
        BooleanLiteralExpression { boolean }
    }

    pub(crate) fn to_string_literal(&self) -> StringLiteralExpression {
        StringLiteralExpression::new(self.boolean.to_string())
    }
}
