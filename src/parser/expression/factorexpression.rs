use crate::config::{WhiteLangFloat, WhiteLangInt};
use crate::parser::parser_traits::{add_parser_error, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::ParserErrorType;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct FactorExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
    is_mult: bool,
}

impl ToAny for FactorExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FactorExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        if let Some(lhs_f64) = lhs_eval.downcast_ref::<WhiteLangFloat>() {
            if let Some(rhs_f64) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if self.is_mult {
                    return Box::new(lhs_f64 * rhs_f64);
                }
                return Box::new(lhs_f64 / rhs_f64);
            }
            if let Some(rhs_isize) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if self.is_mult {
                    return Box::new(lhs_f64 * *rhs_isize as WhiteLangFloat);
                }
                return Box::new(lhs_f64 / *rhs_isize as WhiteLangFloat);
            }
        }
        if let Some(lhs_isize) = lhs_eval.downcast_ref::<WhiteLangInt>() {
            if let Some(rhs_f64) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if self.is_mult {
                    return Box::new(*lhs_isize as WhiteLangFloat * rhs_f64);
                }
                return Box::new(*lhs_isize as WhiteLangFloat / rhs_f64);
            }
            if let Some(rhs_isize) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if self.is_mult {
                    return Box::new(lhs_isize * rhs_isize);
                }
                return Box::new(lhs_isize / rhs_isize);
            }
        }
        unreachable!()
    }

    fn compile(&self) {
        todo!()
    }

    fn transpile(&self) {
        todo!()
    }

    fn validate(&mut self, st: &SymbolTable) {
        self.lhs.validate(st);
        self.rhs.validate(st);
        if self.lhs.get_white_type().ne(&Type::Float)
            && self.lhs.get_white_type().ne(&Type::Integer)
        {
            add_parser_error(
                ParserErrorType::IncompatibleTypes,
                format!(
                    "You cannot multiply/divide two non number types. lhs: {:?} rhs: {:?}",
                    self.lhs, self.rhs
                ),
            );
        }
        if self.rhs.get_white_type().ne(&Type::Float)
            && self.rhs.get_white_type().ne(&Type::Integer)
        {
            add_parser_error(
                ParserErrorType::IncompatibleTypes,
                format!(
                    "You cannot multiply/divide two non number types. lhs: {:?} rhs: {:?}",
                    self.lhs, self.rhs
                ),
            );
        }
    }

    fn debug(&self) -> String {
        let mut builder = String::new();
        builder = builder + &*self.lhs.debug() + " ";
        builder = builder + &*self.operator + " ";
        builder = builder + &*self.rhs.debug();
        builder
    }

    fn get_white_type(&self) -> Type {
        if self.lhs.get_white_type() == Type::Float || self.lhs.get_white_type() == Type::Float {
            return Type::Float;
        }
        Type::Integer
    }

    fn get_expr_type(&self) -> String {
        String::from("FactorExpression")
    }
}
#[allow(dead_code)]
impl FactorExpression {
    pub(crate) fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> FactorExpression {
        FactorExpression {
            lhs,
            operator: operator.clone(),
            rhs,
            is_mult: operator.contains("*"),
        }
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }
    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
