use crate::parser::{Expression, ParserErrorType};
use std::any::Any;
use crate::parser::whitetypes::Type;

pub(crate) struct EqualityExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    errors: Vec<ParserErrorType>
}
impl Expression for EqualityExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self) {
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
        todo!()
    }

    fn get_expr_type(&self) -> String {
        String::from("EqualityExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
impl EqualityExpression {
    pub fn new(lhs: Box<dyn Expression>, operator: String, rhs: Box<dyn Expression>) -> EqualityExpression {
        EqualityExpression {
            lhs,
            operator,
            rhs,
            errors: vec![]
        }
    }
}
