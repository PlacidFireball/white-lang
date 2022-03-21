use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct FactorExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
}

impl ToAny for FactorExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
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

    fn validate(&mut self, st: &SymbolTable) {
        self.lhs.validate(st);
        self.rhs.validate(st);
    }

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
        String::from("FactorExpression")
    }
}
impl FactorExpression {
    pub(crate) fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> FactorExpression {
        FactorExpression {
            lhs,
            operator,
            rhs,
            errors: vec![],
        }
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
