use crate::javascript::JavaScript;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;

use crate::config::{WhiteLangFloat, WhiteLangInt};
use crate::parser::parser_traits::{add_parser_error, try_print_output, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::runtime::Runtime;
use std::any::Any;
#[derive(Clone, Debug)]
pub(crate) struct AdditiveExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    is_add: bool,
}

impl ToAny for AdditiveExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }

    fn to_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Expression for AdditiveExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        // Lots of syntax here, but basically what we are doing is getting whatever
        // the runtime knows, and doing + or - on it based on the operator
        // currently WhiteLangFloat and WhiteLangInt are hard coded to f64 and isize
        // respectively but that will be system dependent in the future, I just
        // abstracted it to make my life easier in the future
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        // debug info
        if self.is_add {
            crate::LOGGER.debug(
                format!(
                    "[ADD_EXPR]: {} + {}",
                    try_print_output(&lhs_eval),
                    try_print_output(&rhs_eval)
                ),
                crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get()),
            );
        } else {
            crate::LOGGER.debug(
                format!(
                    "[ADD_EXPR]: {} - {}",
                    try_print_output(&lhs_eval),
                    try_print_output(&rhs_eval)
                ),
                crate::RUNTIME_DEBUG_LOGGING_ENABLED.with(|cell| !cell.get()),
            );
        }
        if let Some(lhs_float) = lhs_eval.downcast_ref::<WhiteLangFloat>() {
            if let Some(rhs_float) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if self.is_add {
                    return Box::new(lhs_float + rhs_float);
                }
                return Box::new(lhs_float - rhs_float);
            }
            if let Some(rhs_int) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if self.is_add {
                    return Box::new(lhs_float - *rhs_int as WhiteLangFloat);
                }
                return Box::new(lhs_float - *rhs_int as WhiteLangFloat);
            }
        }
        if let Some(lhs_int) = lhs_eval.downcast_ref::<WhiteLangInt>() {
            if let Some(rhs_float) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if self.is_add {
                    return Box::new(*lhs_int as WhiteLangFloat + rhs_float);
                }
                return Box::new(*lhs_int as WhiteLangFloat - rhs_float);
            }
            if let Some(rhs_int) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if self.is_add {
                    return Box::new(lhs_int + rhs_int);
                }
                return Box::new(lhs_int - rhs_int);
            }
        }
        unreachable!()
    }

    fn compile(&self) {}

    fn transpile(&self, javascript: &mut JavaScript) {
        self.lhs.transpile(javascript);
        javascript.append_no_tabs(format!(" {} ", self.operator));
        self.rhs.transpile(javascript);
    }

    fn validate(&mut self, st: &mut SymbolTable) {
        // I have decided that I am not going to allow + being called on strings,
        // gonna do some other op, perhaps a concatenate function in std
        self.lhs.validate(st);
        self.rhs.validate(st);
        if self.lhs.get_white_type() != Type::Integer && self.lhs.get_white_type() != Type::Float {
            add_parser_error(
                ParserErrorType::IncompatibleTypes(
                    self.lhs.get_white_type(),
                    self.rhs.get_white_type(),
                ),
                format!(
                    "You cannot add/subtract two non number types. lhs: {:?} rhs: {:?}",
                    self.lhs, self.rhs
                ),
            );
        }
        if self.rhs.get_white_type() != Type::Integer && self.rhs.get_white_type() != Type::Float {
            add_parser_error(
                ParserErrorType::IncompatibleTypes(
                    self.lhs.get_white_type(),
                    self.rhs.get_white_type(),
                ),
                format!(
                    "You cannot add/subtract two non number types. lhs: {:?} rhs: {:?}",
                    self.lhs, self.rhs
                ),
            );
        }
    }

    // gives debug information of the expression without having to downcast it
    fn debug(&self) -> String {
        format!(
            "{} {} {}",
            self.lhs.debug(),
            self.operator,
            self.rhs.debug()
        )
    }

    fn get_white_type(&self) -> Type {
        if self.rhs.get_white_type() == Type::Float || self.lhs.get_white_type() == Type::Float {
            return Type::Float;
        }
        Type::Integer
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
            is_add: operator.contains("+"),
        }
    }

    #[allow(dead_code)]
    pub fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }
    #[allow(dead_code)]
    pub fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
