use crate::parser::{Expression, ParserErrorType};
use std::any::Any;
use crate::parser::whitetypes::{ListType, Type};

pub(crate) struct ListLiteralExpression {
    exprs: Vec<Box<dyn Expression>>,
    inferred_type: Type,
    errors: Vec<ParserErrorType>
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

    fn validate(&mut self) {
        if self.exprs.is_empty() {

            return;
        }
        self.inferred_type = self.exprs[0].get_white_type();
        for expr in &mut self.exprs {
            expr.validate();
            if expr.get_white_type() != self.inferred_type {
                self.errors.push(ParserErrorType::MismatchedTypes);
            }
        }
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

    fn get_white_type(&self) -> Type {
        self.inferred_type.clone()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
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
            exprs: vec![],
            inferred_type: Type::Initialized,
            errors: vec![]
        }
    }
    pub fn add_expr(&mut self, expr: Box<dyn Expression>) {
        self.exprs.push(expr);
    }
}
