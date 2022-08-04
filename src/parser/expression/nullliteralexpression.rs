use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct NullLiteralExpression {}

impl ToAny for NullLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for NullLiteralExpression {
    fn evaluate(&self, _runtime: &mut Runtime) -> Box<dyn Any> {
        Box::new("null")
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append_no_tabs(String::from("null"));
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

    fn debug(&self) -> String {
        String::from("null")
    }

    fn get_white_type(&self) -> Type {
        Type::Null
    }

    fn get_expr_type(&self) -> String {
        String::from("NullLiteralExpression")
    }
}
impl NullLiteralExpression {
    pub fn new() -> NullLiteralExpression {
        NullLiteralExpression {}
    }
}
