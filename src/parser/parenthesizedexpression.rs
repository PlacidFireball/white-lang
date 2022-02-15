use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ParenthesizedExpression {
    expr: dyn Expression,
}
impl Expression for ParenthesizedExpression {
    fn evaluate(&self) -> Box<dyn Any> {
        self.expr.evaluate()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn debug(&self) -> String {
        String::from("(") + &*self.expr.debug() + &*String::from(")")
    }

    fn get_type(&self) -> String {
        String::from("ParenthesizedExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
