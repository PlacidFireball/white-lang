use crate::parser::whitetypes::{ListType, Type};
use crate::parser::ParserErrorType;

use crate::parser_traits::{Expression, ToAny};
use crate::symbol_table::SymbolTable;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct AdditiveExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for AdditiveExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
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

    fn validate(&mut self, st: &SymbolTable) {
        self.lhs.validate(st);
        self.rhs.validate(st);
    }

    // gives debug information of the expression without having to downcast it
    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        todo!()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        String::from("AdditiveExpression")
    }
}
impl AdditiveExpression {
    pub fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> AdditiveExpression {
        AdditiveExpression {
            lhs,
            operator,
            rhs,
            errors: vec![],
        }
    }

    pub fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }
    pub fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
