use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;
use crate::javascript::JavaScript;

#[derive(Clone, Debug)]
pub struct StringLiteralExpression {
    string_value: String,
}

impl ToAny for StringLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for StringLiteralExpression {
    fn evaluate(&self, _runtime: &mut Runtime) -> Box<dyn Any> {
        Box::new(self.string_value.clone())
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript)  {
        javascript.append(format!("\"{}\"", self.string_value));
    }

    fn validate(&mut self, _st: &SymbolTable) {}

    fn debug(&self) -> String {
        self.string_value.clone()
    }

    fn get_white_type(&self) -> Type {
        Type::String
    }

    fn get_expr_type(&self) -> String {
        String::from("StringLiteralExpression")
    }
}
impl StringLiteralExpression {
    pub fn new(string_value: String) -> StringLiteralExpression {
        StringLiteralExpression { string_value }
    }
}
