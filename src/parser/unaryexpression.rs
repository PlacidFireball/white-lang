use crate::parser::whitetypes::Type;
use crate::parser::{Expression, ParserErrorType, SymbolTable};
use std::any::Any;

pub(crate) struct UnaryExpression {
    operator: String,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
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

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
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
