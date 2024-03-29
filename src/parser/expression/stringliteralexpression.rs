use crate::javascript::JavaScript;
use crate::parser::expression::listliteralexpression::ListLiteralExpression;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub struct StringLiteralExpression {
    string_value: String,
}

impl ToAny for StringLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
    fn to_any_mut(&mut self) -> &mut dyn Any {
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

    fn transpile(&self, javascript: &mut JavaScript) {
        javascript.append_no_tabs(format!("\"{}\"", self.string_value));
    }

    fn validate(&mut self, _st: &mut SymbolTable) {}

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

    pub(crate) fn to_list_literal(&self) -> ListLiteralExpression /*list<char>*/ {
        let mut list = ListLiteralExpression::new();
        for c in self.string_value.chars() {
            list.add_expr(Box::new(StringLiteralExpression::new(c.to_string())));
        }
        list.set_type(Type::ListString);
        list
    }

    pub(crate) fn concatenate(&self, other: &StringLiteralExpression) -> StringLiteralExpression {
        let mut this = self.string_value.clone();
        let that = other.debug();
        this.push_str(that.as_str());
        StringLiteralExpression::new(this)
    }
}
