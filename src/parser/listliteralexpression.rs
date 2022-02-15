use crate::parser::Expression;
use std::any::Any;

pub(crate) struct ListLiteralExpression {
    exprs: Vec<Box<dyn Expression>>
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

    fn validate(&self) {
        todo!()
    }

    fn debug(&self) -> String {
        let mut builder = String::from("[");
        for x in 0..self.exprs.len() {
            builder += &*self.exprs[x].debug();
            if x != self.exprs.len()-1 {
                builder += ", "
            }
        }
        builder += "]";
        builder
    }

    fn get_type(&self) -> String {
        String::from("ListLiteralExpression")
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        todo!()
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        todo!()
    }
}
impl ListLiteralExpression {
    pub fn new() -> ListLiteralExpression {
        ListLiteralExpression {
            exprs: vec![]
        }
    }

    pub fn add_expr(&mut self, expr: Box<dyn Expression>) {
        self.exprs.push(expr);
    }
}
