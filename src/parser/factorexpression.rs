use crate::parser::{Expression, Sided};
use std::any::Any;

pub struct FactorExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
}
impl Expression for FactorExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        todo!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_type(&self) -> String {
        String::from("FactorExpression")
    }
}
impl Sided for FactorExpression {
    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
impl FactorExpression {
    pub(crate) fn new(lhs: Box<dyn Expression>, operator: String, rhs: Box<dyn Expression>) -> FactorExpression {
        FactorExpression {
            lhs,
            operator,
            rhs
        }
    }
}
