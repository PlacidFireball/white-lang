use crate::config::WhiteLangInt;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;
use std::ptr::null;

#[derive(Clone)]
pub(crate) struct NullLiteralExpression {}

impl ToAny for NullLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for NullLiteralExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        Box::new("null")
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {}

    fn debug(&self) -> String {
        String::from("null")
    }

    fn get_white_type(&self) -> Type {
        Type::Null
    }

    fn has_errors(&self) -> bool {
        todo!()
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
