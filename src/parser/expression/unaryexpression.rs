use crate::config::{WhiteLangFloat, WhiteLangInt};
use crate::parser::parser_traits::{Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone)]
pub(crate) struct UnaryExpression {
    operator: String,
    expr: Box<dyn Expression>,
    errors: Vec<ParserErrorType>,
    is_not: bool,
}

impl ToAny for UnaryExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, runtime: &Runtime) -> Box<dyn Any> {
        if self.is_not {
            let eval = self.expr.evaluate(runtime);
            if let Some(eval_bool) = eval.downcast_ref::<bool>() {
                return Box::new(!eval_bool);
            }
        } else {
            let eval = self.expr.evaluate(runtime);
            if let Some(eval_isize) = eval.downcast_ref::<WhiteLangInt>() {
                return Box::new(-1 * eval_isize);
            }
            if let Some(eval_f64) = eval.downcast_ref::<WhiteLangFloat>() {
                return Box::new(-1 as WhiteLangFloat * eval_f64);
            }
        }
        unimplemented!()
    }

    fn compile(&self) -> String {
        todo!()
    }

    fn transpile(&self) -> String {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        if self.operator == "not"
            && (self.expr.get_white_type() == Type::Integer
                || self.expr.get_white_type() == Type::Float)
        {
            self.errors.push(ParserErrorType::BadOperator);
        }
        if self.operator == "-" && self.expr.get_white_type() == Type::Boolean {
            self.errors.push(ParserErrorType::BadOperator);
        }
    }

    fn debug(&self) -> String {
        let mut builder = self.operator.clone();
        builder += &*self.expr.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        self.expr.get_white_type()
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_expr_type(&self) -> String {
        todo!()
    }
}
impl UnaryExpression {
    pub(crate) fn new(operator: String, expr: Box<dyn Expression>) -> UnaryExpression {
        UnaryExpression {
            operator: operator.clone(),
            expr,
            errors: vec![],
            is_not: operator.contains("not"),
        }
    }
}
