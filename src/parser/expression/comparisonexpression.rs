use crate::config::*;
use crate::javascript::JavaScript;
use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use crate::LOGGER;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct ComparisonExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    is_greater: bool,
    is_less: bool,
    is_equal: bool,
}

impl ToAny for ComparisonExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for ComparisonExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        if let Some(lhs_f64) = lhs_eval.downcast_ref::<WhiteLangFloat>() {
            if let Some(rhs_f64) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                return if self.is_greater && self.is_equal {
                    Box::new(lhs_f64 >= rhs_f64)
                } else if self.is_less && self.is_equal {
                    Box::new(lhs_f64 <= rhs_f64)
                } else if self.is_greater {
                    Box::new(lhs_f64 > rhs_f64)
                } else {
                    Box::new(lhs_f64 < rhs_f64)
                };
            }
            if let Some(rhs_isize) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                return if self.is_greater && self.is_equal {
                    Box::new(*lhs_f64 >= *rhs_isize as WhiteLangFloat)
                } else if self.is_less && self.is_equal {
                    Box::new(*lhs_f64 <= *rhs_isize as WhiteLangFloat)
                } else if self.is_greater {
                    Box::new(*lhs_f64 > *rhs_isize as WhiteLangFloat)
                } else {
                    Box::new(*lhs_f64 < *rhs_isize as WhiteLangFloat)
                };
            }
        }
        if let Some(lhs_isize) = lhs_eval.downcast_ref::<isize>() {
            if let Some(rhs_f64) = rhs_eval.downcast_ref::<f64>() {
                return if self.is_greater && self.is_equal {
                    Box::new(*lhs_isize as f64 >= *rhs_f64)
                } else if self.is_less && self.is_equal {
                    Box::new(*lhs_isize as f64 <= *rhs_f64)
                } else if self.is_greater {
                    Box::new(*lhs_isize as f64 > *rhs_f64)
                } else {
                    Box::new((*lhs_isize as f64) < *rhs_f64)
                };
            }
            if let Some(rhs_isize) = rhs_eval.downcast_ref::<isize>() {
                return if self.is_greater && self.is_equal {
                    Box::new(*lhs_isize >= *rhs_isize)
                } else if self.is_less && self.is_equal {
                    Box::new(*lhs_isize <= *rhs_isize)
                } else if self.is_greater {
                    Box::new(*lhs_isize > *rhs_isize)
                } else {
                    Box::new(*lhs_isize < *rhs_isize)
                };
            }
        }
        unreachable!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self, javascript: &mut JavaScript) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        self.lhs.validate(st);
        self.rhs.validate(st);
        if self.lhs.get_white_type() != self.rhs.get_white_type() {
            add_parser_error(
                ParserErrorType::MismatchedTypes(
                    self.lhs.get_white_type(),
                    self.rhs.get_white_type(),
                ),
                format!(
                    "Types must be comparable: lhs: {:?} rhs {:?}",
                    self.lhs.get_white_type(),
                    self.rhs.get_white_type()
                ),
            );
        }
        LOGGER.info(format!(
            "Validated a comparison expression. lhs: {:?} rhs: {:?}",
            self.lhs, self.rhs
        ));
        // TODO: When developing std, comparable might be a cool thing to implement
    }

    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        Type::Boolean
    }

    fn get_expr_type(&self) -> String {
        String::from("ComparisonExpression")
    }
}
#[allow(dead_code)]
impl ComparisonExpression {
    pub fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> ComparisonExpression {
        ComparisonExpression {
            lhs,
            operator: operator.clone(),
            rhs,
            is_greater: operator.contains(">"),
            is_less: operator.contains("<"),
            is_equal: operator.contains("="),
        }
    }
    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }
    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
