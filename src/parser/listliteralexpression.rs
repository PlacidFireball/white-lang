use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ListLiteralExpression {}
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

    fn debug(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        todo!()
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &Box::new(Self)
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &Box::new(Self)
    }
}
