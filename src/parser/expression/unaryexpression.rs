use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct UnaryExpression {
    operator: String,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for UnaryExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for UnaryExpression {
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
        if self.operator == "not" && self.expr.get_white_type() == Type::Integer {
            self.errors.push(ParserErrorType::BadOperator);
        }
        if self.operator == "-" && self.expr.get_white_type() == Type::Boolean {
            self.errors.push(ParserErrorType::BadOperator);
        }
    }

    fn debug(&self) -> String {
        let mut builder = self.operator.clone();
        builder += &*self.expr.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        self.expr.get_white_type()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}
impl UnaryExpression {
    pub(crate) fn new(operator: String, expr: Box<dyn Expression>) -> UnaryExpression {
        UnaryExpression {
            operator,
            expr,
            errors: vec![],
        }
    }
}
