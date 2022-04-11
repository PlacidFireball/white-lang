use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct SyntaxErrorExpression {}

impl ToAny for SyntaxErrorExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for SyntaxErrorExpression {
    fn evaluate(&self, runtime: &Runtime) -> Box<dyn Any> {
        unimplemented!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {}

    fn debug(&self) -> String {
        String::from("Syntax Error")
    }

    fn get_white_type(&self) -> Type {
        Type::Error
    }

    fn has_errors(&self) -> bool {
        todo!()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}
impl SyntaxErrorExpression {
    pub fn new() -> SyntaxErrorExpression {
        SyntaxErrorExpression {}
    }
}
