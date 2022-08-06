use crate::javascript::JavaScript;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct SyntaxErrorExpression {}

impl ToAny for SyntaxErrorExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for SyntaxErrorExpression {
    fn evaluate(&self, _runtime: &mut Runtime) -> Box<dyn Any> {
        panic!("Evaluating SyntaxErrorExpression... panic");
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, _: &mut JavaScript) {
        panic!("You cannot transpile a syntax error :)")
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

    fn debug(&self) -> String {
        String::from("Syntax Error")
    }

    fn get_white_type(&self) -> Type {
        Type::Error
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
