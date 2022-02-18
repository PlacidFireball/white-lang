use crate::parser::whitetypes::Type;
use crate::parser::{Expression, ParserErrorType, SymbolTable};
use std::any::Any;

pub(crate) struct ComparisonExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}
impl Expression for ComparisonExpression {
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
        todo!()
    }

    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("ComparisonExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
impl ComparisonExpression {
    pub fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> ComparisonExpression {
        ComparisonExpression {
            lhs,
            operator,
            rhs,
            errors: vec![],
        }
    }
}
