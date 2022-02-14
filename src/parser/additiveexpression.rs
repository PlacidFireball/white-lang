use crate::parser::{Expression, Sided};
use std::any::Any;

pub(crate) struct AdditiveExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
}
impl Expression for AdditiveExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        String::from("")
    }

    fn transpile(&self) -> String {
        String::from("")
    }

    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_type(&self) -> String {
        String::from("AdditiveExpression")
    }
}
impl Sided for AdditiveExpression {
    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
impl AdditiveExpression {
    pub fn new(lhs: Box<dyn Expression>, operator: String, rhs: Box<dyn Expression>) -> AdditiveExpression {
        AdditiveExpression {
            lhs,
            operator,
            rhs
        }
    }
}
