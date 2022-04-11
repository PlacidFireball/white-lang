use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;

use crate::parser::expression::floatliteralexpression::FloatLiteralExpression;
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct AdditiveExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
    is_add: bool,
}

impl ToAny for AdditiveExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for AdditiveExpression {
    fn evaluate(&self, runtime: &Runtime) -> Box<dyn Any> {
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        // while eval.is<expression> -> eval //TODO
        if lhs_eval.is::<f64>() {}
        if lhs_eval.is::<i64>() {}
        unimplemented!();
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
        // TODO: Figure out what kind of types we should allow +/- to be called on
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
            operator: operator.clone(),
            rhs,
            errors: vec![],
            is_add: operator.contains("+"),
        }
    }

    pub fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }
    pub fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
