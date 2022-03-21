use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct ListLiteralExpression {
    exprs: Vec<Box<dyn Expression>>,
    inferred_type: Type,
    errors: Vec<ParserErrorType>,
}

impl ToAny for ListLiteralExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ListLiteralExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        if self.exprs.is_empty() {
            return;
        }
        self.inferred_type = self.exprs[0].get_white_type();
        for expr in &mut self.exprs {
            expr.validate(st);
            if expr.get_white_type() != self.inferred_type {
                self.errors.push(ParserErrorType::MismatchedTypes);
            }
        }
    }

    fn debug(&self) -> String {
        let mut builder = String::from("[");
        for x in 0..self.exprs.len() {
            builder += &*self.exprs[x].debug();
            if x != self.exprs.len() - 1 {
                builder += ", "
            }
        }
        builder += "]";
        builder
    }

    fn get_white_type(&self) -> Type {
        self.inferred_type.get_list_type()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("ListLiteralExpression")
    }
}
impl ListLiteralExpression {
    pub fn new() -> ListLiteralExpression {
        ListLiteralExpression {
            exprs: vec![],
            inferred_type: Type::Initialized,
            errors: vec![],
        }
    }
    pub fn add_expr(&mut self, expr: Box<dyn Expression>) {
        self.exprs.push(expr);
    }
}
