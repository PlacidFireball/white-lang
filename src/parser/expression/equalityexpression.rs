use crate::config::{WhiteLangBool, WhiteLangFloat, WhiteLangInt, WhiteLangString};
use crate::parser::parser_traits::{try_print_output, Expression, ToAny};
use crate::parser::symbol_table::SymbolTable;
use crate::parser::whitetypes::Type;
use crate::parser::whitetypes::Type::Null;
use crate::runtime::Runtime;
use std::any::Any;

#[derive(Clone, Debug)]
pub(crate) struct EqualityExpression {
    lhs: Box<dyn Expression>,
    operator: String,
    rhs: Box<dyn Expression>,
}

impl ToAny for EqualityExpression {
    fn to_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for EqualityExpression {
    fn evaluate(&self, runtime: &mut Runtime) -> Box<dyn Any> {
        let lhs_eval = self.lhs.evaluate(runtime);
        let rhs_eval = self.rhs.evaluate(runtime);
        let is_equal = self.operator.contains("==");
        if is_equal {
            println!(
                "[EQ_EXPR]: {} == {}",
                try_print_output(&lhs_eval),
                try_print_output(&rhs_eval)
            );
        } else {
            println!(
                "[EQ_EXPR]: {} != {}",
                try_print_output(&lhs_eval),
                try_print_output(&rhs_eval)
            );
        }
        // handle null == null
        if self.lhs.get_white_type() == self.rhs.get_white_type()
            && self.lhs.get_white_type() == Null
        {
            return Box::new(true);
        }
        // handle some type == null or null == some type
        if (self.lhs.get_white_type() != self.rhs.get_white_type())
            && (self.rhs.get_white_type() == Null || self.lhs.get_white_type() == Null)
        {
            if is_equal {
                return Box::new(false);
            }
            return Box::new(true);
        }

        // lhs : int, rhs : int | float
        if let Some(lhs_int) = lhs_eval.downcast_ref::<WhiteLangInt>() {
            if let Some(rhs_int) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if is_equal {
                    return Box::new(lhs_int == rhs_int);
                }
                return Box::new(lhs_int != rhs_int);
            } else if let Some(rhs_float) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if is_equal {
                    return Box::new(*lhs_int as WhiteLangFloat == *rhs_float);
                }
                return Box::new(*lhs_int as WhiteLangFloat != *rhs_float);
            }
        }
        // lhs : string, rhs : string
        if let Some(lhs_str) = lhs_eval.downcast_ref::<WhiteLangString>() {
            if let Some(rhs_str) = rhs_eval.downcast_ref::<WhiteLangString>() {
                if is_equal {
                    return Box::new(lhs_str.eq(rhs_str));
                }
                return Box::new(lhs_str.ne(rhs_str));
            }
        }
        // lhs : bool, rhs : bool
        if let Some(lhs_bool) = lhs_eval.downcast_ref::<WhiteLangBool>() {
            if let Some(rhs_bool) = rhs_eval.downcast_ref::<WhiteLangBool>() {
                if is_equal {
                    return Box::new(lhs_bool == rhs_bool);
                }
                return Box::new(lhs_bool != rhs_bool);
            }
        }
        // lhs : float, rhs: float | int
        if let Some(lhs_float) = lhs_eval.downcast_ref::<WhiteLangFloat>() {
            if let Some(rhs_float) = rhs_eval.downcast_ref::<WhiteLangFloat>() {
                if is_equal {
                    return Box::new(lhs_float == rhs_float);
                }
                return Box::new(lhs_float != rhs_float);
            } else if let Some(rhs_int) = rhs_eval.downcast_ref::<WhiteLangInt>() {
                if is_equal {
                    return Box::new(*lhs_float == *rhs_int as WhiteLangFloat);
                }
                return Box::new(*lhs_float != *rhs_int as WhiteLangFloat);
            }
        }
        Box::new(false)
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
        String::from("EqualityExpression")
    }
}
#[allow(dead_code)]
impl EqualityExpression {
    pub fn new(
        lhs: Box<dyn Expression>,
        operator: String,
        rhs: Box<dyn Expression>,
    ) -> EqualityExpression {
        EqualityExpression {
            lhs,
            operator,
            rhs,
        }
    }

    fn get_lhs(&self) -> &Box<dyn Expression> {
        &self.lhs
    }

    fn get_rhs(&self) -> &Box<dyn Expression> {
        &self.rhs
    }
}
