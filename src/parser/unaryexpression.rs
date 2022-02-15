use crate::parser::Expression;
use std::any::Any;

pub(crate) struct UnaryExpression {
    operator: String,
    expr: Box<dyn Expression>
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

    fn validate(&self) {
        todo!()
    }

    fn debug(&self) -> String {
        let mut builder = self.operator.clone();
        builder += &*self.expr.debug();
        builder
    }

    fn get_type(&self) -> String {
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
            expr
        }
    }
}
